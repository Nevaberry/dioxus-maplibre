use dioxus::prelude::*;
use dioxus_maplibre::{
    Map, MapHandle, FlyToOptions, EaseToOptions, JumpToOptions,
    FitBoundsOptions, Padding, Bounds, LatLng, MapMoveEvent,
};

struct City {
    name: &'static str,
    pos: LatLng,
}

const CITIES: &[City] = &[
    City { name: "Helsinki", pos: LatLng { lat: 60.1699, lng: 24.9384 } },
    City { name: "Tampere", pos: LatLng { lat: 61.4978, lng: 23.7610 } },
    City { name: "Turku", pos: LatLng { lat: 60.4518, lng: 22.2666 } },
    City { name: "Oulu", pos: LatLng { lat: 65.0121, lng: 25.4651 } },
];

#[component]
pub fn Navigation() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut position = use_signal(|| String::from("--"));

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    center: LatLng::new(62.0, 25.0),
                    zoom: 5.0,
                    on_ready: move |handle: MapHandle| {
                        map_handle.set(Some(handle));
                    },
                    on_move: move |e: MapMoveEvent| {
                        position.set(format!("{:.4}, {:.4} z{:.1}", e.center.lat, e.center.lng, e.zoom));
                    },
                }
            }
            div { style: "width: 280px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px; overflow-y: auto;",
                h3 { style: "margin: 0 0 12px 0;", "Navigation" }
                p { "data-testid": "position", "Position: {position}" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 12px;",
                        h4 { style: "margin: 8px 0 4px 0;", "flyTo" }
                        for city in CITIES {
                            {
                                let map = map.clone();
                                let pos = city.pos;
                                rsx! {
                                    button {
                                        style: "padding: 6px 10px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| {
                                            map.fly_to(FlyToOptions {
                                                center: Some(pos),
                                                zoom: Some(12.0),
                                                essential: Some(true),
                                                ..Default::default()
                                            });
                                        },
                                        "{city.name}"
                                    }
                                }
                            }
                        }

                        h4 { style: "margin: 12px 0 4px 0;", "easeTo" }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 6px 10px; border-radius: 4px; border: none; background: #6366f1; color: white; cursor: pointer; font-size: 12px;",
                                    onclick: move |_| {
                                        map.ease_to(EaseToOptions {
                                            bearing: Some(45.0),
                                            pitch: Some(60.0),
                                            duration: Some(2000),
                                            ..Default::default()
                                        });
                                    },
                                    "Tilt & Rotate"
                                }
                            }
                        }

                        h4 { style: "margin: 12px 0 4px 0;", "jumpTo" }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 6px 10px; border-radius: 4px; border: none; background: #22c55e; color: white; cursor: pointer; font-size: 12px;",
                                    onclick: move |_| {
                                        map.jump_to(JumpToOptions {
                                            center: Some(LatLng::new(60.17, 24.94)),
                                            zoom: Some(14.0),
                                            bearing: Some(0.0),
                                            pitch: Some(0.0),
                                            ..Default::default()
                                        });
                                    },
                                    "Jump to Helsinki (instant)"
                                }
                            }
                        }

                        h4 { style: "margin: 12px 0 4px 0;", "fitBounds" }
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 6px 10px; border-radius: 4px; border: none; background: #f59e0b; color: white; cursor: pointer; font-size: 12px;",
                                    onclick: move |_| {
                                        map.fit_bounds(
                                            Bounds::new(
                                                LatLng::new(59.8, 22.0),
                                                LatLng::new(65.5, 26.0),
                                            ),
                                            FitBoundsOptions {
                                                padding: Some(Padding::uniform(50.0)),
                                                ..Default::default()
                                            },
                                        );
                                    },
                                    "Fit All Finland"
                                }
                            }
                        }

                        h4 { style: "margin: 12px 0 4px 0;", "Other" }
                        div { style: "display: flex; gap: 4px; flex-wrap: wrap;",
                            {
                                let map = map.clone();
                                rsx! {
                                    button {
                                        style: "padding: 6px 10px; border-radius: 4px; border: none; background: #555; color: white; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| { map.zoom_in(); },
                                        "Zoom +"
                                    }
                                }
                            }
                            {
                                let map = map.clone();
                                rsx! {
                                    button {
                                        style: "padding: 6px 10px; border-radius: 4px; border: none; background: #555; color: white; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| { map.zoom_out(); },
                                        "Zoom -"
                                    }
                                }
                            }
                            {
                                let map = map.clone();
                                rsx! {
                                    button {
                                        style: "padding: 6px 10px; border-radius: 4px; border: none; background: #555; color: white; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| { map.reset_north(); },
                                        "Reset North"
                                    }
                                }
                            }
                            {
                                let map = map.clone();
                                rsx! {
                                    button {
                                        style: "padding: 6px 10px; border-radius: 4px; border: none; background: #555; color: white; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| { map.pan_by(-100, 0); },
                                        "Pan Left"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
