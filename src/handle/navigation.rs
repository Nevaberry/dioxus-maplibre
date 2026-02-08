//! Navigation and camera MapHandle methods.
#![allow(clippy::needless_pass_by_value)]

use super::MapHandle;
use crate::options::{EaseToOptions, FitBoundsOptions, FlyToOptions, JumpToOptions};
use crate::types::{Bounds, LatLng};

impl MapHandle {
    /// Fly to a location with animation
    pub fn fly_to(&self, options: FlyToOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::fly_to_js(&self.map_id, &json)
        });
    }

    /// Ease to a location with animation
    pub fn ease_to(&self, options: EaseToOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::ease_to_js(&self.map_id, &json)
        });
    }

    /// Jump to a location instantly (no animation)
    pub fn jump_to(&self, options: JumpToOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::jump_to_js(&self.map_id, &json)
        });
    }

    /// Fit the map to the given bounds
    pub fn fit_bounds(&self, bounds: Bounds, options: FitBoundsOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::fit_bounds_js(
                &self.map_id,
                bounds.sw.lng,
                bounds.sw.lat,
                bounds.ne.lng,
                bounds.ne.lat,
                &json,
            )
        });
    }

    /// Pan to a coordinate
    pub fn pan_to(&self, position: LatLng) {
        self.fire_and_forget(|| {
            crate::interop::pan_to_js(&self.map_id, position.lat, position.lng)
        });
    }

    /// Pan by pixel offset (instant, no animation)
    pub fn pan_by(&self, x: i32, y: i32) {
        self.fire_and_forget(|| crate::interop::pan_by_js(&self.map_id, x, y));
    }

    /// Set zoom level
    pub fn zoom_to(&self, zoom: f64) {
        self.fire_and_forget(|| crate::interop::zoom_to_js(&self.map_id, zoom));
    }

    /// Zoom in one level
    pub fn zoom_in(&self) {
        self.fire_and_forget(|| crate::interop::zoom_in_js(&self.map_id));
    }

    /// Zoom out one level
    pub fn zoom_out(&self) {
        self.fire_and_forget(|| crate::interop::zoom_out_js(&self.map_id));
    }

    /// Set bearing (rotation)
    pub fn rotate_to(&self, bearing: f64) {
        self.fire_and_forget(|| crate::interop::rotate_to_js(&self.map_id, bearing));
    }

    /// Set pitch (tilt)
    pub fn set_pitch(&self, pitch: f64) {
        self.fire_and_forget(|| crate::interop::set_pitch_js(&self.map_id, pitch));
    }

    /// Reset bearing to north (0 degrees)
    pub fn reset_north(&self) {
        self.fire_and_forget(|| crate::interop::reset_north_js(&self.map_id));
    }

    /// Set throttle for `on_move` events in milliseconds (0 = every animation frame)
    pub fn set_move_event_throttle(&self, throttle_ms: u32) {
        self.fire_and_forget(|| {
            crate::interop::set_move_event_throttle_js(&self.map_id, throttle_ms)
        });
    }
}
