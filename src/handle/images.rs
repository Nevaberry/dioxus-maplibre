//! Image sprite MapHandle methods.

use super::MapHandle;
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

impl MapHandle {
    /// Load an image from a URL and add it to the map's sprite
    pub fn load_image(&self, id: &str, url: &str) {
        self.fire_and_forget(|| crate::interop::load_image_js(&self.map_id, id, url));
    }

    /// Load an image and wait for it to complete (returns true on success)
    #[cfg(target_arch = "wasm32")]
    pub async fn load_image_async(&self, id: &str, url: &str) -> bool {
        let js = crate::interop::load_image_async_js(&self.map_id, id, url);
        document::eval(&js).join::<bool>().await.unwrap_or(false)
    }

    /// Load an image and wait for it to complete (returns true on success)
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn load_image_async(&self, _id: &str, _url: &str) -> bool {
        false
    }

    /// Check if an image exists in the map's sprite
    #[cfg(target_arch = "wasm32")]
    pub async fn has_image(&self, id: &str) -> bool {
        let js = crate::interop::has_image_js(&self.map_id, id);
        document::eval(&js).join::<bool>().await.unwrap_or(false)
    }

    /// Check if an image exists in the map's sprite
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn has_image(&self, _id: &str) -> bool {
        false
    }

    /// Remove an image from the map's sprite
    pub fn remove_image(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::remove_image_js(&self.map_id, id));
    }
}
