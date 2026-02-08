# Codebase Map

> Updated: 2026-02-08

## System Overview

`Map` owns map lifecycle and JS event loop. It provides `Signal<Option<MapHandle>>` via context.

- Imperative API: app uses `MapHandle` methods.
- Declarative API: child components (`MapSource`, `MapLayer`, etc.) use `use_map_handle()`.
- JS bridge: `interop/*` modules generate JS snippets executed through `document::eval`.

## Directory Structure

```
src/
├── lib.rs
├── types.rs
├── events.rs
├── components/
│   ├── mod.rs
│   ├── map.rs
│   ├── context.rs
│   ├── event_dispatch.rs
│   └── declarative.rs
├── handle/
│   ├── mod.rs
│   ├── sources.rs
│   ├── layers.rs
│   ├── controls.rs
│   ├── markers.rs
│   ├── popups.rs
│   ├── navigation.rs
│   ├── feature_state.rs
│   ├── images.rs
│   ├── style.rs
│   ├── terrain_atmosphere.rs
│   ├── padding.rs
│   ├── layer_events.rs
│   ├── queries.rs
│   ├── getters.rs
│   └── escape_hatch.rs
├── options/
│   ├── mod.rs
│   ├── controls.rs
│   ├── sources.rs
│   ├── layers.rs
│   ├── overlays.rs
│   ├── navigation.rs
│   ├── atmosphere.rs
│   └── queries.rs
└── interop/
    ├── mod.rs
    ├── core.rs
    ├── js_escape.rs
    ├── lifecycle.rs
    ├── sources.rs
    ├── layers.rs
    ├── controls.rs
    ├── markers.rs
    ├── popups.rs
    ├── navigation.rs
    ├── feature_state.rs
    ├── images.rs
    ├── style.rs
    ├── terrain_atmosphere.rs
    ├── padding.rs
    ├── getters.rs
    └── queries.rs
```

## Component Layer

- `Map`: map container, init/destroy, style/throttle live updates.
- `use_map_handle()`: access handle from context.
- Declarative components:
  - `MapSource`
  - `MapLayer`
  - `MapMarker`
  - `MapPopup`
  - `MapControl`

## Event Flow

1. `interop::lifecycle::init_map_js` registers map/marker/layer listeners.
2. JS emits tagged JSON events (`type` field).
3. `Map` parses into `MapEvent`.
4. `event_dispatch` routes typed events to user handlers and sets context handle on `Ready`.

## Handle Layer

`MapHandle` is split by domain; each file implements one concern. All methods route to `interop::*_js` builders.

## Interop Layer

- `core.rs`: map id generation and map lookup snippet.
- `js_escape.rs`: shared escaping for JS string/template literals.
- domain files: one JS-bridge concern per file.

## Testing

- `tests/types.rs`: core types.
- `tests/events.rs`: event model and `MapEvent` envelope.
- `tests/options.rs`: option serialization and builders.
- `examples/showcase`: end-to-end manual behavior validation.
- `e2e/tests`: Playwright smoke/interaction checks.
