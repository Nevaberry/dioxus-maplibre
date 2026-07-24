use dioxus::prelude::*;
use dioxus_maplibre::{
    LatLng, Map, MapClickEvent, MapHandle, MapLifecycleEvent, MapOptions, MapPitchEvent,
    MapRollEvent, MapRotateEvent, MapZoomEvent,
};
use serde_json::json;

fn push_event(mut events: Signal<Vec<String>>, event: String) {
    let mut values = events.write();
    values.insert(0, event);
    values.truncate(10);
}

#[component]
pub fn LifecycleEvents() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let events = use_signal(Vec::<String>::new);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 11.0,
                    options: MapOptions(json!({ "rollEnabled": true, "maxPitch": 85 })),
                    on_ready: move |handle: MapHandle| {
                        push_event(events, "load / ready".into());
                        map_handle.set(Some(handle));
                    },
                    on_click: move |event: MapClickEvent| push_event(events, format!("click {:.4},{:.4}", event.latlng.lat, event.latlng.lng)),
                    on_zoom: move |event: MapZoomEvent| push_event(events, format!("zoomend {:.2}", event.zoom)),
                    on_rotate: move |event: MapRotateEvent| push_event(events, format!("rotateend {:.1}°", event.bearing)),
                    on_pitch: move |event: MapPitchEvent| push_event(events, format!("pitchend {:.1}°", event.pitch)),
                    on_roll: move |event: MapRollEvent| push_event(events, format!("rollend {:.1}°", event.roll)),
                    on_lifecycle: move |event: MapLifecycleEvent| push_event(events, event.event),
                }
            }
            div { style: "width: 310px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px; overflow-y: auto;",
                h3 { style: "margin: 0 0 12px 0;", "Lifecycle & Event Classes" }
                p { "Typed click/move/camera events plus MapLibre 6 lifecycle classes: style.load, terrain, projection, roll, drag, box zoom, idle, and WebGL context." }
                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; gap: 8px; margin: 12px 0;",
                        {
                            let map = map.clone();
                            rsx! { button { "data-testid": "emit-camera-events", onclick: move |_| {
                                map.set_roll(12.0);
                                map.rotate_to(30.0);
                                map.set_pitch(45.0);
                                map.zoom_in();
                            }, "Camera events" } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button { onclick: move |_| map.trigger_repaint(), "Repaint" } }
                        }
                    }
                }
                div { "data-testid": "lifecycle-log", style: "font-family: monospace; font-size: 12px; display: grid; gap: 4px;",
                    for event in events() {
                        div { "{event}" }
                    }
                }
            }
        }
    }
}
