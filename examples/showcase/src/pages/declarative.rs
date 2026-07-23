use dioxus::prelude::*;
use dioxus_maplibre::{
    ControlOptions, GeoJsonSourceOptions, LatLng, LayerOptions, Map, MapControl, MapControlKind,
    MapLayer, MapMarker, MapPopup, MapSource, MapSourceKind, MarkerOptions, PopupOptions,
};
use serde_json::json;

#[component]
pub fn Declarative() -> Element {
    let style: Signal<String> = use_context();
    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 12.0,
                    MapSource {
                        id: "declarative-source",
                        source: MapSourceKind::GeoJson(GeoJsonSourceOptions {
                            data: json!({
                                "type": "FeatureCollection",
                                "features": [
                                    {"type":"Feature","geometry":{"type":"Point","coordinates":[24.94,60.17]},"properties":{"size":18}},
                                    {"type":"Feature","geometry":{"type":"Point","coordinates":[24.96,60.18]},"properties":{"size":12}}
                                ]
                            }),
                            ..Default::default()
                        }),
                        MapLayer {
                            options: LayerOptions::circle("declarative-layer", "declarative-source")
                                .paint(json!({
                                    "circle-radius": ["get", "size"],
                                    "circle-color": "#8b5cf6",
                                    "circle-stroke-color": "#ffffff",
                                    "circle-stroke-width": 2
                                })),
                        }
                    }
                    MapMarker {
                        id: "declarative-marker",
                        position: LatLng::new(60.165, 24.93),
                        options: MarkerOptions { emoji: Some("🦀".into()), ..Default::default() },
                    }
                    MapPopup {
                        id: "declarative-popup",
                        position: LatLng::new(60.18, 24.96),
                        html: "<strong>Declarative popup</strong><br>Mounted and cleaned up by Dioxus.",
                        options: PopupOptions { close_on_click: Some(false), ..Default::default() },
                    }
                    MapControl {
                        kind: MapControlKind::Navigation,
                        options: ControlOptions(json!({ "visualizePitch": true })),
                    }
                    MapControl { kind: MapControlKind::Globe }
                }
            }
            div { style: "width: 300px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Declarative Dioxus API" }
                p { "Sources, layers, markers, popups, and controls as lifecycle-aware child components." }
                p { "data-testid": "declarative-status", "All declarative object categories mounted" }
            }
        }
    }
}
