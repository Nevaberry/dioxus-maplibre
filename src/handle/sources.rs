//! Source-related MapHandle methods.
#![allow(clippy::needless_pass_by_value, clippy::unused_async)]

use super::MapHandle;
use crate::options::{
    CanvasSourceOptions, GeoJsonSourceOptions, ImageSourceOptions, RasterDemSourceOptions,
    RasterSourceOptions, SourceOptions, VectorSourceOptions, VideoSourceOptions,
};
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

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

    /// Add a video source to the map.
    pub fn add_video_source(&self, id: &str, options: VideoSourceOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_video_source_js(&self.map_id, id, &json)
        });
    }

    /// Add an HTML canvas source to the map.
    pub fn add_canvas_source(&self, id: &str, options: CanvasSourceOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_canvas_source_js(&self.map_id, id, &json)
        });
    }

    /// Add a source by type using raw MapLibre source options.
    ///
    /// This supports custom source types and future MapLibre source types
    /// without waiting for a crate release.
    pub fn add_source(&self, id: &str, source_type: &str, options: SourceOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options.0).unwrap_or_default();
            crate::interop::add_source_js(&self.map_id, id, source_type, &json)
        });
    }

    /// Update the data of an existing GeoJSON source
    pub fn update_geojson_source(&self, id: &str, data: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&data).unwrap_or_default();
            crate::interop::update_geojson_source_js(&self.map_id, id, &json)
        });
    }

    /// Apply an incremental GeoJSON diff (`add`, `update`, and `remove`).
    pub fn update_geojson_source_diff(&self, id: &str, diff: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&diff).unwrap_or_else(|_| "{}".into());
            crate::interop::update_geojson_source_diff_js(&self.map_id, id, &json)
        });
    }

    /// Update image, video, or canvas source corner coordinates.
    pub fn set_source_coordinates(&self, id: &str, coordinates: [[f64; 2]; 4]) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&coordinates).unwrap_or_else(|_| "[]".into());
            crate::interop::set_source_coordinates_js(&self.map_id, id, &json)
        });
    }

    /// Replace vector or raster tile URL templates.
    pub fn set_source_tiles(&self, id: &str, tiles: Vec<String>) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&tiles).unwrap_or_else(|_| "[]".into());
            crate::interop::set_source_tiles_js(&self.map_id, id, &json)
        });
    }

    /// Replace a vector or raster source's TileJSON URL.
    pub fn set_source_url(&self, id: &str, url: &str) {
        self.fire_and_forget(|| crate::interop::set_source_url_js(&self.map_id, id, url));
    }

    pub fn play_video_source(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::set_video_playing_js(&self.map_id, id, true));
    }

    pub fn pause_video_source(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::set_video_playing_js(&self.map_id, id, false));
    }

    /// Control alpha premultiplication on a raster tile source (MapLibre 6).
    pub fn set_raster_premultiply_alpha(&self, id: &str, enabled: bool) {
        self.fire_and_forget(|| {
            crate::interop::set_raster_premultiply_alpha_js(&self.map_id, id, enabled)
        });
    }

    /// Configure tile level-of-detail behavior globally or for one source.
    pub fn set_source_tile_lod_params(
        &self,
        max_zoom_levels_on_screen: f64,
        tile_count_max_min_ratio: f64,
        source_id: Option<&str>,
    ) {
        self.fire_and_forget(|| {
            crate::interop::set_source_tile_lod_params_js(
                &self.map_id,
                max_zoom_levels_on_screen,
                tile_count_max_min_ratio,
                source_id,
            )
        });
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_geojson_source_data(&self, id: &str) -> Option<serde_json::Value> {
        document::eval(&crate::interop::get_geojson_source_data_js(
            &self.map_id,
            id,
        ))
        .join::<serde_json::Value>()
        .await
        .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_geojson_source_data(&self, _id: &str) -> Option<serde_json::Value> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_cluster_expansion_zoom(&self, id: &str, cluster_id: u64) -> Option<f64> {
        document::eval(&crate::interop::get_cluster_expansion_zoom_js(
            &self.map_id,
            id,
            cluster_id,
        ))
        .join::<f64>()
        .await
        .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_cluster_expansion_zoom(&self, _id: &str, _cluster_id: u64) -> Option<f64> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_cluster_children(&self, id: &str, cluster_id: u64) -> Vec<serde_json::Value> {
        document::eval(&crate::interop::get_cluster_children_js(
            &self.map_id,
            id,
            cluster_id,
        ))
        .join::<Vec<serde_json::Value>>()
        .await
        .unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_cluster_children(
        &self,
        _id: &str,
        _cluster_id: u64,
    ) -> Vec<serde_json::Value> {
        Vec::new()
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_cluster_leaves(
        &self,
        id: &str,
        cluster_id: u64,
        limit: u32,
        offset: u32,
    ) -> Vec<serde_json::Value> {
        document::eval(&crate::interop::get_cluster_leaves_js(
            &self.map_id,
            id,
            cluster_id,
            limit,
            offset,
        ))
        .join::<Vec<serde_json::Value>>()
        .await
        .unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_cluster_leaves(
        &self,
        _id: &str,
        _cluster_id: u64,
        _limit: u32,
        _offset: u32,
    ) -> Vec<serde_json::Value> {
        Vec::new()
    }

    /// Remove a source from the map
    pub fn remove_source(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::remove_source_js(&self.map_id, id));
    }
}
