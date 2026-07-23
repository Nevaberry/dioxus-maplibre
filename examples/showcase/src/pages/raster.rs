use dioxus::prelude::*;
use dioxus_maplibre::{
    LatLng, LayerOptions, Map, MapHandle, MapOptions, RasterDemSourceOptions, RasterSourceOptions,
};
use serde_json::json;

fn show_layer(map: &MapHandle, selected: &str) {
    for layer in ["relief-demo", "hillshade-demo", "raster-demo"] {
        map.set_layout_property(
            layer,
            "visibility",
            json!(if layer == selected { "visible" } else { "none" }),
        );
    }
}

#[component]
pub fn RasterRelief() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut active = use_signal(|| "color-relief".to_string());
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(47.2, 11.45),
                    zoom: 9.5,
                    options: MapOptions(json!({ "maxPitch": 85, "renderWorldCopies": false })),
                    on_ready: move |handle: MapHandle| {
                        handle.add_raster_dem_source("relief-dem", RasterDemSourceOptions {
                            tiles: Some(vec!["https://demotiles.maplibre.org/terrain-tiles/{z}/{x}/{y}.png".into()]),
                            tile_size: Some(256),
                            max_zoom: Some(12),
                            bounds: Some([11.0, 47.0, 12.0, 48.0]),
                            attribution: Some("AW3D30 (JAXA)".into()),
                            ..Default::default()
                        });
                        handle.add_layer(LayerOptions::color_relief("relief-demo", "relief-dem")
                            .paint(json!({
                                "color-relief-color": [
                                    "interpolate", ["linear"], ["elevation"],
                                    300, "#172554", 800, "#2563eb", 1400, "#22c55e",
                                    2200, "#fde047", 3000, "#f97316", 3800, "#ffffff"
                                ]
                            }))
                        );
                        handle.add_layer(LayerOptions::hillshade("hillshade-demo", "relief-dem")
                            .layout(json!({ "visibility": "none" }))
                            .paint(json!({
                                "hillshade-method": "multidirectional",
                                "hillshade-exaggeration": 0.7,
                                "hillshade-shadow-color": "#111827",
                                "hillshade-highlight-color": "#f8fafc"
                            }))
                        );
                        handle.add_raster_source("osm-raster", RasterSourceOptions {
                            tiles: Some(vec!["https://tile.openstreetmap.org/{z}/{x}/{y}.png".into()]),
                            tile_size: Some(256),
                            max_zoom: Some(19),
                            attribution: Some("© OpenStreetMap contributors".into()),
                            ..Default::default()
                        });
                        handle.add_layer(LayerOptions::raster("raster-demo", "osm-raster")
                            .layout(json!({ "visibility": "none" }))
                            .paint(json!({ "raster-opacity": 0.92 }))
                        );
                        handle.set_raster_premultiply_alpha("osm-raster", true);
                        handle.set_source_tile_lod_params(4.0, 3.0, Some("relief-dem"));
                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 300px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Raster, DEM & Relief" }
                p { "The complete raster family: raster tiles, raster DEM, hillshade, and color-relief." }
                p { "Also exercises MapLibre 6 alpha premultiplication and source LOD controls." }
                p { "data-testid": "raster-mode", "Mode: {active}" }
                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: grid; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            rsx! { button { "data-testid": "show-relief", onclick: move |_| {
                                show_layer(&map, "relief-demo");
                                active.set("color-relief".into());
                            }, "Color relief" } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button { "data-testid": "show-hillshade", onclick: move |_| {
                                show_layer(&map, "hillshade-demo");
                                active.set("hillshade".into());
                            }, "Hillshade" } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button { "data-testid": "show-raster", onclick: move |_| {
                                show_layer(&map, "raster-demo");
                                active.set("raster".into());
                            }, "Raster tiles" } }
                        }
                    }
                }
            }
        }
    }
}
