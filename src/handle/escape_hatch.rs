//! Escape-hatch MapHandle methods.

use super::MapHandle;
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

impl MapHandle {
    /// Execute arbitrary JavaScript against this map's instance.
    ///
    /// The JS code receives `map` as a variable referencing the MapLibre map object.
    /// This is a fire-and-forget operation.
    pub fn eval(&self, js_code: &str) {
        let find = crate::interop::find_map_js(&self.map_id);
        let full_js = format!("(function() {{ {find} {js_code} }})();");
        self.eval_raw(&full_js);
    }

    /// Execute arbitrary JavaScript and return a deserialized result.
    ///
    /// The JS code should return a value. It receives `map` as a variable.
    #[cfg(target_arch = "wasm32")]
    pub async fn eval_async<T: serde::de::DeserializeOwned>(&self, js_code: &str) -> Option<T> {
        let find = crate::interop::find_map_js(&self.map_id);
        let full_js = format!("(function() {{ {find} {js_code} }})();");
        document::eval(&full_js).join::<T>().await.ok()
    }

    #[allow(clippy::unused_async)]
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn eval_async<T: serde::de::DeserializeOwned>(&self, _js_code: &str) -> Option<T> {
        None
    }
}
