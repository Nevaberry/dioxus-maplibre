/**
 * Generates an HTML page that renders a single MapLibre fixture.
 * Port of upstream run_render_tests.ts:255-796 getImageFromStyle().
 *
 * The page:
 * 1. Loads MapLibre GL JS from CDN
 * 2. Creates a map with exact fixture dimensions and test options
 * 3. Waits for 'load', runs operations, then sets window.__fixtureReady = true
 */

import { SERVER_PORT } from "./constants";
import { generateOperationsScript } from "./operations";

// Serve MapLibre from our local server cache to avoid CDN latency on every page load
const LOCAL_JS = `http://localhost:${SERVER_PORT}/cdn/maplibre-gl.js`;
const LOCAL_CSS = `http://localhost:${SERVER_PORT}/cdn/maplibre-gl.css`;

export interface FixtureTestData {
  id: string;
  width: number;
  height: number;
  pixelRatio: number;
  allowed: number;
  threshold: number;
  fadeDuration?: number;
  localIdeographFontFamily?: string | false;
  crossSourceCollisions?: boolean;
  maxPitch?: number;
  continuesRepaint?: boolean;
  debug?: boolean;
  showOverdrawInspector?: boolean;
  showPadding?: boolean;
  collisionDebug?: boolean;
  operations?: any[];
  addFakeCanvas?: { id: string; image: string };
  reportWidth?: number;
  reportHeight?: number;
}

export function generateFixturePage(
  style: any,
  testData: FixtureTestData,
): string {
  const { width, height } = testData;
  const operationsScript = generateOperationsScript();

  // Serialize the style and test data for injection into the page
  const styleJSON = JSON.stringify(style);
  const optionsJSON = JSON.stringify(testData);

  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Fixture: ${testData.id}</title>
  <link rel="icon" href="about:blank">
  <link rel="stylesheet" href="${LOCAL_CSS}">
  <style>
    body { margin: 0; padding: 0; }
    #map {
      box-sizing: content-box;
      width: ${width}px;
      height: ${height}px;
    }
  </style>
</head>
<body>
  <div id="map"></div>
  <script src="${LOCAL_JS}"></script>
  <script>
    window.__fixtureReady = false;
    window.__fixtureError = null;

    ${operationsScript}

    (async function() {
      try {
        const style = ${styleJSON};
        const options = ${optionsJSON};

        const map = new maplibregl.Map({
          container: 'map',
          style: style,
          interactive: false,
          attributionControl: false,
          maxPitch: options.maxPitch,
          pixelRatio: options.pixelRatio,
          canvasContextAttributes: { preserveDrawingBuffer: true, powerPreference: 'default' },
          fadeDuration: options.fadeDuration || 0,
          localIdeographFontFamily: options.localIdeographFontFamily || false,
          crossSourceCollisions: options.crossSourceCollisions !== undefined ? options.crossSourceCollisions : true,
          maxCanvasSize: [8192, 8192]
        });

        // Keep rendering by default (upstream behavior)
        map.repaint = options.continuesRepaint !== undefined ? options.continuesRepaint : true;

        if (options.debug) map.showTileBoundaries = true;
        if (options.showOverdrawInspector) map.showOverdrawInspector = true;
        if (options.showPadding) map.showPadding = true;

        await map.once('load');

        if (options.collisionDebug) {
          map.showCollisionBoxes = true;
          if (options.operations) {
            options.operations.push(['wait']);
          } else {
            options.operations = [['wait']];
          }
        }

        await applyOperations(options, map);

        window.__fixtureReady = true;
      } catch (err) {
        console.error('Fixture error:', err);
        window.__fixtureError = err.message || String(err);
        window.__fixtureReady = true;
      }
    })();
  </script>
</body>
</html>`;
}
