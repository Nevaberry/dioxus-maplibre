use dioxus::prelude::*;
use dioxus_maplibre::{GeoJsonSourceOptions, LayerOptions, LatLng, Map, MapHandle};
use serde_json::json;

#[component]
pub fn Symbols() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut loaded = use_signal(|| false);

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    center: LatLng::new(60.17, 24.94),
                    zoom: 12.0,
                    on_ready: move |handle: MapHandle| {
                        map_handle.set(Some(handle.clone()));

                        spawn(async move {
                            // Load a custom marker icon (awaits completion)
                            let success = handle.load_image_async(
                                "custom-marker",
                                "https://maplibre.org/maplibre-gl-js/docs/assets/custom_marker.png",
                            ).await;

                            if !success {
                                tracing::warn!("Failed to load custom marker image");
                            }

                            // Add GeoJSON source with labeled points
                            handle.add_geojson_source("symbol-points", GeoJsonSourceOptions {
                                data: json!({
                                    "type": "FeatureCollection",
                                    "features": [
                                        {
                                            "type": "Feature",
                                            "geometry": { "type": "Point", "coordinates": [24.94, 60.17] },
                                            "properties": { "title": "HQ", "description": "Command Center" }
                                        },
                                        {
                                            "type": "Feature",
                                            "geometry": { "type": "Point", "coordinates": [24.96, 60.18] },
                                            "properties": { "title": "Alpha", "description": "Outpost Alpha" }
                                        },
                                        {
                                            "type": "Feature",
                                            "geometry": { "type": "Point", "coordinates": [24.92, 60.16] },
                                            "properties": { "title": "Bravo", "description": "Outpost Bravo" }
                                        }
                                    ]
                                }),
                                ..Default::default()
                            });

                            // Add symbol layer with icon + text labels
                            handle.add_layer(LayerOptions::symbol("symbol-layer", "symbol-points")
                                .layout(json!({
                                    "icon-image": "custom-marker",
                                    "icon-size": 0.5,
                                    "icon-allow-overlap": true,
                                    "text-field": ["get", "title"],
                                    "text-offset": [0, 1.5],
                                    "text-anchor": "top",
                                    "text-size": 12
                                }))
                                .paint(json!({
                                    "text-color": "#ffffff",
                                    "text-halo-color": "#000000",
                                    "text-halo-width": 1
                                }))
                            );

                            loaded.set(true);
                        });
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Symbols" }
                p { "Custom icon markers with text labels using symbol layers." }
                p { style: "margin-top: 8px;",
                    "data-testid": "symbol-status",
                    if loaded() { "Status: Loaded" } else { "Status: Loading..." }
                }
            }
        }
    }
}
