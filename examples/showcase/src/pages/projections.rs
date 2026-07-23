use dioxus::prelude::*;
use dioxus_maplibre::{
    ControlPosition, LatLng, Map, MapHandle, MapLifecycleEvent, MapOptions, MapRollEvent,
    ProjectionOptions, SkyOptions,
};
use serde_json::json;

#[component]
pub fn Projections() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut projection = use_signal(|| "globe".to_string());
    let mut last_event = use_signal(|| "waiting for projectiontransition".to_string());
    let mut roll = use_signal(|| 0.0_f64);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(25.0, 10.0),
                    zoom: 1.25,
                    pitch: 20.0,
                    options: MapOptions(json!({
                        "maxPitch": 85,
                        "rollEnabled": true,
                        "renderWorldCopies": false
                    })),
                    on_ready: move |handle: MapHandle| {
                        handle.set_projection(ProjectionOptions::globe());
                        handle.set_sky(SkyOptions(json!({
                            "sky-color": "#071329",
                            "horizon-color": "#8ec5ff",
                            "sky-horizon-blend": 0.65,
                            "horizon-fog-blend": 0.5,
                            "atmosphere-blend": ["interpolate", ["linear"], ["zoom"], 0, 1, 5, 0]
                        })));
                        handle.add_globe_control(ControlPosition::TopRight);
                        handle.add_navigation_control(ControlPosition::TopRight);
                        map_handle.set(Some(handle));
                    },
                    on_roll: move |event: MapRollEvent| roll.set(event.roll),
                    on_lifecycle: move |event: MapLifecycleEvent| {
                        if event.event == "projectiontransition" || event.event.starts_with("roll") {
                            last_event.set(event.event);
                        }
                    },
                }
            }
            div { style: "width: 300px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px; overflow-y: auto;",
                h3 { style: "margin: 0 0 12px 0;", "Projection & Globe" }
                p { "MapLibre 5+ globe and MapLibre 6 camera roll through typed APIs." }
                p { "data-testid": "projection-status", "Projection: {projection}" }
                p { "data-testid": "projection-event", "Event: {last_event}" }
                p { "data-testid": "roll-value", "Roll: {roll:.1}°" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: grid; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            rsx! { button {
                                "data-testid": "projection-globe",
                                onclick: move |_| {
                                    map.set_projection(ProjectionOptions::globe());
                                    projection.set("globe".into());
                                },
                                "Globe"
                            } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button {
                                "data-testid": "projection-mercator",
                                onclick: move |_| {
                                    map.set_projection(ProjectionOptions::mercator());
                                    projection.set("mercator".into());
                                },
                                "Mercator"
                            } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button {
                                "data-testid": "projection-perspective",
                                onclick: move |_| {
                                    map.set_projection(ProjectionOptions::vertical_perspective());
                                    projection.set("vertical-perspective".into());
                                },
                                "Vertical perspective"
                            } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button {
                                "data-testid": "roll-camera",
                                onclick: move |_| map.set_roll(20.0),
                                "Roll camera 20°"
                            } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button {
                                onclick: move |_| {
                                    map.set_roll(0.0);
                                    map.reset_north_pitch();
                                },
                                "Reset orientation"
                            } }
                        }
                    }
                }
            }
        }
    }
}
