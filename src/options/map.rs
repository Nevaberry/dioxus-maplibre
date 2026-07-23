//! Map-wide passthrough options, projections, and interaction identifiers.

use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value};

/// Additional options passed directly to `maplibregl.Map`.
///
/// MapLibre's constructor surface evolves faster than this crate. This wrapper
/// keeps every upstream option available immediately while commonly used
/// options remain available as dedicated `Map` props. Values in `MapOptions`
/// override the corresponding dedicated prop, except for `container`, which is
/// always managed by Dioxus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MapOptions(pub Value);

impl Default for MapOptions {
    fn default() -> Self {
        Self(Value::Object(JsonMap::new()))
    }
}

impl MapOptions {
    /// Create constructor options from a JSON object.
    pub fn new(value: Value) -> Self {
        Self(value)
    }
}

impl From<Value> for MapOptions {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

/// A MapLibre projection specification.
///
/// The JSON passthrough supports projection expressions as well as named
/// projection objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectionOptions(pub Value);

impl ProjectionOptions {
    /// Create a projection from any valid MapLibre projection specification.
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    /// Web Mercator projection.
    pub fn mercator() -> Self {
        Self(serde_json::json!({ "type": "mercator" }))
    }

    /// Globe projection, which adaptively transitions to Mercator at high zoom.
    pub fn globe() -> Self {
        Self(serde_json::json!({ "type": "globe" }))
    }

    /// Vertical-perspective projection.
    pub fn vertical_perspective() -> Self {
        Self(serde_json::json!({ "type": "vertical-perspective" }))
    }
}

/// Built-in gesture handler that can be enabled or disabled at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapInteraction {
    ScrollZoom,
    BoxZoom,
    DragPan,
    DragRotate,
    Keyboard,
    DoubleClickZoom,
    TouchZoomRotate,
    TouchPitch,
}

impl MapInteraction {
    pub(crate) fn js_property(self) -> &'static str {
        match self {
            Self::ScrollZoom => "scrollZoom",
            Self::BoxZoom => "boxZoom",
            Self::DragPan => "dragPan",
            Self::DragRotate => "dragRotate",
            Self::Keyboard => "keyboard",
            Self::DoubleClickZoom => "doubleClickZoom",
            Self::TouchZoomRotate => "touchZoomRotate",
            Self::TouchPitch => "touchPitch",
        }
    }
}
