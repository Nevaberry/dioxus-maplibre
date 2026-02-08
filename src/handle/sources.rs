//! Source-related MapHandle methods.

use super::MapHandle;
use crate::options::{
    GeoJsonSourceOptions, ImageSourceOptions, RasterDemSourceOptions, RasterSourceOptions,
    VectorSourceOptions,
};

impl MapHandle {
    /// Add a GeoJSON source to the map
    pub fn add_geojson_source(&self, id: &str, options: GeoJsonSourceOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_geojson_source_js(&self.map_id, id, &json)
        });
    }

    /// Add a vector tile source to the map
    pub fn add_vector_source(&self, id: &str, options: VectorSourceOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_vector_source_js(&self.map_id, id, &json)
        });
    }

    /// Add a raster tile source to the map
    pub fn add_raster_source(&self, id: &str, options: RasterSourceOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_raster_source_js(&self.map_id, id, &json)
        });
    }

    /// Add a raster DEM source (for terrain)
    pub fn add_raster_dem_source(&self, id: &str, options: RasterDemSourceOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_raster_dem_source_js(&self.map_id, id, &json)
        });
    }

    /// Add an image source to the map
    pub fn add_image_source(&self, id: &str, options: ImageSourceOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_image_source_js(&self.map_id, id, &json)
        });
    }

    /// Update the data of an existing GeoJSON source
    pub fn update_geojson_source(&self, id: &str, data: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&data).unwrap_or_default();
            crate::interop::update_geojson_source_js(&self.map_id, id, &json)
        });
    }

    /// Remove a source from the map
    pub fn remove_source(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::remove_source_js(&self.map_id, id));
    }
}
