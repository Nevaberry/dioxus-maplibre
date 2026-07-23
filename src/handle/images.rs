//! Image sprite MapHandle methods.
#![allow(clippy::needless_pass_by_value, clippy::unused_async)]

use super::MapHandle;
use crate::options::MissingImageOptions;
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

    /// Install a generated checkerboard resolver for missing style images.
    ///
    /// MapLibre 6 invokes this resolver before emitting `styleimagemissing`.
    pub fn set_missing_image_resolver(&self, options: MissingImageOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_else(|_| "{}".into());
            crate::interop::set_missing_image_resolver_js(&self.map_id, &json)
        });
    }

    /// Remove the currently installed missing-style-image resolver.
    pub fn clear_missing_image_resolver(&self) {
        self.fire_and_forget(|| crate::interop::clear_missing_image_resolver_js(&self.map_id));
    }

    /// List image IDs currently available to the active style.
    #[cfg(target_arch = "wasm32")]
    pub async fn list_images(&self) -> Vec<String> {
        let js = crate::interop::list_images_js(&self.map_id);
        document::eval(&js)
            .join::<Vec<String>>()
            .await
            .unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn list_images(&self) -> Vec<String> {
        Vec::new()
    }
}
