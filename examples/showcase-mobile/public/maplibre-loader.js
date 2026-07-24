// MapLibre GL JS 6 is ESM-only. dioxus-maplibre intentionally speaks to the
// historical global, so expose the vendored module namespace once it loads.
void import("/vendor/maplibre/maplibre-gl.mjs")
  .then((maplibregl) => {
    globalThis.maplibregl = maplibregl;
    globalThis.dispatchEvent(new CustomEvent("dioxus-maplibre:ready"));
  })
  .catch((error) => {
    console.error("[showcase-mobile] Failed to load vendored MapLibre GL JS", error);
  });
