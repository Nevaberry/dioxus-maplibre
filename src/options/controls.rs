//! Control positioning and viewport padding options.

use serde::{Deserialize, Serialize};

/// Passthrough options for any built-in MapLibre control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ControlOptions(pub serde_json::Value);

impl Default for ControlOptions {
    fn default() -> Self {
        Self(serde_json::json!({}))
    }
}

impl ControlOptions {
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}

/// Options required by MapLibre's terrain toggle control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerrainControlOptions {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exaggeration: Option<f64>,
}

/// Position of a map control on the map canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ControlPosition {
    TopLeft,
    #[default]
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Padding values for map viewport operations.
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
