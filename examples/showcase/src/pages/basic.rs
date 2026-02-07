use dioxus::prelude::*;
use dioxus_maplibre::{Map, MapHandle, MapClickEvent, MapMoveEvent, LatLng};

#[component]
pub fn Basic() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut last_click = use_signal(|| None::<MapClickEvent>);
    let mut position = use_signal(|| String::from("--"));
    let mut event_log = use_signal(Vec::<String>::new);
    let style: Signal<String> = use_context();

    let mut log_event = move |msg: String| {
        event_log.write().push(msg);
        // Keep last 20 entries
        let len = event_log.read().len();
        if len > 20 {
            event_log.write().drain(0..len - 20);
        }
    };

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 12.0,
                    on_ready: move |handle: MapHandle| {
                        log_event(format!("Map ready (id: {})", handle.map_id()));
                        map_handle.set(Some(handle));
                    },
                    on_click: move |e: MapClickEvent| {
                        log_event(format!("Click: {:.4}, {:.4}", e.latlng.lat, e.latlng.lng));
                        last_click.set(Some(e));
                    },
                    on_move: move |e: MapMoveEvent| {
                        position.set(format!("{:.4}, {:.4} z{:.1}", e.center.lat, e.center.lng, e.zoom));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; overflow-y: auto; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Basic Map" }

                p { "Position: {position}" }

                if let Some(click) = last_click() {
                    p { "Last click: {click.latlng.lat:.4}, {click.latlng.lng:.4}" }
                }

                h4 { style: "margin: 16px 0 8px 0;", "Event Log" }
                div { "data-testid": "event-log", style: "font-family: monospace; font-size: 11px;",
                    for (i, entry) in event_log.read().iter().enumerate() {
                        p { key: "{i}", style: "margin: 2px 0; color: #88aa88;", "{entry}" }
                    }
                }
            }
        }
    }
}
