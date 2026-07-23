// MapLibre GL JS 6 is ESM-only. The Rust bridge expects its namespace on the
// historical global, so load the module and expose it there.
void import("https://unpkg.com/maplibre-gl@6.0.0/dist/maplibre-gl.mjs")
  .then((maplibregl) => {
    globalThis.maplibregl = maplibregl;
  })
  .catch((error) => {
    console.error("[dioxus-maplibre] Failed to load MapLibre GL JS", error);
  });
