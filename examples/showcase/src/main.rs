use dioxus::prelude::*;

mod pages;
use pages::*;

const DARK_STYLE: &str = "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json";
const LIGHT_STYLE: &str = "https://basemaps.cartocdn.com/gl/positron-gl-style/style.json";

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Basic {},
    #[route("/markers")]
    Markers {},
    #[route("/sources")]
    Sources {},
    #[route("/layers")]
    Layers {},
    #[route("/controls")]
    Controls {},
    #[route("/navigation")]
    Navigation {},
    #[route("/interaction")]
    Interaction {},
    #[route("/terrain")]
    Terrain {},
    #[route("/style")]
    StyleSwitcher {},
    #[route("/eval")]
    EvalDemo {},
    #[route("/heatmap")]
    Heatmap {},
    #[route("/symbols")]
    Symbols {},
    #[route("/popups")]
    Popups {},
    #[route("/buildings")]
    Buildings {},
    #[route("/patterns")]
    Patterns {},
    #[route("/query")]
    Query {},
    #[route("/animation")]
    Animation {},
    #[route("/fog")]
    Fog {},
}

#[component]
fn AppLayout() -> Element {
    let mut style = use_context_provider(|| Signal::new(DARK_STYLE.to_string()));
    let dark_bg = if style().contains("dark-matter") {
        "#3b82f6"
    } else {
        "#333"
    };
    let light_bg = if style().contains("dark-matter") {
        "#333"
    } else {
        "#3b82f6"
    };

    rsx! {
        div {
            style: "display: flex; height: calc(100vh - 16px);",
            nav {
                style: "width: 200px; background: #1a1a2e; padding: 16px; display: flex; flex-direction: column; gap: 4px; overflow-y: auto; flex-shrink: 0;",
                h3 { style: "color: #e0e0e0; margin: 0 0 12px 0; font-size: 14px;", "dioxus-maplibre" }
                NavLink { to: Route::Basic {}, label: "Basic Map" }
                NavLink { to: Route::Markers {}, label: "Markers" }
                NavLink { to: Route::Sources {}, label: "Sources" }
                NavLink { to: Route::Layers {}, label: "Layers" }
                NavLink { to: Route::Controls {}, label: "Controls" }
                NavLink { to: Route::Navigation {}, label: "Navigation" }
                NavLink { to: Route::Interaction {}, label: "Interaction" }
                NavLink { to: Route::Terrain {}, label: "Terrain" }
                NavLink { to: Route::StyleSwitcher {}, label: "Style" }
                NavLink { to: Route::EvalDemo {}, label: "Eval" }
                NavLink { to: Route::Heatmap {}, label: "Heatmap" }
                NavLink { to: Route::Symbols {}, label: "Symbols" }
                NavLink { to: Route::Popups {}, label: "Popups" }
                NavLink { to: Route::Buildings {}, label: "Buildings" }
                NavLink { to: Route::Patterns {}, label: "Patterns" }
                NavLink { to: Route::Query {}, label: "Query" }
                NavLink { to: Route::Animation {}, label: "Animation" }
                NavLink { to: Route::Fog {}, label: "Fog" }

                div { style: "margin-top: auto; padding-top: 12px; border-top: 1px solid #333;",
                    p { style: "color: #888; font-size: 11px; margin: 0 0 6px 0; text-transform: uppercase; letter-spacing: 1px;", "Theme" }
                    div { style: "display: flex; gap: 4px;",
                        button {
                            style: "flex: 1; padding: 6px; border-radius: 4px; border: none; cursor: pointer; font-size: 12px; color: white; background: {dark_bg};",
                            onclick: move |_| style.set(DARK_STYLE.into()),
                            "Dark"
                        }
                        button {
                            style: "flex: 1; padding: 6px; border-radius: 4px; border: none; cursor: pointer; font-size: 12px; color: white; background: {light_bg};",
                            onclick: move |_| style.set(LIGHT_STYLE.into()),
                            "Light"
                        }
                    }
                }
            }
            div {
                style: "flex: 1; position: relative;",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn NavLink(to: Route, label: &'static str) -> Element {
    rsx! {
        Link {
            to,
            style: "color: #b0b0cc; text-decoration: none; padding: 8px 12px; border-radius: 6px; font-size: 13px; display: block;",
            active_class: "nav-active",
            "{label}"
        }
    }
}

#[allow(non_snake_case)]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
