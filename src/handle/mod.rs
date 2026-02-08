//! MapHandle - the primary API for interacting with a MapLibre map.

mod controls;
mod escape_hatch;
mod feature_state;
mod getters;
mod images;
mod layer_events;
mod layers;
mod markers;
mod navigation;
mod padding;
mod popups;
mod queries;
mod sources;
mod style;
mod terrain_atmosphere;

use crate::options::ControlPosition;

/// A handle to a MapLibre map instance.
///
/// This is a lightweight `Clone` wrapper. Store it in a `Signal<Option<MapHandle>>`
/// and set it in the `on_ready` callback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapHandle {
    map_id: String,
}

impl MapHandle {
    /// Create a new `MapHandle` (called internally by the `Map` component).
    #[allow(dead_code)] // Used only on wasm32 target
    pub(crate) fn new(map_id: String) -> Self {
        Self { map_id }
    }

    /// Get the internal map ID (useful for debugging).
    pub fn map_id(&self) -> &str {
        &self.map_id
    }

    /// Fire-and-forget: spawn an async eval that we don't wait for.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn fire_and_forget(&self, js_fn: impl FnOnce() -> String) {
        let js = js_fn();
        dioxus::prelude::spawn(async move {
            let _ = document::eval(&js).await;
        });
    }

    #[allow(clippy::unused_self)]
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn fire_and_forget(&self, _js_fn: impl FnOnce() -> String) {
        // No-op on non-wasm targets.
    }

    /// Execute raw JS without wrapping (for escape hatch).
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn eval_raw(&self, js: &str) {
        let js = js.to_string();
        dioxus::prelude::spawn(async move {
            let _ = document::eval(&js).await;
        });
    }

    #[allow(clippy::unused_self)]
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn eval_raw(&self, _js: &str) {}
}

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

pub(crate) fn control_position_str(pos: ControlPosition) -> &'static str {
    match pos {
        ControlPosition::TopLeft => "top-left",
        ControlPosition::TopRight => "top-right",
        ControlPosition::BottomLeft => "bottom-left",
        ControlPosition::BottomRight => "bottom-right",
    }
}
