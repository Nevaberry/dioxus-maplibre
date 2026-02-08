//! Layer option model and builder helpers.

use serde::{Deserialize, Serialize};
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[must_use]
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
    pub fn new(
        id: impl Into<String>,
        layer_type: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
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
