//! Terrain and atmosphere option models.

use serde::{Deserialize, Serialize};
/// Options for setting terrain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkyOptions(pub serde_json::Value);

/// Options for setting fog/atmosphere (passthrough to MapLibre spec)
///
/// The fog spec supports color, horizon-blend, range, star-intensity, and more.
/// Pass any valid MapLibre fog specification as a JSON value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FogOptions(pub serde_json::Value);
