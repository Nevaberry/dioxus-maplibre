//! Map-wide camera, projection, interaction, and coordinate methods.
#![allow(clippy::needless_pass_by_value, clippy::unused_async)]

use super::MapHandle;
use crate::options::{MapInteraction, ProjectionOptions};
use crate::types::{Bounds, LatLng, Point};
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

impl MapHandle {
    pub fn set_center(&self, center: LatLng) {
        self.fire_and_forget(|| {
            crate::interop::set_center_js(&self.map_id, center.lat, center.lng)
        });
    }

    pub fn set_zoom(&self, value: f64) {
        self.fire_and_forget(|| crate::interop::set_zoom_js(&self.map_id, value));
    }

    pub fn set_bearing(&self, value: f64) {
        self.fire_and_forget(|| crate::interop::set_bearing_js(&self.map_id, value));
    }

    pub fn set_roll(&self, value: f64) {
        self.fire_and_forget(|| crate::interop::set_roll_js(&self.map_id, value));
    }

    pub fn set_center_elevation(&self, value: f64) {
        self.fire_and_forget(|| crate::interop::set_center_elevation_js(&self.map_id, value));
    }

    pub fn set_center_clamped_to_ground(&self, enabled: bool) {
        self.fire_and_forget(|| {
            crate::interop::set_center_clamped_to_ground_js(&self.map_id, enabled)
        });
    }

    pub fn set_vertical_field_of_view(&self, value: f64) {
        self.fire_and_forget(|| crate::interop::set_field_of_view_js(&self.map_id, value));
    }

    pub fn set_min_zoom(&self, value: f64) {
        self.fire_and_forget(|| crate::interop::set_min_zoom_js(&self.map_id, value));
    }

    pub fn set_max_zoom(&self, value: f64) {
        self.fire_and_forget(|| crate::interop::set_max_zoom_js(&self.map_id, value));
    }

    pub fn set_min_pitch(&self, value: f64) {
        self.fire_and_forget(|| crate::interop::set_min_pitch_js(&self.map_id, value));
    }

    pub fn set_max_pitch(&self, value: f64) {
        self.fire_and_forget(|| crate::interop::set_max_pitch_js(&self.map_id, value));
    }

    pub fn set_projection(&self, options: ProjectionOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options.0).unwrap_or_else(|_| "{}".into());
            crate::interop::set_projection_js(&self.map_id, &json)
        });
    }

    pub fn set_render_world_copies(&self, enabled: bool) {
        self.fire_and_forget(|| crate::interop::set_render_world_copies_js(&self.map_id, enabled));
    }

    pub fn set_max_bounds(&self, bounds: Option<Bounds>) {
        self.fire_and_forget(|| {
            let json = bounds.map_or_else(
                || "null".into(),
                |value| {
                    serde_json::to_string(&[
                        [value.sw.lng, value.sw.lat],
                        [value.ne.lng, value.ne.lat],
                    ])
                    .unwrap_or_else(|_| "null".into())
                },
            );
            crate::interop::set_max_bounds_js(&self.map_id, &json)
        });
    }

    pub fn set_global_state_property(&self, name: &str, value: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&value).unwrap_or_else(|_| "null".into());
            crate::interop::set_global_state_property_js(&self.map_id, name, &json)
        });
    }

    pub fn set_interaction_enabled(&self, interaction: MapInteraction, enabled: bool) {
        self.fire_and_forget(|| {
            crate::interop::set_interaction_enabled_js(
                &self.map_id,
                interaction.js_property(),
                enabled,
            )
        });
    }

    pub fn resize(&self) {
        self.fire_and_forget(|| crate::interop::resize_js(&self.map_id));
    }

    pub fn stop(&self) {
        self.fire_and_forget(|| crate::interop::stop_js(&self.map_id));
    }

    pub fn reset_north_pitch(&self) {
        self.fire_and_forget(|| crate::interop::reset_north_pitch_js(&self.map_id));
    }

    pub fn trigger_repaint(&self) {
        self.fire_and_forget(|| crate::interop::trigger_repaint_js(&self.map_id));
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_roll(&self) -> Option<f64> {
        document::eval(&crate::interop::get_roll_js(&self.map_id))
            .join::<f64>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_roll(&self) -> Option<f64> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_center_elevation(&self) -> Option<f64> {
        document::eval(&crate::interop::get_center_elevation_js(&self.map_id))
            .join::<f64>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_center_elevation(&self) -> Option<f64> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_vertical_field_of_view(&self) -> Option<f64> {
        document::eval(&crate::interop::get_field_of_view_js(&self.map_id))
            .join::<f64>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_vertical_field_of_view(&self) -> Option<f64> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_min_zoom(&self) -> Option<f64> {
        document::eval(&crate::interop::get_min_zoom_js(&self.map_id))
            .join::<f64>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_min_zoom(&self) -> Option<f64> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_max_zoom(&self) -> Option<f64> {
        document::eval(&crate::interop::get_max_zoom_js(&self.map_id))
            .join::<f64>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_max_zoom(&self) -> Option<f64> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_min_pitch(&self) -> Option<f64> {
        document::eval(&crate::interop::get_min_pitch_js(&self.map_id))
            .join::<f64>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_min_pitch(&self) -> Option<f64> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_max_pitch(&self) -> Option<f64> {
        document::eval(&crate::interop::get_max_pitch_js(&self.map_id))
            .join::<f64>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_max_pitch(&self) -> Option<f64> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_projection(&self) -> Option<serde_json::Value> {
        document::eval(&crate::interop::get_projection_js(&self.map_id))
            .join::<serde_json::Value>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_projection(&self) -> Option<serde_json::Value> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn is_center_clamped_to_ground(&self) -> Option<bool> {
        document::eval(&crate::interop::get_center_clamped_to_ground_js(
            &self.map_id,
        ))
        .join::<bool>()
        .await
        .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn is_center_clamped_to_ground(&self) -> Option<bool> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_render_world_copies(&self) -> Option<bool> {
        document::eval(&crate::interop::get_render_world_copies_js(&self.map_id))
            .join::<bool>()
            .await
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_render_world_copies(&self) -> Option<bool> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn project(&self, coordinate: LatLng) -> Option<Point> {
        document::eval(&crate::interop::project_js(
            &self.map_id,
            coordinate.lat,
            coordinate.lng,
        ))
        .join::<Point>()
        .await
        .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn project(&self, _coordinate: LatLng) -> Option<Point> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn unproject(&self, point: Point) -> Option<LatLng> {
        document::eval(&crate::interop::unproject_js(
            &self.map_id,
            point.x,
            point.y,
        ))
        .join::<LatLng>()
        .await
        .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn unproject(&self, _point: Point) -> Option<LatLng> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn query_terrain_elevation(&self, coordinate: LatLng) -> Option<f64> {
        document::eval(&crate::interop::query_terrain_elevation_js(
            &self.map_id,
            coordinate.lat,
            coordinate.lng,
        ))
        .join::<Option<f64>>()
        .await
        .ok()
        .flatten()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn query_terrain_elevation(&self, _coordinate: LatLng) -> Option<f64> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_global_state_property(&self, name: &str) -> Option<serde_json::Value> {
        document::eval(&crate::interop::get_global_state_property_js(
            &self.map_id,
            name,
        ))
        .join::<serde_json::Value>()
        .await
        .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_global_state_property(&self, _name: &str) -> Option<serde_json::Value> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn is_interaction_enabled(&self, interaction: MapInteraction) -> Option<bool> {
        document::eval(&crate::interop::is_interaction_enabled_js(
            &self.map_id,
            interaction.js_property(),
        ))
        .join::<bool>()
        .await
        .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn is_interaction_enabled(&self, _interaction: MapInteraction) -> Option<bool> {
        None
    }
}
