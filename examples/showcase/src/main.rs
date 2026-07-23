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
    #[route("/coverage")]
    Coverage {},
    #[route("/declarative")]
    Declarative {},
    #[route("/markers")]
    Markers {},
    #[route("/sources")]
    Sources {},
    #[route("/media-sources")]
    MediaSources {},
    #[route("/layers")]
    Layers {},
    #[route("/raster-relief")]
    RasterRelief {},
    #[route("/controls")]
    Controls {},
    #[route("/navigation")]
    Navigation {},
    #[route("/camera-state")]
    CameraState {},
    #[route("/interaction")]
    Interaction {},
    #[route("/lifecycle")]
    LifecycleEvents {},
    #[route("/terrain")]
    Terrain {},
    #[route("/style")]
    StyleSwitcher {},
    #[route("/expressions")]
    Expressions {},
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
    #[route("/projections")]
    Projections {},
    #[route("/stress")]
    Stress {},
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
                style: "width: 220px; background: #1a1a2e; padding: 16px; display: flex; flex-direction: column; gap: 4px; overflow-y: auto; flex-shrink: 0;",
                h3 { style: "color: #e0e0e0; margin: 0 0 12px 0; font-size: 14px;", "dioxus-maplibre" }
                NavSection { label: "Overview" }
                NavLink { to: Route::Basic {}, label: "Basic Map" }
                NavLink { to: Route::Coverage {}, label: "Feature Coverage" }
                NavLink { to: Route::Declarative {}, label: "Declarative API" }

                NavSection { label: "Data & rendering" }
                NavLink { to: Route::Markers {}, label: "Markers" }
                NavLink { to: Route::Popups {}, label: "Popups" }
                NavLink { to: Route::Sources {}, label: "GeoJSON & Clusters" }
                NavLink { to: Route::MediaSources {}, label: "Media Sources" }
                NavLink { to: Route::Layers {}, label: "Vector Layers" }
                NavLink { to: Route::RasterRelief {}, label: "Raster & Relief" }
                NavLink { to: Route::Heatmap {}, label: "Heatmap" }
                NavLink { to: Route::Symbols {}, label: "Symbols" }
                NavLink { to: Route::Buildings {}, label: "3D Buildings" }
                NavLink { to: Route::Patterns {}, label: "Images & Patterns" }

                NavSection { label: "Map behavior" }
                NavLink { to: Route::Controls {}, label: "Controls" }
                NavLink { to: Route::Navigation {}, label: "Navigation" }
                NavLink { to: Route::CameraState {}, label: "Camera & State" }
                NavLink { to: Route::Interaction {}, label: "Interaction" }
                NavLink { to: Route::LifecycleEvents {}, label: "Events & Lifecycle" }
                NavLink { to: Route::Terrain {}, label: "Terrain" }
                NavLink { to: Route::Fog {}, label: "Sky & Fog" }
                NavLink { to: Route::Projections {}, label: "Projection & Globe" }

                NavSection { label: "Style & runtime" }
                NavLink { to: Route::StyleSwitcher {}, label: "Style" }
                NavLink { to: Route::Expressions {}, label: "Expressions" }
                NavLink { to: Route::EvalDemo {}, label: "Eval / Integrations" }
                NavLink { to: Route::Query {}, label: "Query" }
                NavLink { to: Route::Animation {}, label: "Animation" }
                NavLink { to: Route::Stress {}, label: "Stress (1M+)" }

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
fn NavSection(label: &'static str) -> Element {
    rsx! {
        div {
            style: "color: #64748b; font-size: 10px; font-weight: 700; letter-spacing: 0.12em; text-transform: uppercase; margin: 12px 8px 3px;",
            "{label}"
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
