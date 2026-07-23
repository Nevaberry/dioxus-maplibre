//! Control-related MapHandle methods.
#![allow(clippy::needless_pass_by_value)]

use super::{MapHandle, control_position_str};
use crate::options::{ControlOptions, ControlPosition, TerrainControlOptions};

impl MapHandle {
    /// Add a navigation control (zoom +/- buttons and compass)
    pub fn add_navigation_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_navigation_control_js(&self.map_id, pos)
        });
    }

    /// Add a navigation control with upstream MapLibre options.
    pub fn add_navigation_control_with_options(
        &self,
        position: ControlPosition,
        options: ControlOptions,
    ) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            let json = serde_json::to_string(&options.0).unwrap_or_else(|_| "{}".into());
            crate::interop::add_navigation_control_with_options_js(&self.map_id, pos, &json)
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

    /// Add a geolocate control with upstream MapLibre options.
    pub fn add_geolocate_control_with_options(
        &self,
        position: ControlPosition,
        options: ControlOptions,
    ) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            let json = serde_json::to_string(&options.0).unwrap_or_else(|_| "{}".into());
            crate::interop::add_geolocate_control_with_options_js(&self.map_id, pos, &json)
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

    /// Add a scale control with upstream MapLibre options.
    pub fn add_scale_control_with_options(
        &self,
        position: ControlPosition,
        options: ControlOptions,
    ) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            let json = serde_json::to_string(&options.0).unwrap_or_else(|_| "{}".into());
            crate::interop::add_scale_control_with_options_js(&self.map_id, pos, &json)
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

    /// Add a fullscreen control with upstream MapLibre options.
    pub fn add_fullscreen_control_with_options(
        &self,
        position: ControlPosition,
        options: ControlOptions,
    ) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            let json = serde_json::to_string(&options.0).unwrap_or_else(|_| "{}".into());
            crate::interop::add_fullscreen_control_with_options_js(&self.map_id, pos, &json)
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

    /// Add an attribution control with upstream MapLibre options.
    pub fn add_attribution_control_with_options(
        &self,
        position: ControlPosition,
        options: ControlOptions,
    ) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            let json = serde_json::to_string(&options.0).unwrap_or_else(|_| "{}".into());
            crate::interop::add_attribution_control_with_options_js(&self.map_id, pos, &json)
        });
    }

    /// Remove an attribution control.
    pub fn remove_attribution_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::remove_attribution_control_js(&self.map_id, pos)
        });
    }

    /// Add a globe/Mercator projection toggle control.
    pub fn add_globe_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_globe_control_js(&self.map_id, pos)
        });
    }

    pub fn remove_globe_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::remove_globe_control_js(&self.map_id, pos)
        });
    }

    /// Add the MapLibre logo control.
    pub fn add_logo_control(&self, position: ControlPosition, options: ControlOptions) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            let json = serde_json::to_string(&options.0).unwrap_or_else(|_| "{}".into());
            crate::interop::add_logo_control_js(&self.map_id, pos, &json)
        });
    }

    pub fn remove_logo_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::remove_logo_control_js(&self.map_id, pos)
        });
    }

    /// Add a terrain on/off control.
    pub fn add_terrain_control(&self, position: ControlPosition, options: TerrainControlOptions) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            let json = serde_json::to_string(&options).unwrap_or_else(|_| "{}".into());
            crate::interop::add_terrain_control_js(&self.map_id, pos, &json)
        });
    }

    pub fn remove_terrain_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::remove_terrain_control_js(&self.map_id, pos)
        });
    }
}
