//! Source option models.

use serde::{Deserialize, Serialize};
/// Options for adding a GeoJSON source
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_zoom: Option<u32>,

    /// Maximum zoom level (default 22)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_zoom: Option<u32>,

    /// Geographic bounds [sw_lng, sw_lat, ne_lng, ne_lat]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<[f64; 4]>,

    /// Attribution HTML string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,
}

/// Options for adding a raster tile source
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_zoom: Option<u32>,

    /// Maximum zoom level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_zoom: Option<u32>,
}

/// Options for adding a raster DEM source (for terrain)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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
