//! Viewport padding MapHandle methods.
#![allow(clippy::needless_pass_by_value, clippy::unused_async)]

use super::MapHandle;
use crate::options::Padding;
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

impl MapHandle {
    /// Set viewport padding
    pub fn set_padding(&self, padding: Padding) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&padding).unwrap_or_default();
            crate::interop::set_padding_js(&self.map_id, &json)
        });
    }

    /// Get current viewport padding
    #[cfg(target_arch = "wasm32")]
    pub async fn get_padding(&self) -> Option<Padding> {
        let js = crate::interop::get_padding_js(&self.map_id);
        document::eval(&js).join::<Padding>().await.ok()
    }

    /// Get current viewport padding
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_padding(&self) -> Option<Padding> {
        None
    }
}
