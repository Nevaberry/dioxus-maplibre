//! Event types for map interactions

use serde::{Deserialize, Serialize};
use crate::types::{Bounds, LatLng, Point};

/// Event fired when the map is clicked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapClickEvent {
    /// Geographic coordinates of the click
    pub latlng: LatLng,
    /// Screen pixel coordinates of the click
    pub point: Point,
}

/// Event fired when the map is double-clicked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapDblClickEvent {
    /// Geographic coordinates of the double-click
    pub latlng: LatLng,
    /// Screen pixel coordinates
    pub point: Point,
}

/// Event fired on right-click / context menu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapContextMenuEvent {
    /// Geographic coordinates
    pub latlng: LatLng,
    /// Screen pixel coordinates
    pub point: Point,
}

/// Event fired when a marker is clicked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerClickEvent {
    /// Marker ID
    pub marker_id: String,
    /// Geographic coordinates of the marker
    pub latlng: LatLng,
}

/// Event fired when hovering over a marker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerHoverEvent {
    /// Marker ID
    pub marker_id: String,
    /// Geographic coordinates of the marker
    pub latlng: LatLng,
    /// Whether the mouse is entering (true) or leaving (false)
    pub hover: bool,
    /// Mouse cursor X position (viewport pixels)
    pub cursor_x: f64,
    /// Mouse cursor Y position (viewport pixels)
    pub cursor_y: f64,
}

/// Event fired when the map view changes (pan/zoom)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMoveEvent {
    /// New center position
    pub center: LatLng,
    /// New zoom level
    pub zoom: f64,
    /// Current viewport bounds (sw/ne corners)
    #[serde(default)]
    pub bounds: Option<Bounds>,
}

/// Event fired when the zoom level changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapZoomEvent {
    /// New zoom level
    pub zoom: f64,
}

/// Event fired when the bearing (rotation) changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapRotateEvent {
    /// New bearing in degrees
    pub bearing: f64,
}

/// Event fired when the pitch changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPitchEvent {
    /// New pitch in degrees
    pub pitch: f64,
}

/// Event fired when a feature in a layer is clicked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerClickEvent {
    /// Layer ID where the click occurred
    pub layer_id: String,
    /// GeoJSON feature ID (numeric, if present)
    pub feature_id: Option<i64>,
    /// Feature properties from GeoJSON
    pub properties: serde_json::Value,
    /// Geographic coordinates of the click
    pub latlng: LatLng,
}

/// Event fired when hovering over a feature in a layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerHoverEvent {
    /// Layer ID where the hover occurred
    pub layer_id: String,
    /// GeoJSON feature ID (numeric, if present). None when mouse leaves.
    pub feature_id: Option<i64>,
    /// Feature properties from GeoJSON. None when mouse leaves.
    pub properties: Option<serde_json::Value>,
    /// Geographic coordinates
    pub latlng: LatLng,
    /// Cursor X position (screen coordinates)
    pub cursor_x: f64,
    /// Cursor Y position (screen coordinates)
    pub cursor_y: f64,
}

/// Internal event enum for communication from JS
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
pub enum MapEvent {
    #[serde(rename = "click")]
    Click(MapClickEvent),
    #[serde(rename = "dblclick")]
    DblClick(MapDblClickEvent),
    #[serde(rename = "contextmenu")]
    ContextMenu(MapContextMenuEvent),
    #[serde(rename = "marker_click")]
    MarkerClick(MarkerClickEvent),
    #[serde(rename = "marker_hover")]
    MarkerHover(MarkerHoverEvent),
    #[serde(rename = "move")]
    Move(MapMoveEvent),
    #[serde(rename = "zoom")]
    Zoom(MapZoomEvent),
    #[serde(rename = "rotate")]
    Rotate(MapRotateEvent),
    #[serde(rename = "pitch")]
    Pitch(MapPitchEvent),
    #[serde(rename = "layer_click")]
    LayerClick(LayerClickEvent),
    #[serde(rename = "layer_hover")]
    LayerHover(LayerHoverEvent),
}
