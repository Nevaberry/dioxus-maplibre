//! Terrain, sky, and fog MapHandle methods.

use super::MapHandle;
use crate::options::{FogOptions, SkyOptions, TerrainOptions};

impl MapHandle {
    /// Enable 3D terrain
    pub fn set_terrain(&self, options: TerrainOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::set_terrain_js(&self.map_id, &json)
        });
    }

    /// Remove 3D terrain
    pub fn remove_terrain(&self) {
        self.fire_and_forget(|| crate::interop::remove_terrain_js(&self.map_id));
    }

    /// Set sky properties
    pub fn set_sky(&self, options: SkyOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options.0).unwrap_or_default();
            crate::interop::set_sky_js(&self.map_id, &json)
        });
    }

    /// Remove sky
    pub fn remove_sky(&self) {
        self.fire_and_forget(|| crate::interop::remove_sky_js(&self.map_id));
    }

    /// Set fog/atmosphere properties
    pub fn set_fog(&self, options: FogOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options.0).unwrap_or_default();
            crate::interop::set_fog_js(&self.map_id, &json)
        });
    }

    /// Remove fog/atmosphere
    pub fn remove_fog(&self) {
        self.fire_and_forget(|| crate::interop::remove_fog_js(&self.map_id));
    }
}
