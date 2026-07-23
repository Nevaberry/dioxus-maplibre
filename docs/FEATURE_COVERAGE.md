# MapLibre Feature Coverage

This repository targets MapLibre GL JS 6.0.0 on Dioxus 0.7.9 and Rust 1.97.1. “Complete” means every MapLibre core feature area has a typed API, a future-proof JSON passthrough, or the scoped JavaScript bridge—and a representative working showcase route. It does not mean that third-party plugins are bundled into the crate.

MapLibre 6 requires WebGL2 and uses ES modules. The showcase loader pins both its JavaScript and CSS assets to 6.0.0.

## Core API matrix

| Area | Library coverage | Showcase |
|---|---|---|
| Map construction | Dedicated props plus `MapOptions` passthrough | Basic Map, Camera & State |
| Styles | URL and inline style, paint/layout/filter, light, sky, runtime-state replay | Style, Layers, Sky & Fog |
| Sources | GeoJSON, vector MVT/MLT, raster, raster DEM, image, video, canvas, generic/custom | GeoJSON & Clusters, Media Sources, Raster & Relief |
| Layers | Background, fill, line, symbol, raster, circle, fill extrusion, heatmap, hillshade, color relief | Vector Layers, Symbols, Heatmap, 3D Buildings, Raster & Relief |
| Markers and popups | Full option passthrough, drag/hover/click, DOM identity/classes, lifecycle cleanup | Markers, Popups, Declarative API |
| Controls | Navigation, geolocate, scale, fullscreen, attribution, globe, logo, terrain | Controls, Terrain, Projection & Globe |
| Camera and navigation | Jump/ease/fly/fit, pan/zoom/rotate, roll, elevation, FOV, constraints, padding | Navigation, Camera & State, Projection & Globe |
| Projections | Mercator, globe, vertical perspective, custom JSON, project/unproject | Projection & Globe, Camera & State |
| Interactions | Box zoom, double-click zoom, drag pan/rotate, keyboard, scroll zoom, touch pitch/zoom-rotate | Interaction, Camera & State |
| Events | Pointer, touch/camera, layer, marker, error, roll, projection, terrain, style, idle, drag/box zoom, WebGL context | Basic Map, Interaction, Events & Lifecycle |
| Feature data | Rendered/source queries, numeric/string IDs, feature/global state, GeoJSON diffs and clusters | Query, Interaction, Expressions, GeoJSON & Clusters |
| Images | Load/add/remove/list plus MapLibre 6 missing-style-image resolver | Images & Patterns, Stress |
| Terrain and atmosphere | DEM, terrain, skirt length, elevation, hillshade, color relief, sky/horizon/fog | Terrain, Raster & Relief, Sky & Fog |
| Expressions | Data-driven paint/layout/filter values, nested properties, global state | Expressions, Layers, Symbols |
| Animation | Camera, point/line GeoJSON updates, timed loops, repaint/stop controls | Animation, Navigation |
| Runtime integration | Typed eval result, custom source/layer/protocol/plugin bridge | Eval / Integrations |
| Reliability | Declarative cleanup, style replay, source/layer registries, hot reload, 1M+ point stress | Declarative API, Style, Stress |

## Current MapLibre 6 additions represented

- WebGL2/ESM loading.
- Camera roll options, setters, getters, navigation options, and roll lifecycle events.
- `setMissingStyleImageResolver` generated-image path.
- Globe and vertical-perspective projections.
- `terrainSkirtLength` construction passthrough.
- `fill-layer-opacity` and `line-layer-opacity` style properties.
- Nested GeoJSON properties and string feature IDs.
- Raster alpha premultiplication and source tile LOD controls.
- Color-relief layers and multidirectional hillshade.
- MLT vector source encoding passthrough.
- Global-state expressions.

## Plugin boundary

Three.js/Babylon custom 3D renderers, deck.gl, Terra Draw, Mapbox GL Draw, PMTiles protocols, RTL text, geocoders, and other ecosystem packages are not MapLibre core classes. They require their own JavaScript packages and policies. `MapHandle::eval`, `eval_async`, generic source options, and custom layer specifications are the supported integration boundary; the Eval route demonstrates it without forcing those dependencies on every crate user.

## Browser verification

`e2e/tests/showcase.spec.ts` loads every showcase route and asserts no JavaScript/page errors during startup. It also exercises declarative mounting, media source controls, all built-in control categories, terrain state, raster/relief switching, image resolution, coordinate projection, interactions, global state, lifecycle events, sky removal, runtime projection switching, and camera roll.
