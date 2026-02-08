//! Navigation and animation option models.

use serde::{Deserialize, Serialize};

use crate::types::LatLng;

use super::controls::Padding;
/// Options for `fly_to` animation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
