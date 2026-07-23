# Repository Guide

## Project

`dioxus-maplibre` is a MapLibre GL JS 6 wrapper for Dioxus 0.7. The pinned baseline is Rust 1.97.1, Dioxus 0.7.9, MapLibre GL JS 6.0.0, Bun 1.3.14, and Playwright 1.61.1.

The API deliberately has three levels:

1. Typed Rust options and `MapHandle` methods for common MapLibre operations.
2. Transparent JSON wrappers such as `MapOptions`, `SourceOptions`, and `ControlOptions` for forward compatibility.
3. `eval` / `eval_async` for custom layers, protocols, plugins, and other JavaScript integrations.

See `docs/CODEBASE_MAP.md` and `docs/FEATURE_COVERAGE.md` before changing architecture or coverage claims.

## Verification

```bash
cargo fmt --all --check
cargo check --locked --all-targets --all-features
cargo check --locked --target wasm32-unknown-unknown --all-features
cargo test --locked --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features

cd examples/showcase
cargo clippy --all-targets -- -D warnings
cargo check --target wasm32-unknown-unknown
dx bundle --web --release --debug-symbols=false --out-dir dist --locked

cd ../../e2e
bun install --frozen-lockfile
bun run typecheck
bun run test --project=chromium
```

Use `bun run test`, not `bun test`; the latter invokes Bun's own test runner.

## Architecture

- `Map` owns the container, MapLibre instance, event loop, and `MapHandle` context.
- `MapHandle` is a cheap cloned map ID. Domain implementations live in `src/handle/`.
- JavaScript generators live by domain in `src/interop/` and execute through Dioxus `document::eval`.
- `MapSource`, `MapLayer`, `MapMarker`, `MapPopup`, and `MapControl` are lifecycle-aware declarative children. The imperative API remains available for dynamic use cases.
- Runtime sources, layers, images, terrain, and sky are tracked and replayed after style changes.
- Browser registries use `window.__dioxus_maplibre_*`; cleanup must remove both actual-container and logical-map aliases without deleting a live shared object early.

## Adding Features

- Put serializable public options in `src/options/` and re-export them from `src/lib.rs`.
- Put public operations in the appropriate `src/handle/` file and JS generation in the matching `src/interop/` file.
- Extend `src/events.rs`, lifecycle listeners, dispatch, and `MapProps` together for new events.
- Add native serialization/generator tests and a working showcase action. Add Playwright coverage for behavior that can fail only in a browser.
- Preserve the JSON/eval escape hatch when a MapLibre API is too dynamic to model usefully.

## Important Details

- MapLibre coordinates are `[longitude, latitude]`; `LatLng::new` takes `(latitude, longitude)` and serializes accordingly.
- MapLibre 6 is ESM-only and requires WebGL2.
- `FeatureId` supports both numbers and strings.
- Fog is part of MapLibre's `SkySpecification`; compatibility methods delegate to `setSky`, because there is no `map.setFog`.
- Paint, layout, filters, expressions, inline styles, and custom specifications remain `serde_json::Value` by design.
- All browser interop must stay safely no-op/`None` on non-wasm targets.
