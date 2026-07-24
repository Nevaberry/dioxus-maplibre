use dioxus::prelude::*;
use dioxus_maplibre::{
    ControlOptions, ControlPosition, LatLng, Map, MapHandle, MapOptions, RasterDemSourceOptions,
    TerrainControlOptions,
};
use serde_json::json;

#[component]
pub fn Controls() -> Element {
    let style: Signal<String> = use_context();
    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 12.0,
                    options: MapOptions(json!({ "attributionControl": false, "maplibreLogo": false })),
                    on_ready: move |handle: MapHandle| {
                        handle.add_navigation_control_with_options(
                            ControlPosition::TopRight,
                            ControlOptions(json!({ "showZoom": true, "showCompass": true, "visualizePitch": true })),
                        );
                        handle.add_scale_control_with_options(
                            ControlPosition::BottomLeft,
                            ControlOptions(json!({ "maxWidth": 120, "unit": "metric" })),
                        );
                        handle.add_fullscreen_control(ControlPosition::TopLeft);
                        handle.add_geolocate_control(ControlPosition::TopRight);
                        handle.add_attribution_control_with_options(
                            ControlPosition::BottomRight,
                            ControlOptions(json!({ "compact": true, "customAttribution": "Dioxus showcase" })),
                        );
                        handle.add_globe_control(ControlPosition::TopLeft);
                        handle.add_logo_control(ControlPosition::BottomLeft, ControlOptions::default());
                        handle.add_raster_dem_source("control-dem", RasterDemSourceOptions {
                            tiles: Some(vec!["https://demotiles.maplibre.org/terrain-tiles/{z}/{x}/{y}.png".into()]),
                            tile_size: Some(256),
                            max_zoom: Some(12),
                            bounds: Some([11.0, 47.0, 12.0, 48.0]),
                            attribution: Some("AW3D30 (JAXA)".into()),
                            ..Default::default()
                        });
                        handle.add_terrain_control(ControlPosition::TopLeft, TerrainControlOptions {
                            source: "control-dem".into(),
                            exaggeration: Some(1.2),
                        });
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Controls" }
                p { "Navigation control: top-right" }
                p { "Scale control: bottom-left" }
                p { "Fullscreen control: top-left" }
                p { "Geolocate control: top-right" }
                p { "Attribution, Globe, Logo, and Terrain controls included." }
            }
        }
    }
}
