use dioxus::prelude::*;
use dioxus_maplibre::{
    Bounds, GeoJsonSourceOptions, LayerOptions, LatLng, Map, MapClickEvent, MapHandle,
    MapMoveEvent, MapZoomEvent, QueryFeature, QueryOptions,
};
use serde_json::json;

const MAX_DEBUG_LINES: usize = 160;

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

fn feature_summary(feature: &QueryFeature) -> String {
    let name = feature
        .properties
        .get("name")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("<none>");
    let category = feature
        .properties
        .get("category")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("<none>");
    format!(
        "id={:?} source={} layer={:?} name={name} category={category}",
        feature.id, feature.source, feature.source_layer
    )
}

fn emit_debug(mut debug_log: Signal<Vec<String>>, level: &'static str, message: impl Into<String>) {
    let message = message.into();
    let line = format!("[showcase/query][{level}] {message}");

    match level {
        "ERROR" => tracing::error!("{line}"),
        "WARN" => tracing::warn!("{line}"),
        "INFO" => tracing::info!("{line}"),
        _ => tracing::debug!("{line}"),
    }

    {
        let mut lines = debug_log.write();
        lines.push(line.clone());
        let overflow = lines.len().saturating_sub(MAX_DEBUG_LINES);
        if overflow > 0 {
            lines.drain(0..overflow);
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let payload = serde_json::to_string(&line).unwrap_or_else(|_| "\"<log-serialize-failed>\"".to_string());
        let method = match level {
            "ERROR" => "error",
            "WARN" => "warn",
            _ => "log",
        };
        let js = format!("console.{method}({payload});");
        spawn(async move {
            let _ = document::eval(&js).await;
        });
    }
}

#[cfg(target_arch = "wasm32")]
async fn sleep_ms(ms: u32) {
    gloo_timers::future::TimeoutFuture::new(ms).await;
}

#[cfg(not(target_arch = "wasm32"))]
async fn sleep_ms(_ms: u32) {}

async fn refresh_viewport_count(
    map: MapHandle,
    mut feature_count: Signal<usize>,
    mut query_result: Signal<String>,
    debug_log: Signal<Vec<String>>,
    reason: String,
) {
    const MAX_ATTEMPTS: usize = 20;
    const RETRY_MS: u32 = 80;

    for attempt in 1..=MAX_ATTEMPTS {
        let features = map.query_rendered_features(viewport_query_options()).await;
        let layered_count = features.len();
        let sample = features
            .first()
            .map(feature_summary)
            .unwrap_or_else(|| "<none>".to_string());

        emit_debug(
            debug_log,
            "INFO",
            format!(
                "{reason}: attempt #{attempt} layered={} sample={sample}",
                layered_count
            ),
        );

        if layered_count > 0 {
            feature_count.set(layered_count);
            query_result.set(format!("{} features in viewport", layered_count));
            return;
        }

        if attempt == MAX_ATTEMPTS {
            emit_debug(
                debug_log,
                "WARN",
                format!("{reason}: layered query remained 0 after {MAX_ATTEMPTS} attempts; keeping existing count"),
            );
            return;
        }

        sleep_ms(RETRY_MS).await;
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

#[component]
pub fn Query() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut query_result = use_signal(|| String::new());
    let mut feature_count = use_signal(|| 0usize);
    let mut move_event_count = use_signal(|| 0usize);
    let debug_log = use_signal(Vec::<String>::new);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 13.0,
                    on_ready: move |handle: MapHandle| {
                        emit_debug(debug_log, "INFO", format!("on_ready fired map_id={}", handle.map_id()));

                        let points = sample_points();
                        let point_count = points
                            .get("features")
                            .and_then(serde_json::Value::as_array)
                            .map_or(0, std::vec::Vec::len);
                        emit_debug(debug_log, "INFO", format!("adding source query-points with {point_count} sample points"));

                        handle.add_geojson_source("query-points", GeoJsonSourceOptions {
                            data: points,
                            generate_id: Some(true),
                            ..Default::default()
                        });

                        emit_debug(debug_log, "INFO", "adding layer query-circles from source query-points");
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

                        let map = handle.clone();
                        spawn(async move {
                            refresh_viewport_count(
                                map,
                                feature_count,
                                query_result,
                                debug_log,
                                "initial".to_string(),
                            )
                            .await;
                        });

                        map_handle.set(Some(handle));
                    },
                    on_move: move |e: MapMoveEvent| {
                        let event_no = move_event_count() + 1;
                        move_event_count.set(event_no);

                        let phase = e.phase.unwrap_or_else(|| "move".to_string());
                        let should_log_move = phase != "move" || event_no <= 8 || event_no % 10 == 0;
                        if should_log_move {
                            emit_debug(
                                debug_log,
                                "INFO",
                                format!(
                                    "on_move #{event_no} phase={phase} center=({}, {}) zoom={} bounds={:?}",
                                    e.center.lat, e.center.lng, e.zoom, e.bounds
                                ),
                            );
                        }

                        if let Some(bounds) = e.bounds {
                            let geometric_count = count_sample_points_in_bounds(&bounds);
                            if should_log_move {
                                emit_debug(
                                    debug_log,
                                    "INFO",
                                    format!("on_move #{event_no} phase={phase} geometric bounds count={geometric_count}"),
                                );
                            }
                            feature_count.set(geometric_count);
                            query_result.set(format!("{geometric_count} features in viewport (bounds)"));
                        }

                        if map_handle.read().is_none() {
                            emit_debug(debug_log, "WARN", format!("on_move #{event_no} ignored: map_handle missing"));
                        }
                    },
                    on_zoom: move |e: MapZoomEvent| {
                        emit_debug(debug_log, "DEBUG", format!("on_zoom event zoom={}", e.zoom));
                    },
                    on_click: move |e: MapClickEvent| {
                        emit_debug(
                            debug_log,
                            "INFO",
                            format!("on_click latlng=({}, {}) point=({}, {})", e.latlng.lat, e.latlng.lng, e.point.x, e.point.y),
                        );

                        if let Some(ref map) = *map_handle.read() {
                            let map = map.clone();
                            let point = e.point;
                            spawn(async move {
                                let features = map.query_rendered_features_at(
                                    point,
                                    viewport_query_options(),
                                ).await;
                                feature_count.set(features.len());
                                let sample = features
                                    .first()
                                    .map(feature_summary)
                                    .unwrap_or_else(|| "<none>".to_string());
                                emit_debug(
                                    debug_log,
                                    "INFO",
                                    format!("on_click point query count={} sample={sample}", features.len()),
                                );

                                if let Some(f) = features.first() {
                                    query_result.set(serde_json::to_string_pretty(&f.properties).unwrap_or_default());
                                } else {
                                    query_result.set("No features at click point".into());
                                }
                            });
                        } else {
                            emit_debug(debug_log, "WARN", "on_click ignored: map_handle missing");
                        }
                    },
                }
            }
            div { style: "width: 360px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Feature Query" }
                p { "Click on a point to query its properties." }
                p { "20 points in two categories (A=blue, B=red)." }
                p { style: "color: #9ec9ff;", "Move events observed: {move_event_count}" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "margin-top: 16px;",
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "query-viewport",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer; width: 100%;",
                                    onclick: move |_| {
                                        emit_debug(debug_log, "INFO", "manual Query Viewport button clicked");
                                        let map = map.clone();
                                        spawn(async move {
                                            refresh_viewport_count(
                                                map,
                                                feature_count,
                                                query_result,
                                                debug_log,
                                                "manual button".to_string(),
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
                details {
                    open: true,
                    style: "margin-top: 8px;",
                    summary { style: "cursor: pointer; color: #9ec9ff;", "Debug Log (latest {debug_log.read().len()})" }
                    div {
                        "data-testid": "query-debug-log",
                        style: "margin-top: 6px; background: #0d1117; padding: 8px; border-radius: 4px; font-size: 10px; max-height: 260px; overflow-y: auto; white-space: pre-wrap; font-family: monospace;",
                        for (i, line) in debug_log.read().iter().enumerate() {
                            p { key: "{i}", style: "margin: 0 0 4px 0;", "{line}" }
                        }
                    }
                }
            }
        }
    }
}
