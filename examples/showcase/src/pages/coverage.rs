use dioxus::prelude::*;

const CORE_AREAS: &[(&str, &str)] = &[
    (
        "Map construction",
        "MapOptions passthrough, WebGL2, lifecycle, resize",
    ),
    (
        "Styles",
        "URL/inline style, paint, layout, filter, light, sky, replay",
    ),
    (
        "Sources",
        "GeoJSON, vector MVT/MLT, raster, DEM, image, video, canvas, custom",
    ),
    (
        "Layers",
        "All 10 style layer types plus custom layers through eval",
    ),
    (
        "Markers & popups",
        "DOM options, drag/hover/click, occlusion, declarative lifecycle",
    ),
    (
        "Controls",
        "Navigation, geolocate, scale, fullscreen, attribution, globe, logo, terrain",
    ),
    (
        "Camera",
        "jump/ease/fly/fit, roll, elevation, FOV, padding, constraints",
    ),
    (
        "Projection",
        "Mercator, globe, vertical perspective, project/unproject",
    ),
    (
        "Interactions",
        "All eight built-in gesture handlers and cooperative gestures",
    ),
    (
        "Events",
        "Pointer, camera, layer, marker, lifecycle, roll, projection, terrain",
    ),
    (
        "Feature data",
        "Queries, numeric/string IDs, feature state, clusters, GeoJSON diffs",
    ),
    (
        "Images",
        "load/add/remove/list and MapLibre 6 missing-image resolver",
    ),
    (
        "Terrain & atmosphere",
        "DEM, terrain, relief, hillshade, sky, horizon, fog",
    ),
    (
        "Expressions",
        "Filters, data-driven styling, nested properties, global state",
    ),
    (
        "Animation",
        "Camera, point/line GeoJSON updates, timed loops, repaint and stop",
    ),
    (
        "Runtime integrations",
        "Custom JS, custom layers/protocols/plugins via scoped eval",
    ),
    (
        "Reliability",
        "Style replay, cleanup, hot reload, stress and E2E route sweep",
    ),
];

#[component]
pub fn Coverage() -> Element {
    rsx! {
        div { style: "height: 100%; overflow-y: auto; background: #0f172a; color: #e2e8f0; padding: 28px; box-sizing: border-box;",
            div { style: "max-width: 980px; margin: 0 auto;",
                h1 { style: "margin: 0 0 8px;", "MapLibre Feature Coverage" }
                p { style: "color: #94a3b8; margin: 0 0 20px;", "MapLibre GL JS 6.0.0 · Dioxus 0.7.9 · Rust 1.97.1" }
                p { "data-testid": "coverage-status", style: "padding: 12px; background: #14532d; border-radius: 8px;", "17 / 17 core feature areas represented in the showcase" }
                p { style: "color: #cbd5e1;", "‘Comprehensive’ covers MapLibre core. Third-party renderers and plugins (Three.js, deck.gl, Terra Draw, PMTiles, RTL text) remain integrations; the Eval page demonstrates the supported bridge for them." }
                div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 12px; margin-top: 20px;",
                    for (name, detail) in CORE_AREAS {
                        div { style: "padding: 14px; border: 1px solid #334155; border-radius: 8px; background: #111c31;",
                            div { style: "font-weight: 700; color: #60a5fa;", "✓ {name}" }
                            div { style: "margin-top: 5px; font-size: 13px; color: #cbd5e1;", "{detail}" }
                        }
                    }
                }
            }
        }
    }
}
