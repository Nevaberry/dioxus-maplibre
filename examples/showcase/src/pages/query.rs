use dioxus::prelude::*;
use dioxus_maplibre::{
    Bounds, GeoJsonSourceOptions, LayerOptions, LatLng, Map, MapClickEvent, MapHandle,
    MapMoveEvent, QueryOptions,
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

fn viewport_query_options() -> QueryOptions {
    QueryOptions {
        layers: Some(vec!["query-circles".into()]),
        ..Default::default()
    }
}

fn count_sample_points_in_bounds(bounds: &Bounds) -> usize {
    sample_points()
        .get("features")
        .and_then(serde_json::Value::as_array)
        .map_or(0, |features| {
            features
                .iter()
                .filter_map(|feature| {
                    let coordinates = feature
                        .get("geometry")
                        .and_then(|g| g.get("coordinates"))
                        .and_then(serde_json::Value::as_array)?;
                    if coordinates.len() != 2 {
                        return None;
                    }
                    let lng = coordinates.first()?.as_f64()?;
                    let lat = coordinates.get(1)?.as_f64()?;
                    Some(LatLng::new(lat, lng))
                })
                .filter(|point| bounds.contains(point))
                .count()
        })
}

async fn refresh_viewport_count(
    map: MapHandle,
    mut feature_count: Signal<usize>,
    mut query_result: Signal<String>,
) {
    let features = map.query_rendered_features(viewport_query_options()).await;
    let count = features.len();
    feature_count.set(count);
    query_result.set(format!("{count} features in viewport"));
}

#[component]
pub fn Query() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut query_result = use_signal(|| String::new());
    let mut feature_count = use_signal(|| 0usize);
    let mut move_event_count = use_signal(|| 0usize);
    let mut move_event_throttle_ms = use_signal(|| 80u32);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 13.0,
                    move_event_throttle_ms: move_event_throttle_ms(),
                    on_ready: move |handle: MapHandle| {
                        let points = sample_points();
                        handle.add_geojson_source("query-points", GeoJsonSourceOptions {
                            data: points,
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
                    on_move: move |e: MapMoveEvent| {
                        move_event_count += 1;
                        if let Some(bounds) = e.bounds {
                            let geometric_count = count_sample_points_in_bounds(&bounds);
                            feature_count.set(geometric_count);
                            query_result.set(format!("{geometric_count} features in viewport (bounds)"));
                        }
                    },
                    on_click: move |e: MapClickEvent| {
                        if let Some(ref map) = *map_handle.read() {
                            let map = map.clone();
                            let point = e.point;
                            spawn(async move {
                                let features = map.query_rendered_features_at(
                                    point,
                                    viewport_query_options(),
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
            div { style: "width: 360px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Feature Query" }
                p { "Click on a point to query its properties." }
                p { "20 points in two categories (A=blue, B=red)." }
                p { style: "color: #9ec9ff;", "Move events observed: {move_event_count}" }
                p { style: "color: #9ec9ff; margin-top: 8px;", "Move throttle: {move_event_throttle_ms} ms" }
                input {
                    "data-testid": "move-throttle-range",
                    style: "width: 100%; margin-top: 4px;",
                    r#type: "range",
                    min: "0",
                    max: "320",
                    step: "10",
                    value: "{move_event_throttle_ms}",
                    oninput: move |evt| {
                        if let Ok(parsed) = evt.value().parse::<u32>() {
                            move_event_throttle_ms.set(parsed);
                        }
                    }
                }
                div { style: "display: flex; gap: 6px; margin-top: 6px; flex-wrap: wrap;",
                    button {
                        style: "padding: 4px 8px; border-radius: 4px; border: none; background: #2d6cdf; color: white; cursor: pointer;",
                        onclick: move |_| {
                            move_event_throttle_ms.set(40);
                        },
                        "40ms"
                    }
                    button {
                        style: "padding: 4px 8px; border-radius: 4px; border: none; background: #2d6cdf; color: white; cursor: pointer;",
                        onclick: move |_| {
                            move_event_throttle_ms.set(80);
                        },
                        "80ms"
                    }
                    button {
                        style: "padding: 4px 8px; border-radius: 4px; border: none; background: #2d6cdf; color: white; cursor: pointer;",
                        onclick: move |_| {
                            move_event_throttle_ms.set(120);
                        },
                        "120ms"
                    }
                    button {
                        style: "padding: 4px 8px; border-radius: 4px; border: none; background: #2d6cdf; color: white; cursor: pointer;",
                        onclick: move |_| {
                            move_event_throttle_ms.set(200);
                        },
                        "200ms"
                    }
                }
                p { style: "margin-top: 4px; color: #a8b2c3;", "Lower = smoother updates, higher = less CPU/event load." }

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
                                            refresh_viewport_count(
                                                map,
                                                feature_count,
                                                query_result,
                                            )
                                            .await;
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
