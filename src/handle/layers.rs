//! Layer-related MapHandle methods.
#![allow(clippy::needless_pass_by_value)]

use super::MapHandle;
use crate::options::LayerOptions;

impl MapHandle {
    /// Add a layer to the map
    pub fn add_layer(&self, options: LayerOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_layer_js(&self.map_id, &json)
        });
    }

    /// Remove a layer from the map
    pub fn remove_layer(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::remove_layer_js(&self.map_id, id));
    }

    /// Set a paint property on a layer
    pub fn set_paint_property(&self, layer_id: &str, name: &str, value: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&value).unwrap_or_default();
            crate::interop::set_paint_property_js(&self.map_id, layer_id, name, &json)
        });
    }

    /// Set a layout property on a layer
    pub fn set_layout_property(&self, layer_id: &str, name: &str, value: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&value).unwrap_or_default();
            crate::interop::set_layout_property_js(&self.map_id, layer_id, name, &json)
        });
    }

    /// Move a layer to a different position in the layer stack
    ///
    /// If `before_id` is `Some`, the layer is moved before that layer.
    /// If `before_id` is `None`, the layer is moved to the top.
    pub fn move_layer(&self, layer_id: &str, before_id: Option<&str>) {
        self.fire_and_forget(|| crate::interop::move_layer_js(&self.map_id, layer_id, before_id));
    }

    /// Set a filter on a layer
    pub fn set_filter(&self, layer_id: &str, filter: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&filter).unwrap_or_default();
            crate::interop::set_filter_js(&self.map_id, layer_id, &json)
        });
    }
}
