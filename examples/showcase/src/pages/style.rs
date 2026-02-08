use dioxus::prelude::*;
use dioxus_maplibre::{LatLng, Map, MapHandle};

struct StyleEntry {
    name: &'static str,
    url: &'static str,
}

const STYLES: &[StyleEntry] = &[
    StyleEntry { name: "Dark Matter", url: "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json" },
    StyleEntry { name: "Positron", url: "https://basemaps.cartocdn.com/gl/positron-gl-style/style.json" },
    StyleEntry { name: "Voyager", url: "https://basemaps.cartocdn.com/gl/voyager-gl-style/style.json" },
    StyleEntry { name: "OSM Liberty", url: "https://tiles.openfreemap.org/styles/liberty" },
    StyleEntry { name: "MapLibre Demo", url: "https://demotiles.maplibre.org/style.json" },
];

#[component]
pub fn StyleSwitcher() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut current_style = use_signal(|| "Dark Matter".to_string());
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 12.0,
                    on_ready: move |handle: MapHandle| {
                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Style Switcher" }
                p { "Current: {current_style}" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 12px;",
                        for style_entry in STYLES {
                            {
                                let map = map.clone();
                                let url = style_entry.url;
                                let name = style_entry.name;
                                rsx! {
                                    button {
                                        style: "padding: 8px; border-radius: 4px; border: none; background: #333; color: white; cursor: pointer; text-align: left; font-size: 12px;",
                                        onclick: move |_| {
                                            map.set_style(url);
                                            current_style.set(name.to_string());
                                        },
                                        "{name}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
