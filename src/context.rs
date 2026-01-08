//! Map context for child component communication

use dioxus::prelude::*;

/// Context provided by Map component to its children (Marker, Popup, etc.)
#[derive(Clone)]
pub struct MapContext {
    /// Unique map ID for JS interop
    pub map_id: String,
    /// Signal indicating if the map is ready for interactions
    pub is_ready: Signal<bool>,
}

impl MapContext {
    pub fn new(map_id: String) -> Self {
        Self {
            map_id,
            is_ready: Signal::new(false),
        }
    }
}
