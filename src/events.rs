//! Event types for map interactions

use serde::{Deserialize, Serialize};
use crate::types::{LatLng, Point};

/// Event fired when the map is clicked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapClickEvent {
    /// Geographic coordinates of the click
    pub latlng: LatLng,
    /// Screen pixel coordinates of the click
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

/// Event fired when the map view changes (pan/zoom)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMoveEvent {
    /// New center position
    pub center: LatLng,
    /// New zoom level
    pub zoom: f64,
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
#[allow(dead_code)] // Reserved for future typed event parsing
pub enum MapEvent {
    #[serde(rename = "click")]
    Click(MapClickEvent),
    #[serde(rename = "marker_click")]
    MarkerClick(MarkerClickEvent),
    #[serde(rename = "move")]
    Move(MapMoveEvent),
    #[serde(rename = "layer_click")]
    LayerClick(LayerClickEvent),
    #[serde(rename = "layer_hover")]
    LayerHover(LayerHoverEvent),
}
