use dioxus::prelude::*;
use dioxus_maplibre::{
    Map, MapHandle, GeoJsonSourceOptions, LayerOptions, LatLng,
};
use serde_json::json;

#[component]
pub fn Sources() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    center: LatLng::new(60.17, 24.94),
                    zoom: 11.0,
                    on_ready: move |handle: MapHandle| {
                        // GeoJSON source with clustering
                        handle.add_geojson_source("points", GeoJsonSourceOptions {
                            data: json!({
                                "type": "FeatureCollection",
                                "features": [
                                    {"type": "Feature", "geometry": {"type": "Point", "coordinates": [24.94, 60.17]}, "properties": {"name": "Helsinki Center"}},
                                    {"type": "Feature", "geometry": {"type": "Point", "coordinates": [24.93, 60.18]}, "properties": {"name": "Kallio"}},
                                    {"type": "Feature", "geometry": {"type": "Point", "coordinates": [24.95, 60.16]}, "properties": {"name": "Katajanokka"}},
                                    {"type": "Feature", "geometry": {"type": "Point", "coordinates": [24.92, 60.19]}, "properties": {"name": "Pasila"}},
                                    {"type": "Feature", "geometry": {"type": "Point", "coordinates": [24.88, 60.21]}, "properties": {"name": "Maunula"}},
                                    {"type": "Feature", "geometry": {"type": "Point", "coordinates": [25.08, 60.21]}, "properties": {"name": "Vuosaari"}},
                                    {"type": "Feature", "geometry": {"type": "Point", "coordinates": [24.96, 60.20]}, "properties": {"name": "Käpylä"}},
                                    {"type": "Feature", "geometry": {"type": "Point", "coordinates": [24.97, 60.15]}, "properties": {"name": "Laajasalo"}},
                                ]
                            }),
                            cluster: Some(true),
                            cluster_radius: Some(50),
                            cluster_max_zoom: Some(14),
                            ..Default::default()
                        });

                        // Clustered circles
                        handle.add_layer(LayerOptions::circle("clusters", "points")
                            .filter(json!(["has", "point_count"]))
                            .paint(json!({
                                "circle-color": [
                                    "step", ["get", "point_count"],
                                    "#51bbd6", 5, "#f1f075", 10, "#f28cb1"
                                ],
                                "circle-radius": [
                                    "step", ["get", "point_count"],
                                    20, 5, 30, 10, 40
                                ]
                            }))
                        );

                        // Cluster count labels
                        handle.add_layer(LayerOptions::symbol("cluster-count", "points")
                            .filter(json!(["has", "point_count"]))
                            .layout(json!({
                                "text-field": ["get", "point_count_abbreviated"],
                                "text-size": 12
                            }))
                        );

                        // Unclustered points
                        handle.add_layer(LayerOptions::circle("unclustered-point", "points")
                            .filter(json!(["!", ["has", "point_count"]]))
                            .paint(json!({
                                "circle-color": "#11b4da",
                                "circle-radius": 6,
                                "circle-stroke-width": 1,
                                "circle-stroke-color": "#fff"
                            }))
                        );

                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Sources" }
                p { "GeoJSON source with clustering." }
                p { "Zoom in/out to see clusters merge/split." }

                if let Some(ref map) = *map_handle.read() {
                    {
                        let map = map.clone();
                        rsx! {
                            button {
                                style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer; margin-top: 12px;",
                                onclick: move |_| {
                                    // Add more points dynamically
                                    map.update_geojson_source("points", json!({
                                        "type": "FeatureCollection",
                                        "features": [
                                            {"type": "Feature", "geometry": {"type": "Point", "coordinates": [24.94, 60.17]}, "properties": {"name": "Helsinki"}},
                                            {"type": "Feature", "geometry": {"type": "Point", "coordinates": [23.76, 61.50]}, "properties": {"name": "Tampere"}},
                                            {"type": "Feature", "geometry": {"type": "Point", "coordinates": [22.27, 60.45]}, "properties": {"name": "Turku"}},
                                            {"type": "Feature", "geometry": {"type": "Point", "coordinates": [25.47, 65.01]}, "properties": {"name": "Oulu"}},
                                            {"type": "Feature", "geometry": {"type": "Point", "coordinates": [27.68, 62.89]}, "properties": {"name": "Kuopio"}},
                                        ]
                                    }));
                                },
                                "Update to Finnish Cities"
                            }
                        }
                    }
                }
            }
        }
    }
}
