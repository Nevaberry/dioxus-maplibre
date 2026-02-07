use dioxus::prelude::*;
use dioxus_maplibre::{FogOptions, LatLng, Map, MapHandle};
use serde_json::json;

#[component]
pub fn Fog() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut fog_enabled = use_signal(|| true);
    let mut preset = use_signal(|| "dawn".to_string());
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 11.0,
                    pitch: 45.0,
                    on_ready: move |handle: MapHandle| {
                        // Apply default dawn fog
                        handle.set_fog(FogOptions(json!({
                            "color": "#dc9f9f",
                            "horizon-blend": 0.05,
                            "high-color": "#245bde",
                            "space-color": "#000000",
                            "star-intensity": 0.15
                        })));

                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Fog / Atmosphere" }
                p { "Atmospheric effects with fog presets." }
                p { "data-testid": "fog-preset", style: "margin-top: 8px;", "Preset: {preset}" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            let enabled = fog_enabled();
                            rsx! {
                                button {
                                    "data-testid": "toggle-fog",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        if enabled {
                                            map.remove_fog();
                                        } else {
                                            map.set_fog(FogOptions(json!({
                                                "color": "#dc9f9f",
                                                "horizon-blend": 0.05,
                                                "high-color": "#245bde",
                                                "space-color": "#000000",
                                                "star-intensity": 0.15
                                            })));
                                            preset.set("dawn".into());
                                        }
                                        fog_enabled.set(!enabled);
                                    },
                                    if enabled { "Disable Fog" } else { "Enable Fog" }
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "preset-dawn",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #f59e0b; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        map.set_fog(FogOptions(json!({
                                            "color": "#dc9f9f",
                                            "horizon-blend": 0.05,
                                            "high-color": "#245bde",
                                            "space-color": "#000000",
                                            "star-intensity": 0.15
                                        })));
                                        fog_enabled.set(true);
                                        preset.set("dawn".into());
                                    },
                                    "Dawn"
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "preset-night",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #1e1b4b; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        map.set_fog(FogOptions(json!({
                                            "color": "#0a0a2e",
                                            "horizon-blend": 0.02,
                                            "high-color": "#000033",
                                            "space-color": "#000000",
                                            "star-intensity": 0.8
                                        })));
                                        fog_enabled.set(true);
                                        preset.set("night".into());
                                    },
                                    "Night"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
