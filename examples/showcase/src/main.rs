//! dioxus-maplibre showcase
//!
//! This app demonstrates all features of the dioxus-maplibre crate.
//! It's also used as the target for E2E tests.

use dioxus::prelude::*;
use dioxus_maplibre::{
    Map, Marker, LatLng,
    MapClickEvent, MarkerClickEvent, MapMoveEvent,
    fly_to,
};

// Static marker data: (id, name, lat, lng)
const MARKERS: &[(&str, &str, f64, f64)] = &[
    ("helsinki", "Helsinki", 60.1699, 24.9384),
    ("tampere", "Tampere", 61.4978, 23.7610),
    ("turku", "Turku", 60.4518, 22.2666),
    ("oulu", "Oulu", 65.0121, 25.4651),
];

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    // Track click position for display
    let mut last_click = use_signal(|| None::<LatLng>);

    // Track map position
    let mut map_center = use_signal(|| LatLng::helsinki());
    let mut map_zoom = use_signal(|| 10.0_f64);

    // Track clicked marker
    let mut clicked_marker = use_signal(|| None::<String>);

    rsx! {
        div {
            style: "display: flex; height: 100vh;",

            // Sidebar
            div {
                style: "width: 300px; padding: 16px; background: #f5f5f5; overflow-y: auto;",

                h1 { style: "margin-bottom: 16px;", "dioxus-maplibre" }

                // Click info
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: white; border-radius: 8px;",
                    h3 { "Last Click" }
                    if let Some(pos) = last_click() {
                        p { "Lat: {pos.lat:.4}" }
                        p { "Lng: {pos.lng:.4}" }
                    } else {
                        p { style: "color: #888;", "Click on the map" }
                    }
                }

                // Map position
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: white; border-radius: 8px;",
                    h3 { "Map Position" }
                    p { "Center: {map_center().lat:.4}, {map_center().lng:.4}" }
                    p { "Zoom: {map_zoom():.1}" }
                }

                // Clicked marker
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: white; border-radius: 8px;",
                    h3 { "Marker Clicked" }
                    if let Some(ref id) = clicked_marker() {
                        p { "{id}" }
                    } else {
                        p { style: "color: #888;", "Click a marker" }
                    }
                }

                // Quick nav buttons
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: white; border-radius: 8px;",
                    h3 { "Quick Navigation" }
                    for &(_id, name, lat, lng) in MARKERS.iter() {
                        button {
                            style: "display: block; width: 100%; padding: 8px; margin-bottom: 8px; cursor: pointer;",
                            onclick: move |_| {
                                fly_to("map_main", LatLng::new(lat, lng), Some(12.0));
                            },
                            "{name}"
                        }
                    }
                }
            }

            // Map container
            div {
                style: "flex: 1;",

                Map {
                    style: "https://basemaps.cartocdn.com/gl/voyager-gl-style/style.json",
                    center: LatLng::helsinki(),
                    zoom: 6.0,
                    height: "100%",
                    width: "100%",

                    on_click: move |e: MapClickEvent| {
                        last_click.set(Some(e.latlng));
                    },

                    on_marker_click: move |e: MarkerClickEvent| {
                        clicked_marker.set(Some(e.marker_id.clone()));
                    },

                    on_move: move |e: MapMoveEvent| {
                        map_center.set(e.center);
                        map_zoom.set(e.zoom);
                    },

                    // Add markers
                    for &(id, name, lat, lng) in MARKERS.iter() {
                        Marker {
                            id: id.to_string(),
                            position: LatLng::new(lat, lng),
                            popup: Some(format!("<b>{name}</b><br>Click to select")),
                        }
                    }
                }
            }
        }
    }
}
