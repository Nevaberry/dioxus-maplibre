# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

dioxus-maplibre is a MapLibre GL JS wrapper for Dioxus 0.7+. It provides Rust components that interface with the MapLibre GL JS library via JavaScript interop.

**Stack**: Rust, Dioxus 0.7, WASM, MapLibre GL JS (CDN)

For detailed architecture diagrams and module documentation, see [docs/CODEBASE_MAP.md](docs/CODEBASE_MAP.md).

## Build & Development Commands

```bash
cargo build                 # Build the library
cargo test                  # Run unit tests
cargo check                 # Check compilation

# Run showcase app (for manual testing)
cd examples/showcase && dx serve --port 8080

# E2E tests with Playwright (requires Bun)
cd e2e && bun install && bun test
```

## Architecture

### JS Interop Pattern

The `interop/bridge.rs` module generates JavaScript code strings executed via `document::eval()`.

**Key globals:**
- `window.__dioxus_maplibre_maps[containerId]` - Map instance registry
- `window.__dioxus_maplibre_markers[mapId]` - Marker registry per map
- `window.__dioxus_maplibre_sources[mapId]` - Source registry per map
- `window.__dioxus_maplibre_layers[mapId]` - Layer registry per map
- `window.__dioxus_maplibre_sendEvent(json)` - Global event callback

**Why global sendEvent?** Markers are added via separate `document::eval()` calls, which create isolated contexts. `dioxus.send()` only works within the eval that created it. The global callback bridges this gap.

### Component Lifecycle

1. `Map` renders container div with generated UUID
2. `use_effect` spawns async init (polls for MapLibre GL JS, waits for container, creates map)
3. Event loop (`eval.recv()`) processes events and updates signals
4. When `is_ready` signal becomes true, child components render
5. Each child executes JS to add itself to the map

### ID Mismatch Handling (Hot-Reload)

Dioxus hot-reload can remount components with new UUIDs. The bridge handles this:
- Container finder falls back to any `div[id^="map_"][id$="_container"]`
- Marker lookup falls back to first available map in registry
- Map stored under both `actualContainerId` and original `map_id`

## Public API

```rust
// Components
Map, Marker, Popup, GeoJsonSource, CircleLayer

// Types
LatLng, MapPosition, Bounds, Point

// Events
MapClickEvent, MarkerClickEvent, MarkerHoverEvent, MapMoveEvent, LayerClickEvent, LayerHoverEvent

// Functions
fly_to(map_id, latlng, zoom)  // Animated pan
pan_by(x, y)                   // Instant pixel offset

// Context
MapContext { map_id, is_ready }
```

## Adding New Features

**New map interactions:**
1. Add JS generator function in `interop/bridge.rs`
2. Export from `interop/mod.rs` (pub(crate))
3. Add Rust wrapper in `components/map.rs` with `#[cfg(target_arch = "wasm32")]`
4. Export from `lib.rs`

**New event types:**
1. Add struct in `events.rs` with serde Deserialize
2. Add variant to `MapEvent` enum
3. Add match arm in map.rs event loop
4. Add optional callback prop to MapProps

**New components:**
1. Create file in `src/components/`
2. Export from `src/components/mod.rs`
3. Re-export from `src/lib.rs`

## Gotchas

- **Coordinate order**: MapLibre uses `[lng, lat]`, not `[lat, lng]`. Use `LatLng::to_array()`.
- **Layer ordering**: Layers must be added after their source. Wrap layers inside source components.
- **Popup content**: HTML string, not RSX. No reactivity in popups.
- **Feature IDs**: MapLibre feature state requires numeric IDs (`i64`). String IDs not supported.
- **Platform guards**: All JS interop must use `#[cfg(target_arch = "wasm32")]`.
