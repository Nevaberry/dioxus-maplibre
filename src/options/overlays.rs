//! Marker and popup option models.

use serde::{Deserialize, Serialize};

use super::controls::Padding;
/// Options for adding a marker to the map
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarkerOptions {
    /// ID of a DOM element to use as the marker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,

    /// Space-separated CSS classes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// Pixel offset from the marker anchor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<[f64; 2]>,

    /// Marker anchor (`center`, `top`, `bottom-left`, and so on).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_tolerance: Option<f64>,

    /// `map`, `viewport`, or `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_alignment: Option<String>,

    /// `map`, `viewport`, or `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch_alignment: Option<String>,

    /// CSS opacity as a number or string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<serde_json::Value>,

    /// CSS opacity when covered by terrain or the globe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity_when_covered: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpixel_positioning: Option<bool>,

    /// Emoji to display instead of default marker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    /// HTML content for a popup attached to the marker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popup_html: Option<String>,
}

/// Options for creating a popup
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

    /// Close when the map starts moving.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_move: Option<bool>,

    /// Focus the first focusable popup child after opening.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_after_open: Option<bool>,

    /// Max width CSS value (e.g., "300px")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_width: Option<String>,

    /// CSS class name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpixel_positioning: Option<bool>,

    /// Opacity while the location is occluded by the globe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_occluded_opacity: Option<serde_json::Value>,

    /// Padding from map-container edges during popup placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
}
