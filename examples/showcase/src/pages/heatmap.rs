use dioxus::prelude::*;
use dioxus_maplibre::{GeoJsonSourceOptions, LayerOptions, LatLng, Map, MapHandle};
use serde_json::json;

/// Generate deterministic scatter points around a center
fn generate_heatmap_data(center: LatLng, count: usize) -> serde_json::Value {
    let mut features = Vec::with_capacity(count);
    // Simple deterministic pseudo-random based on index
    for i in 0..count {
        let angle = (i as f64) * 2.399_963; // golden angle in radians
        let r = (i as f64).sqrt() * 0.002;
        let lng = center.lng + r * angle.cos();
        let lat = center.lat + r * angle.sin();
        let weight = ((i % 7) as f64 + 1.0) / 7.0;
        features.push(json!({
            "type": "Feature",
            "geometry": { "type": "Point", "coordinates": [lng, lat] },
            "properties": { "weight": weight }
        }));
    }
    json!({ "type": "FeatureCollection", "features": features })
}

#[component]
pub fn Heatmap() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut intensity = use_signal(|| 1.0_f64);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 13.0,
                    on_ready: move |handle: MapHandle| {
                        let data = generate_heatmap_data(LatLng::new(60.17, 24.94), 200);

                        handle.add_geojson_source("heatmap-data", GeoJsonSourceOptions {
                            data,
                            ..Default::default()
                        });

                        handle.add_layer(LayerOptions::heatmap("heat-layer", "heatmap-data")
                            .paint(json!({
                                "heatmap-weight": ["get", "weight"],
                                "heatmap-intensity": 1,
                                "heatmap-radius": 25,
                                "heatmap-color": [
                                    "interpolate", ["linear"], ["heatmap-density"],
                                    0, "rgba(0,0,0,0)",
                                    0.2, "#3b82f6",
                                    0.4, "#8b5cf6",
                                    0.6, "#ec4899",
                                    0.8, "#f97316",
                                    1.0, "#eab308"
                                ]
                            }))
                        );

                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Heatmap" }
                p { "200-point heatmap with adjustable intensity." }
                p { style: "margin-top: 8px;", "Intensity: {intensity:.1}" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; gap: 8px; margin-top: 12px;",
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "heatmap-intensity-up",
                                    style: "flex: 1; padding: 8px; border-radius: 4px; border: none; background: #6366f1; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        let new_val = (intensity() + 0.5).min(5.0);
                                        intensity.set(new_val);
                                        map.set_paint_property("heat-layer", "heatmap-intensity", json!(new_val));
                                    },
                                    "+"
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "heatmap-intensity-down",
                                    style: "flex: 1; padding: 8px; border-radius: 4px; border: none; background: #6366f1; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        let new_val = (intensity() - 0.5).max(0.5);
                                        intensity.set(new_val);
                                        map.set_paint_property("heat-layer", "heatmap-intensity", json!(new_val));
                                    },
                                    "-"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
