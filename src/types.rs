//! Core geographic types for dioxus-maplibre

use serde::{Deserialize, Serialize};

/// A geographic coordinate (latitude/longitude pair)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct LatLng {
    /// Latitude in degrees (-90 to 90)
    pub lat: f64,
    /// Longitude in degrees (-180 to 180)
    pub lng: f64,
}

impl LatLng {
    /// Create a new coordinate
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }

    /// Helsinki, Finland - example default location
    pub fn helsinki() -> Self {
        Self::new(60.1699, 24.9384)
    }

    /// Convert to [lng, lat] array format used by MapLibre
    pub fn to_array(&self) -> [f64; 2] {
        [self.lng, self.lat]
    }

    /// Create from [lng, lat] array format used by MapLibre
    pub fn from_array(arr: [f64; 2]) -> Self {
        Self {
            lng: arr[0],
            lat: arr[1],
        }
    }
}

/// Map position (center + zoom)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MapPosition {
    /// Center coordinate
    pub center: LatLng,
    /// Zoom level (0-22)
    pub zoom: f64,
}

impl MapPosition {
    pub fn new(center: LatLng, zoom: f64) -> Self {
        Self { center, zoom }
    }
}

impl Default for MapPosition {
    fn default() -> Self {
        Self {
            center: LatLng::helsinki(),
            zoom: 10.0,
        }
    }
}

/// A bounding box defined by southwest and northeast corners
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Bounds {
    /// Southwest corner (min lat, min lng)
    pub sw: LatLng,
    /// Northeast corner (max lat, max lng)
    pub ne: LatLng,
}

impl Bounds {
    pub fn new(sw: LatLng, ne: LatLng) -> Self {
        Self { sw, ne }
    }

    /// Check if a point is within these bounds
    pub fn contains(&self, point: &LatLng) -> bool {
        point.lat >= self.sw.lat
            && point.lat <= self.ne.lat
            && point.lng >= self.sw.lng
            && point.lng <= self.ne.lng
    }

    /// Get the center of the bounds
    pub fn center(&self) -> LatLng {
        LatLng {
            lat: f64::midpoint(self.sw.lat, self.ne.lat),
            lng: f64::midpoint(self.sw.lng, self.ne.lng),
        }
    }
}

/// A feature returned by query methods (`queryRenderedFeatures`, `querySourceFeatures`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryFeature {
    /// Feature ID (numeric, if present)
    #[serde(default)]
    pub id: Option<i64>,
    /// GeoJSON geometry
    pub geometry: serde_json::Value,
    /// Feature properties
    pub properties: serde_json::Value,
    /// Source ID
    pub source: String,
    /// Source layer (for vector tile sources)
    #[serde(default)]
    pub source_layer: Option<String>,
}

/// A point in screen pixel coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Point {
    /// X coordinate in pixels from left edge
    pub x: f64,
    /// Y coordinate in pixels from top edge
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
