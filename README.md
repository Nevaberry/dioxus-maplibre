# dioxus-maplibre

[![Crates.io](https://img.shields.io/crates/v/dioxus-maplibre.svg)](https://crates.io/crates/dioxus-maplibre)
[![License](https://img.shields.io/crates/l/dioxus-maplibre.svg)](https://github.com/Nevaberry/dioxus-maplibre#license)

A [MapLibre GL JS](https://maplibre.org/) wrapper for [Dioxus](https://dioxuslabs.com/) 0.7+.

## Installation

```bash
cargo add dioxus-maplibre
```

You also need to include the MapLibre GL JS library in your HTML:

```html
<link href="https://unpkg.com/maplibre-gl/dist/maplibre-gl.css" rel="stylesheet" />
<script src="https://unpkg.com/maplibre-gl/dist/maplibre-gl.js"></script>
```

## Usage

```rust
use dioxus::prelude::*;
use dioxus_maplibre::{Map, Marker, Popup, LatLng};

fn App() -> Element {
    rsx! {
        Map {
            style: "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json",
            center: LatLng::new(60.17, 24.94),
            zoom: 10.0,

            Marker {
                position: LatLng::new(60.17, 24.94),
                Popup { content: "Hello!" }
            }
        }
    }
}
```

## Components

- **Map** - The main map container
- **Marker** - Add markers to the map
- **Popup** - Attach popups to markers

## Types

- `LatLng` - Geographic coordinates (latitude/longitude)
- `MapPosition` - Map center + zoom level
- `Bounds` - Bounding box (southwest/northeast corners)
- `Point` - Screen pixel coordinates

## Events

- `MapClickEvent` - Fired when the map is clicked
- `MarkerClickEvent` - Fired when a marker is clicked
- `MarkerHoverEvent` - Fired when hovering over a marker
- `MapMoveEvent` - Fired when the map moves

## Functions

- `fly_to(map_id, latlng, zoom)` - Animate the map to a location
- `pan_by(x, y)` - Pan the map by pixel offset

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
