//! Async getter MapHandle methods.

use super::MapHandle;
use crate::types::{Bounds, LatLng};
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

impl MapHandle {
    /// Get the current zoom level
    #[cfg(target_arch = "wasm32")]
    pub async fn get_zoom(&self) -> Option<f64> {
        let js = crate::interop::get_zoom_js(&self.map_id);
        document::eval(&js).join::<f64>().await.ok()
    }

    /// Get the current center coordinate
    #[cfg(target_arch = "wasm32")]
    pub async fn get_center(&self) -> Option<LatLng> {
        let js = crate::interop::get_center_js(&self.map_id);
        document::eval(&js).join::<LatLng>().await.ok()
    }

    /// Get the current bearing (rotation)
    #[cfg(target_arch = "wasm32")]
    pub async fn get_bearing(&self) -> Option<f64> {
        let js = crate::interop::get_bearing_js(&self.map_id);
        document::eval(&js).join::<f64>().await.ok()
    }

    /// Get the current pitch (tilt)
    #[cfg(target_arch = "wasm32")]
    pub async fn get_pitch(&self) -> Option<f64> {
        let js = crate::interop::get_pitch_js(&self.map_id);
        document::eval(&js).join::<f64>().await.ok()
    }

    /// Get the current viewport bounds
    #[cfg(target_arch = "wasm32")]
    pub async fn get_bounds(&self) -> Option<Bounds> {
        let js = crate::interop::get_bounds_js(&self.map_id);
        document::eval(&js).join::<Bounds>().await.ok()
    }

    // No-op stubs for native targets
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_zoom(&self) -> Option<f64> {
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_center(&self) -> Option<LatLng> {
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_bearing(&self) -> Option<f64> {
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_pitch(&self) -> Option<f64> {
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_bounds(&self) -> Option<Bounds> {
        None
    }
}
