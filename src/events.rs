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

/// Internal event enum for communication from JS
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum MapEvent {
    #[serde(rename = "click")]
    Click(MapClickEvent),
    #[serde(rename = "marker_click")]
    MarkerClick(MarkerClickEvent),
    #[serde(rename = "move")]
    Move(MapMoveEvent),
}
