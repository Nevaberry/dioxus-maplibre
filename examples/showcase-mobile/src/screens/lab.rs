use dioxus::prelude::*;
use dioxus_maplibre::{
    FeatureIdentifier, FogOptions, GeoJsonSourceOptions, LatLng, LayerClickEvent, LayerHoverEvent,
    LayerOptions, LayerPressEvent, Map, MapHandle, MapOptions, MissingImageOptions,
    ProjectionOptions, RasterDemSourceOptions, TerrainOptions,
};
use serde_json::json;

use crate::components::{AppIcon, BackButton, IconKind, SheetHandle};
use crate::data;
use crate::state::{Connectivity, LabPanel, MobileContext};

#[component]
pub fn LabScreen() -> Element {
    let state = use_context::<MobileContext>();

    match state.lab_panel() {
        LabPanel::Home => rsx! { LabHome {} },
        LabPanel::Layers => rsx! { LayersPanel {} },
        LabPanel::Interaction => rsx! { InteractionPanel {} },
        LabPanel::Camera => rsx! { CameraPanel {} },
    }
}

#[component]
fn LabHome() -> Element {
    let state = use_context::<MobileContext>();

    rsx! {
        section { class: "screen content-screen lab-home", "data-testid": "lab-screen",
            header { class: "content-header",
                span { class: "eyebrow", "MapLibre feature lab" }
                h1 { "Experiment" }
                p { "Every control drives the real map through dioxus-maplibre." }
            }

            div { class: "feature-card-grid",
                FeatureCard {
                    title: "Layers",
                    detail: "Buildings, heatmap, symbols, terrain & fog",
                    icon: IconKind::Layers,
                    test_id: "lab-layers",
                    accent: "teal",
                    onclick: move |_| state.set_lab_panel(LabPanel::Layers),
                }
                FeatureCard {
                    title: "Interaction",
                    detail: "Hover, press, release, tap & feature state",
                    icon: IconKind::Events,
                    test_id: "lab-interaction",
                    accent: "amber",
                    onclick: move |_| state.set_lab_panel(LabPanel::Interaction),
                }
                FeatureCard {
                    title: "Camera & style",
                    detail: "Pitch, bearing, roll, projection & themes",
                    icon: IconKind::Camera,
                    test_id: "lab-camera",
                    accent: "violet",
                    onclick: move |_| state.set_lab_panel(LabPanel::Camera),
                }
                FeatureCard {
                    title: "Offline packs",
                    detail: "Cache regions, styles, data and elevation",
                    icon: IconKind::Download,
                    test_id: "lab-offline",
                    accent: "blue",
                    onclick: move |_| state.set_tab(crate::state::AppTab::Offline),
                }
            }

            div { class: "lab-capability-strip glass",
                span { "Dioxus 0.7" }
                i {}
                span { "MapLibre 6" }
                i {}
                span { "Touch ready" }
            }
        }
    }
}

#[component]
fn FeatureCard(
    title: &'static str,
    detail: &'static str,
    icon: IconKind,
    test_id: &'static str,
    accent: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "feature-card {accent}",
            "data-testid": test_id,
            onclick: move |event| onclick.call(event),
            span { class: "feature-card-icon", AppIcon { kind: icon } }
            span { class: "feature-card-copy", strong { "{title}" } small { "{detail}" } }
            AppIcon { kind: IconKind::ChevronRight }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LabLayer {
    Buildings,
    Heatmap,
    Symbols,
    Terrain,
    Fog,
}

impl LabLayer {
    const ALL: [Self; 5] = [
        Self::Buildings,
        Self::Heatmap,
        Self::Symbols,
        Self::Terrain,
        Self::Fog,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Buildings => "3D buildings",
            Self::Heatmap => "Heatmap",
            Self::Symbols => "Symbols",
            Self::Terrain => "Terrain",
            Self::Fog => "Fog",
        }
    }

    const fn test_id(self) -> &'static str {
        match self {
            Self::Buildings => "layer-buildings",
            Self::Heatmap => "layer-heatmap",
            Self::Symbols => "layer-symbols",
            Self::Terrain => "layer-terrain",
            Self::Fog => "layer-fog",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
struct LayerVisibility {
    buildings: bool,
    heatmap: bool,
    symbols: bool,
    terrain: bool,
    fog: bool,
}

impl Default for LayerVisibility {
    fn default() -> Self {
        Self {
            buildings: true,
            heatmap: true,
            symbols: true,
            terrain: false,
            fog: true,
        }
    }
}

impl LayerVisibility {
    const fn get(self, layer: LabLayer) -> bool {
        match layer {
            LabLayer::Buildings => self.buildings,
            LabLayer::Heatmap => self.heatmap,
            LabLayer::Symbols => self.symbols,
            LabLayer::Terrain => self.terrain,
            LabLayer::Fog => self.fog,
        }
    }

    fn toggle(&mut self, layer: LabLayer) -> bool {
        let value = !self.get(layer);
        match layer {
            LabLayer::Buildings => self.buildings = value,
            LabLayer::Heatmap => self.heatmap = value,
            LabLayer::Symbols => self.symbols = value,
            LabLayer::Terrain => self.terrain = value,
            LabLayer::Fog => self.fog = value,
        }
        value
    }
}

#[component]
fn LayersPanel() -> Element {
    let state = use_context::<MobileContext>();
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut visibility = use_signal(LayerVisibility::default);
    let style = if state.connectivity() == Connectivity::Online {
        data::ONLINE_DARK_STYLE
    } else {
        "/offline/helsinki-style.json"
    };

    rsx! {
        section { class: "screen map-screen lab-map-screen", "data-testid": "layers-panel",
            PanelHeading { title: "Layers", subtitle: "Compose the map", state }

            div { class: "map-stage lab-map-stage",
                Map {
                    style,
                    center: LatLng::new(60.1699, 24.943),
                    zoom: 13.3,
                    pitch: 52.0,
                    bearing: -14.0,
                    options: MapOptions(json!({ "attributionControl": false, "maxPitch": 85 })),
                    on_ready: move |handle: MapHandle| {
                        handle.set_missing_image_resolver(MissingImageOptions {
                            primary_color: "#6ee7e0".into(),
                            secondary_color: "#0b5363".into(),
                            ..Default::default()
                        });
                        handle.add_geojson_source("lab-buildings", GeoJsonSourceOptions {
                            data: data::building_data(),
                            ..Default::default()
                        });
                        handle.add_layer(LayerOptions::fill_extrusion("lab-buildings-layer", "lab-buildings")
                            .paint(json!({
                                "fill-extrusion-color": "#54b7b2",
                                "fill-extrusion-height": ["get", "height"],
                                "fill-extrusion-base": ["get", "base_height"],
                                "fill-extrusion-opacity": 0.76
                            })));
                        handle.add_geojson_source("lab-heat", GeoJsonSourceOptions {
                            data: data::helsinki_places(),
                            ..Default::default()
                        });
                        handle.add_layer(LayerOptions::heatmap("lab-heat-layer", "lab-heat")
                            .paint(json!({
                                "heatmap-weight": ["/", ["get", "weight"], 18],
                                "heatmap-radius": 38,
                                "heatmap-opacity": 0.7,
                                "heatmap-color": ["interpolate", ["linear"], ["heatmap-density"],
                                    0, "rgba(0,0,0,0)", 0.35, "#0891b2", 0.72, "#facc15", 1, "#f97316"]
                            })));
                        let symbol_map = handle.clone();
                        spawn(async move {
                            let _ = symbol_map
                                .load_image_async("mobile-pin", "/offline/mobile-pin.png")
                                .await;
                            symbol_map.add_layer(
                                LayerOptions::symbol("lab-symbol-layer", "lab-heat").layout(json!({
                                    "icon-image": "mobile-pin",
                                    "icon-size": 0.58,
                                    "icon-allow-overlap": true
                                })),
                            );
                        });
                        handle.add_raster_dem_source("lab-dem", RasterDemSourceOptions {
                            tiles: Some(vec!["/offline/terrain-tile.png".into()]),
                            tile_size: Some(256),
                            encoding: Some("terrarium".into()),
                            max_zoom: Some(13),
                            ..Default::default()
                        });
                        handle.set_fog(FogOptions(json!({
                            "fog-color": "#8eb1b4",
                            "fog-ground-blend": 0.38,
                            "horizon-fog-blend": 0.25,
                            "atmosphere-blend": 0.45
                        })));
                        map_handle.set(Some(handle));
                    },
                }
            }

            div { class: "bottom-sheet layer-sheet", "data-testid": "layer-controls",
                SheetHandle {}
                div { class: "sheet-kicker", "Runtime style API" }
                h2 { "Visible layers" }
                div { class: "layer-control-list",
                    for layer in LabLayer::ALL {
                        button {
                            key: "{layer:?}",
                            class: "layer-control-row",
                            "data-testid": layer.test_id(),
                            "aria-pressed": visibility().get(layer),
                            onclick: move |_| {
                                let enabled = visibility.write().toggle(layer);
                                if let Some(map) = map_handle.read().as_ref() {
                                    match layer {
                                        LabLayer::Buildings => set_layer_visible(map, "lab-buildings-layer", enabled),
                                        LabLayer::Heatmap => set_layer_visible(map, "lab-heat-layer", enabled),
                                        LabLayer::Symbols => set_layer_visible(map, "lab-symbol-layer", enabled),
                                        LabLayer::Terrain if enabled => map.set_terrain(TerrainOptions {
                                            source: "lab-dem".into(),
                                            exaggeration: Some(0.42),
                                        }),
                                        LabLayer::Terrain => map.remove_terrain(),
                                        LabLayer::Fog if enabled => map.set_fog(FogOptions(json!({
                                            "fog-color": "#8eb1b4", "fog-ground-blend": 0.38,
                                            "horizon-fog-blend": 0.25, "atmosphere-blend": 0.45
                                        }))),
                                        LabLayer::Fog => map.remove_fog(),
                                    }
                                }
                            },
                            span { class: "layer-order-grip", "⋮⋮" }
                            strong { "{layer.label()}" }
                            span { class: if visibility().get(layer) { "switch on" } else { "switch" }, i {} }
                        }
                    }
                }
            }
        }
    }
}

fn set_layer_visible(map: &MapHandle, id: &str, visible: bool) {
    map.set_layout_property(
        id,
        "visibility",
        json!(if visible { "visible" } else { "none" }),
    );
}

#[derive(Debug, Clone, PartialEq)]
struct EventLine {
    action: &'static str,
    feature: String,
    detail: String,
}

impl EventLine {
    fn new(action: &'static str, properties: &serde_json::Value, detail: String) -> Self {
        Self {
            action,
            feature: properties
                .get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("Map feature")
                .to_string(),
            detail,
        }
    }
}

#[component]
fn InteractionPanel() -> Element {
    let state = use_context::<MobileContext>();
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut pressed = use_signal(|| None::<dioxus_maplibre::FeatureId>);
    let mut events = use_signal(Vec::<EventLine>::new);
    let style = if state.connectivity() == Connectivity::Online {
        data::ONLINE_DARK_STYLE
    } else {
        "/offline/helsinki-style.json"
    };

    rsx! {
        section { class: "screen map-screen lab-map-screen", "data-testid": "interaction-panel",
            PanelHeading { title: "Interaction", subtitle: "Touch the districts", state }

            div { class: "map-stage interaction-map-stage",
                Map {
                    style,
                    center: LatLng::new(60.1698, 24.945),
                    zoom: 13.1,
                    options: MapOptions(json!({ "attributionControl": false })),
                    on_ready: move |handle: MapHandle| {
                        handle.add_geojson_source("interaction-areas", GeoJsonSourceOptions {
                            data: data::interaction_areas(),
                            ..Default::default()
                        });
                        handle.add_layer(LayerOptions::fill("interaction-fill", "interaction-areas")
                            .paint(json!({
                                "fill-color": ["case", ["boolean", ["feature-state", "pressed"], false], "#f4ad42", "#20b8ad"],
                                "fill-opacity": ["case", ["boolean", ["feature-state", "hover"], false], 0.64, 0.38]
                            })));
                        handle.add_layer(LayerOptions::line("interaction-line", "interaction-areas")
                            .paint(json!({ "line-color": "#9ff7ee", "line-width": 2.5 })));
                        handle.on_layer_hover("interaction-fill");
                        handle.on_layer_press("interaction-fill");
                        handle.on_layer_click("interaction-fill");
                        map_handle.set(Some(handle));
                    },
                    on_layer_hover: move |event: LayerHoverEvent| {
                        let Some(id) = event.feature_id else { return };
                        if let Some(map) = map_handle.read().as_ref() {
                            map.set_feature_state(
                                &FeatureIdentifier::new("interaction-areas", id),
                                json!({ "hover": true }),
                            );
                        }
                        if let Some(properties) = event.properties.as_ref() {
                            push_event(&mut events, EventLine::new(
                                "HOVER",
                                properties,
                                format!("{:.3}, {:.3}", event.latlng.lat, event.latlng.lng),
                            ));
                        }
                    },
                    on_layer_press: move |event: LayerPressEvent| {
                        let action = if event.pressed { "PRESS" } else { "RELEASE" };
                        if let Some(id) = event.feature_id.clone()
                            && let Some(map) = map_handle.read().as_ref()
                        {
                            let feature = FeatureIdentifier::new("interaction-areas", id.clone());
                            if event.pressed {
                                map.set_feature_state(&feature, json!({ "pressed": true }));
                                pressed.set(Some(id));
                            } else {
                                map.remove_feature_state_property(&feature, "pressed");
                                pressed.set(None);
                            }
                        }
                        push_event(&mut events, EventLine::new(
                            action,
                            &event.properties,
                            format!("pointer {:.0} × {:.0}", event.cursor_x, event.cursor_y),
                        ));
                    },
                    on_layer_click: move |event: LayerClickEvent| {
                        push_event(&mut events, EventLine::new(
                            "TAP",
                            &event.properties,
                            format!("{:.4}, {:.4}", event.latlng.lat, event.latlng.lng),
                        ));
                    },
                }
                div { class: if pressed().is_some() { "touch-target pressed" } else { "touch-target" },
                    span {}
                    small { if pressed().is_some() { "Pressed" } else { "Press a district" } }
                }
            }

            div { class: "bottom-sheet event-sheet", "data-testid": "event-console",
                SheetHandle {}
                div { class: "sheet-title-row",
                    div { span { class: "sheet-kicker", "Event stream" } h2 { "Interaction console" } }
                    button { onclick: move |_| events.set(Vec::new()), "Clear" }
                }
                if events().is_empty() {
                    div { class: "empty-events",
                        AppIcon { kind: IconKind::Events }
                        p { "Hover with a mouse or press, release and tap on touch." }
                    }
                } else {
                    ol { class: "event-list",
                        for (index, event) in events().iter().enumerate().rev() {
                            li { key: "{index}-{event.action}-{event.feature}",
                                strong { "{event.action}" }
                                span { "{event.feature}" }
                                small { "{event.detail}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn push_event(events: &mut Signal<Vec<EventLine>>, event: EventLine) {
    let mut list = events.write();
    list.push(event);
    if list.len() > 8 {
        list.remove(0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapTheme {
    Dark,
    Light,
}

#[component]
fn CameraPanel() -> Element {
    let state = use_context::<MobileContext>();
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut pitch = use_signal(|| 42.0_f64);
    let mut bearing = use_signal(|| -20.0_f64);
    let mut roll = use_signal(|| 0.0_f64);
    let mut globe = use_signal(|| false);
    let mut theme = use_signal(|| MapTheme::Dark);
    let style = match (state.connectivity(), theme()) {
        (Connectivity::Online, MapTheme::Light) => data::ONLINE_LIGHT_STYLE,
        (Connectivity::Online, MapTheme::Dark) => data::ONLINE_DARK_STYLE,
        (Connectivity::Offline, _) => "/offline/helsinki-style.json",
    };

    rsx! {
        section { class: "screen map-screen lab-map-screen", "data-testid": "camera-panel",
            PanelHeading { title: "Camera & style", subtitle: "Shape the viewport", state }

            div { class: "map-stage camera-map-stage", key: "camera-{theme():?}-{state.connectivity():?}",
                Map {
                    style,
                    center: LatLng::new(60.1699, 24.943),
                    zoom: 12.6,
                    pitch: pitch(),
                    bearing: bearing(),
                    options: MapOptions(json!({ "attributionControl": false, "maxPitch": 85, "rollEnabled": true })),
                    on_ready: move |handle: MapHandle| {
                        handle.set_roll(roll());
                        if globe() {
                            handle.set_projection(ProjectionOptions::globe());
                        }
                        map_handle.set(Some(handle));
                    },
                }
            }

            div { class: "bottom-sheet camera-sheet", "data-testid": "camera-controls",
                SheetHandle {}
                div { class: "sheet-kicker", "Camera API" }
                h2 { "Perspective" }
                CameraRange { label: "Pitch", value: pitch(), min: 0.0, max: 85.0, test_id: "camera-pitch",
                    oninput: move |value| { pitch.set(value); if let Some(map) = map_handle.read().as_ref() { map.set_pitch(value); } }
                }
                CameraRange { label: "Bearing", value: bearing(), min: -180.0, max: 180.0, test_id: "camera-bearing",
                    oninput: move |value| { bearing.set(value); if let Some(map) = map_handle.read().as_ref() { map.set_bearing(value); } }
                }
                CameraRange { label: "Roll", value: roll(), min: -25.0, max: 25.0, test_id: "camera-roll",
                    oninput: move |value| { roll.set(value); if let Some(map) = map_handle.read().as_ref() { map.set_roll(value); } }
                }
                div { class: "camera-option-row",
                    button {
                        "data-testid": "camera-globe",
                        "aria-pressed": globe(),
                        onclick: move |_| {
                            let enabled = !globe();
                            globe.set(enabled);
                            if let Some(map) = map_handle.read().as_ref() {
                                map.set_projection(if enabled { ProjectionOptions::globe() } else { ProjectionOptions::mercator() });
                            }
                        },
                        AppIcon { kind: IconKind::Globe }
                        "Globe"
                        span { class: if globe() { "switch on" } else { "switch" }, i {} }
                    }
                    div { class: "theme-segment", role: "group", "aria-label": "Map style",
                        button { class: if theme() == MapTheme::Dark { "active" } else { "" }, onclick: move |_| theme.set(MapTheme::Dark), "Dark" }
                        button { class: if theme() == MapTheme::Light { "active" } else { "" }, onclick: move |_| theme.set(MapTheme::Light), "Light" }
                    }
                }
            }
        }
    }
}

#[component]
fn CameraRange(
    label: &'static str,
    value: f64,
    min: f64,
    max: f64,
    test_id: &'static str,
    oninput: EventHandler<f64>,
) -> Element {
    rsx! {
        label { class: "camera-range",
            span { strong { "{label}" } output { "{value:.0}°" } }
            input {
                "data-testid": test_id,
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "1",
                value: "{value}",
                oninput: move |event| {
                    if let Ok(value) = event.value().parse::<f64>() {
                        oninput.call(value);
                    }
                },
            }
        }
    }
}

#[component]
fn PanelHeading(title: &'static str, subtitle: &'static str, state: MobileContext) -> Element {
    rsx! {
        header { class: "panel-heading",
            BackButton { label: "Back to feature lab", onclick: move |_| state.set_lab_panel(LabPanel::Home) }
            div { span { class: "eyebrow", "{subtitle}" } h1 { "{title}" } }
        }
    }
}
