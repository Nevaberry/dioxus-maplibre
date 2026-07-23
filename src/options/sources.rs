//! Source option models.

use serde::{Deserialize, Serialize};
/// Options for adding a GeoJSON source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeoJsonSourceOptions {
    /// GeoJSON data (FeatureCollection, Feature, or Geometry)
    pub data: serde_json::Value,

    /// Enable clustering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<bool>,

    /// Radius of each cluster (in pixels, default 50)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_radius: Option<u32>,

    /// Max zoom level to cluster points (default 14)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_max_zoom: Option<u32>,

    /// Custom cluster properties (MapLibre expression format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_properties: Option<serde_json::Value>,

    /// Automatically assign numeric IDs to features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_id: Option<bool>,

    /// Property to use as feature ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promote_id: Option<String>,

    /// Maximum zoom at which tiles are created (default 18).
    #[serde(rename = "maxzoom", skip_serializing_if = "Option::is_none")]
    pub max_zoom: Option<u32>,

    /// Tile buffer in pixels (default 128).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<u32>,

    /// Douglas-Peucker simplification tolerance (default 0.375).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,

    /// Whether line-distance metrics are calculated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_metrics: Option<bool>,

    /// Minimum number of points required to form a cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_min_points: Option<u32>,

    /// Attribution HTML string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,

    /// Worker-side GeoJSON filter expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

/// Options for adding a vector tile source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VectorSourceOptions {
    /// TileJSON URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Array of tile URL templates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Vec<String>>,

    /// Minimum zoom level (default 0)
    #[serde(rename = "minzoom", skip_serializing_if = "Option::is_none")]
    pub min_zoom: Option<u32>,

    /// Maximum zoom level (default 22)
    #[serde(rename = "maxzoom", skip_serializing_if = "Option::is_none")]
    pub max_zoom: Option<u32>,

    /// Geographic bounds [sw_lng, sw_lat, ne_lng, ne_lat]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<[f64; 4]>,

    /// Attribution HTML string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,

    /// Tile scheme (`xyz` or `tms`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    /// Property or per-source-layer mapping used as feature IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promote_id: Option<serde_json::Value>,

    /// Vector encoding (`mvt` or MapLibre Tile `mlt`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Marks tiles as volatile for cache handling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatile: Option<bool>,
}

/// Options for adding a raster tile source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RasterSourceOptions {
    /// TileJSON URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Array of tile URL templates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Vec<String>>,

    /// Tile size in pixels (default 512)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_size: Option<u32>,

    /// Minimum zoom level
    #[serde(rename = "minzoom", skip_serializing_if = "Option::is_none")]
    pub min_zoom: Option<u32>,

    /// Maximum zoom level
    #[serde(rename = "maxzoom", skip_serializing_if = "Option::is_none")]
    pub max_zoom: Option<u32>,

    /// Geographic bounds [sw_lng, sw_lat, ne_lng, ne_lat].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<[f64; 4]>,

    /// Tile scheme (`xyz` or `tms`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    /// Attribution HTML string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,

    /// Marks tiles as volatile for cache handling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatile: Option<bool>,
}

/// Options for adding a raster DEM source (for terrain)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RasterDemSourceOptions {
    /// TileJSON URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Array of tile URL templates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Vec<String>>,

    /// Tile size in pixels (default 512)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_size: Option<u32>,

    /// Encoding type: "mapbox" or "terrarium"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(rename = "minzoom", skip_serializing_if = "Option::is_none")]
    pub min_zoom: Option<u32>,

    #[serde(rename = "maxzoom", skip_serializing_if = "Option::is_none")]
    pub max_zoom: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<[f64; 4]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatile: Option<bool>,

    /// Custom red-channel decoding factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_factor: Option<f64>,

    /// Custom green-channel decoding factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_factor: Option<f64>,

    /// Custom blue-channel decoding factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_factor: Option<f64>,

    /// Custom elevation decoding base shift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_shift: Option<f64>,
}

/// Options for adding an image source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageSourceOptions {
    /// Image URL
    pub url: String,

    /// Four corner coordinates as `[[lng, lat], [lng, lat], [lng, lat], [lng, lat]]`
    /// Order: top-left, top-right, bottom-right, bottom-left
    pub coordinates: [[f64; 2]; 4],
}

/// Options for adding a video source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSourceOptions {
    /// Video URLs in browser preference order.
    pub urls: Vec<String>,
    /// Top-left, top-right, bottom-right, bottom-left coordinates.
    pub coordinates: [[f64; 2]; 4],
}

/// Options for adding an HTML canvas source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasSourceOptions {
    /// ID of the `HTMLCanvasElement` to use.
    pub canvas: String,
    /// Top-left, top-right, bottom-right, bottom-left coordinates.
    pub coordinates: [[f64; 2]; 4],
    /// Re-read canvas pixels every frame when true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate: Option<bool>,
}

/// Future-proof source options for custom or newly introduced source types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SourceOptions(pub serde_json::Value);

impl SourceOptions {
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
