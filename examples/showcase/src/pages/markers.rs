use dioxus::prelude::*;
use dioxus_maplibre::{Map, MapHandle, MarkerOptions, MarkerClickEvent, MarkerDragEndEvent, LatLng};

#[component]
pub fn Markers() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut clicked_marker = use_signal(|| None::<String>);
    let mut marker_count = use_signal(|| 0u32);
    let mut drag_position = use_signal(|| None::<LatLng>);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 11.0,
                    on_ready: move |handle: MapHandle| {
                        // Default markers
                        handle.add_marker("helsinki", LatLng::new(60.1699, 24.9384), MarkerOptions::default());
                        handle.add_marker("tampere", LatLng::new(61.4978, 23.7610), MarkerOptions {
                            color: Some("#ef4444".into()),
                            ..Default::default()
                        });
                        // Emoji marker with popup
                        handle.add_marker("oulu", LatLng::new(65.0121, 25.4651), MarkerOptions {
                            emoji: Some("🏔️".into()),
                            popup_html: Some("<b>Oulu</b><p>Northern city</p>".into()),
                            ..Default::default()
                        });
                        // Draggable marker
                        handle.add_marker("draggable", LatLng::new(60.45, 24.94), MarkerOptions {
                            color: Some("#22c55e".into()),
                            draggable: Some(true),
                            ..Default::default()
                        });
                        marker_count.set(4);
                        map_handle.set(Some(handle));
                    },
                    on_marker_click: move |e: MarkerClickEvent| {
                        clicked_marker.set(Some(e.marker_id));
                    },
                    on_marker_dragend: move |e: MarkerDragEndEvent| {
                        drag_position.set(Some(e.latlng));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Markers" }
                p { "Markers on map: {marker_count}" }

                if let Some(id) = clicked_marker() {
                    p { "data-testid": "clicked-marker", "Clicked: {id}" }
                }

                if let Some(pos) = drag_position() {
                    p { "data-testid": "drag-position", "Dragged to: {pos.lat:.4}, {pos.lng:.4}" }
                }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        let count = marker_count() + 1;
                                        let lat = 60.0 + (count as f64) * 0.05;
                                        map.add_marker(
                                            &format!("dynamic_{count}"),
                                            LatLng::new(lat, 24.94),
                                            MarkerOptions::default(),
                                        );
                                        marker_count.set(count);
                                    },
                                    "Add Marker"
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #ef4444; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        map.remove_marker("helsinki");
                                    },
                                    "Remove Helsinki"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
