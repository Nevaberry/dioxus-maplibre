use dioxus::prelude::*;
use dioxus_maplibre::{
    Map, MapHandle, GeoJsonSourceOptions, LayerOptions, LatLng,
};
use serde_json::json;

#[component]
pub fn Layers() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut circle_visible = use_signal(|| true);

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    center: LatLng::new(60.17, 24.94),
                    zoom: 12.0,
                    on_ready: move |handle: MapHandle| {
                        // Points source
                        handle.add_geojson_source("demo-points", GeoJsonSourceOptions {
                            data: json!({
                                "type": "FeatureCollection",
                                "features": [
                                    {"type": "Feature", "id": 1, "geometry": {"type": "Point", "coordinates": [24.94, 60.17]}, "properties": {"name": "A", "size": 10}},
                                    {"type": "Feature", "id": 2, "geometry": {"type": "Point", "coordinates": [24.96, 60.18]}, "properties": {"name": "B", "size": 20}},
                                    {"type": "Feature", "id": 3, "geometry": {"type": "Point", "coordinates": [24.92, 60.16]}, "properties": {"name": "C", "size": 15}},
                                ]
                            }),
                            generate_id: Some(true),
                            ..Default::default()
                        });

                        // Circle layer
                        handle.add_layer(LayerOptions::circle("demo-circles", "demo-points")
                            .paint(json!({
                                "circle-radius": ["get", "size"],
                                "circle-color": "#3b82f6",
                                "circle-opacity": 0.7,
                                "circle-stroke-width": 2,
                                "circle-stroke-color": "#fff"
                            }))
                        );

                        // Line source
                        handle.add_geojson_source("route-line", GeoJsonSourceOptions {
                            data: json!({
                                "type": "Feature",
                                "geometry": {
                                    "type": "LineString",
                                    "coordinates": [
                                        [24.94, 60.17], [24.95, 60.175],
                                        [24.96, 60.18], [24.97, 60.178]
                                    ]
                                }
                            }),
                            ..Default::default()
                        });

                        // Line layer
                        handle.add_layer(LayerOptions::line("route", "route-line")
                            .paint(json!({
                                "line-color": "#ef4444",
                                "line-width": 4,
                                "line-dasharray": [2, 1]
                            }))
                            .layout(json!({
                                "line-cap": "round",
                                "line-join": "round"
                            }))
                        );

                        // Polygon source
                        handle.add_geojson_source("area-fill", GeoJsonSourceOptions {
                            data: json!({
                                "type": "Feature",
                                "geometry": {
                                    "type": "Polygon",
                                    "coordinates": [[
                                        [24.93, 60.165], [24.95, 60.165],
                                        [24.95, 60.175], [24.93, 60.175],
                                        [24.93, 60.165]
                                    ]]
                                }
                            }),
                            ..Default::default()
                        });

                        // Fill layer
                        handle.add_layer(LayerOptions::fill("area", "area-fill")
                            .paint(json!({
                                "fill-color": "#22c55e",
                                "fill-opacity": 0.3
                            }))
                        );

                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Layers" }
                p { "Circle, line, and fill layers." }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            let visible = circle_visible();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #6366f1; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        if visible {
                                            map.set_layout_property("demo-circles", "visibility", json!("none"));
                                        } else {
                                            map.set_layout_property("demo-circles", "visibility", json!("visible"));
                                        }
                                        circle_visible.set(!visible);
                                    },
                                    if visible { "Hide Circles" } else { "Show Circles" }
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #ef4444; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        map.set_paint_property("demo-circles", "circle-color", json!("#f59e0b"));
                                    },
                                    "Change Circle Color"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
