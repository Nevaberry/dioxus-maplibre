//! Style-related MapHandle methods.
#![allow(clippy::needless_pass_by_value, clippy::unused_async)]

use super::MapHandle;
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

impl MapHandle {
    /// Change the map's style URL
    pub fn set_style(&self, url: &str) {
        self.fire_and_forget(|| crate::interop::set_style_js(&self.map_id, url));
    }

    /// Set an inline MapLibre style specification and replay managed runtime objects.
    pub fn set_style_json(&self, style: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&style).unwrap_or_else(|_| "null".into());
            crate::interop::set_style_json_js(&self.map_id, &json)
        });
    }

    /// Set the style light specification.
    pub fn set_light(&self, light: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&light).unwrap_or_else(|_| "null".into());
            crate::interop::set_light_js(&self.map_id, &json)
        });
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_style(&self) -> Option<serde_json::Value> {
        document::eval(&crate::interop::get_style_js(&self.map_id))
            .join::<serde_json::Value>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_style(&self) -> Option<serde_json::Value> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_light(&self) -> Option<serde_json::Value> {
        document::eval(&crate::interop::get_light_js(&self.map_id))
            .join::<serde_json::Value>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_light(&self) -> Option<serde_json::Value> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_sky(&self) -> Option<serde_json::Value> {
        document::eval(&crate::interop::get_sky_js(&self.map_id))
            .join::<serde_json::Value>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_sky(&self) -> Option<serde_json::Value> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_terrain(&self) -> Option<serde_json::Value> {
        document::eval(&crate::interop::get_terrain_js(&self.map_id))
            .join::<serde_json::Value>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_terrain(&self) -> Option<serde_json::Value> {
        None
    }
}
