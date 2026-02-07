# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

dioxus-maplibre is a MapLibre GL JS wrapper for Dioxus 0.7+. It provides a `Map` component and `MapHandle` API that covers the full MapLibre GL JS surface.

**Stack**: Rust, Dioxus 0.7, WASM, MapLibre GL JS (CDN)

For detailed architecture diagrams and module documentation, see [docs/CODEBASE_MAP.md](docs/CODEBASE_MAP.md).

## Build & Development Commands

```bash
cargo build                 # Build the library
cargo test                  # Run unit tests (55 tests)
cargo check                 # Check compilation

# Run showcase app (for manual testing)
cd examples/showcase && dx serve --port 8080

# E2E tests with Playwright
cd e2e && npm install && npx playwright test
```

## Architecture

### MapHandle-centric API

All map operations go through `MapHandle`, a lightweight `Clone` wrapper around a map ID string. Users receive it via the `on_ready` callback and store it in a signal.

```rust
Map {
    on_ready: move |map: MapHandle| {
        map.add_geojson_source("points", opts);
        map.add_layer(LayerOptions::circle("dots", "points").paint(json!({...})));
        map.on_layer_click("dots");
        map_handle.set(Some(map));
    },
}
```

**No child components for map objects** — everything (sources, layers, markers, controls) is added via MapHandle methods, not as child components.

### JS Interop Pattern

`interop/bridge.rs` generates JavaScript code strings executed via `document::eval()`.

**Key globals:**
- `window.__dioxus_maplibre_maps[containerId]` — Map instance registry
- `window.__dioxus_maplibre_markers[mapId]` — Marker registry per map
- `window.__dioxus_maplibre_sendEvent(json)` — Global event callback

**Why global sendEvent?** Each `document::eval()` creates an isolated JS context with its own `dioxus.send()`. Markers, layers, and other objects added via separate evals need a shared callback to route events back to the map's event channel.

### Component Lifecycle

1. `Map` renders container div with generated UUID
2. `use_effect` spawns async init (polls for MapLibre GL JS, waits for container, creates map)
3. Event loop (`eval.recv()`) processes events from JS
4. On `"ready"` event: creates `MapHandle`, calls `on_ready`
5. User adds map objects via MapHandle methods

### ID Mismatch Handling (Hot-Reload)

Dioxus hot-reload can remount components with new UUIDs. The bridge handles this:
- Container finder falls back to any `div[id^="map_"][id$="_container"]`
- Map operations fall back to first available map in registry
- Map stored under both `actualContainerId` and original `map_id`

### Fire-and-Forget vs Async

- **Fire-and-forget**: `spawn(async { let _ = document::eval(&js).await; })` — for add/set/remove operations
- **Async getters**: `document::eval(&js).join::<T>().await` — for `get_zoom()`, `get_center()`, etc.

### Platform Guards

All JS interop is gated with `#[cfg(target_arch = "wasm32")]` with no-op stubs for native targets. MapHandle methods silently no-op on non-wasm. Async getters return `None`.

## Public API

```rust
// Component
Map

// Handle (all map operations)
MapHandle
  // Sources: add_geojson_source, add_vector_source, add_raster_source,
  //          add_raster_dem_source, add_image_source, update_geojson_source, remove_source
  // Layers:  add_layer, remove_layer, set_paint_property, set_layout_property, set_filter
  // Controls: add_navigation_control, add_geolocate_control, add_scale_control,
  //           add_fullscreen_control, add_attribution_control
  // Markers: add_marker, remove_marker, update_marker_position
  // Popups:  add_popup, remove_popup
  // Nav:     fly_to, ease_to, jump_to, fit_bounds, pan_to, pan_by,
  //          zoom_to, zoom_in, zoom_out, rotate_to, set_pitch, reset_north
  // Feature: set_feature_state, remove_feature_state
  // Images:  load_image, remove_image
  // Style:   set_style
  // Terrain: set_terrain, remove_terrain, set_sky, remove_sky
  // Events:  on_layer_click, on_layer_hover
  // Getters: get_zoom, get_center, get_bearing, get_pitch, get_bounds (async)
  // Escape:  eval, eval_async::<T>

// Types
LatLng, MapPosition, Bounds, Point, Padding

// Options (serde → camelCase JSON)
GeoJsonSourceOptions, VectorSourceOptions, RasterSourceOptions,
RasterDemSourceOptions, ImageSourceOptions,
LayerOptions (builder), MarkerOptions, PopupOptions,
FlyToOptions, EaseToOptions, JumpToOptions, FitBoundsOptions,
TerrainOptions, SkyOptions, FeatureIdentifier, ControlPosition

// Events
MapClickEvent, MapDblClickEvent, MapContextMenuEvent,
MarkerClickEvent, MarkerHoverEvent,
MapMoveEvent, MapZoomEvent, MapRotateEvent, MapPitchEvent,
LayerClickEvent, LayerHoverEvent
```

## Adding New Features

**New map operations:**
1. Add JS generator function in `interop/bridge.rs`
2. Add method to `MapHandle` in `handle.rs`
3. Re-export new types from `lib.rs`

**New event types:**
1. Add struct in `events.rs` with serde Deserialize
2. Add variant to `MapEvent` enum
3. Add JS listener in `init_map_js()` (bridge.rs)
4. Add match arm in `map.rs` event loop
5. Add optional callback prop to `MapProps`

**New option types:**
1. Add struct in `options.rs` with `#[serde(rename_all = "camelCase")]`
2. Re-export from `lib.rs`
3. Add serialization test in `tests/options.rs`

## Module Map

```
src/
├── lib.rs          # Public API re-exports
├── types.rs        # LatLng, MapPosition, Bounds, Point
├── events.rs       # All event types + MapEvent enum
├── options.rs      # All option/builder types (serde → JSON)
├── handle.rs       # MapHandle with all methods
├── components/
│   ├── mod.rs      # Only exports Map
│   └── map.rs      # Map component, props, event loop
└── interop/
    ├── mod.rs      # Re-exports bridge
    └── bridge.rs   # ~50 JS generator functions
```

## Gotchas

- **Coordinate order**: MapLibre uses `[lng, lat]`, not `[lat, lng]`. Use `LatLng::to_array()`.
- **Paint/layout as Value**: Use `serde_json::json!({})` — MapLibre's style spec is too large to type.
- **Layer events separate from layers**: Call `map.on_layer_click("id")` after `map.add_layer(...)`.
- **Popup content**: HTML string, not RSX. No reactivity in popups.
- **Feature IDs**: MapLibre feature state requires numeric IDs (`i64`). String IDs not supported.
- **Platform guards**: All JS interop uses `#[cfg(target_arch = "wasm32")]`. No-op on native.
- **Children are HTML overlays**: Map children are HTML divs, not map objects.
