use dioxus::prelude::*;

mod pages;
use pages::*;

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
}

#[component]
fn AppLayout() -> Element {
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
