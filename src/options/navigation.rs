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

    /// Target roll around the camera boresight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f64>,

    /// Target center elevation in meters above sea level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

    /// Animation duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    /// If true, animation is considered essential (not affected by prefers-reduced-motion)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,

    /// Whether animation should run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate: Option<bool>,

    /// Target offset in screen pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<[f64; 2]>,

    /// Keep camera height constant relative to sea level during terrain movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_elevation: Option<bool>,

    /// Flight-path curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve: Option<f64>,

    /// Lowest zoom allowed along the flight arc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_zoom: Option<f64>,

    /// Average flight speed in screenfuls per curve unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,

    /// Average linear speed in screenfuls per second.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_speed: Option<f64>,

    /// Maximum animation duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<u32>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

    /// Animation duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<[f64; 2]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_elevation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_end_events: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub around: Option<LatLng>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ease_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_move_start: Option<bool>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

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

    /// Target bearing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<f64>,

    /// Target pitch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,

    /// Target roll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f64>,

    /// Target elevation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

    /// Target offset in screen pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<[f64; 2]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_elevation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_zoom: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<u32>,
}
