use dioxus::prelude::*;
use dioxus_maplibre::{
    FogOptions, LatLng, Map, MapHandle, MapOptions, RasterDemSourceOptions, TerrainOptions,
};
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
                    center: LatLng::new(47.27, 11.39),
                    zoom: 10.5,
                    pitch: 68.0,
                    options: MapOptions(json!({ "maxPitch": 85, "terrainSkirtLength": "auto" })),
                    on_ready: move |handle: MapHandle| {
                        handle.add_raster_dem_source("fog-dem", RasterDemSourceOptions {
                            tiles: Some(vec!["https://demotiles.maplibre.org/terrain-tiles/{z}/{x}/{y}.png".into()]),
                            tile_size: Some(256),
                            max_zoom: Some(12),
                            bounds: Some([11.0, 47.0, 12.0, 48.0]),
                            attribution: Some("AW3D30 (JAXA)".into()),
                            ..Default::default()
                        });
                        handle.set_terrain(TerrainOptions {
                            source: "fog-dem".into(),
                            exaggeration: Some(1.2),
                        });
                        // `set_fog` is the backward-compatible alias for MapLibre's setSky.
                        handle.set_fog(FogOptions(json!({
                            "sky-color": "#7aa5d2",
                            "sky-horizon-blend": 0.55,
                            "horizon-color": "#f6c6a8",
                            "horizon-fog-blend": 0.75,
                            "fog-color": "#f1c6b8",
                            "fog-ground-blend": 0.65,
                            "atmosphere-blend": ["interpolate", ["linear"], ["zoom"], 0, 1, 12, 0]
                        })));

                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Fog / Atmosphere" }
                p { "MapLibre sky, horizon, atmosphere, and terrain fog properties." }
                p { "The legacy set_fog API now correctly delegates to setSky." }
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
                                                "sky-color": "#7aa5d2",
                                                "sky-horizon-blend": 0.55,
                                                "horizon-color": "#f6c6a8",
                                                "horizon-fog-blend": 0.75,
                                                "fog-color": "#f1c6b8",
                                                "fog-ground-blend": 0.65,
                                                "atmosphere-blend": ["interpolate", ["linear"], ["zoom"], 0, 1, 12, 0]
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
                                            "sky-color": "#7aa5d2",
                                            "sky-horizon-blend": 0.55,
                                            "horizon-color": "#f6c6a8",
                                            "horizon-fog-blend": 0.75,
                                            "fog-color": "#f1c6b8",
                                            "fog-ground-blend": 0.65,
                                            "atmosphere-blend": ["interpolate", ["linear"], ["zoom"], 0, 1, 12, 0]
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
                                            "sky-color": "#080d2b",
                                            "sky-horizon-blend": 0.35,
                                            "horizon-color": "#172554",
                                            "horizon-fog-blend": 0.7,
                                            "fog-color": "#111827",
                                            "fog-ground-blend": 0.8,
                                            "atmosphere-blend": ["interpolate", ["linear"], ["zoom"], 0, 1, 12, 0]
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
