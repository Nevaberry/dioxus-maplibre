//! Runtime style image options.

use serde::{Deserialize, Serialize};

/// Built-in checkerboard generator for MapLibre 6's missing-style-image resolver.
///
/// This is useful for patterns and deterministic demos without external image
/// hosting. Applications needing network or domain-specific resolution can use
/// `MapHandle::eval` to install their own async resolver.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissingImageOptions {
    pub width: u32,
    pub height: u32,
    pub cell_size: u32,
    pub primary_color: String,
    pub secondary_color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_ratio: Option<f64>,
}

impl Default for MissingImageOptions {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
            cell_size: 8,
            primary_color: "#2563eb".into(),
            secondary_color: "#bfdbfe".into(),
            pixel_ratio: Some(2.0),
        }
    }
}
