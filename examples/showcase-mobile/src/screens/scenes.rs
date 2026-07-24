use dioxus::prelude::*;
use dioxus_maplibre::{
    FeatureIdentifier, GeoJsonSourceOptions, LatLng, LayerClickEvent, LayerOptions,
    LayerPressEvent, Map, MapErrorEvent, MapHandle, MapOptions, RasterDemSourceOptions, SkyOptions,
    TerrainOptions,
};
use serde_json::json;

use crate::components::{MapControlStack, SheetHandle};
use crate::data;
use crate::state::{AppTab, Connectivity, LabPanel, MobileContext, SceneKind};

#[component]
pub fn ScenesScreen() -> Element {
    let state = use_context::<MobileContext>();
    let scene = state.scene();

    rsx! {
        section { class: "screen map-screen", "data-testid": "scenes-screen",
            div { class: "scene-heading",
                span { class: "eyebrow", "{scene.eyebrow()}" }
                h1 { "{scene_title(scene)}" }
            }

            div { class: "scene-rail glass", role: "tablist", "aria-label": "Map scenes",
                for option in SceneKind::ALL {
                    button {
                        key: "{option:?}",
                        role: "tab",
                        class: if scene == option { "active" } else { "" },
                        "aria-selected": scene == option,
                        "data-testid": "scene-{option.label().to_ascii_lowercase()}",
                        onclick: move |_| state.set_scene(option),
                        "{option.label()}"
                    }
                }
            }

            div { class: "map-stage", key: "scene-{scene:?}",
                match scene {
                    SceneKind::Helsinki => rsx! { HelsinkiScene {} },
                    SceneKind::Buildings => rsx! { BuildingsScene {} },
                    SceneKind::Matterhorn => rsx! { MatterhornScene {} },
                    SceneKind::Tokyo => rsx! { TokyoScene {} },
                }
            }
        }
    }
}

const fn scene_title(scene: SceneKind) -> &'static str {
    match scene {
        SceneKind::Helsinki => "Explore Helsinki",
        SceneKind::Buildings => "Helsinki in 3D",
        SceneKind::Matterhorn => "Matterhorn",
        SceneKind::Tokyo => "Tokyo signals",
    }
}

fn open_layers(state: MobileContext) {
    state.set_lab_panel(LabPanel::Layers);
    state.set_tab(AppTab::Lab);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FeatureSelection {
    name: String,
    category: String,
    detail: String,
}

#[component]
fn HelsinkiScene() -> Element {
    let state = use_context::<MobileContext>();
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut selection = use_signal(|| None::<FeatureSelection>);
    let mut map_error = use_signal(|| None::<String>);
    let style = data::scene_style(SceneKind::Helsinki, state.connectivity());

    rsx! {
        Map {
            style,
            center: data::scene_center(SceneKind::Helsinki),
            zoom: 12.2,
            pitch: 18.0,
            options: MapOptions(json!({ "attributionControl": false, "maxPitch": 75 })),
            on_ready: move |handle: MapHandle| {
                handle.add_geojson_source("mobile-places", GeoJsonSourceOptions {
                    data: data::helsinki_places(),
                    cluster: Some(true),
                    cluster_radius: Some(48),
                    cluster_max_zoom: Some(14),
                    ..Default::default()
                });
                handle.add_layer(LayerOptions::circle("place-clusters", "mobile-places")
                    .filter(json!(["has", "point_count"]))
                    .paint(json!({
                        "circle-radius": ["step", ["get", "point_count"], 18, 5, 23, 10, 29],
                        "circle-color": ["step", ["get", "point_count"], "#0f8298", 5, "#0d9488", 10, "#d99b2b"],
                        "circle-stroke-width": 2,
                        "circle-stroke-color": "rgba(220,255,255,0.72)"
                    })));
                handle.add_layer(LayerOptions::circle("place-points", "mobile-places")
                    .filter(json!(["!", ["has", "point_count"]]))
                    .paint(json!({
                        "circle-radius": ["interpolate", ["linear"], ["get", "weight"], 3, 6, 18, 11],
                        "circle-color": "#6ee7e0",
                        "circle-stroke-width": 2,
                        "circle-stroke-color": "#072235"
                    })));
                handle.on_layer_click("place-clusters");
                handle.on_layer_click("place-points");
                map_handle.set(Some(handle));
            },
            on_error: move |error: MapErrorEvent| {
                map_error.set(Some(error.message.unwrap_or_else(|| "Map unavailable".into())));
            },
            on_layer_click: move |event: LayerClickEvent| {
                if event.layer_id == "place-clusters" {
                    let count = event.properties.get("point_count")
                        .and_then(serde_json::Value::as_u64)
                        .unwrap_or(0);
                    selection.set(Some(FeatureSelection {
                        name: format!("{count} places"),
                        category: "Cluster".into(),
                        detail: "Tap closer to inspect individual features".into(),
                    }));
                } else {
                    selection.set(Some(FeatureSelection {
                        name: property(&event.properties, "name", "Helsinki place"),
                        category: property(&event.properties, "category", "Feature"),
                        detail: format!("{:.4}, {:.4}", event.latlng.lat, event.latlng.lng),
                    }));
                }
            },
        }

        MapControlStack {
            on_compass: move |_| {
                if let Some(map) = map_handle.read().as_ref() {
                    map.reset_north_pitch();
                }
            },
            on_layers: move |_| open_layers(state),
        }

        if let Some(message) = map_error() {
            div { class: "map-error glass", "data-testid": "scene-map-error", "{message}" }
        }

        div { class: "bottom-sheet compact-sheet", "data-testid": "helsinki-sheet",
            SheetHandle {}
            if let Some(feature) = selection() {
                div { class: "sheet-kicker", "{feature.category}" }
                h2 { "{feature.name}" }
                p { "{feature.detail}" }
            } else {
                div { class: "sheet-kicker", "Live places" }
                h2 { "Clusters & symbols" }
                p { "Tap a cluster or point to inspect source data." }
            }
        }
    }
}

#[component]
fn BuildingsScene() -> Element {
    let state = use_context::<MobileContext>();
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut selection = use_signal(|| FeatureSelection {
        name: "Helsinki Cathedral".into(),
        category: "Landmark".into(),
        detail: "Height 62 m".into(),
    });
    let mut pressed_id = use_signal(|| None::<dioxus_maplibre::FeatureId>);
    let style = data::scene_style(SceneKind::Buildings, state.connectivity());

    rsx! {
        Map {
            style,
            center: LatLng::new(60.1694, 24.9470),
            zoom: 14.2,
            pitch: 62.0,
            bearing: -18.0,
            options: MapOptions(json!({ "attributionControl": false, "maxPitch": 85 })),
            on_ready: move |handle: MapHandle| {
                handle.add_geojson_source("mobile-buildings", GeoJsonSourceOptions {
                    data: data::building_data(),
                    ..Default::default()
                });
                handle.add_layer(LayerOptions::fill_extrusion("mobile-buildings-3d", "mobile-buildings")
                    .paint(json!({
                        "fill-extrusion-color": [
                            "case",
                            ["boolean", ["feature-state", "pressed"], false], "#f5ad3d",
                            ["interpolate", ["linear"], ["get", "height"], 20, "#35657c", 85, "#69d3cc"]
                        ],
                        "fill-extrusion-height": ["get", "height"],
                        "fill-extrusion-base": ["get", "base_height"],
                        "fill-extrusion-opacity": 0.9
                    })));
                handle.on_layer_click("mobile-buildings-3d");
                handle.on_layer_press("mobile-buildings-3d");
                map_handle.set(Some(handle));
            },
            on_layer_press: move |event: LayerPressEvent| {
                let Some(id) = event.feature_id else { return };
                let feature = FeatureIdentifier::new("mobile-buildings", id.clone());
                if let Some(map) = map_handle.read().as_ref() {
                    if event.pressed {
                        map.set_feature_state(&feature, json!({ "pressed": true }));
                        pressed_id.set(Some(id));
                    } else {
                        map.remove_feature_state_property(&feature, "pressed");
                        pressed_id.set(None);
                    }
                }
            },
            on_layer_click: move |event: LayerClickEvent| {
                let height = event.properties.get("height")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(0);
                selection.set(FeatureSelection {
                    name: property(&event.properties, "name", "Building"),
                    category: property(&event.properties, "category", "Building"),
                    detail: format!("Height {height} m"),
                });
            },
        }

        MapControlStack {
            on_compass: move |_| {
                if let Some(map) = map_handle.read().as_ref() { map.reset_north_pitch(); }
            },
            on_layers: move |_| open_layers(state),
        }

        div { class: "bottom-sheet feature-sheet", "data-testid": "building-details",
            SheetHandle {}
            div { class: "sheet-kicker", "Feature details" }
            h2 { "{selection().name}" }
            div { class: "feature-detail-grid",
                span { "{selection().category}" }
                span { "{selection().detail}" }
                span { class: "accent", if pressed_id().is_some() { "Pressed" } else { "Query ready" } }
            }
        }
    }
}

#[component]
fn MatterhornScene() -> Element {
    let mut state = use_context::<MobileContext>();
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut exaggeration = use_signal(|| 1.4_f64);
    let connectivity = state.connectivity();
    let terrain_pack_ready = state.offline.read().matterhorn_ready;
    let terrain_available = connectivity == Connectivity::Online || terrain_pack_ready;
    let style = data::scene_style(SceneKind::Matterhorn, connectivity);

    rsx! {
        Map {
            style,
            center: data::scene_center(SceneKind::Matterhorn),
            zoom: 11.1,
            pitch: 70.0,
            bearing: 24.0,
            max_bounds: dioxus_maplibre::Bounds::new(
                LatLng::new(45.86, 7.48),
                LatLng::new(46.08, 7.83),
            ),
            options: MapOptions(json!({ "attributionControl": false, "maxPitch": 85, "terrainSkirtLength": "auto" })),
            on_ready: move |handle: MapHandle| {
                handle.add_geojson_source("mobile-contours", GeoJsonSourceOptions {
                    data: data::terrain_contours(),
                    ..Default::default()
                });
                handle.add_layer(LayerOptions::line("mobile-contours-line", "mobile-contours")
                    .paint(json!({
                        "line-color": "rgba(202, 235, 228, 0.55)",
                        "line-width": 1.25,
                        "line-dasharray": [2, 2]
                    })));
                if terrain_available {
                    handle.add_raster_dem_source("mobile-terrain", RasterDemSourceOptions {
                        tiles: Some(vec![if connectivity == Connectivity::Online {
                            data::TERRAIN_TILES.into()
                        } else {
                            data::OFFLINE_TERRAIN_TILE.into()
                        }]),
                        tile_size: Some(256),
                        encoding: Some("terrarium".into()),
                        max_zoom: Some(13),
                        bounds: Some([7.48, 45.86, 7.83, 46.08]),
                        attribution: Some("Elevation tiles: AWS Open Data".into()),
                        ..Default::default()
                    });
                    handle.set_terrain(TerrainOptions {
                        source: "mobile-terrain".into(),
                        exaggeration: Some(exaggeration()),
                    });
                }
                handle.set_sky(SkyOptions(json!({
                    "sky-color": "#10283a",
                    "horizon-color": "#91a4aa",
                    "fog-color": "#cbd5d1",
                    "fog-ground-blend": 0.68,
                    "horizon-fog-blend": 0.55,
                    "atmosphere-blend": 0.75
                })));
                map_handle.set(Some(handle));
            },
        }

        MapControlStack {
            on_compass: move |_| {
                if let Some(map) = map_handle.read().as_ref() { map.reset_north_pitch(); }
            },
            on_layers: move |_| open_layers(state),
        }

        div { class: "location-pin", span { "▲" } strong { "Matterhorn" } }

        div { class: "bottom-sheet terrain-sheet", "data-testid": "terrain-sheet",
            SheetHandle {}
            div { class: "sheet-title-row",
                div { span { class: "sheet-kicker", "Terrain" } h2 { "Exaggeration" } }
                strong { "{exaggeration():.1}×" }
            }
            input {
                "data-testid": "terrain-exaggeration",
                class: "range-control",
                r#type: "range",
                min: "0.5",
                max: "3",
                step: "0.1",
                value: "{exaggeration()}",
                disabled: !terrain_available,
                oninput: move |event| {
                    if let Ok(value) = event.value().parse::<f64>() {
                        exaggeration.set(value);
                        if let Some(map) = map_handle.read().as_ref() {
                            map.set_terrain(TerrainOptions {
                                source: "mobile-terrain".into(),
                                exaggeration: Some(value),
                            });
                        }
                    }
                },
            }
            if !terrain_available {
                button {
                    class: "inline-notice",
                    onclick: move |_| {
                        state.set_tab(AppTab::Offline);
                        let mut offline = state.offline.write();
                        offline.selected_pack = crate::state::PackId::Matterhorn;
                        offline.view = crate::state::OfflineView::SelectArea;
                    },
                    "Download the terrain pack for offline elevation"
                }
            }
        }
    }
}

#[component]
fn TokyoScene() -> Element {
    let state = use_context::<MobileContext>();
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut heatmap_visible = use_signal(|| true);
    let mut clusters_visible = use_signal(|| true);
    let style = data::scene_style(SceneKind::Tokyo, state.connectivity());

    rsx! {
        Map {
            style,
            center: data::scene_center(SceneKind::Tokyo),
            zoom: 11.7,
            pitch: 18.0,
            options: MapOptions(json!({ "attributionControl": false })),
            on_ready: move |handle: MapHandle| {
                let points = data::tokyo_heat_data();
                handle.add_geojson_source("tokyo-heat", GeoJsonSourceOptions {
                    data: points.clone(),
                    ..Default::default()
                });
                handle.add_layer(LayerOptions::heatmap("tokyo-heat-layer", "tokyo-heat")
                    .paint(json!({
                        "heatmap-weight": ["get", "weight"],
                        "heatmap-intensity": 1.15,
                        "heatmap-radius": 27,
                        "heatmap-opacity": 0.86,
                        "heatmap-color": [
                            "interpolate", ["linear"], ["heatmap-density"],
                            0, "rgba(0,0,0,0)", 0.2, "#0b6e9c", 0.45, "#22c55e",
                            0.7, "#facc15", 0.88, "#f97316", 1, "#ef4444"
                        ]
                    })));
                handle.add_geojson_source("tokyo-clusters", GeoJsonSourceOptions {
                    data: points,
                    cluster: Some(true),
                    cluster_radius: Some(42),
                    cluster_max_zoom: Some(15),
                    ..Default::default()
                });
                handle.add_layer(LayerOptions::circle("tokyo-cluster-layer", "tokyo-clusters")
                    .filter(json!(["has", "point_count"]))
                    .paint(json!({
                        "circle-radius": ["step", ["get", "point_count"], 13, 12, 18, 30, 23],
                        "circle-color": "rgba(9, 64, 76, 0.82)",
                        "circle-stroke-width": 2,
                        "circle-stroke-color": "#65e4de"
                    })));
                map_handle.set(Some(handle));
            },
        }

        MapControlStack {
            on_compass: move |_| {
                if let Some(map) = map_handle.read().as_ref() { map.reset_north_pitch(); }
            },
            on_layers: move |_| open_layers(state),
        }

        div { class: "heat-legend glass", span { "High" } i {} span { "Low" } }

        div { class: "bottom-sheet data-sheet", "data-testid": "tokyo-data-sheet",
            SheetHandle {}
            div { class: "sheet-kicker", "Live data" }
            LayerToggle {
                label: "Heatmap",
                active: heatmap_visible(),
                test_id: "toggle-heatmap",
                onclick: move |_| {
                    let visible = !heatmap_visible();
                    heatmap_visible.set(visible);
                    if let Some(map) = map_handle.read().as_ref() {
                        map.set_layout_property("tokyo-heat-layer", "visibility", json!(if visible { "visible" } else { "none" }));
                    }
                },
            }
            LayerToggle {
                label: "Clusters",
                active: clusters_visible(),
                test_id: "toggle-clusters",
                onclick: move |_| {
                    let visible = !clusters_visible();
                    clusters_visible.set(visible);
                    if let Some(map) = map_handle.read().as_ref() {
                        map.set_layout_property("tokyo-cluster-layer", "visibility", json!(if visible { "visible" } else { "none" }));
                    }
                },
            }
        }
    }
}

#[component]
fn LayerToggle(
    label: &'static str,
    active: bool,
    test_id: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "layer-toggle-row",
            "data-testid": test_id,
            "aria-pressed": active,
            onclick: move |event| onclick.call(event),
            span { class: "layer-swatch" }
            strong { "{label}" }
            span { class: if active { "switch on" } else { "switch" }, i {} }
        }
    }
}

fn property(properties: &serde_json::Value, key: &str, fallback: &str) -> String {
    properties
        .get(key)
        .and_then(serde_json::Value::as_str)
        .unwrap_or(fallback)
        .to_string()
}
