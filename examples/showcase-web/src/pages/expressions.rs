use dioxus::prelude::*;
use dioxus_maplibre::{GeoJsonSourceOptions, LatLng, LayerOptions, Map, MapHandle};
use serde_json::json;

#[component]
pub fn Expressions() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut selected = use_signal(|| "all".to_string());
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 11.5,
                    on_ready: move |handle: MapHandle| {
                        handle.add_geojson_source("expression-points", GeoJsonSourceOptions {
                            data: json!({
                                "type": "FeatureCollection",
                                "features": [
                                    {"type":"Feature","id":"tram-1","geometry":{"type":"Point","coordinates":[24.94,60.17]},"properties":{"name":"Tram","meta":{"category":"rail"},"score":90}},
                                    {"type":"Feature","id":"ferry-1","geometry":{"type":"Point","coordinates":[24.98,60.16]},"properties":{"name":"Ferry","meta":{"category":"water"},"score":70}},
                                    {"type":"Feature","id":"bike-1","geometry":{"type":"Point","coordinates":[24.91,60.18]},"properties":{"name":"Bike","meta":{"category":"active"},"score":50}}
                                ]
                            }),
                            ..Default::default()
                        });
                        handle.add_layer(LayerOptions::circle("expression-circles", "expression-points")
                            .filter(json!([
                                "case",
                                ["==", ["to-string", ["global-state", "category"]], "all"],
                                true,
                                ["==", ["get", "category", ["get", "meta"]], ["global-state", "category"]]
                            ]))
                            .paint(json!({
                                "circle-radius": ["interpolate", ["linear"], ["get", "score"], 0, 8, 100, 24],
                                "circle-color": ["match", ["get", "category", ["get", "meta"]],
                                    "rail", "#ef4444", "water", "#3b82f6", "active", "#22c55e", "#a855f7"],
                                "circle-stroke-color": "#ffffff",
                                "circle-stroke-width": 2
                            }))
                        );
                        handle.set_global_state_property("category", json!("all"));
                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 300px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Expressions & Global State" }
                p { "Data-driven interpolate/match expressions, filters, string feature IDs, and nested GeoJSON properties." }
                p { "data-testid": "global-state-value", "Category: {selected}" }
                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-top: 16px;",
                        for category in ["all", "rail", "water", "active"] {
                            {
                                let map = map.clone();
                                rsx! { button {
                                    onclick: move |_| {
                                        map.set_global_state_property("category", json!(category));
                                        selected.set(category.into());
                                    },
                                    "{category}"
                                } }
                            }
                        }
                    }
                }
            }
        }
    }
}
