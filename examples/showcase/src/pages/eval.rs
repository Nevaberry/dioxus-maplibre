use dioxus::prelude::*;
use dioxus_maplibre::{Map, MapHandle, LatLng};

#[component]
pub fn EvalDemo() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut result = use_signal(|| String::from("--"));
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(60.17, 24.94),
                    zoom: 12.0,
                    on_ready: move |handle: MapHandle| {
                        map_handle.set(Some(handle));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Eval Escape Hatch" }
                p { "Execute raw JS against the map instance." }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 12px;",
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer; font-size: 12px;",
                                    onclick: move |_| {
                                        // Fire-and-forget raw JS
                                        map.eval("map.setCenter([25.0, 60.2]);");
                                    },
                                    "eval: setCenter"
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #6366f1; color: white; cursor: pointer; font-size: 12px;",
                                    onclick: move |_| {
                                        let map = map.clone();
                                        spawn(async move {
                                            if let Some(zoom) = map.get_zoom().await {
                                                result.set(format!("Zoom: {zoom:.2}"));
                                            }
                                        });
                                    },
                                    "get_zoom (async)"
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #22c55e; color: white; cursor: pointer; font-size: 12px;",
                                    onclick: move |_| {
                                        let map = map.clone();
                                        spawn(async move {
                                            if let Some(center) = map.get_center().await {
                                                result.set(format!("Center: {:.4}, {:.4}", center.lat, center.lng));
                                            }
                                        });
                                    },
                                    "get_center (async)"
                                }
                            }
                        }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #f59e0b; color: white; cursor: pointer; font-size: 12px;",
                                    onclick: move |_| {
                                        let map = map.clone();
                                        spawn(async move {
                                            if let Some(bounds) = map.get_bounds().await {
                                                result.set(format!(
                                                    "Bounds: SW({:.2},{:.2}) NE({:.2},{:.2})",
                                                    bounds.sw.lat, bounds.sw.lng,
                                                    bounds.ne.lat, bounds.ne.lng
                                                ));
                                            }
                                        });
                                    },
                                    "get_bounds (async)"
                                }
                            }
                        }
                    }

                    p { style: "margin-top: 16px; font-family: monospace; color: #88aa88;",
                        "data-testid": "eval-result",
                        "{result}"
                    }
                }
            }
        }
    }
}
