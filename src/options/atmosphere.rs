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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkyOptions(pub serde_json::Value);

/// Compatibility wrapper for MapLibre sky/fog properties.
///
/// Use the `SkySpecification` keys (`fog-color`, `fog-ground-blend`, and the
/// other sky/horizon properties). `MapHandle::set_fog` delegates to `setSky`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FogOptions(pub serde_json::Value);
