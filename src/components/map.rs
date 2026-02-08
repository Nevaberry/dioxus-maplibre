//! Main Map component

use dioxus::prelude::*;

use crate::events::{
    LayerClickEvent, LayerHoverEvent, MapClickEvent, MapContextMenuEvent, MapDblClickEvent,
    MapMoveEvent, MapPitchEvent, MapRotateEvent, MapZoomEvent, MarkerClickEvent,
    MarkerDragEndEvent, MarkerDragStartEvent, MarkerHoverEvent,
};
use crate::handle::MapHandle;
use crate::interop::generate_map_id;
use crate::types::{Bounds, LatLng};

/// Props for the Map component
#[derive(Props, Clone, PartialEq)]
pub struct MapProps {
    /// MapLibre style URL
    #[props(default = "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json".to_string())]
    pub style: String,

    /// Initial center coordinate
    #[props(default = LatLng::helsinki())]
    pub center: LatLng,

    /// Initial zoom level (0-22)
    #[props(default = 10.0)]
    pub zoom: f64,

    /// Initial bearing in degrees (0-360)
    #[props(default = 0.0)]
    pub bearing: f64,

    /// Initial pitch in degrees (0-85)
    #[props(default = 0.0)]
    pub pitch: f64,

    /// Minimum zoom level
    #[props(optional)]
    pub min_zoom: Option<f64>,

    /// Maximum zoom level
    #[props(optional)]
    pub max_zoom: Option<f64>,

    /// Maximum bounds as [[sw_lng, sw_lat], [ne_lng, ne_lat]]
    #[props(optional)]
    pub max_bounds: Option<Bounds>,

    /// Enable cooperative gestures (require Ctrl/Cmd to zoom with scroll)
    #[props(optional)]
    pub cooperative_gestures: Option<bool>,

    /// Container height (CSS value)
    #[props(default = "100%".to_string())]
    pub height: String,

    /// Container width (CSS value)
    #[props(default = "100%".to_string())]
    pub width: String,

    /// Throttle for `on_move` updates in milliseconds (0 = every animation frame)
    #[props(default = 80)]
    pub move_event_throttle_ms: u32,

    /// Called when the map is ready with a MapHandle
    #[props(optional)]
    pub on_ready: Option<EventHandler<MapHandle>>,

    /// Called when the map is clicked
    #[props(optional)]
    pub on_click: Option<EventHandler<MapClickEvent>>,

    /// Called when the map is double-clicked
    #[props(optional)]
    pub on_dblclick: Option<EventHandler<MapDblClickEvent>>,

    /// Called on right-click / context menu
    #[props(optional)]
    pub on_contextmenu: Option<EventHandler<MapContextMenuEvent>>,

    /// Called when a marker is clicked
    #[props(optional)]
    pub on_marker_click: Option<EventHandler<MarkerClickEvent>>,

    /// Called when hovering over a marker
    #[props(optional)]
    pub on_marker_hover: Option<EventHandler<MarkerHoverEvent>>,

    /// Called when a draggable marker starts being dragged
    #[props(optional)]
    pub on_marker_dragstart: Option<EventHandler<MarkerDragStartEvent>>,

    /// Called when a draggable marker is dropped after dragging
    #[props(optional)]
    pub on_marker_dragend: Option<EventHandler<MarkerDragEndEvent>>,

    /// Called when the map view changes (pan/zoom/rotate/pitch)
    #[props(optional)]
    pub on_move: Option<EventHandler<MapMoveEvent>>,

    /// Called when zoom changes
    #[props(optional)]
    pub on_zoom: Option<EventHandler<MapZoomEvent>>,

    /// Called when bearing (rotation) changes
    #[props(optional)]
    pub on_rotate: Option<EventHandler<MapRotateEvent>>,

    /// Called when pitch changes
    #[props(optional)]
    pub on_pitch: Option<EventHandler<MapPitchEvent>>,

    /// Called when a feature in a layer is clicked
    #[props(optional)]
    pub on_layer_click: Option<EventHandler<LayerClickEvent>>,

    /// Called when hovering over a feature in a layer
    #[props(optional)]
    pub on_layer_hover: Option<EventHandler<LayerHoverEvent>>,

    /// Child elements (HTML overlays, not map objects — use MapHandle for those)
    pub children: Element,
}

/// The main Map component
///
/// Renders a MapLibre GL JS map. Use the `on_ready` callback to receive a `MapHandle`
/// for adding sources, layers, markers, and other map objects.
///
/// # Examples
///
/// ```rust,ignore
/// use dioxus_maplibre::{Map, MapHandle, LatLng};
///
/// fn App() -> Element {
///     rsx! {
///         Map {
///             center: LatLng::new(60.17, 24.94),
///             zoom: 12.0,
///             on_ready: move |map: MapHandle| {
///                 // map is ready — add sources, layers, markers, etc.
///             },
///         }
///     }
/// }
/// ```
#[component]
pub fn Map(props: MapProps) -> Element {
    let map_id = use_hook(generate_map_id);
    let container_id = format!("{map_id}_container");

    #[allow(unused_variables, unused_mut)]
    let mut init_started = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    {
        use crate::interop::{destroy_map_js, init_map_js, set_move_event_throttle_js};
        use tracing::debug;

        let style = props.style.clone();
        let center = props.center;
        let zoom = props.zoom;
        let bearing = props.bearing;
        let pitch = props.pitch;
        let min_zoom = props.min_zoom;
        let max_zoom = props.max_zoom;
        let max_bounds = props.max_bounds;
        let cooperative_gestures = props.cooperative_gestures;
        let move_event_throttle_ms = props.move_event_throttle_ms;
        let on_ready = props.on_ready;
        let on_click = props.on_click;
        let on_dblclick = props.on_dblclick;
        let on_contextmenu = props.on_contextmenu;
        let on_marker_click = props.on_marker_click;
        let on_marker_hover = props.on_marker_hover;
        let on_marker_dragstart = props.on_marker_dragstart;
        let on_marker_dragend = props.on_marker_dragend;
        let on_move = props.on_move;
        let on_zoom = props.on_zoom;
        let on_rotate = props.on_rotate;
        let on_pitch = props.on_pitch;
        let on_layer_click = props.on_layer_click;
        let on_layer_hover = props.on_layer_hover;

        {
            let map_id = map_id.clone();
            let container_id = container_id.clone();

            use_effect(move || {
                if init_started() {
                    debug!("Map init already started, skipping");
                    return;
                }
                init_started.set(true);

                let container_id = container_id.clone();
                let map_id = map_id.clone();
                let style = style.clone();

                let max_bounds_str = max_bounds.map(|b| {
                    format!(
                        "[[{}, {}], [{}, {}]]",
                        b.sw.lng, b.sw.lat, b.ne.lng, b.ne.lat
                    )
                });

                debug!("Starting map initialization for: {}", map_id);

                spawn(async move {
                    let init_js = init_map_js(
                        &container_id,
                        &map_id,
                        &style,
                        center.lng,
                        center.lat,
                        zoom,
                        bearing,
                        pitch,
                        min_zoom,
                        max_zoom,
                        max_bounds_str.as_deref(),
                        cooperative_gestures,
                        move_event_throttle_ms,
                    );

                    let mut eval = document::eval(&init_js);
                    debug!("Map init JS executed for: {}", map_id);

                    loop {
                        match eval.recv::<String>().await {
                            Ok(json) => {
                                debug!("Received event: {}", json);

                                if let Ok(event) =
                                    serde_json::from_str::<serde_json::Value>(&json)
                                {
                                    match event.get("type").and_then(|t| t.as_str()) {
                                        Some("ready") => {
                                            debug!("Map ready!");
                                            let handle = MapHandle::new(map_id.clone());
                                            if let Some(handler) = &on_ready {
                                                handler.call(handle);
                                            }
                                        }
                                        Some("click") => {
                                            if let Ok(e) = serde_json::from_value::<MapClickEvent>(event.clone())
                                                && let Some(handler) = &on_click
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("dblclick") => {
                                            if let Ok(e) = serde_json::from_value::<MapDblClickEvent>(event.clone())
                                                && let Some(handler) = &on_dblclick
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("contextmenu") => {
                                            if let Ok(e) = serde_json::from_value::<MapContextMenuEvent>(event.clone())
                                                && let Some(handler) = &on_contextmenu
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("marker_click") => {
                                            if let Ok(e) = serde_json::from_value::<MarkerClickEvent>(event.clone())
                                                && let Some(handler) = &on_marker_click
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("marker_hover") => {
                                            if let Ok(e) = serde_json::from_value::<MarkerHoverEvent>(event.clone())
                                                && let Some(handler) = &on_marker_hover
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("marker_dragstart") => {
                                            if let Ok(e) = serde_json::from_value::<MarkerDragStartEvent>(event.clone())
                                                && let Some(handler) = &on_marker_dragstart
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("marker_dragend") => {
                                            if let Ok(e) = serde_json::from_value::<MarkerDragEndEvent>(event.clone())
                                                && let Some(handler) = &on_marker_dragend
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("move") => {
                                            if let Ok(e) = serde_json::from_value::<MapMoveEvent>(event.clone())
                                                && let Some(handler) = &on_move
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("zoom") => {
                                            if let Ok(e) = serde_json::from_value::<MapZoomEvent>(event.clone())
                                                && let Some(handler) = &on_zoom
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("rotate") => {
                                            if let Ok(e) = serde_json::from_value::<MapRotateEvent>(event.clone())
                                                && let Some(handler) = &on_rotate
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("pitch") => {
                                            if let Ok(e) = serde_json::from_value::<MapPitchEvent>(event.clone())
                                                && let Some(handler) = &on_pitch
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("layer_click") => {
                                            if let Ok(e) = serde_json::from_value::<LayerClickEvent>(event.clone())
                                                && let Some(handler) = &on_layer_click
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        Some("layer_hover") => {
                                            if let Ok(e) = serde_json::from_value::<LayerHoverEvent>(event.clone())
                                                && let Some(handler) = &on_layer_hover
                                            {
                                                handler.call(e);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Err(e) => {
                                debug!("Event channel closed: {:?}", e);
                                break;
                            }
                        }
                    }
                });
            });
        }

        // Live style switching: detect style prop changes after initialization
        {
            let mut tracked_style = use_signal(|| props.style.clone());
            if tracked_style() != props.style && init_started() {
                let map_id = map_id.clone();
                let new_style = props.style.clone();
                tracked_style.set(new_style.clone());
                spawn(async move {
                    let js = crate::interop::set_style_js(&map_id, &new_style);
                    let _ = document::eval(&js).await;
                });
            }
        }

        // Live move throttle switching: detect prop changes after initialization
        {
            let mut tracked_move_throttle = use_signal(|| props.move_event_throttle_ms);
            if tracked_move_throttle() != props.move_event_throttle_ms && init_started() {
                let map_id = map_id.clone();
                let new_throttle = props.move_event_throttle_ms;
                tracked_move_throttle.set(new_throttle);
                spawn(async move {
                    let js = set_move_event_throttle_js(&map_id, new_throttle);
                    let _ = document::eval(&js).await;
                });
            }
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
            {props.children}
        }
    }
}
