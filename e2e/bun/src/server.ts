/**
 * Fixture server using Bun.serve().
 * Serves fixture pages, assets, and manifest.
 * Also caches MapLibre CDN files locally for fast page loads.
 */

import { readFileSync, writeFileSync, existsSync, mkdirSync } from "fs";
import { join, extname, dirname } from "path";
import {
  SERVER_PORT,
  MAPLIBRE_JS_CDN,
  MAPLIBRE_CSS_CDN,
  DEFAULT_WIDTH,
  DEFAULT_HEIGHT,
  DEFAULT_PIXEL_RATIO,
  DEFAULT_ALLOWED,
  DEFAULT_THRESHOLD,
} from "./constants";
import { localizeURLs } from "./localize-urls";
import { generateFixturePage } from "./fixture-page";
import { generateReportPage } from "./report-page";

const BUN_DIR = dirname(import.meta.dir); // e2e/bun/
const FIXTURES_DIR = join(BUN_DIR, "fixtures");
const RESULTS_DIR = join(BUN_DIR, "results");
const ASSETS_DIR = join(BUN_DIR, "fixtures", "assets");
const CACHE_DIR = join(BUN_DIR, "fixtures", ".cache");
// Fallback: if assets symlink doesn't exist, try submodule directly
const SUBMODULE_ASSETS = join(
  dirname(BUN_DIR),
  "maplibre-gl-js/test/integration/assets",
);

function getAssetsDir(): string {
  if (existsSync(ASSETS_DIR)) return ASSETS_DIR;
  if (existsSync(SUBMODULE_ASSETS)) return SUBMODULE_ASSETS;
  throw new Error(`Assets directory not found at ${ASSETS_DIR} or ${SUBMODULE_ASSETS}`);
}

const MIME_TYPES: Record<string, string> = {
  ".html": "text/html",
  ".js": "application/javascript",
  ".json": "application/json",
  ".css": "text/css",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".gif": "image/gif",
  ".svg": "image/svg+xml",
  ".pbf": "application/x-protobuf",
  ".mvt": "application/vnd.mapbox-vector-tile",
  ".webp": "image/webp",
  ".mp4": "video/mp4",
};

function corsHeaders(): Record<string, string> {
  return {
    "Access-Control-Allow-Origin": "*",
    "Access-Control-Allow-Methods": "GET, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type",
  };
}

function serveFile(filePath: string): Response {
  try {
    const file = Bun.file(filePath);
    const ext = extname(filePath);
    const contentType = MIME_TYPES[ext] || "application/octet-stream";
    return new Response(file, {
      headers: {
        ...corsHeaders(),
        "Content-Type": contentType,
        "Cache-Control": "public, max-age=3600",
      },
    });
  } catch {
    return new Response("Not Found", { status: 404, headers: corsHeaders() });
  }
}

function handleFixture(category: string, test: string): Response {
  const fixtureDir = join(FIXTURES_DIR, category, test);
  const stylePath = join(fixtureDir, "style.json");

  if (!existsSync(stylePath)) {
    return new Response(`Fixture not found: ${category}/${test}`, {
      status: 404,
      headers: corsHeaders(),
    });
  }

  const styleRaw = readFileSync(stylePath, "utf8");
  let style: any;
  try {
    style = JSON.parse(styleRaw);
  } catch {
    return new Response("Invalid style.json", {
      status: 500,
      headers: corsHeaders(),
    });
  }

  // Apply URL localization
  const assetsDir = getAssetsDir();
  localizeURLs(style, SERVER_PORT, assetsDir);

  // Extract test data with defaults
  const testData = {
    id: `${category}/${test}`,
    width: DEFAULT_WIDTH,
    height: DEFAULT_HEIGHT,
    pixelRatio: DEFAULT_PIXEL_RATIO,
    allowed: DEFAULT_ALLOWED,
    threshold: DEFAULT_THRESHOLD,
    ...style.metadata?.test,
  };

  const html = generateFixturePage(style, testData);
  return new Response(html, {
    headers: {
      ...corsHeaders(),
      "Content-Type": "text/html; charset=utf-8",
      "Cache-Control": "no-cache",
    },
  });
}

// Pre-cache MapLibre CDN files at startup
async function ensureCdnCache() {
  mkdirSync(CACHE_DIR, { recursive: true });

  const files = [
    { url: MAPLIBRE_JS_CDN, name: "maplibre-gl.js" },
    { url: MAPLIBRE_CSS_CDN, name: "maplibre-gl.css" },
  ];

  for (const { url, name } of files) {
    const cachePath = join(CACHE_DIR, name);
    if (!existsSync(cachePath)) {
      console.log(`Downloading ${name} from CDN...`);
      const resp = await fetch(url);
      if (!resp.ok) throw new Error(`Failed to fetch ${url}: ${resp.status}`);
      writeFileSync(cachePath, Buffer.from(await resp.arrayBuffer()));
      console.log(`Cached ${name} (${(Bun.file(cachePath).size / 1024).toFixed(0)} KB)`);
    }
  }
}

await ensureCdnCache();

const server = Bun.serve({
  port: SERVER_PORT,
  fetch(req) {
    const url = new URL(req.url);
    const pathname = url.pathname;

    // CORS preflight
    if (req.method === "OPTIONS") {
      return new Response(null, { status: 204, headers: corsHeaders() });
    }

    // Health check
    if (pathname === "/health") {
      return new Response("ok", { headers: corsHeaders() });
    }

    // Report viewer
    if (pathname === "/report") {
      return new Response(generateReportPage(), {
        headers: {
          ...corsHeaders(),
          "Content-Type": "text/html; charset=utf-8",
          "Cache-Control": "no-cache",
        },
      });
    }

    // Results files: /results/...
    if (pathname.startsWith("/results/")) {
      const relPath = pathname.slice("/results/".length);
      const filePath = join(RESULTS_DIR, decodeURIComponent(relPath));
      return serveFile(filePath);
    }

    // Manifest
    if (pathname === "/manifest.json") {
      const manifestPath = join(FIXTURES_DIR, "manifest.json");
      if (!existsSync(manifestPath)) {
        return new Response("Manifest not found. Run: bun run scripts/copy-fixtures.ts", {
          status: 404,
          headers: corsHeaders(),
        });
      }
      return serveFile(manifestPath);
    }

    // Fixture page: /fixture/{category}/{test}
    const fixtureMatch = pathname.match(/^\/fixture\/([^/]+)\/(.+)$/);
    if (fixtureMatch) {
      return handleFixture(
        decodeURIComponent(fixtureMatch[1]),
        decodeURIComponent(fixtureMatch[2]),
      );
    }

    // Assets: /assets/...
    if (pathname.startsWith("/assets/")) {
      const assetPath = pathname.slice("/assets/".length);
      const assetsDir = getAssetsDir();
      const filePath = join(assetsDir, assetPath);
      return serveFile(filePath);
    }

    // Fixture files: /fixtures/...
    if (pathname.startsWith("/fixtures/")) {
      const relPath = pathname.slice("/fixtures/".length);
      const filePath = join(FIXTURES_DIR, relPath);
      return serveFile(filePath);
    }

    // Cached CDN files: /cdn/maplibre-gl.js, /cdn/maplibre-gl.css
    if (pathname.startsWith("/cdn/")) {
      const name = pathname.slice("/cdn/".length);
      const cachePath = join(CACHE_DIR, name);
      if (existsSync(cachePath)) {
        return serveFile(cachePath);
      }
    }

    // sparse204 — returns 204 for sparse-tileset tests
    if (pathname.startsWith("/sparse204/")) {
      return new Response(null, { status: 204, headers: corsHeaders() });
    }

    return new Response("Not Found", { status: 404, headers: corsHeaders() });
  },
});

console.log(`Fixture server running on http://localhost:${server.port}`);
