import { readFile, exists } from "fs/promises";
import { join, extname } from "path";

const PORT = process.env.PORT ? parseInt(process.env.PORT) : 3000;
const ROOT_DIR = join(import.meta.dir, "..");
const ASSETS_DIR = join(ROOT_DIR, "assets");
const FIXTURES_DIR = join(ROOT_DIR, "fixtures");

const MIME_TYPES: Record<string, string> = {
  ".html": "text/html",
  ".css": "text/css",
  ".js": "text/javascript",
  ".ts": "text/javascript",
  ".json": "application/json",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
  ".geojson": "application/geo+json",
  ".mvt": "application/vnd.mapbox-vector-tile",
  ".pbf": "application/x-protobuf",
  ".mp4": "video/mp4",
  ".webm": "video/webm",
};

/**
 * Transform style.json to replace local:// URLs with server URLs
 */
function transformStyle(style: any, baseUrl: string): any {
  const transformed = JSON.parse(JSON.stringify(style));

  // Transform sprite URL
  if (transformed.sprite && typeof transformed.sprite === "string") {
    transformed.sprite = transformLocalUrl(transformed.sprite, baseUrl);
  } else if (Array.isArray(transformed.sprite)) {
    transformed.sprite = transformed.sprite.map((s: any) => {
      if (typeof s === "string") {
        return transformLocalUrl(s, baseUrl);
      }
      if (s.url) {
        return { ...s, url: transformLocalUrl(s.url, baseUrl) };
      }
      return s;
    });
  }

  // Transform glyphs URL
  if (transformed.glyphs) {
    transformed.glyphs = transformLocalUrl(transformed.glyphs, baseUrl);
  }

  // Transform source URLs
  if (transformed.sources) {
    for (const [sourceId, source] of Object.entries(transformed.sources) as [
      string,
      any
    ][]) {
      if (source.tiles) {
        source.tiles = source.tiles.map((tile: string) =>
          transformLocalUrl(tile, baseUrl)
        );
      }
      if (source.url) {
        source.url = transformLocalUrl(source.url, baseUrl);
      }
      if (source.data && typeof source.data === "string") {
        source.data = transformLocalUrl(source.data, baseUrl);
      }
      // Handle video sources
      if (source.urls) {
        source.urls = source.urls.map((url: string) =>
          transformLocalUrl(url, baseUrl)
        );
      }
    }
  }

  return transformed;
}

/**
 * Transform a local:// URL to a server URL
 */
function transformLocalUrl(url: string, baseUrl: string): string {
  if (url.startsWith("local://")) {
    const localPath = url.replace("local://", "");
    return `${baseUrl}/assets/${localPath}`;
  }
  return url;
}

/**
 * Handle /fixture/:category/:test route for rendering fixture test maps
 */
async function handleFixtureRequest(
  category: string,
  test: string,
  baseUrl: string
): Promise<Response> {
  const stylePath = join(FIXTURES_DIR, category, test, "style.json");

  if (!(await exists(stylePath))) {
    return new Response(`Fixture not found: ${category}/${test}`, {
      status: 404,
    });
  }

  const styleContent = await readFile(stylePath, "utf-8");
  const style = JSON.parse(styleContent);

  // Transform style to replace local:// URLs
  const transformedStyle = transformStyle(style, baseUrl);

  // Extract test metadata
  const testMeta = style.metadata?.test || {};
  const width = testMeta.width || 512;
  const height = testMeta.height || 512;
  const pixelRatio = testMeta.pixelRatio || 1;
  const collisionDebug = testMeta.collisionDebug || false;
  const showOverdrawInspector = testMeta.showOverdrawInspector || false;
  const showPadding = testMeta.showPadding || false;
  const crossSourceCollisions = testMeta.crossSourceCollisions !== false;
  const fadeDuration = testMeta.fadeDuration ?? 0;
  const localIdeographFontFamily = testMeta.localIdeographFontFamily;
  const operations = testMeta.operations || [];
  const addFakeCanvas = testMeta.addFakeCanvas;

  // Generate fake canvas script if needed
  const fakeCanvasScript = addFakeCanvas
    ? generateFakeCanvasScript(addFakeCanvas, category, test, baseUrl)
    : "";

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
  ${addFakeCanvas ? `<canvas id="${addFakeCanvas.id}" style="display:none;"></canvas>` : ""}
  <div id="map"></div>
  <script src="https://unpkg.com/maplibre-gl@5.0.0/dist/maplibre-gl.js"></script>
  <script>
    (async function() {
      try {
      ${fakeCanvasScript}

      const style = ${JSON.stringify(transformedStyle)};

      const map = new maplibregl.Map({
        container: 'map',
        style: style,
        center: style.center || [0, 0],
        zoom: style.zoom !== undefined ? style.zoom : 0,
        bearing: style.bearing || 0,
        pitch: style.pitch || 0,
        roll: style.roll || 0,
        attributionControl: false,
        preserveDrawingBuffer: true,
        fadeDuration: ${fadeDuration},
        crossSourceCollisions: ${crossSourceCollisions},
        interactive: false,
        pixelRatio: ${pixelRatio},
        ${localIdeographFontFamily ? `localIdeographFontFamily: "${localIdeographFontFamily}",` : ""}
      });

      ${collisionDebug ? "map.showCollisionBoxes = true;" : ""}
      ${showOverdrawInspector ? "map.showOverdrawInspector = true;" : ""}
      ${showPadding ? "map.showPadding = true;" : ""}

      // Set global state if present in style (use setGlobalStateProperty for each key)
      ${style.state ? `
      try {
        const globalState = ${JSON.stringify(style.state)};
        for (const [key, value] of Object.entries(globalState)) {
          map.setGlobalStateProperty(key, value);
        }
      } catch (e) {
        console.warn('Failed to set global state:', e);
      }
      ` : ""}

      // Handle operations after map loads
      const operations = ${JSON.stringify(operations)};

      async function executeOperations() {
        for (const op of operations) {
          const [action, ...args] = op;
          try {
            await executeOperation(map, action, args);
          } catch (e) {
            console.error('Operation failed:', action, args, e);
          }
        }
      }

      function executeOperation(map, action, args) {
        return new Promise((resolve, reject) => {
          try {
            switch (action) {
              case 'wait':
                // wait operation has 3 modes:
                // 1. No args: wait for map to finish loading
                // 2. String arg: wait for specific event
                // 3. Numeric arg: sleep for N ms then trigger render
                if (args.length === 0 || args[0] === undefined) {
                  // Wait for map to finish loading
                  if (map.loaded()) {
                    resolve();
                  } else {
                    map.once('idle', resolve);
                  }
                } else if (typeof args[0] === 'string') {
                  // Wait for specific event
                  map.once(args[0], () => resolve());
                } else if (typeof args[0] === 'number') {
                  // Sleep for N ms then trigger render
                  setTimeout(() => {
                    map._render();
                    resolve();
                  }, args[0]);
                } else {
                  resolve();
                }
                break;
              case 'sleep':
                setTimeout(resolve, args[0]);
                break;
              case 'setCenter':
                map.setCenter(args[0]);
                resolve();
                break;
              case 'setZoom':
                map.setZoom(args[0]);
                resolve();
                break;
              case 'setBearing':
                map.setBearing(args[0]);
                resolve();
                break;
              case 'setPitch':
                map.setPitch(args[0]);
                resolve();
                break;
              case 'setFilter':
                map.setFilter(args[0], args[1]);
                resolve();
                break;
              case 'setLayoutProperty':
                map.setLayoutProperty(args[0], args[1], args[2]);
                resolve();
                break;
              case 'setPaintProperty':
                map.setPaintProperty(args[0], args[1], args[2]);
                resolve();
                break;
              case 'setGlobalState':
                // setGlobalState operation: args[0] is object with key-value pairs
                for (const [key, value] of Object.entries(args[0])) {
                  map.setGlobalStateProperty(key, value);
                }
                resolve();
                break;
              case 'setGlobalStateProperty':
                // setGlobalStateProperty operation: args[0] is key, args[1] is value
                map.setGlobalStateProperty(args[0], args[1]);
                resolve();
                break;
              case 'setFeatureState':
                map.setFeatureState(args[0], args[1]);
                resolve();
                break;
              case 'removeFeatureState':
                map.removeFeatureState(args[0], args[1]);
                resolve();
                break;
              case 'addLayer':
                map.addLayer(args[0], args[1]);
                resolve();
                break;
              case 'removeLayer':
                map.removeLayer(args[0]);
                resolve();
                break;
              case 'addSource':
                map.addSource(args[0], args[1]);
                resolve();
                break;
              case 'removeSource':
                map.removeSource(args[0]);
                resolve();
                break;
              case 'setStyle':
                map.setStyle(args[0]);
                map.once('idle', resolve);
                break;
              case 'setGlyphs':
                map.setGlyphs(args[0]);
                resolve();
                break;
              case 'setSprite':
                map.setSprite(args[0]);
                resolve();
                break;
              case 'addImage':
                const img = new Image();
                img.onload = () => {
                  map.addImage(args[0], img, args[2] || {});
                  resolve();
                };
                img.onerror = reject;
                img.src = args[1].replace('local://', '${baseUrl}/assets/');
                break;
              case 'removeImage':
                map.removeImage(args[0]);
                resolve();
                break;
              case 'updateImage':
                const updateImg = new Image();
                updateImg.onload = () => {
                  map.updateImage(args[0], updateImg);
                  resolve();
                };
                updateImg.onerror = reject;
                updateImg.src = args[1].replace('local://', '${baseUrl}/assets/');
                break;
              case 'setLight':
                map.setLight(args[0]);
                resolve();
                break;
              case 'setTerrain':
                map.setTerrain(args[0]);
                resolve();
                break;
              case 'setFog':
                map.setFog(args[0]);
                resolve();
                break;
              case 'setSky':
                map.setSky(args[0]);
                resolve();
                break;
              default:
                console.warn('Unknown operation:', action);
                resolve();
            }
          } catch (e) {
            reject(e);
          }
        });
      }

      map.on('load', async () => {
        window.__mapLoaded = true;
        if (operations.length > 0) {
          await executeOperations();
        }
      });

      map.on('idle', () => {
        window.__mapReady = true;
      });

      map.on('error', (e) => {
        console.error('Map error:', e);
        window.__mapError = e;
      });
      } catch (e) {
        console.error('Fixture initialization error:', e);
        window.__mapError = e;
        window.__mapReady = true; // Signal ready so tests don't hang
      }
    })();
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

/**
 * Generate script for fake canvas (used in canvas source tests)
 */
function generateFakeCanvasScript(
  config: { id: string; image: string },
  category: string,
  test: string,
  baseUrl: string
): string {
  // The image path is relative to the fixture directory
  const imagePath = config.image.startsWith("./")
    ? config.image.slice(2)
    : config.image;
  const imageUrl = `${baseUrl}/fixtures/${category}/${test}/${imagePath}`;

  return `
    // Load fake canvas image
    const fakeCanvas = document.getElementById('${config.id}');
    const img = new Image();
    await new Promise((resolve, reject) => {
      img.onload = () => {
        fakeCanvas.width = img.width;
        fakeCanvas.height = img.height;
        const ctx = fakeCanvas.getContext('2d');
        ctx.drawImage(img, 0, 0);
        resolve();
      };
      img.onerror = reject;
      img.src = '${imageUrl}';
    });
  `;
}

/**
 * Handle asset requests (tiles, sprites, glyphs, images)
 */
async function handleAssetRequest(assetPath: string): Promise<Response> {
  const filePath = join(ASSETS_DIR, assetPath);

  // Security: prevent directory traversal
  if (!filePath.startsWith(ASSETS_DIR)) {
    return new Response("Forbidden", { status: 403 });
  }

  if (!(await exists(filePath))) {
    return new Response(`Asset not found: ${assetPath}`, { status: 404 });
  }

  const ext = extname(filePath);
  const content = await readFile(filePath);
  const contentType = MIME_TYPES[ext] || "application/octet-stream";

  return new Response(content, {
    headers: {
      "Content-Type": contentType,
      "Cache-Control": "public, max-age=31536000",
      "Access-Control-Allow-Origin": "*",
    },
  });
}

/**
 * Handle fixture asset requests (images in fixture directories)
 */
async function handleFixtureAssetRequest(
  category: string,
  test: string,
  assetPath: string
): Promise<Response> {
  const filePath = join(FIXTURES_DIR, category, test, assetPath);

  // Security: prevent directory traversal
  if (!filePath.startsWith(FIXTURES_DIR)) {
    return new Response("Forbidden", { status: 403 });
  }

  if (!(await exists(filePath))) {
    return new Response(`Asset not found: ${assetPath}`, { status: 404 });
  }

  const ext = extname(filePath);
  const content = await readFile(filePath);
  const contentType = MIME_TYPES[ext] || "application/octet-stream";

  return new Response(content, {
    headers: {
      "Content-Type": contentType,
      "Cache-Control": "no-cache",
      "Access-Control-Allow-Origin": "*",
    },
  });
}

async function handleRequest(req: Request): Promise<Response> {
  const url = new URL(req.url);
  let pathname = url.pathname;
  const baseUrl = `${url.protocol}//${url.host}`;

  // Handle CORS preflight
  if (req.method === "OPTIONS") {
    return new Response(null, {
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Methods": "GET, OPTIONS",
        "Access-Control-Allow-Headers": "*",
      },
    });
  }

  // Handle fixture route: /fixture/:category/:test
  const fixtureMatch = pathname.match(/^\/fixture\/([^/]+)\/([^/]+)\/?$/);
  if (fixtureMatch) {
    const [, category, test] = fixtureMatch;
    return handleFixtureRequest(category, test, baseUrl);
  }

  // Handle fixture asset route: /fixtures/:category/:test/:asset
  const fixtureAssetMatch = pathname.match(
    /^\/fixtures\/([^/]+)\/([^/]+)\/(.+)$/
  );
  if (fixtureAssetMatch) {
    const [, category, test, assetPath] = fixtureAssetMatch;
    return handleFixtureAssetRequest(category, test, assetPath);
  }

  // Handle asset requests: /assets/*
  if (pathname.startsWith("/assets/")) {
    const assetPath = pathname.slice(8); // Remove "/assets/"
    return handleAssetRequest(assetPath);
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
    external: ["maplibre-gl"],
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

  Routes:
    /                         - Demo UI
    /fixture/:category/:test  - Render fixture
    /assets/*                 - Test assets (tiles, sprites, glyphs)
    /fixtures/:cat/:test/*    - Fixture-specific assets

  Press Ctrl+C to stop
`);
