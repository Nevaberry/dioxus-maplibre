//! Option types for MapLibre GL JS operations
//!
//! All structs serialize to camelCase JSON matching the MapLibre JS API.
//! Optional fields are skipped when `None` for clean output.

use serde::{Deserialize, Serialize};

use crate::types::LatLng;

/// Position of a map control on the map canvas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ControlPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for ControlPosition {
    fn default() -> Self {
        Self::TopRight
    }
}

/// Padding values for map viewport operations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Padding {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

impl Padding {
    pub fn uniform(value: f64) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }
}

/// Options for adding a GeoJSON source
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageSourceOptions {
    /// Image URL
    pub url: String,

    /// Four corner coordinates [[lng,lat], [lng,lat], [lng,lat], [lng,lat]]
    /// Order: top-left, top-right, bottom-right, bottom-left
    pub coordinates: [[f64; 2]; 4],
}

/// Options for adding a map layer
///
/// # Examples
///
/// ```
/// use dioxus_maplibre::LayerOptions;
/// use serde_json::json;
///
/// let layer = LayerOptions::circle("my-circles", "my-source")
///     .paint(json!({
///         "circle-radius": 6,
///         "circle-color": "#3b82f6"
///     }));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerOptions {
    /// Unique layer ID
    pub id: String,

    /// Layer type: circle, fill, line, symbol, fill-extrusion, heatmap, raster, background
    #[serde(rename = "type")]
    pub layer_type: String,

    /// Source ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Source layer (for vector tile sources)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer: Option<String>,

    /// Paint properties (MapLibre style spec)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint: Option<serde_json::Value>,

    /// Layout properties (MapLibre style spec)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<serde_json::Value>,

    /// Filter expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,

    /// Minimum zoom level for this layer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_zoom: Option<f64>,

    /// Maximum zoom level for this layer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_zoom: Option<f64>,
}

impl LayerOptions {
    /// Create a new layer with the given type
    pub fn new(id: impl Into<String>, layer_type: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            layer_type: layer_type.into(),
            source: Some(source.into()),
            source_layer: None,
            paint: None,
            layout: None,
            filter: None,
            min_zoom: None,
            max_zoom: None,
        }
    }

    /// Create a circle layer
    pub fn circle(id: impl Into<String>, source: impl Into<String>) -> Self {
        Self::new(id, "circle", source)
    }

    /// Create a fill layer
    pub fn fill(id: impl Into<String>, source: impl Into<String>) -> Self {
        Self::new(id, "fill", source)
    }

    /// Create a line layer
    pub fn line(id: impl Into<String>, source: impl Into<String>) -> Self {
        Self::new(id, "line", source)
    }

    /// Create a symbol layer
    pub fn symbol(id: impl Into<String>, source: impl Into<String>) -> Self {
        Self::new(id, "symbol", source)
    }

    /// Create a fill-extrusion layer
    pub fn fill_extrusion(id: impl Into<String>, source: impl Into<String>) -> Self {
        Self::new(id, "fill-extrusion", source)
    }

    /// Create a heatmap layer
    pub fn heatmap(id: impl Into<String>, source: impl Into<String>) -> Self {
        Self::new(id, "heatmap", source)
    }

    /// Create a raster layer
    pub fn raster(id: impl Into<String>, source: impl Into<String>) -> Self {
        Self::new(id, "raster", source)
    }

    /// Create a background layer (no source needed)
    pub fn background(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            layer_type: "background".into(),
            source: None,
            source_layer: None,
            paint: None,
            layout: None,
            filter: None,
            min_zoom: None,
            max_zoom: None,
        }
    }

    /// Set the source layer (for vector tile sources)
    pub fn source_layer(mut self, layer: impl Into<String>) -> Self {
        self.source_layer = Some(layer.into());
        self
    }

    /// Set paint properties
    pub fn paint(mut self, paint: serde_json::Value) -> Self {
        self.paint = Some(paint);
        self
    }

    /// Set layout properties
    pub fn layout(mut self, layout: serde_json::Value) -> Self {
        self.layout = Some(layout);
        self
    }

    /// Set a filter expression
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Set minimum zoom level
    pub fn min_zoom(mut self, zoom: f64) -> Self {
        self.min_zoom = Some(zoom);
        self
    }

    /// Set maximum zoom level
    pub fn max_zoom(mut self, zoom: f64) -> Self {
        self.max_zoom = Some(zoom);
        self
    }
}

/// Options for adding a marker to the map
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarkerOptions {
    /// CSS color string (default "#3b82f6")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Whether the marker is draggable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,

    /// Rotation angle in degrees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,

    /// Scale factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,

    /// Emoji to display instead of default marker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    /// HTML content for a popup attached to the marker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popup_html: Option<String>,
}

/// Options for creating a popup
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PopupOptions {
    /// Pixel offset [x, y] from anchor point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<[f64; 2]>,

    /// Anchor position: "top", "bottom", "left", "right", "center", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,

    /// Show close button (default true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_button: Option<bool>,

    /// Close popup on map click (default true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_click: Option<bool>,

    /// Max width CSS value (e.g., "300px")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_width: Option<String>,

    /// CSS class name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
}

/// Options for `fly_to` animation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlyToOptions {
    /// Target center
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<LatLng>,

    /// Target zoom level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<f64>,

    /// Target bearing in degrees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<f64>,

    /// Target pitch in degrees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,

    /// Animation duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    /// If true, animation is considered essential (not affected by prefers-reduced-motion)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,

    /// Viewport padding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
}

/// Options for `ease_to` animation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EaseToOptions {
    /// Target center
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<LatLng>,

    /// Target zoom level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<f64>,

    /// Target bearing in degrees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<f64>,

    /// Target pitch in degrees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,

    /// Animation duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    /// Viewport padding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
}

/// Options for `jump_to` (instant, no animation)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JumpToOptions {
    /// Target center
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<LatLng>,

    /// Target zoom level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<f64>,

    /// Target bearing in degrees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<f64>,

    /// Target pitch in degrees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,

    /// Viewport padding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
}

/// Options for `fit_bounds`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FitBoundsOptions {
    /// Viewport padding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,

    /// Maximum zoom level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_zoom: Option<f64>,

    /// Animation duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    /// If true, use linear easing (no curve)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<bool>,
}

/// Options for setting terrain
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerrainOptions {
    /// Source ID of a raster-dem source
    pub source: String,

    /// Terrain exaggeration factor (default 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exaggeration: Option<f64>,
}

/// Options for setting sky (passthrough to MapLibre spec)
///
/// The sky spec is complex with many expression-based properties.
/// Pass any valid MapLibre sky specification as a JSON value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyOptions(pub serde_json::Value);

/// Options for querying rendered or source features
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryOptions {
    /// Restrict query to specific layer IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,

    /// Filter expression to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

/// Identifies a feature for feature state operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureIdentifier {
    /// Source ID
    pub source: String,

    /// Feature ID (must be numeric for MapLibre feature state)
    pub id: i64,

    /// Source layer (required for vector tile sources)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer: Option<String>,
}
