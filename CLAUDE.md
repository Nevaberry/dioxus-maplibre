# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

dioxus-maplibre is a MapLibre GL JS wrapper for Dioxus 0.7+. It provides Rust components that interface with the MapLibre GL JS library via JavaScript interop.

## Build & Development Commands

```bash
# Build the library
cargo build

# Run unit tests
cargo test

# Check compilation
cargo check

# Run showcase app (for manual testing)
cd examples/showcase && dx serve --port 8080

# E2E tests with Playwright (requires Bun)
cd e2e && bun install && bun test
```

## Architecture

### Module Structure

```
src/
├── lib.rs          # Public API exports
├── types.rs        # LatLng, MapPosition, Bounds, Point
├── events.rs       # MapClickEvent, MarkerClickEvent, MapMoveEvent
├── context.rs      # MapContext (shared between Map and children)
├── components/
│   ├── mod.rs      # Component exports
│   ├── map.rs      # Map component + fly_to/pan_by functions
│   ├── marker.rs   # Marker component
│   └── popup.rs    # Popup component
└── interop/
    ├── mod.rs      # Interop exports (pub(crate))
    └── bridge.rs   # JS code generation functions
```

### JS Interop Pattern

The `interop/bridge.rs` module generates JavaScript code strings that are executed via `document::eval()`.

**Key globals set by init:**
- `window.__dioxus_maplibre_maps[containerId]` - Map instance registry
- `window.__dioxus_maplibre_markers[mapId]` - Marker registry per map
- `window.__dioxus_maplibre_sendEvent(json)` - Global event callback

**Why global sendEvent?** Markers are added via separate `document::eval()` calls, which create isolated contexts. `dioxus.send()` only works within the eval that created it. The global callback bridges this gap.

### Component Lifecycle

1. `Map` component renders a container div with generated UUID
2. `use_effect` spawns async init that:
   - Waits for MapLibre GL JS to load (polling)
   - Waits for container element (with fallback finder for hot-reload)
   - Creates map instance and stores in global registry
   - Sets up event listeners that call `dioxus.send()`
3. Event loop (`eval.recv()`) processes events and updates signals
4. When `is_ready` signal becomes true, child `Marker` components render
5. Each `Marker` executes JS to add itself to the map

### ID Mismatch Handling

Dioxus hot-reload can remount components with new UUIDs. The bridge handles this:
- Container finder falls back to any `div[id^="map_"][id$="_container"]`
- Marker lookup falls back to first available map in registry
- Map stored under both `actualContainerId` and original `map_id`

## Public API

```rust
// Components
Map, Marker, Popup

// Types
LatLng, MapPosition, Bounds, Point

// Events
MapClickEvent, MarkerClickEvent, MarkerHoverEvent, MapMoveEvent

// Functions
fly_to(map_id, latlng, zoom)  // Animated pan
pan_by(x, y)                   // Instant pixel offset

// Context
MapContext { map_id, is_ready }
```

## Adding New Features

When adding new map interactions:

1. Add JS generator function in `interop/bridge.rs`
2. Export from `interop/mod.rs` (pub(crate))
3. Add Rust wrapper in `components/map.rs` with `#[cfg(target_arch = "wasm32")]`
4. Export from `lib.rs`

For new event types:
1. Add struct in `events.rs` with serde Deserialize
2. Add match arm in map.rs event loop
3. Add optional callback prop to MapProps
