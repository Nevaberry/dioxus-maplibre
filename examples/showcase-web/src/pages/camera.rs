use dioxus::prelude::*;
use dioxus_maplibre::{Bounds, LatLng, Map, MapHandle, MapInteraction, MapOptions, Point};
use serde_json::json;

#[component]
pub fn CameraState() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut world_copies = use_signal(|| true);
    let mut interactions = use_signal(|| true);
    let mut readout = use_signal(|| "ready".to_string());
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 10.0,
                    options: MapOptions(json!({
                        "maxPitch": 85,
                        "rollEnabled": true,
                        "centerClampedToGround": false,
                        "zoomSnap": 0.25
                    })),
                    on_ready: move |handle: MapHandle| map_handle.set(Some(handle)),
                }
            }
            div { style: "width: 310px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px; overflow-y: auto;",
                h3 { style: "margin: 0 0 12px 0;", "Camera & Map State" }
                p { "Direct setters/getters, projection math, constraints, FOV, world copies, and gesture handlers." }
                p { "data-testid": "camera-readout", "{readout}" }
                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: grid; gap: 8px; margin-top: 14px;",
                        {
                            let map = map.clone();
                            rsx! { button { onclick: move |_| {
                                map.set_center(LatLng::new(61.5, 23.76));
                                map.set_zoom(12.0);
                                map.set_bearing(30.0);
                                map.set_roll(8.0);
                                map.set_pitch(55.0);
                            }, "Set full camera state" } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button { onclick: move |_| {
                                map.set_vertical_field_of_view(50.0);
                                map.set_center_elevation(250.0);
                                map.set_center_clamped_to_ground(false);
                                readout.set("FOV 50°, elevation 250 m".into());
                            }, "FOV & elevation" } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button { onclick: move |_| {
                                map.set_max_bounds(Some(Bounds::new(
                                    LatLng::new(59.5, 22.0),
                                    LatLng::new(62.5, 27.5),
                                )));
                                map.set_min_zoom(5.0);
                                map.set_max_zoom(18.0);
                                map.set_max_pitch(85.0);
                                readout.set("Finland bounds + camera constraints".into());
                            }, "Apply constraints" } }
                        }
                        {
                            let map = map.clone();
                            let enabled = world_copies();
                            rsx! { button { "data-testid": "toggle-world-copies", onclick: move |_| {
                                map.set_render_world_copies(!enabled);
                                world_copies.set(!enabled);
                            }, if enabled { "Disable world copies" } else { "Enable world copies" } } }
                        }
                        {
                            let map = map.clone();
                            let enabled = interactions();
                            rsx! { button { "data-testid": "toggle-drag-pan", onclick: move |_| {
                                map.set_interaction_enabled(MapInteraction::DragPan, !enabled);
                                map.set_interaction_enabled(MapInteraction::ScrollZoom, !enabled);
                                interactions.set(!enabled);
                            }, if enabled { "Disable pan + wheel" } else { "Enable pan + wheel" } } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button { "data-testid": "project-coordinate", onclick: move |_| {
                                let map = map.clone();
                                spawn(async move {
                                    let coordinate = LatLng::new(60.1699, 24.9384);
                                    if let Some(point) = map.project(coordinate).await {
                                        let round_trip = map.unproject(Point::new(point.x, point.y)).await;
                                        readout.set(format!(
                                            "project → ({:.1}, {:.1}); unproject → {:?}",
                                            point.x, point.y, round_trip
                                        ));
                                    }
                                });
                            }, "Project / unproject Helsinki" } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button { onclick: move |_| {
                                let map = map.clone();
                                spawn(async move {
                                    let roll = map.get_roll().await.unwrap_or_default();
                                    let fov = map.get_vertical_field_of_view().await.unwrap_or_default();
                                    let projection = map.get_projection().await.unwrap_or(json!(null));
                                    readout.set(format!("roll={roll:.1}, fov={fov:.1}, projection={projection}"));
                                });
                            }, "Read typed getters" } }
                        }
                    }
                }
            }
        }
    }
}
