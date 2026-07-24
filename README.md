# dioxus-maplibre

[![Crates.io](https://img.shields.io/crates/v/dioxus-maplibre.svg)](https://crates.io/crates/dioxus-maplibre)
[![License](https://img.shields.io/crates/l/dioxus-maplibre.svg)](https://github.com/Nevaberry/dioxus-maplibre#license)

A [MapLibre GL JS](https://maplibre.org/) 6 wrapper for [Dioxus](https://dioxuslabs.com/) 0.7. The crate combines typed Rust APIs for common operations with JSON passthroughs and scoped JavaScript evaluation for the full, evolving MapLibre surface.

Current baseline: Rust 1.97.1, Dioxus 0.7.9, and MapLibre GL JS 6.0.0. MapLibre 6 requires WebGL2 and is distributed as an ES module.

## Installation

```bash
cargo add dioxus-maplibre
```

Include MapLibre assets in your HTML:

```html
<link href="https://unpkg.com/maplibre-gl@6.0.0/dist/maplibre-gl.css" rel="stylesheet" />
<script type="module">
  import * as maplibregl from "https://unpkg.com/maplibre-gl@6.0.0/dist/maplibre-gl.mjs";
  globalThis.maplibregl = maplibregl;
</script>
```

## Usage

### Imperative (`MapHandle`)

```rust,ignore
use dioxus::prelude::*;
use dioxus_maplibre::{FlyToOptions, LatLng, Map, MapHandle};

fn App() -> Element {
    let mut map = use_signal(|| None::<MapHandle>);

    rsx! {
        Map {
            style: "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json",
            center: LatLng::new(60.17, 24.94),
            zoom: 10.0,
            on_ready: move |handle| map.set(Some(handle)),
        }
        button {
            onclick: move |_| {
                if let Some(handle) = map() {
                    handle.fly_to(FlyToOptions {
                        center: Some(LatLng::new(60.17, 24.94)),
                        zoom: Some(12.0),
                        ..Default::default()
                    });
                }
            },
            "Fly"
        }
    }
}
```

### Declarative Components

```rust,ignore
use dioxus::prelude::*;
use dioxus_maplibre::{
    LatLng, LayerOptions, Map, MapLayer, MapMarker, MapSource, MapSourceKind,
    GeoJsonSourceOptions,
};
use serde_json::json;

fn App() -> Element {
    rsx! {
        Map {
            MapSource {
                id: "points",
                source: MapSourceKind::GeoJson(GeoJsonSourceOptions {
                    data: json!({"type": "FeatureCollection", "features": []}),
                    ..Default::default()
                }),
                MapLayer {
                    options: LayerOptions::circle("point-layer", "points")
                        .paint(json!({"circle-radius": 5, "circle-color": "#3b82f6"})),
                }
            }

            MapMarker {
                id: "helsinki",
                position: LatLng::new(60.17, 24.94),
            }
        }
    }
}
```

## Public API

- `Map` root component
- Future-proof `MapOptions` plus typed camera, bounds, projection, and interaction APIs
- Typed pointer, camera, marker, layer hover/press/release/click, roll, error, and lifecycle callbacks
- `MapHandle` imperative API with style replay across style switches
- `use_map_handle()` context hook
- Declarative helpers: `MapSource`, `MapLayer`, `MapMarker`, `MapPopup`, `MapControl`
- All core source types: GeoJSON, vector (MVT/MLT), raster, raster DEM, image, video, canvas, and custom passthrough
- All ten style layer types: background, fill, line, symbol, raster, circle, fill extrusion, heatmap, hillshade, and color relief
- Navigation, geolocation, scale, fullscreen, attribution, globe, logo, and terrain controls
- Markers, popups, images, missing-image resolution, queries, feature/global state, terrain, sky/fog, globe, animation, and raw eval integrations
- Options, types, and events exported from the crate root

See [MapLibre feature coverage](docs/FEATURE_COVERAGE.md) for the API-to-showcase matrix and the boundary between MapLibre core and third-party plugins.

## Development

```bash
cargo fmt --check
cargo test --locked --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features
```

Run showcase app:

```bash
cd examples/showcase-web
dx serve --web --port 8080 --locked
```

Run the installable, offline-capable phone showcase:

```bash
cd examples/showcase-mobile
dx serve --web --port 8081 --locked
```

`showcase-mobile` is a mobile PWA on the `wasm32` web target. It vendors the
MapLibre 6 runtime, includes local basemaps/data/elevation, and demonstrates
touch press/release events plus downloadable offline packs.

See [CONTRIBUTING.md](CONTRIBUTING.md) for full setup and e2e workflow.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
