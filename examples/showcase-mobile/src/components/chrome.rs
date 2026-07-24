use dioxus::prelude::*;

use super::{AppIcon, IconKind};
use crate::offline_runtime;
use crate::state::{AppTab, Connectivity, MobileContext, OfflineView};

#[component]
pub fn BottomNavigation() -> Element {
    let mut state = use_context::<MobileContext>();
    let active = state.active_tab();

    rsx! {
        nav { class: "bottom-navigation", "aria-label": "Primary",
            NavButton {
                label: "Scenes",
                icon: IconKind::Map,
                active: active == AppTab::Scenes,
                test_id: "nav-scenes",
                onclick: move |_| state.set_tab(AppTab::Scenes),
            }
            NavButton {
                label: "Lab",
                icon: IconKind::Events,
                active: active == AppTab::Lab,
                test_id: "nav-lab",
                onclick: move |_| state.set_tab(AppTab::Lab),
            }
            NavButton {
                label: "Offline",
                icon: IconKind::Download,
                active: active == AppTab::Offline,
                test_id: "nav-offline",
                onclick: move |_| {
                    if state.active_tab() != AppTab::Offline {
                        state.offline.write().view = OfflineView::Packs;
                    }
                    state.set_tab(AppTab::Offline);
                },
            }
        }
    }
}

#[component]
fn NavButton(
    label: &'static str,
    icon: IconKind,
    active: bool,
    test_id: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let class = if active {
        "nav-button active"
    } else {
        "nav-button"
    };
    rsx! {
        button {
            class,
            "data-testid": test_id,
            "aria-current": if active { Some("page") } else { None },
            onclick: move |event| onclick.call(event),
            AppIcon { kind: icon }
            span { "{label}" }
        }
    }
}

#[component]
pub fn ConnectivityToggle() -> Element {
    let state = use_context::<MobileContext>();
    let connectivity = state.connectivity();

    rsx! {
        div {
            class: "connectivity-toggle glass",
            "data-testid": "connectivity-toggle",
            role: "group",
            "aria-label": "Connectivity mode",
            for option in [Connectivity::Online, Connectivity::Offline] {
                button {
                    key: "{option:?}",
                    class: if connectivity == option { "selected" } else { "" },
                    "data-testid": if option == Connectivity::Online { "mode-online" } else { "mode-offline" },
                    "aria-pressed": connectivity == option,
                    onclick: move |_| {
                        state.set_connectivity(option);
                        offline_runtime::persist_connectivity(option);
                    },
                    if option == Connectivity::Online {
                        span { class: "connectivity-dot" }
                    }
                    "{option.label()}"
                }
            }
        }
    }
}

#[component]
pub fn BackButton(label: &'static str, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "back-button glass",
            "aria-label": label,
            onclick: move |event| onclick.call(event),
            AppIcon { kind: IconKind::ArrowLeft }
        }
    }
}

#[component]
pub fn MapControlStack(
    on_compass: EventHandler<MouseEvent>,
    on_layers: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "map-control-stack",
            button {
                class: "round-map-button glass",
                "aria-label": "Reset compass",
                onclick: move |event| on_compass.call(event),
                AppIcon { kind: IconKind::Compass }
            }
            button {
                class: "round-map-button glass",
                "aria-label": "Open layers",
                onclick: move |event| on_layers.call(event),
                AppIcon { kind: IconKind::Layers }
            }
        }
    }
}

#[component]
pub fn SheetHandle() -> Element {
    rsx! { div { class: "sheet-handle", "aria-hidden": "true" } }
}
