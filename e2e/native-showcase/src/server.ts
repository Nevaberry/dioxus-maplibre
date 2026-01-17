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

// Handle /fixture/:category/:test route for rendering fixture test maps
async function handleFixtureRequest(
  category: string,
  test: string
): Promise<Response> {
  const stylePath = join(ROOT_DIR, "fixtures", category, test, "style.json");

  if (!(await exists(stylePath))) {
    return new Response(`Fixture not found: ${category}/${test}`, {
      status: 404,
    });
  }

  const styleContent = await readFile(stylePath, "utf-8");
  const style = JSON.parse(styleContent);

  // Extract dimensions from metadata.test
  const width = style.metadata?.test?.width || 512;
  const height = style.metadata?.test?.height || 512;

  // Generate HTML page that renders the map at the exact size
  const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Fixture: ${category}/${test}</title>
  <link href="https://unpkg.com/maplibre-gl@5.0.0/dist/maplibre-gl.css" rel="stylesheet" />
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { background: transparent; }
    #map {
      width: ${width}px;
      height: ${height}px;
      background: transparent;
    }
    .maplibregl-ctrl-logo, .maplibregl-ctrl-attrib { display: none !important; }
    .maplibregl-canvas { background: transparent !important; }
  </style>
</head>
<body>
  <div id="map"></div>
  <script src="https://unpkg.com/maplibre-gl@5.0.0/dist/maplibre-gl.js"></script>
  <script>
    const style = ${styleContent};

    const map = new maplibregl.Map({
      container: 'map',
      style: style,
      center: style.center || [0, 0],
      zoom: style.zoom !== undefined ? style.zoom : 0,
      bearing: style.bearing || 0,
      pitch: style.pitch || 0,
      attributionControl: false,
      preserveDrawingBuffer: true,
      fadeDuration: 0,
      crossSourceCollisions: style.metadata?.test?.crossSourceCollisions !== false,
      interactive: false
    });

    map.on('idle', () => {
      window.__mapReady = true;
    });

    map.on('load', () => {
      // Signal that the map has loaded
      window.__mapLoaded = true;
    });
  </script>
</body>
</html>`;

  return new Response(html, {
    headers: {
      "Content-Type": "text/html",
      "Cache-Control": "no-cache",
    },
  });
}

async function handleRequest(req: Request): Promise<Response> {
  const url = new URL(req.url);
  let pathname = url.pathname;

  // Handle fixture route: /fixture/:category/:test
  const fixtureMatch = pathname.match(/^\/fixture\/([^/]+)\/([^/]+)\/?$/);
  if (fixtureMatch) {
    const [, category, test] = fixtureMatch;
    return handleFixtureRequest(category, test);
  }

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

Bun.serve({
  port: PORT,
  fetch: handleRequest,
});

console.log(`
  MapLibre Native Showcase
  ========================
  Server running at http://localhost:${PORT}

  Press Ctrl+C to stop
`);
