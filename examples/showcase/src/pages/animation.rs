use dioxus::prelude::*;
use dioxus_maplibre::{GeoJsonSourceOptions, LayerOptions, LatLng, Map, MapHandle};
use gloo_timers::future::TimeoutFuture;
use serde_json::json;

/// Generate a point at angle `t` on a circle
fn orbit_point(center_lng: f64, center_lat: f64, radius: f64, t: f64) -> [f64; 2] {
    [
        center_lng + radius * t.cos(),
        center_lat + radius * t.sin(),
    ]
}

#[component]
pub fn Animation() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut running = use_signal(|| false);
    let mut frame = use_signal(|| 0u32);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 13.0,
                    on_ready: move |handle: MapHandle| {
                        // Orbiting points source
                        handle.add_geojson_source("orbit-points", GeoJsonSourceOptions {
                            data: json!({
                                "type": "FeatureCollection",
                                "features": []
                            }),
                            ..Default::default()
                        });

                        handle.add_layer(LayerOptions::circle("orbit-layer", "orbit-points")
                            .paint(json!({
                                "circle-radius": 8,
                                "circle-color": "#3b82f6",
                                "circle-stroke-width": 2,
                                "circle-stroke-color": "#fff"
                            }))
                        );

                        // Progressive line source
                        handle.add_geojson_source("progressive-line", GeoJsonSourceOptions {
                            data: json!({
                                "type": "Feature",
                                "geometry": {
                                    "type": "LineString",
                                    "coordinates": [[24.94, 60.17]]
                                }
                            }),
                            ..Default::default()
                        });

                        handle.add_layer(LayerOptions::line("line-layer", "progressive-line")
                            .paint(json!({
                                "line-color": "#ef4444",
                                "line-width": 3,
                                "line-opacity": 0.8
                            }))
                            .layout(json!({
                                "line-cap": "round",
                                "line-join": "round"
                            }))
                        );

                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Animation" }
                p { "Orbiting points and progressive line reveal." }
                p { "data-testid": "frame-counter", "Frame: {frame}" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            let is_running = running();
                            rsx! {
                                button {
                                    "data-testid": "toggle-animation",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        if is_running {
                                            running.set(false);
                                        } else {
                                            running.set(true);
                                            let map = map.clone();
                                            spawn(async move {
                                                loop {
                                                    if !running() {
                                                        break;
                                                    }

                                                    let f = frame();
                                                    let t = (f as f64) * 0.05;

                                                    // Update orbiting points (3 points at different offsets)
                                                    let mut features = Vec::new();
                                                    for i in 0..3 {
                                                        let offset = (i as f64) * std::f64::consts::TAU / 3.0;
                                                        let pt = orbit_point(24.94, 60.17, 0.008, t + offset);
                                                        features.push(json!({
                                                            "type": "Feature",
                                                            "geometry": { "type": "Point", "coordinates": pt }
                                                        }));
                                                    }
                                                    map.update_geojson_source("orbit-points", json!({
                                                        "type": "FeatureCollection",
                                                        "features": features
                                                    }));

                                                    // Progressive line: spiral outward
                                                    let num_coords = ((f + 1) as usize).min(100);
                                                    let coords: Vec<[f64; 2]> = (0..num_coords)
                                                        .map(|i| {
                                                            let angle = (i as f64) * 0.15;
                                                            let r = 0.001 + (i as f64) * 0.0001;
                                                            [24.94 + r * angle.cos(), 60.17 + r * angle.sin()]
                                                        })
                                                        .collect();

                                                    map.update_geojson_source("progressive-line", json!({
                                                        "type": "Feature",
                                                        "geometry": {
                                                            "type": "LineString",
                                                            "coordinates": coords
                                                        }
                                                    }));

                                                    frame.set(f + 1);

                                                    TimeoutFuture::new(50).await;
                                                }
                                            });
                                        }
                                    },
                                    if is_running { "Stop" } else { "Start" }
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "reset-animation",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #ef4444; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        running.set(false);
                                        frame.set(0);
                                        map.update_geojson_source("orbit-points", json!({
                                            "type": "FeatureCollection",
                                            "features": []
                                        }));
                                        map.update_geojson_source("progressive-line", json!({
                                            "type": "Feature",
                                            "geometry": {
                                                "type": "LineString",
                                                "coordinates": [[24.94, 60.17]]
                                            }
                                        }));
                                    },
                                    "Reset"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
