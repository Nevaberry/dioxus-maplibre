use dioxus::prelude::*;
use dioxus_maplibre::{
    Map, MapHandle, RasterDemSourceOptions, TerrainOptions,
    FlyToOptions, ControlPosition, LatLng,
};

#[component]
pub fn Terrain() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut terrain_enabled = use_signal(|| false);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(47.27, 11.39),
                    zoom: 12.0,
                    pitch: 70.0,
                    on_ready: move |handle: MapHandle| {
                        handle.add_navigation_control(ControlPosition::TopRight);

                        // Add terrain DEM source (demo tiles cover Innsbruck area)
                        handle.add_raster_dem_source("terrain-dem", RasterDemSourceOptions {
                            url: Some("https://demotiles.maplibre.org/terrain-tiles/tiles.json".into()),
                            tile_size: Some(256),
                            ..Default::default()
                        });

                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Terrain" }
                p { "3D terrain with raster DEM source." }
                p { "Location: Innsbruck, Austrian Alps" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            let enabled = terrain_enabled();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        if enabled {
                                            map.remove_terrain();
                                        } else {
                                            map.set_terrain(TerrainOptions {
                                                source: "terrain-dem".into(),
                                                exaggeration: Some(1.5),
                                            });
                                        }
                                        terrain_enabled.set(!enabled);
                                    },
                                    if enabled { "Disable Terrain" } else { "Enable Terrain" }
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #6366f1; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        map.fly_to(FlyToOptions {
                                            center: Some(LatLng::new(47.37, 11.10)),
                                            zoom: Some(14.0),
                                            pitch: Some(76.0),
                                            bearing: Some(160.0),
                                            essential: Some(true),
                                            ..Default::default()
                                        });
                                    },
                                    "Fly to Nordkette"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
