# dioxus-maplibre

[![Crates.io](https://img.shields.io/crates/v/dioxus-maplibre.svg)](https://crates.io/crates/dioxus-maplibre)
[![License](https://img.shields.io/crates/l/dioxus-maplibre.svg)](https://github.com/Nevaberry/dioxus-maplibre#license)

A [MapLibre GL JS](https://maplibre.org/) wrapper for [Dioxus](https://dioxuslabs.com/) 0.7+.

## Installation

```bash
cargo add dioxus-maplibre
```

Include MapLibre assets in your HTML:

```html
<link href="https://unpkg.com/maplibre-gl/dist/maplibre-gl.css" rel="stylesheet" />
<script src="https://unpkg.com/maplibre-gl/dist/maplibre-gl.js"></script>
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
- `MapHandle` imperative API
- `use_map_handle()` context hook
- Declarative helpers: `MapSource`, `MapLayer`, `MapMarker`, `MapPopup`, `MapControl`
- Options/types/events exported from crate root

## Development

```bash
cargo test
cargo check
```

Run showcase app:

```bash
cd examples/showcase
dx serve --port 8080
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for full setup and e2e workflow.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
