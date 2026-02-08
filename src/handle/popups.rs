//! Popup-related MapHandle methods.

use super::MapHandle;
use crate::options::PopupOptions;
use crate::types::LatLng;

impl MapHandle {
    /// Add a standalone popup at a position
    pub fn add_popup(&self, id: &str, position: LatLng, html: &str, options: PopupOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_popup_js(&self.map_id, id, position.lat, position.lng, html, &json)
        });
    }

    /// Remove a popup
    pub fn remove_popup(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::remove_popup_js(&self.map_id, id));
    }
}
