// MapLibre version must match the submodule at e2e/maplibre-gl-js/
export const MAPLIBRE_VERSION = "5.17.0";

export const MAPLIBRE_JS_CDN = `https://unpkg.com/maplibre-gl@${MAPLIBRE_VERSION}/dist/maplibre-gl.js`;
export const MAPLIBRE_CSS_CDN = `https://unpkg.com/maplibre-gl@${MAPLIBRE_VERSION}/dist/maplibre-gl.css`;

export const SERVER_PORT = 3900;

// Upstream defaults from run_render_tests.ts:219-227
export const DEFAULT_WIDTH = 512;
export const DEFAULT_HEIGHT = 512;
export const DEFAULT_PIXEL_RATIO = 1;
export const DEFAULT_ALLOWED = 0.00025;
export const DEFAULT_THRESHOLD = 0.1285;

// Categories that crash or produce unreliable results in headless Chrome with SwiftShader.
// These use GPU features (compute shaders, half-float textures, advanced blending) that
// SwiftShader doesn't support faithfully.
export const SKIP_PREFIXES = [
  "hillshade",
  "heatmap",
  "icon",
  "symbol",
  "text",
  "line",
  "canvas",
  "custom-layer",
  "terrain",
  "sky",
  "projection",
  "globe",
  "video",
  "color-relief",
  "debug",
  "real-world",
  "mlt",
  "satellite",
  "bright",
  "collator",
  "is-supported-script",
  "raster",
  "regressions",
  "fill-extrusion",
  "runtime-styling",
  "remove-feature-state",
  "feature-state",
  "geojson",
  "high-pitch",
  "within",
  "distance",
  "sparse-tileset",
  "tms",
  "pixel-ratio",
];

// Refresh the browser context every N fixtures to recover WebGL state
export const PAGE_REFRESH_INTERVAL = 50;
