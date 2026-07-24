use dioxus::prelude::*;
use dioxus_maplibre::{LatLng, Map, MapHandle, PopupOptions};

#[component]
pub fn Popups() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut popup_count = use_signal(|| 3u32);
    let mut next_popup_id = use_signal(|| 4u32);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 13.0,
                    on_ready: move |handle: MapHandle| {
                        // Popup 1: default options
                        handle.add_popup(
                            "popup-1",
                            LatLng::new(60.1699, 24.9384),
                            "<b>Helsinki Center</b><p>Default popup options</p>",
                            PopupOptions::default(),
                        );

                        // Popup 2: custom anchor and no close button
                        handle.add_popup(
                            "popup-2",
                            LatLng::new(60.175, 24.945),
                            "<b>Kallio</b><p>No close button, anchored bottom</p>",
                            PopupOptions {
                                anchor: Some("bottom".into()),
                                close_button: Some(false),
                                ..Default::default()
                            },
                        );

                        // Popup 3: max width set
                        handle.add_popup(
                            "popup-3",
                            LatLng::new(60.165, 24.930),
                            "<b>Eira</b><p>Max width 200px popup with longer content that wraps around.</p>",
                            PopupOptions {
                                max_width: Some("200px".into()),
                                ..Default::default()
                            },
                        );

                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Popups" }
                p { "Standalone popups with varied options." }
                p { "data-testid": "popup-count", "Popups: {popup_count}" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "add-popup",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        let id = next_popup_id();
                                        let lat = 60.17 + (id as f64 - 4.0) * 0.003;
                                        map.add_popup(
                                            &format!("popup-{id}"),
                                            LatLng::new(lat, 24.94),
                                            &format!("<b>Popup {id}</b><p>Dynamically added</p>"),
                                            PopupOptions::default(),
                                        );
                                        popup_count += 1;
                                        next_popup_id += 1;
                                    },
                                    "Add Popup"
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "remove-popup",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #ef4444; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        map.remove_popup("popup-1");
                                        if popup_count() > 0 {
                                            popup_count -= 1;
                                        }
                                    },
                                    "Remove Popup 1"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
