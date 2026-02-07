use dioxus::prelude::*;
use dioxus_maplibre::{Map, MapHandle, ControlPosition, LatLng};

#[component]
pub fn Controls() -> Element {
    let style: Signal<String> = use_context();
    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 12.0,
                    on_ready: move |handle: MapHandle| {
                        handle.add_navigation_control(ControlPosition::TopRight);
                        handle.add_scale_control(ControlPosition::BottomLeft);
                        handle.add_fullscreen_control(ControlPosition::TopLeft);
                        handle.add_geolocate_control(ControlPosition::TopRight);
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Controls" }
                p { "Navigation control: top-right" }
                p { "Scale control: bottom-left" }
                p { "Fullscreen control: top-left" }
                p { "Geolocate control: top-right" }
            }
        }
    }
}
