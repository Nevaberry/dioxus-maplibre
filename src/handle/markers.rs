//! Marker-related MapHandle methods.

use super::MapHandle;
use crate::options::MarkerOptions;
use crate::types::LatLng;

impl MapHandle {
    /// Add a marker at the given position
    pub fn add_marker(&self, id: &str, position: LatLng, options: MarkerOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_marker_js(&self.map_id, id, position.lat, position.lng, &json)
        });
    }

    /// Remove a marker
    pub fn remove_marker(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::remove_marker_js(&self.map_id, id));
    }

    /// Update a marker's position
    pub fn update_marker_position(&self, id: &str, position: LatLng) {
        self.fire_and_forget(|| {
            crate::interop::update_marker_position_js(&self.map_id, id, position.lat, position.lng)
        });
    }
}
