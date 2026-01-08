//! Main Map component

use dioxus::prelude::*;

use crate::context::MapContext;
use crate::events::{MapClickEvent, MapMoveEvent, MarkerClickEvent};
use crate::interop::generate_map_id;
use crate::types::LatLng;

/// Event sent from JS when hovering over a marker
#[derive(Debug, Clone, serde::Deserialize)]
pub struct MarkerHoverEvent {
    pub marker_id: String,
    pub latlng: LatLng,
    pub hover: bool,
    /// Mouse cursor X position (viewport pixels)
    pub cursor_x: f64,
    /// Mouse cursor Y position (viewport pixels)
    pub cursor_y: f64,
}

/// Props for the Map component
#[derive(Props, Clone, PartialEq)]
pub struct MapProps {
    /// MapLibre style URL (e.g., "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json")
    #[props(default = "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json".to_string())]
    pub style: String,

    /// Initial center coordinate
    #[props(default = LatLng::helsinki())]
    pub center: LatLng,

    /// Initial zoom level (0-22)
    #[props(default = 10.0)]
    pub zoom: f64,

    /// Container height (CSS value)
    #[props(default = "100%".to_string())]
    pub height: String,

    /// Container width (CSS value)
    #[props(default = "100%".to_string())]
    pub width: String,

    /// Callback when map is clicked
    #[props(optional)]
    pub on_click: Option<EventHandler<MapClickEvent>>,

    /// Callback when a marker is clicked
    #[props(optional)]
    pub on_marker_click: Option<EventHandler<MarkerClickEvent>>,

    /// Callback when hovering over a marker
    #[props(optional)]
    pub on_marker_hover: Option<EventHandler<MarkerHoverEvent>>,

    /// Callback when map view changes
    #[props(optional)]
    pub on_move: Option<EventHandler<MapMoveEvent>>,

    /// Child components (Marker, etc.)
    #[props(optional)]
    pub children: Element,
}

/// The main Map component
#[component]
pub fn Map(props: MapProps) -> Element {
    // Generate unique map ID on first render
    let map_id = use_hook(generate_map_id);
    let container_id = format!("{map_id}_container");

    // Track if map is ready
    let mut is_ready = use_signal(|| false);

    // Track if initialization has been started (to prevent multiple inits)
    let mut init_started = use_signal(|| false);

    // Create context for child components
    let ctx = MapContext {
        map_id: map_id.clone(),
        is_ready,
    };
    use_context_provider(|| ctx);

    // Only initialize map on web/wasm targets
    #[cfg(target_arch = "wasm32")]
    {
        use crate::interop::{destroy_map_js, init_map_js};
        use tracing::{debug, error};

        // Store props for effect closure
        let style = props.style.clone();
        let center = props.center;
        let zoom = props.zoom;
        let on_click = props.on_click;
        let on_marker_click = props.on_marker_click;
        let on_marker_hover = props.on_marker_hover;
        let on_move = props.on_move;

        // Initialize map and set up event loop - only once
        {
            let map_id = map_id.clone();
            let container_id = container_id.clone();

            use_effect(move || {
                // Only initialize once per component instance
                if init_started() {
                    debug!("Map init already started, skipping");
                    return;
                }
                init_started.set(true);

                // Clone values for the async block
                let container_id = container_id.clone();
                let map_id = map_id.clone();
                let style = style.clone();

                debug!("Starting map initialization for: {}", map_id);

                // Spawn the async initialization
                spawn(async move {
                    // Create the eval that will receive events from the map
                    // We use the SAME eval to execute the init code so dioxus.send() works
                    let init_js = init_map_js(
                        &container_id,
                        &map_id,
                        &style,
                        center.lng,
                        center.lat,
                        zoom,
                    );

                    // Execute init JS in the event loop's eval context
                    let mut eval = document::eval(&init_js);
                    debug!("Map init JS executed in event loop eval for: {}", map_id);

                    // Process events from JS
                    loop {
                        match eval.recv::<String>().await {
                            Ok(json) => {
                                debug!("Received event: {}", json);

                                // Parse the event
                                if let Ok(event) = serde_json::from_str::<serde_json::Value>(&json) {
                                    match event.get("type").and_then(|t| t.as_str()) {
                                        Some("ready") => {
                                            debug!("Map ready!");
                                            is_ready.set(true);
                                        }
                                        Some("click") => {
                                            if let Ok(click_event) = serde_json::from_value::<MapClickEvent>(event.clone()) {
                                                if let Some(handler) = &on_click {
                                                    handler.call(click_event);
                                                }
                                            }
                                        }
                                        Some("marker_click") => {
                                            if let Ok(marker_event) = serde_json::from_value::<MarkerClickEvent>(event.clone()) {
                                                if let Some(handler) = &on_marker_click {
                                                    handler.call(marker_event);
                                                }
                                            }
                                        }
                                        Some("marker_hover") => {
                                            if let Ok(hover_event) = serde_json::from_value::<MarkerHoverEvent>(event.clone()) {
                                                if let Some(handler) = &on_marker_hover {
                                                    handler.call(hover_event);
                                                }
                                            }
                                        }
                                        Some("move") => {
                                            if let Ok(move_event) = serde_json::from_value::<MapMoveEvent>(event.clone()) {
                                                if let Some(handler) = &on_move {
                                                    handler.call(move_event);
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Err(e) => {
                                // Channel closed, component unmounting
                                debug!("Event channel closed: {:?}", e);
                                break;
                            }
                        }
                    }
                });
            });
        }

        // Cleanup on unmount
        {
            let map_id = map_id.clone();
            use_drop(move || {
                debug!("Cleaning up map: {}", map_id);
                let cleanup_js = destroy_map_js(&map_id);
                spawn(async move {
                    let _ = document::eval(&cleanup_js).await;
                });
            });
        }
    }

    rsx! {
        div {
            id: "{container_id}",
            style: "width: {props.width}; height: {props.height};",

            // Render children (markers) only when map is ready
            if is_ready() {
                {props.children}
            }
        }
    }
}

/// Fly to a location on the map
#[cfg(target_arch = "wasm32")]
pub fn fly_to(map_id: &str, latlng: LatLng, zoom: Option<f64>) {
    use crate::interop::fly_to_js;
    let js = fly_to_js(map_id, latlng.lat, latlng.lng, zoom);
    spawn(async move {
        let _ = document::eval(&js).await;
    });
}

/// No-op on non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn fly_to(_map_id: &str, _latlng: LatLng, _zoom: Option<f64>) {}

/// Pan the map by pixel offset (instant, no animation)
/// Useful for compensating visual center when sidebars open/close
#[cfg(target_arch = "wasm32")]
pub fn pan_by(x: i32, y: i32) {
    use crate::interop::pan_by_js;
    let js = pan_by_js(x, y);
    spawn(async move {
        let _ = document::eval(&js).await;
    });
}

/// No-op on non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn pan_by(_x: i32, _y: i32) {}
