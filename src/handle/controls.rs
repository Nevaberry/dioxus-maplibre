//! Control-related MapHandle methods.

use super::{MapHandle, control_position_str};
use crate::options::ControlPosition;

impl MapHandle {
    /// Add a navigation control (zoom +/- buttons and compass)
    pub fn add_navigation_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_navigation_control_js(&self.map_id, pos)
        });
    }

    /// Remove a navigation control.
    pub fn remove_navigation_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::remove_navigation_control_js(&self.map_id, pos)
        });
    }

    /// Add a geolocate control.
    pub fn add_geolocate_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_geolocate_control_js(&self.map_id, pos)
        });
    }

    /// Remove a geolocate control.
    pub fn remove_geolocate_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::remove_geolocate_control_js(&self.map_id, pos)
        });
    }

    /// Add a scale control.
    pub fn add_scale_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_scale_control_js(&self.map_id, pos)
        });
    }

    /// Remove a scale control.
    pub fn remove_scale_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::remove_scale_control_js(&self.map_id, pos)
        });
    }

    /// Add a fullscreen control.
    pub fn add_fullscreen_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_fullscreen_control_js(&self.map_id, pos)
        });
    }

    /// Remove a fullscreen control.
    pub fn remove_fullscreen_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::remove_fullscreen_control_js(&self.map_id, pos)
        });
    }

    /// Add an attribution control.
    pub fn add_attribution_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_attribution_control_js(&self.map_id, pos)
        });
    }

    /// Remove an attribution control.
    pub fn remove_attribution_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::remove_attribution_control_js(&self.map_id, pos)
        });
    }
}
