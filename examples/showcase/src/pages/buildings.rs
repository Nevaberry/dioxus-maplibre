use dioxus::prelude::*;
use dioxus_maplibre::{GeoJsonSourceOptions, LayerOptions, LatLng, Map, MapHandle};
use serde_json::json;

fn building_data() -> serde_json::Value {
    json!({
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "properties": { "height": 80, "base_height": 0, "name": "Tower A", "color": "#3b82f6" },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[
                        [24.938, 60.170], [24.940, 60.170],
                        [24.940, 60.171], [24.938, 60.171],
                        [24.938, 60.170]
                    ]]
                }
            },
            {
                "type": "Feature",
                "properties": { "height": 120, "base_height": 0, "name": "Tower B", "color": "#8b5cf6" },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[
                        [24.942, 60.170], [24.944, 60.170],
                        [24.944, 60.172], [24.942, 60.172],
                        [24.942, 60.170]
                    ]]
                }
            },
            {
                "type": "Feature",
                "properties": { "height": 50, "base_height": 0, "name": "Building C", "color": "#22c55e" },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[
                        [24.935, 60.168], [24.938, 60.168],
                        [24.938, 60.170], [24.935, 60.170],
                        [24.935, 60.168]
                    ]]
                }
            },
            {
                "type": "Feature",
                "properties": { "height": 200, "base_height": 0, "name": "Skyscraper D", "color": "#ef4444" },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[
                        [24.946, 60.169], [24.948, 60.169],
                        [24.948, 60.171], [24.946, 60.171],
                        [24.946, 60.169]
                    ]]
                }
            },
            {
                "type": "Feature",
                "properties": { "height": 30, "base_height": 0, "name": "Low E", "color": "#f59e0b" },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[
                        [24.940, 60.167], [24.944, 60.167],
                        [24.944, 60.169], [24.940, 60.169],
                        [24.940, 60.167]
                    ]]
                }
            }
        ]
    })
}

#[component]
pub fn Buildings() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut color_by_height = use_signal(|| true);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.170, 24.941),
                    zoom: 15.0,
                    pitch: 60.0,
                    bearing: -17.6,
                    on_ready: move |handle: MapHandle| {
                        handle.add_geojson_source("buildings", GeoJsonSourceOptions {
                            data: building_data(),
                            ..Default::default()
                        });

                        handle.add_layer(LayerOptions::fill_extrusion("buildings-3d", "buildings")
                            .paint(json!({
                                "fill-extrusion-color": [
                                    "interpolate", ["linear"], ["get", "height"],
                                    30, "#22c55e",
                                    80, "#3b82f6",
                                    120, "#8b5cf6",
                                    200, "#ef4444"
                                ],
                                "fill-extrusion-height": ["get", "height"],
                                "fill-extrusion-base": ["get", "base_height"],
                                "fill-extrusion-opacity": 0.85
                            }))
                        );

                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Buildings (Fill-Extrusion)" }
                p { "3D buildings with data-driven height and color." }
                p { style: "margin-top: 8px; font-size: 11px; color: #999;", "Pitch the map to see 3D effect." }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    "data-testid": "toggle-opacity",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        map.set_paint_property("buildings-3d", "fill-extrusion-opacity", json!(0.4));
                                    },
                                    "Reduce Opacity"
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            let using_height = color_by_height();
                            rsx! {
                                button {
                                    "data-testid": "toggle-color",
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #6366f1; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        if using_height {
                                            map.set_paint_property("buildings-3d", "fill-extrusion-color", json!(["get", "color"]));
                                        } else {
                                            map.set_paint_property("buildings-3d", "fill-extrusion-color", json!([
                                                "interpolate", ["linear"], ["get", "height"],
                                                30, "#22c55e",
                                                80, "#3b82f6",
                                                120, "#8b5cf6",
                                                200, "#ef4444"
                                            ]));
                                        }
                                        color_by_height.set(!using_height);
                                    },
                                    if using_height { "Color by Property" } else { "Color by Height" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
