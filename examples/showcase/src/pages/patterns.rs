use dioxus::prelude::*;
use dioxus_maplibre::{GeoJsonSourceOptions, LayerOptions, LatLng, Map, MapHandle};
use serde_json::json;

#[component]
pub fn Patterns() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut status = use_signal(|| "Loading...".to_string());
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 13.0,
                    on_ready: move |handle: MapHandle| {
                        map_handle.set(Some(handle.clone()));

                        spawn(async move {
                            // Load a pattern image (using a publicly available pattern SVG data URI)
                            let loaded = handle.load_image_async(
                                "stripe-pattern",
                                "https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png"
                            ).await;

                            if loaded {
                                // Add polygon source
                                handle.add_geojson_source("pattern-area", GeoJsonSourceOptions {
                                    data: json!({
                                        "type": "FeatureCollection",
                                        "features": [
                                            {
                                                "type": "Feature",
                                                "geometry": {
                                                    "type": "Polygon",
                                                    "coordinates": [[
                                                        [24.92, 60.165], [24.96, 60.165],
                                                        [24.96, 60.175], [24.92, 60.175],
                                                        [24.92, 60.165]
                                                    ]]
                                                }
                                            },
                                            {
                                                "type": "Feature",
                                                "geometry": {
                                                    "type": "Polygon",
                                                    "coordinates": [[
                                                        [24.93, 60.176], [24.95, 60.176],
                                                        [24.95, 60.182], [24.93, 60.182],
                                                        [24.93, 60.176]
                                                    ]]
                                                }
                                            }
                                        ]
                                    }),
                                    ..Default::default()
                                });

                                // Fill layer with pattern
                                handle.add_layer(LayerOptions::fill("pattern-fill", "pattern-area")
                                    .paint(json!({
                                        "fill-pattern": "stripe-pattern",
                                        "fill-opacity": 0.8
                                    }))
                                );

                                // Also add an outline
                                handle.add_layer(LayerOptions::line("pattern-outline", "pattern-area")
                                    .paint(json!({
                                        "line-color": "#3b82f6",
                                        "line-width": 2
                                    }))
                                );

                                status.set("Loaded".to_string());
                            } else {
                                status.set("Failed to load image".to_string());
                            }
                        });
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Patterns (Image Fills)" }
                p { "Fill layers using image patterns loaded via load_image_async." }
                p {
                    "data-testid": "pattern-status",
                    style: "margin-top: 8px;",
                    "Status: {status}"
                }
            }
        }
    }
}
