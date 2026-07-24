use dioxus::prelude::*;
use dioxus_maplibre::{GeoJsonSourceOptions, LatLng, LayerOptions, Map, MapHandle, MapOptions};
use serde_json::json;

use crate::components::{AppIcon, BackButton, IconKind, SheetHandle};
use crate::data;
use crate::offline_runtime;
use crate::state::{Connectivity, MobileContext, OfflineView, PackId};

#[component]
pub fn OfflineScreen() -> Element {
    let state = use_context::<MobileContext>();
    let view = state.offline_view();

    match view {
        OfflineView::Packs => rsx! { PackLibrary {} },
        OfflineView::SelectArea => rsx! { SelectArea {} },
        OfflineView::Downloading => rsx! { DownloadingPack {} },
        OfflineView::Ready => rsx! { ReadyMap {} },
    }
}

#[component]
fn PackLibrary() -> Element {
    let state = use_context::<MobileContext>();
    let offline = state.offline();
    let used = offline.storage_used_mb();

    rsx! {
        section { class: "screen content-screen offline-library", "data-testid": "offline-screen",
            header { class: "content-header offline-header",
                div {
                    span { class: "eyebrow", "Maps without a signal" }
                    h1 { "Offline packs" }
                    p { "Styles, data, elevation and the app shell stay on this device." }
                }
                span { class: "offline-cloud", AppIcon { kind: IconKind::Download } }
            }

            div { class: "storage-card glass",
                div { class: "storage-copy",
                    span { "Device storage" }
                    strong { "{used} MB cached" }
                    small { "of 512 MB demo budget" }
                }
                div { class: "storage-ring", style: "--storage: {f64::from(used) / 5.12:.1}%", span { "{used}" } small { "MB" } }
            }

            div { class: "pack-list",
                PackCard { pack: PackId::Helsinki, state }
                PackCard { pack: PackId::Matterhorn, state }
                PackCard { pack: PackId::Archipelago, state }
            }

            if let Some(error) = offline.error.as_ref() {
                div { class: "offline-fact warning glass", "data-testid": "offline-error",
                    p { strong { "Pack unavailable. " } "{error}" }
                }
            } else if offline.runtime_supported {
                div { class: "offline-fact glass",
                    span { class: "status-dot" }
                    p {
                        strong { "Offline means offline." }
                        " The included styles and datasets make no network requests."
                    }
                }
            } else {
                div { class: "offline-fact warning glass", "data-testid": "offline-unsupported",
                    p {
                        strong { "Secure origin required. " }
                        "Install and cache packs over HTTPS or localhost."
                    }
                }
            }
        }
    }
}

#[component]
fn PackCard(pack: PackId, state: MobileContext) -> Element {
    let offline = state.offline();
    let ready = offline.is_ready(pack);
    let can_open = ready || offline.runtime_supported;
    let (description, thumb_class, size) = pack_meta(pack);

    rsx! {
        article { class: "pack-card", "data-testid": "pack-{pack.slug()}",
            button {
                class: "pack-main",
                disabled: !can_open,
                onclick: move |_| open_pack(state, pack, ready),
                div { class: "pack-thumbnail {thumb_class}",
                    span { class: "mini-pin" }
                    if pack == PackId::Matterhorn { span { class: "mini-peak", "▲" } }
                }
                div { class: "pack-copy",
                    span { class: "pack-title-row",
                        strong { "{pack.label()}" }
                        if ready { span { class: "ready-pill", AppIcon { kind: IconKind::Check } "Ready" } }
                    }
                    small { "{description}" }
                    span { "{size}" }
                }
                AppIcon { kind: IconKind::ChevronRight }
            }
        }
    }
}

fn open_pack(mut state: MobileContext, pack: PackId, ready: bool) {
    let mut offline = state.offline.write();
    offline.selected_pack = pack;
    offline.error = None;
    offline.view = if ready {
        OfflineView::Ready
    } else {
        OfflineView::SelectArea
    };
}

const fn pack_meta(pack: PackId) -> (&'static str, &'static str, &'static str) {
    match pack {
        PackId::Helsinki => (
            "Streets, places & 3D buildings",
            "helsinki",
            "Bundled starter pack",
        ),
        PackId::Matterhorn => (
            "Contours, fog & elevation tile",
            "matterhorn",
            "1.3 MB download",
        ),
        PackId::Archipelago => (
            "Islands, routes & selected region",
            "archipelago",
            "620 KB download",
        ),
    }
}

#[component]
fn SelectArea() -> Element {
    let mut state = use_context::<MobileContext>();
    let pack = state.offline().selected_pack;
    let mut detail = use_signal(|| 2_u8);
    let (center, zoom, style) = pack_map(pack);

    rsx! {
        section { class: "screen map-screen offline-map-screen", "data-testid": "select-area-screen",
            OfflineHeading { title: "Select area", state }

            div { class: "map-stage offline-map-stage",
                Map {
                    style,
                    center,
                    zoom,
                    options: MapOptions(json!({ "attributionControl": false })),
                    on_ready: move |handle: MapHandle| add_pack_preview(&handle, pack),
                }
                div { class: "selection-label glass", "{pack.label()} pack" }
            }

            div { class: "bottom-sheet select-sheet", "data-testid": "area-sheet",
                SheetHandle {}
                div { class: "sheet-kicker", "Download region" }
                h2 { "{pack.label()}" }
                p { "Drag and zoom the map, then choose how much local detail to retain." }
                label { class: "detail-range",
                    span { strong { "Detail" } output { "Level {detail}" } }
                    input {
                        "data-testid": "offline-detail",
                        r#type: "range",
                        min: "1",
                        max: "3",
                        step: "1",
                        value: "{detail}",
                        oninput: move |event| if let Ok(value) = event.value().parse() { detail.set(value); },
                    }
                    span { class: "range-labels", small { "Compact" } small { "Detailed" } }
                }
                div { class: "download-estimate",
                    span { "Estimated pack" }
                    strong {
                        if detail() == 1 {
                            "480 KB"
                        } else if detail() == 2 {
                            "1.3 MB"
                        } else {
                            "2.1 MB"
                        }
                    }
                }
                button {
                    class: "primary-action",
                    "data-testid": "start-download",
                    onclick: move |_| {
                        {
                            let mut offline = state.offline.write();
                            offline.progress = 0;
                            offline.component = "App shell".into();
                            offline.paused = false;
                            offline.error = None;
                            offline.view = OfflineView::Downloading;
                        }
                        offline_runtime::download_pack(pack, state);
                    },
                    AppIcon { kind: IconKind::Download }
                    "Download for offline"
                }
            }
        }
    }
}

#[component]
fn DownloadingPack() -> Element {
    let mut state = use_context::<MobileContext>();
    let offline = state.offline();
    let pack = offline.selected_pack;
    let progress = offline.progress;
    let paused = offline.paused;
    let component = offline.component;
    let (center, zoom, style) = pack_map(pack);

    rsx! {
        section { class: "screen map-screen offline-map-screen downloading-screen", "data-testid": "downloading-screen",
            OfflineHeading { title: "Saving map", state }

            div { class: "map-stage offline-map-stage download-map",
                Map {
                    style,
                    center,
                    zoom,
                    options: MapOptions(json!({ "attributionControl": false, "interactive": false })),
                    on_ready: move |handle: MapHandle| add_pack_preview(&handle, pack),
                }
                div { class: "download-scrim" }
                div {
                    class: "download-progress-ring",
                    style: "--progress: {progress}%",
                    "data-testid": "download-progress",
                    strong { "{progress}%" }
                    small { if paused { "Paused" } else { "Caching" } }
                }
            }

            div { class: "bottom-sheet download-sheet",
                SheetHandle {}
                span { class: "sheet-kicker", "Offline pack" }
                h2 { "Saving {pack.label()}" }
                p { "{component}" }
                div { class: "progress-track", span { style: "width: {progress}%" } }
                div { class: "download-actions",
                    button {
                        class: "secondary-action",
                        "data-testid": "pause-download",
                        onclick: move |_| {
                            offline_runtime::pause_or_resume(paused);
                            state.offline.write().paused = !paused;
                        },
                        AppIcon { kind: IconKind::Pause }
                        if paused { "Resume" } else { "Pause" }
                    }
                    button {
                        class: "text-action danger",
                        "data-testid": "cancel-download",
                        onclick: move |_| {
                            offline_runtime::cancel_download();
                            let mut offline = state.offline.write();
                            offline.view = OfflineView::Packs;
                            offline.progress = 0;
                            offline.paused = false;
                        },
                        "Cancel"
                    }
                }
            }
        }
    }
}

#[component]
fn ReadyMap() -> Element {
    let state = use_context::<MobileContext>();
    let pack = state.offline().selected_pack;
    let (center, zoom, style) = pack_map(pack);

    rsx! {
        section { class: "screen map-screen offline-map-screen", "data-testid": "offline-ready-screen",
            OfflineHeading { title: pack.label(), state }

            div { class: "map-stage offline-map-stage ready-map-stage",
                Map {
                    style,
                    center,
                    zoom,
                    pitch: if pack == PackId::Matterhorn { 58.0 } else { 20.0 },
                    options: MapOptions(json!({ "attributionControl": false })),
                    on_ready: move |handle: MapHandle| add_pack_preview(&handle, pack),
                }
                div { class: "offline-ready-badge glass", "data-testid": "offline-ready-badge",
                    AppIcon { kind: IconKind::Check }
                    "Offline ready"
                }
            }

            div { class: "bottom-sheet ready-sheet",
                SheetHandle {}
                span { class: "sheet-kicker", "Stored on device" }
                h2 { "{pack.label()} is ready" }
                p { "Switch to Offline above the pack library—or disable the network entirely—and this map still works." }
                div { class: "offline-feature-chips",
                    span { "Local style" }
                    span { "GeoJSON" }
                    if pack == PackId::Matterhorn { span { "Elevation" } }
                    span { "App shell" }
                }
                button {
                    class: "primary-action compact",
                    onclick: move |_| {
                        state.set_connectivity(Connectivity::Offline);
                        offline_runtime::persist_connectivity(Connectivity::Offline);
                    },
                    "Use offline mode"
                }
            }
        }
    }
}

fn add_pack_preview(handle: &MapHandle, pack: PackId) {
    match pack {
        PackId::Helsinki => {
            handle.add_geojson_source(
                "pack-preview",
                GeoJsonSourceOptions {
                    data: data::helsinki_places(),
                    ..Default::default()
                },
            );
            handle.add_layer(
                LayerOptions::circle("pack-preview-points", "pack-preview").paint(json!({
                    "circle-radius": 7,
                    "circle-color": "#65ded5",
                    "circle-stroke-color": "#062638",
                    "circle-stroke-width": 2
                })),
            );
        }
        PackId::Matterhorn => {
            handle.add_geojson_source(
                "pack-preview",
                GeoJsonSourceOptions {
                    data: data::terrain_contours(),
                    ..Default::default()
                },
            );
            handle.add_layer(
                LayerOptions::line("pack-preview-contours", "pack-preview").paint(json!({
                    "line-color": "#f0c978", "line-width": 2, "line-dasharray": [2, 1]
                })),
            );
        }
        PackId::Archipelago => {
            handle.add_geojson_source(
                "pack-preview",
                GeoJsonSourceOptions {
                    data: data::archipelago_selection(),
                    ..Default::default()
                },
            );
            handle.add_layer(
                LayerOptions::fill("pack-preview-area", "pack-preview").paint(json!({
                    "fill-color": "#43c8bc", "fill-opacity": 0.24
                })),
            );
            handle.add_layer(
                LayerOptions::line("pack-preview-border", "pack-preview").paint(json!({
                    "line-color": "#8ef4ea", "line-width": 3, "line-dasharray": [2, 1]
                })),
            );
        }
    }
}

fn pack_map(pack: PackId) -> (LatLng, f64, &'static str) {
    match pack {
        PackId::Helsinki => (
            LatLng::new(60.1699, 24.9384),
            12.3,
            "/offline/helsinki-style.json",
        ),
        PackId::Matterhorn => (
            LatLng::new(45.9763, 7.6586),
            11.0,
            "/offline/matterhorn-style.json",
        ),
        PackId::Archipelago => (
            LatLng::new(60.07, 24.94),
            9.3,
            "/offline/archipelago-style.json",
        ),
    }
}

#[component]
fn OfflineHeading(title: &'static str, state: MobileContext) -> Element {
    let mut state = state;
    rsx! {
        header { class: "panel-heading offline-panel-heading",
            BackButton {
                label: "Back to offline packs",
                onclick: move |_| state.offline.write().view = OfflineView::Packs,
            }
            div { span { class: "eyebrow", "Offline maps" } h1 { "{title}" } }
        }
    }
}
