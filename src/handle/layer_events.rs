//! Layer event registration MapHandle methods.

use super::MapHandle;

impl MapHandle {
    /// Register a click handler on a layer (events dispatched via `on_layer_click`).
    pub fn on_layer_click(&self, layer_id: &str) {
        self.fire_and_forget(|| crate::interop::register_layer_click_js(&self.map_id, layer_id));
    }

    /// Unregister a click handler on a layer.
    pub fn off_layer_click(&self, layer_id: &str) {
        self.fire_and_forget(|| crate::interop::unregister_layer_click_js(&self.map_id, layer_id));
    }

    /// Register hover handlers on a layer (events dispatched via `on_layer_hover`).
    pub fn on_layer_hover(&self, layer_id: &str) {
        self.fire_and_forget(|| crate::interop::register_layer_hover_js(&self.map_id, layer_id));
    }

    /// Unregister hover handlers on a layer.
    pub fn off_layer_hover(&self, layer_id: &str) {
        self.fire_and_forget(|| crate::interop::unregister_layer_hover_js(&self.map_id, layer_id));
    }
}
