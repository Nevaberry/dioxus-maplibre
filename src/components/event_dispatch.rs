//! Typed event dispatch from JS bridge events to Dioxus handlers.

use dioxus::prelude::{EventHandler, WritableExt};

use crate::events::{
    LayerClickEvent, LayerHoverEvent, MapClickEvent, MapContextMenuEvent, MapDblClickEvent,
    MapErrorEvent, MapEvent, MapMoveEvent, MapPitchEvent, MapRotateEvent, MapZoomEvent,
    MarkerClickEvent, MarkerDragEndEvent, MarkerDragStartEvent, MarkerHoverEvent,
};
use crate::handle::MapHandle;

use super::context::MapHandleSignal;

#[derive(Clone, Default)]
pub(crate) struct MapEventHandlers {
    pub on_ready: Option<EventHandler<MapHandle>>,
    pub on_error: Option<EventHandler<MapErrorEvent>>,
    pub on_click: Option<EventHandler<MapClickEvent>>,
    pub on_dblclick: Option<EventHandler<MapDblClickEvent>>,
    pub on_contextmenu: Option<EventHandler<MapContextMenuEvent>>,
    pub on_marker_click: Option<EventHandler<MarkerClickEvent>>,
    pub on_marker_hover: Option<EventHandler<MarkerHoverEvent>>,
    pub on_marker_dragstart: Option<EventHandler<MarkerDragStartEvent>>,
    pub on_marker_dragend: Option<EventHandler<MarkerDragEndEvent>>,
    pub on_move: Option<EventHandler<MapMoveEvent>>,
    pub on_zoom: Option<EventHandler<MapZoomEvent>>,
    pub on_rotate: Option<EventHandler<MapRotateEvent>>,
    pub on_pitch: Option<EventHandler<MapPitchEvent>>,
    pub on_layer_click: Option<EventHandler<LayerClickEvent>>,
    pub on_layer_hover: Option<EventHandler<LayerHoverEvent>>,
}

impl MapEventHandlers {
    pub fn dispatch(&self, map_id: &str, event: MapEvent, mut map_handle_signal: MapHandleSignal) {
        match event {
            MapEvent::Ready => {
                let handle = MapHandle::new(map_id.to_string());
                map_handle_signal.set(Some(handle.clone()));
                if let Some(handler) = &self.on_ready {
                    handler.call(handle);
                }
            }
            MapEvent::Click(event) => {
                if let Some(handler) = &self.on_click {
                    handler.call(event);
                }
            }
            MapEvent::DblClick(event) => {
                if let Some(handler) = &self.on_dblclick {
                    handler.call(event);
                }
            }
            MapEvent::ContextMenu(event) => {
                if let Some(handler) = &self.on_contextmenu {
                    handler.call(event);
                }
            }
            MapEvent::MarkerClick(event) => {
                if let Some(handler) = &self.on_marker_click {
                    handler.call(event);
                }
            }
            MapEvent::MarkerHover(event) => {
                if let Some(handler) = &self.on_marker_hover {
                    handler.call(event);
                }
            }
            MapEvent::MarkerDragStart(event) => {
                if let Some(handler) = &self.on_marker_dragstart {
                    handler.call(event);
                }
            }
            MapEvent::MarkerDragEnd(event) => {
                if let Some(handler) = &self.on_marker_dragend {
                    handler.call(event);
                }
            }
            MapEvent::Move(event) => {
                if let Some(handler) = &self.on_move {
                    handler.call(event);
                }
            }
            MapEvent::Zoom(event) => {
                if let Some(handler) = &self.on_zoom {
                    handler.call(event);
                }
            }
            MapEvent::Rotate(event) => {
                if let Some(handler) = &self.on_rotate {
                    handler.call(event);
                }
            }
            MapEvent::Pitch(event) => {
                if let Some(handler) = &self.on_pitch {
                    handler.call(event);
                }
            }
            MapEvent::LayerClick(event) => {
                if let Some(handler) = &self.on_layer_click {
                    handler.call(event);
                }
            }
            MapEvent::LayerHover(event) => {
                if let Some(handler) = &self.on_layer_hover {
                    handler.call(event);
                }
            }
            MapEvent::Error(event) => {
                if let Some(handler) = &self.on_error {
                    handler.call(event.clone());
                }
                tracing::error!(
                    map_id,
                    message = ?event.message,
                    "MapLibre bridge error event"
                );
            }
        }
    }
}
