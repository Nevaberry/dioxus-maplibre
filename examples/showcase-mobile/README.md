# MapLibre Mobile Lab

A phone-first, installable Dioxus web app for testing `dioxus-maplibre` with
touch input and unreliable connectivity. It is a mobile PWA because the library's
JavaScript bridge intentionally runs only on the `wasm32` web target; native
non-WASM targets remain safe no-ops.

## What it demonstrates

- Four live scenes: clustered Helsinki places, queried 3D buildings,
  Matterhorn terrain/fog, and a Tokyo heatmap.
- A feature lab for runtime layer visibility, symbols, terrain, fog, camera
  pitch/bearing/roll, projections, and style changes.
- Mouse and touch event ordering: hover, press, release, and tap.
- Region-pack download, pause/resume/cancel, Cache Storage persistence, and a
  service-worker app shell that reloads without a network connection.
- Local MapLibre 6 ESM/CSS, local styles and GeoJSON, and a local Terrarium DEM
  tile. Offline mode does not request a remote basemap.

## Run it

```bash
cd examples/showcase-mobile
dx serve --web --port 8081 --locked
```

Open `http://localhost:8081`. Use the browser's **Install app** action to run it
standalone on a phone or desktop. A production bundle is built with:

```bash
dx bundle --web --release --debug-symbols=false --out-dir dist --locked
```

Service workers require a secure origin. `localhost` works for desktop testing;
serve the production bundle over HTTPS when opening it from another phone or
computer. The app shows a warning and disables new pack downloads on an
insecure origin instead of claiming that the pack is offline-ready.

## Verify offline behavior

1. Open **Offline**, select Matterhorn or Archipelago, and download the pack.
2. Choose **Use offline mode**.
3. Disable the browser/network and reload.
4. The app shell, MapLibre engine, local style, GeoJSON, and downloaded pack
   continue to work.

The automated equivalent is `cd e2e && bun run test:mobile`.

## Vendored assets

MapLibre GL JS 6.0.0 is vendored under `public/vendor/maplibre/` so the renderer
itself is available offline. Its upstream license is included beside the files.
The bundled terrain tile comes from the AWS Open Data Terrarium elevation tile
set and is attributed in the terrain source.
