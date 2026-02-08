//! Main Map component.

use dioxus::prelude::*;

use super::context::MapHandleSignal;
#[cfg(target_arch = "wasm32")]
use super::event_dispatch::MapEventHandlers;
#[cfg(target_arch = "wasm32")]
use crate::events::MapEvent;
use crate::events::{
    LayerClickEvent, LayerHoverEvent, MapClickEvent, MapContextMenuEvent, MapDblClickEvent,
    MapMoveEvent, MapPitchEvent, MapRotateEvent, MapZoomEvent, MarkerClickEvent,
    MarkerDragEndEvent, MarkerDragStartEvent, MarkerHoverEvent,
};
use crate::handle::MapHandle;
use crate::interop::generate_map_id;
use crate::types::{Bounds, LatLng};

/// Props for the `Map` component.
#[derive(Props, Clone, PartialEq)]
pub struct MapProps {
    /// MapLibre style URL.
    #[props(default = "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json".to_string())]
    pub style: String,

    /// Initial center coordinate.
    #[props(default = LatLng::helsinki())]
    pub center: LatLng,

    /// Initial zoom level (0-22).
    #[props(default = 10.0)]
    pub zoom: f64,

    /// Initial bearing in degrees (0-360).
    #[props(default = 0.0)]
    pub bearing: f64,

    /// Initial pitch in degrees (0-85).
    #[props(default = 0.0)]
    pub pitch: f64,

    /// Minimum zoom level.
    #[props(optional)]
    pub min_zoom: Option<f64>,

    /// Maximum zoom level.
    #[props(optional)]
    pub max_zoom: Option<f64>,

    /// Maximum bounds as `[[sw_lng, sw_lat], [ne_lng, ne_lat]]`.
    #[props(optional)]
    pub max_bounds: Option<Bounds>,

    /// Enable cooperative gestures (require Ctrl/Cmd to zoom with scroll).
    #[props(optional)]
    pub cooperative_gestures: Option<bool>,

    /// Container height (CSS value).
    #[props(default = "100%".to_string())]
    pub height: String,

    /// Container width (CSS value).
    #[props(default = "100%".to_string())]
    pub width: String,

    /// Throttle for `on_move` updates in milliseconds (0 = every animation frame).
    #[props(default = 80)]
    pub move_event_throttle_ms: u32,

    /// Called when the map is ready with a `MapHandle`.
    #[props(optional)]
    pub on_ready: Option<EventHandler<MapHandle>>,

    /// Called when the map is clicked.
    #[props(optional)]
    pub on_click: Option<EventHandler<MapClickEvent>>,

    /// Called when the map is double-clicked.
    #[props(optional)]
    pub on_dblclick: Option<EventHandler<MapDblClickEvent>>,

    /// Called on right-click / context menu.
    #[props(optional)]
    pub on_contextmenu: Option<EventHandler<MapContextMenuEvent>>,

    /// Called when a marker is clicked.
    #[props(optional)]
    pub on_marker_click: Option<EventHandler<MarkerClickEvent>>,

    /// Called when hovering over a marker.
    #[props(optional)]
    pub on_marker_hover: Option<EventHandler<MarkerHoverEvent>>,

    /// Called when a draggable marker starts being dragged.
    #[props(optional)]
    pub on_marker_dragstart: Option<EventHandler<MarkerDragStartEvent>>,

    /// Called when a draggable marker is dropped after dragging.
    #[props(optional)]
    pub on_marker_dragend: Option<EventHandler<MarkerDragEndEvent>>,

    /// Called when the map view changes (pan/zoom/rotate/pitch).
    #[props(optional)]
    pub on_move: Option<EventHandler<MapMoveEvent>>,

    /// Called when zoom changes.
    #[props(optional)]
    pub on_zoom: Option<EventHandler<MapZoomEvent>>,

    /// Called when bearing (rotation) changes.
    #[props(optional)]
    pub on_rotate: Option<EventHandler<MapRotateEvent>>,

    /// Called when pitch changes.
    #[props(optional)]
    pub on_pitch: Option<EventHandler<MapPitchEvent>>,

    /// Called when a feature in a layer is clicked.
    #[props(optional)]
    pub on_layer_click: Option<EventHandler<LayerClickEvent>>,

    /// Called when hovering over a feature in a layer.
    #[props(optional)]
    pub on_layer_hover: Option<EventHandler<LayerHoverEvent>>,

    /// Child elements rendered inside map container.
    pub children: Element,
}

/// The main `Map` component.
#[component]
pub fn Map(props: MapProps) -> Element {
    let map_id = use_hook(generate_map_id);
    let container_id = format!("{map_id}_container");

    #[allow(unused_variables, unused_mut)]
    let mut init_started = use_signal(|| false);

    #[allow(unused_mut)]
    let mut map_handle_signal: MapHandleSignal = use_signal(|| None::<MapHandle>);
    use_context_provider(|| map_handle_signal);

    #[cfg(target_arch = "wasm32")]
    {
        use crate::interop::{destroy_map_js, init_map_js, set_move_event_throttle_js};

        let handlers = MapEventHandlers {
            on_ready: props.on_ready,
            on_click: props.on_click,
            on_dblclick: props.on_dblclick,
            on_contextmenu: props.on_contextmenu,
            on_marker_click: props.on_marker_click,
            on_marker_hover: props.on_marker_hover,
            on_marker_dragstart: props.on_marker_dragstart,
            on_marker_dragend: props.on_marker_dragend,
            on_move: props.on_move,
            on_zoom: props.on_zoom,
            on_rotate: props.on_rotate,
            on_pitch: props.on_pitch,
            on_layer_click: props.on_layer_click,
            on_layer_hover: props.on_layer_hover,
        };

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

        {
            let map_id = map_id.clone();
            let container_id = container_id.clone();
            let handlers = handlers.clone();

            use_effect(move || {
                if init_started() {
                    return;
                }
                init_started.set(true);

                let container_id = container_id.clone();
                let map_id = map_id.clone();
                let style = style.clone();
                let handlers = handlers.clone();
                let map_handle_signal = map_handle_signal;

                let max_bounds_str = max_bounds.map(|b| {
                    format!(
                        "[[{}, {}], [{}, {}]]",
                        b.sw.lng, b.sw.lat, b.ne.lng, b.ne.lat
                    )
                });

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

                    while let Ok(json) = eval.recv::<String>().await {
                        if let Ok(event) = serde_json::from_str::<MapEvent>(&json) {
                            handlers.dispatch(&map_id, event, map_handle_signal);
                        }
                    }
                });
            });
        }

        // Live style switching: detect style prop changes after initialization.
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

        // Live move throttle switching: detect prop changes after initialization.
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

        // Cleanup on unmount.
        {
            let map_id = map_id.clone();
            use_drop(move || {
                map_handle_signal.set(None);
                let cleanup_js = destroy_map_js(&map_id);
                spawn(async move {
                    let _ = document::eval(&cleanup_js).await;
                });
            });
        }
    }

    rsx! {
        div {
            class: "map-container",
            id: "{container_id}",
            style: "width: {props.width}; height: {props.height};",
            {props.children}
        }
    }
}
