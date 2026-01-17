import { readFile, exists } from "fs/promises";
import { join, extname } from "path";

const PORT = process.env.PORT ? parseInt(process.env.PORT) : 3000;
const ROOT_DIR = join(import.meta.dir, "..");

const MIME_TYPES: Record<string, string> = {
  ".html": "text/html",
  ".css": "text/css",
  ".js": "text/javascript",
  ".ts": "text/javascript",
  ".json": "application/json",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
  ".geojson": "application/geo+json",
};

async function handleRequest(req: Request): Promise<Response> {
  const url = new URL(req.url);
  let pathname = url.pathname;

  // Default to index.html
  if (pathname === "/") {
    pathname = "/index.html";
  }

  const filePath = join(ROOT_DIR, pathname);

  // Security: prevent directory traversal
  if (!filePath.startsWith(ROOT_DIR)) {
    return new Response("Forbidden", { status: 403 });
  }

  const fileExists = await exists(filePath);
  if (!fileExists) {
    // Try with .ts extension for module imports
    const tsPath = filePath.replace(/\.js$/, ".ts");
    if (await exists(tsPath)) {
      return serveTypeScript(tsPath);
    }
    return new Response("Not found", { status: 404 });
  }

  const ext = extname(filePath);

  // Transpile TypeScript on the fly
  if (ext === ".ts") {
    return serveTypeScript(filePath);
  }

  const content = await readFile(filePath);
  const contentType = MIME_TYPES[ext] || "application/octet-stream";

  return new Response(content, {
    headers: {
      "Content-Type": contentType,
      "Cache-Control": "no-cache",
    },
  });
}

async function serveTypeScript(filePath: string): Promise<Response> {
  const result = await Bun.build({
    entrypoints: [filePath],
    target: "browser",
    format: "esm",
    minify: false,
    sourcemap: "inline",
    external: ["maplibre-gl"], // Use CDN version
  });

  if (!result.success) {
    console.error("Build failed:", result.logs);
    return new Response(`Build error: ${result.logs.join("\n")}`, {
      status: 500,
      headers: { "Content-Type": "text/plain" },
    });
  }

  const output = await result.outputs[0].text();

  return new Response(output, {
    headers: {
      "Content-Type": "text/javascript",
      "Cache-Control": "no-cache",
    },
  });
}

const server = Bun.serve({
  port: PORT,
  fetch: handleRequest,
});

console.log(`
  MapLibre Native Showcase
  ========================
  Server running at http://localhost:${PORT}

  Press Ctrl+C to stop
`);
