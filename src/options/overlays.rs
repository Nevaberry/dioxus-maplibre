//! Marker and popup option models.

use serde::{Deserialize, Serialize};
/// Options for adding a marker to the map
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

    /// Max width CSS value (e.g., "300px")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_width: Option<String>,

    /// CSS class name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
}
