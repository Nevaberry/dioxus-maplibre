use dioxus::prelude::*;

mod components;
mod data;
mod offline_runtime;
mod screens;
mod state;

use components::{BottomNavigation, ConnectivityToggle};
use screens::{LabScreen, OfflineScreen, ScenesScreen};
use state::{AppTab, MobileContext};

#[allow(clippy::volatile_composites)]
const MOBILE_CSS: Asset = asset!("/assets/mobile.css");

fn main() {
    dioxus::launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    let state = use_context_provider(MobileContext::new);

    use_effect(move || {
        offline_runtime::initialize(state);
    });

    let active_tab = state.active_tab();
    let runtime_ready = state.runtime_ready();

    rsx! {
        document::Title { "MapLibre Mobile Lab" }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, viewport-fit=cover, user-scalable=no"
        }
        document::Meta { name: "theme-color", content: "#071525" }
        document::Link { rel: "manifest", href: "/manifest.webmanifest" }
        document::Stylesheet { href: "/vendor/maplibre/maplibre-gl.css" }
        document::Stylesheet { href: MOBILE_CSS }
        document::Script { src: "/offline-runtime.js" }
        document::Script { src: "/maplibre-loader.js" }

        main {
            class: "mobile-app",
            "data-testid": "mobile-showcase",
            div { class: "app-ambient" }

            if runtime_ready {
                match active_tab {
                    AppTab::Scenes => rsx! { ScenesScreen {} },
                    AppTab::Lab => rsx! { LabScreen {} },
                    AppTab::Offline => rsx! { OfflineScreen {} },
                }

                if active_tab != AppTab::Offline || state.offline_view().shows_connectivity_toggle() {
                    ConnectivityToggle {}
                }
                BottomNavigation {}
            } else {
                div { class: "launch-screen", "data-testid": "launch-screen",
                    span { class: "launch-mark", "M" }
                    strong { "MapLibre Mobile Lab" }
                    small { "Restoring offline maps…" }
                }
            }
        }
    }
}
