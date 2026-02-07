use dioxus::prelude::*;
use dioxus_maplibre::{
    GeoJsonSourceOptions, LayerOptions, LatLng, Map, MapClickEvent, MapHandle, QueryOptions,
};
use serde_json::json;

fn sample_points() -> serde_json::Value {
    let mut features = Vec::new();
    for i in 0..20 {
        let angle = (i as f64) * 0.314;
        let r = 0.01 + (i as f64) * 0.001;
        let lng = 24.94 + r * angle.cos();
        let lat = 60.17 + r * angle.sin();
        features.push(json!({
            "type": "Feature",
            "id": i,
            "geometry": { "type": "Point", "coordinates": [lng, lat] },
            "properties": { "name": format!("Point {}", i), "category": if i % 2 == 0 { "A" } else { "B" } }
        }));
    }
    json!({ "type": "FeatureCollection", "features": features })
}

#[component]
pub fn Query() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut query_result = use_signal(|| String::new());
    let mut feature_count = use_signal(|| 0usize);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 13.0,
                    on_ready: move |handle: MapHandle| {
                        handle.add_geojson_source("query-points", GeoJsonSourceOptions {
                            data: sample_points(),
                            generate_id: Some(true),
                            ..Default::default()
                        });

                        handle.add_layer(LayerOptions::circle("query-circles", "query-points")
                            .paint(json!({
                                "circle-radius": 8,
                                "circle-color": [
                                    "match", ["get", "category"],
                                    "A", "#3b82f6",
                                    "B", "#ef4444",
                                    "#888"
                                ],
                                "circle-stroke-width": 2,
                                "circle-stroke-color": "#fff"
                            }))
                        );

                        map_handle.set(Some(handle));
                    },
                    on_click: move |e: MapClickEvent| {
                        if let Some(ref map) = *map_handle.read() {
                            let map = map.clone();
                            let point = e.point;
                            spawn(async move {
                                let features = map.query_rendered_features_at(
                                    point,
                                    QueryOptions {
                                        layers: Some(vec!["query-circles".into()]),
                                        ..Default::default()
                                    },
                                ).await;
                                feature_count.set(features.len());
                                if let Some(f) = features.first() {
                                    query_result.set(serde_json::to_string_pretty(&f.properties).unwrap_or_default());
                                } else {
                                    query_result.set("No features at click point".into());
                                }
                            });
                        }
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Feature Query" }
                p { "Click on a point to query its properties." }
                p { "20 points in two categories (A=blue, B=red)." }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "margin-top: 16px;",
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "query-viewport",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer; width: 100%;",
                                    onclick: move |_| {
                                        let map = map.clone();
                                        spawn(async move {
                                            let features = map.query_rendered_features(
                                                QueryOptions {
                                                    layers: Some(vec!["query-circles".into()]),
                                                    ..Default::default()
                                                },
                                            ).await;
                                            feature_count.set(features.len());
                                            query_result.set(format!("{} features in viewport", features.len()));
                                        });
                                    },
                                    "Query Viewport"
                                }
                            }
                        }
                    }
                }

                p {
                    "data-testid": "feature-count",
                    style: "margin-top: 12px;",
                    "Features: {feature_count}"
                }
                pre {
                    "data-testid": "query-result",
                    style: "margin-top: 8px; background: #0d1117; padding: 8px; border-radius: 4px; font-size: 11px; max-height: 200px; overflow-y: auto; white-space: pre-wrap;",
                    "{query_result}"
                }
            }
        }
    }
}
