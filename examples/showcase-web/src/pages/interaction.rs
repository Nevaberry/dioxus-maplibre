use dioxus::prelude::*;
use dioxus_maplibre::{
    FeatureId, FeatureIdentifier, GeoJsonSourceOptions, LatLng, LayerClickEvent, LayerHoverEvent,
    LayerOptions, LayerPressEvent, Map, MapHandle,
};
use serde_json::json;

#[component]
pub fn Interaction() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut clicked_feature = use_signal(|| None::<String>);
    let mut hovered_feature = use_signal(|| None::<String>);
    let mut pressed_feature = use_signal(|| None::<String>);
    let mut released_feature = use_signal(|| None::<String>);
    let mut prev_hover_id = use_signal(|| None::<FeatureId>);
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 12.0,
                    on_ready: move |handle: MapHandle| {
                        // Source with features that have numeric IDs
                        handle.add_geojson_source("interactive", GeoJsonSourceOptions {
                            data: json!({
                                "type": "FeatureCollection",
                                "features": [
                                    {"type": "Feature", "id": 1, "geometry": {"type": "Point", "coordinates": [24.94, 60.17]}, "properties": {"name": "Senate Square", "category": "landmark"}},
                                    {"type": "Feature", "id": 2, "geometry": {"type": "Point", "coordinates": [24.93, 60.18]}, "properties": {"name": "Temppeliaukion Church", "category": "landmark"}},
                                    {"type": "Feature", "id": 3, "geometry": {"type": "Point", "coordinates": [24.95, 60.16]}, "properties": {"name": "Suomenlinna", "category": "landmark"}},
                                    {"type": "Feature", "id": 4, "geometry": {"type": "Point", "coordinates": [24.98, 60.19]}, "properties": {"name": "Linnanmäki", "category": "amusement"}},
                                    {"type": "Feature", "id": 5, "geometry": {"type": "Point", "coordinates": [24.96, 60.175]}, "properties": {"name": "Central Station", "category": "transport"}},
                                ]
                            }),
                            ..Default::default()
                        });

                        // Circle layer with hover-aware styling
                        handle.add_layer(LayerOptions::circle("interactive-circles", "interactive")
                            .paint(json!({
                                "circle-radius": [
                                    "case",
                                    ["boolean", ["feature-state", "pressed"], false], 20,
                                    ["boolean", ["feature-state", "hover"], false], 16,
                                    10
                                ],
                                "circle-color": [
                                    "case",
                                    ["boolean", ["feature-state", "pressed"], false], "#ef4444",
                                    ["boolean", ["feature-state", "hover"], false], "#f59e0b",
                                    "#3b82f6"
                                ],
                                "circle-stroke-width": [
                                    "case", ["boolean", ["feature-state", "pressed"], false],
                                    4, 2
                                ],
                                "circle-stroke-color": "#fff"
                            }))
                        );

                        // Register interaction handlers
                        handle.on_layer_click("interactive-circles");
                        handle.on_layer_hover("interactive-circles");
                        handle.on_layer_press("interactive-circles");

                        map_handle.set(Some(handle));
                    },
                    on_layer_click: move |e: LayerClickEvent| {
                        let name = e.properties.get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown");
                        clicked_feature.set(Some(format!("{name} (id: {:?})", e.feature_id)));
                    },
                    on_layer_hover: move |e: LayerHoverEvent| {
                        let Some(ref map) = *map_handle.read() else { return };

                        // Remove previous hover state
                        if let Some(prev_id) = prev_hover_id() {
                            map.remove_feature_state(&FeatureIdentifier {
                                source: "interactive".into(),
                                id: prev_id,
                                source_layer: None,
                            });
                        }

                        // Set new hover state
                        if let Some(id) = e.feature_id {
                            map.set_feature_state(
                                &FeatureIdentifier {
                                    source: "interactive".into(),
                                    id: id.clone(),
                                    source_layer: None,
                                },
                                json!({"hover": true}),
                            );
                            prev_hover_id.set(Some(id));

                            let name = e.properties
                                .as_ref()
                                .and_then(|p| p.get("name"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown");
                            hovered_feature.set(Some(name.to_string()));
                        } else {
                            prev_hover_id.set(None);
                            hovered_feature.set(None);
                            pressed_feature.set(None);
                        }
                    },
                    on_layer_press: move |e: LayerPressEvent| {
                        let name = e.properties.get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown");

                        if let Some(id) = e.feature_id {
                            let feature = FeatureIdentifier::new("interactive", id);
                            if e.pressed {
                                if let Some(ref map) = *map_handle.read() {
                                    map.set_feature_state(&feature, json!({"pressed": true}));
                                }
                                released_feature.set(None);
                                pressed_feature.set(Some(name.to_string()));
                            } else {
                                if let Some(ref map) = *map_handle.read() {
                                    map.remove_feature_state_property(&feature, "pressed");
                                }
                                pressed_feature.set(None);
                                released_feature.set(Some(name.to_string()));
                            }
                        }
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Interaction" }
                p { "Hover over circles to see feature state changes." }
                p { "Press and hold to see mouse-down state, then release to click and select." }

                div { "data-testid": "interaction-sequence", style: "display: grid; gap: 5px; margin: 14px 0; color: #94a3b8;",
                    span { "1. Hover → orange" }
                    span { "2. Mouse down → red" }
                    span { "3. Mouse up → released" }
                    span { "4. Click → selected" }
                }

                if let Some(name) = hovered_feature() {
                    p { "data-testid": "hover-info",
                        span { style: "color: #f59e0b;", "Hovering: " }
                        "{name}"
                    }
                }

                if let Some(name) = pressed_feature() {
                    p { "data-testid": "press-info",
                        span { style: "color: #ef4444;", "Pressed: " }
                        "{name}"
                    }
                }

                if let Some(name) = released_feature() {
                    p { "data-testid": "release-info",
                        span { style: "color: #22c55e;", "Released: " }
                        "{name}"
                    }
                }

                if let Some(info) = clicked_feature() {
                    p { "data-testid": "click-info",
                        span { style: "color: #3b82f6;", "Clicked: " }
                        "{info}"
                    }
                }
            }
        }
    }
}
