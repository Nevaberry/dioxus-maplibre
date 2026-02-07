//! MapHandle — the primary API for interacting with a MapLibre map
//!
//! `MapHandle` is a lightweight, cloneable wrapper around a map ID string.
//! All map operations are methods on this handle.
//!
//! # Examples
//!
//! ```rust,ignore
//! use dioxus_maplibre::{Map, MapHandle, FlyToOptions, LatLng};
//!
//! fn App() -> Element {
//!     let mut map_handle = use_signal(|| None::<MapHandle>);
//!
//!     rsx! {
//!         Map {
//!             on_ready: move |handle: MapHandle| {
//!                 map_handle.set(Some(handle));
//!             },
//!         }
//!         if let Some(map) = map_handle() {
//!             button {
//!                 onclick: move |_| {
//!                     map.fly_to(FlyToOptions {
//!                         center: Some(LatLng::new(60.17, 24.94)),
//!                         zoom: Some(12.0),
//!                         ..Default::default()
//!                     });
//!                 },
//!                 "Fly to Helsinki"
//!             }
//!         }
//!     }
//! }
//! ```

use crate::options::{
    ControlPosition, EaseToOptions, FeatureIdentifier, FitBoundsOptions, FlyToOptions,
    GeoJsonSourceOptions, ImageSourceOptions, JumpToOptions, LayerOptions, MarkerOptions,
    PopupOptions, RasterDemSourceOptions, RasterSourceOptions, SkyOptions, TerrainOptions,
    VectorSourceOptions,
};
use crate::types::{Bounds, LatLng};

/// A handle to a MapLibre map instance
///
/// This is a lightweight `Clone` wrapper. Store it in a `Signal<Option<MapHandle>>`
/// and set it in the `on_ready` callback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapHandle {
    map_id: String,
}

impl MapHandle {
    /// Create a new MapHandle (called internally by the Map component)
    #[allow(dead_code)] // Used only on wasm32 target
    pub(crate) fn new(map_id: String) -> Self {
        Self { map_id }
    }

    /// Get the internal map ID (useful for debugging)
    pub fn map_id(&self) -> &str {
        &self.map_id
    }

    // =========================================================================
    // Sources
    // =========================================================================

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

    // =========================================================================
    // Layers
    // =========================================================================

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

    /// Set a filter on a layer
    pub fn set_filter(&self, layer_id: &str, filter: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&filter).unwrap_or_default();
            crate::interop::set_filter_js(&self.map_id, layer_id, &json)
        });
    }

    // =========================================================================
    // Controls
    // =========================================================================

    /// Add a navigation control (zoom +/- buttons and compass)
    pub fn add_navigation_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_navigation_control_js(&self.map_id, pos)
        });
    }

    /// Add a geolocate control
    pub fn add_geolocate_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_geolocate_control_js(&self.map_id, pos)
        });
    }

    /// Add a scale control
    pub fn add_scale_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_scale_control_js(&self.map_id, pos)
        });
    }

    /// Add a fullscreen control
    pub fn add_fullscreen_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_fullscreen_control_js(&self.map_id, pos)
        });
    }

    /// Add an attribution control
    pub fn add_attribution_control(&self, position: ControlPosition) {
        self.fire_and_forget(|| {
            let pos = control_position_str(position);
            crate::interop::add_attribution_control_js(&self.map_id, pos)
        });
    }

    // =========================================================================
    // Markers
    // =========================================================================

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

    // =========================================================================
    // Popups
    // =========================================================================

    /// Add a standalone popup at a position
    pub fn add_popup(&self, id: &str, position: LatLng, html: &str, options: PopupOptions) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&options).unwrap_or_default();
            crate::interop::add_popup_js(
                &self.map_id,
                id,
                position.lat,
                position.lng,
                html,
                &json,
            )
        });
    }

    /// Remove a popup
    pub fn remove_popup(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::remove_popup_js(&self.map_id, id));
    }

    // =========================================================================
    // Navigation
    // =========================================================================

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

    // =========================================================================
    // Feature State
    // =========================================================================

    /// Set feature state for styling (hover effects, selection, etc.)
    pub fn set_feature_state(&self, feature: &FeatureIdentifier, state: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&state).unwrap_or_default();
            crate::interop::set_feature_state_js(
                &self.map_id,
                &feature.source,
                feature.id,
                feature.source_layer.as_deref(),
                &json,
            )
        });
    }

    /// Remove all feature state
    pub fn remove_feature_state(&self, feature: &FeatureIdentifier) {
        self.fire_and_forget(|| {
            crate::interop::remove_feature_state_js(
                &self.map_id,
                &feature.source,
                feature.id,
                feature.source_layer.as_deref(),
            )
        });
    }

    // =========================================================================
    // Images
    // =========================================================================

    /// Load an image from a URL and add it to the map's sprite
    pub fn load_image(&self, id: &str, url: &str) {
        self.fire_and_forget(|| crate::interop::load_image_js(&self.map_id, id, url));
    }

    /// Remove an image from the map's sprite
    pub fn remove_image(&self, id: &str) {
        self.fire_and_forget(|| crate::interop::remove_image_js(&self.map_id, id));
    }

    // =========================================================================
    // Style
    // =========================================================================

    /// Change the map's style URL
    pub fn set_style(&self, url: &str) {
        self.fire_and_forget(|| crate::interop::set_style_js(&self.map_id, url));
    }

    // =========================================================================
    // Terrain & Sky
    // =========================================================================

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

    // =========================================================================
    // Layer Events
    // =========================================================================

    /// Register a click handler on a layer (events dispatched via on_layer_click)
    pub fn on_layer_click(&self, layer_id: &str) {
        self.fire_and_forget(|| crate::interop::register_layer_click_js(&self.map_id, layer_id));
    }

    /// Register hover handlers on a layer (events dispatched via on_layer_hover)
    pub fn on_layer_hover(&self, layer_id: &str) {
        self.fire_and_forget(|| crate::interop::register_layer_hover_js(&self.map_id, layer_id));
    }

    // =========================================================================
    // Async Getters
    // =========================================================================

    /// Get the current zoom level
    #[cfg(target_arch = "wasm32")]
    pub async fn get_zoom(&self) -> Option<f64> {
        let js = crate::interop::get_zoom_js(&self.map_id);
        document::eval(&js).join::<f64>().await.ok()
    }

    /// Get the current center coordinate
    #[cfg(target_arch = "wasm32")]
    pub async fn get_center(&self) -> Option<LatLng> {
        let js = crate::interop::get_center_js(&self.map_id);
        document::eval(&js).join::<LatLng>().await.ok()
    }

    /// Get the current bearing (rotation)
    #[cfg(target_arch = "wasm32")]
    pub async fn get_bearing(&self) -> Option<f64> {
        let js = crate::interop::get_bearing_js(&self.map_id);
        document::eval(&js).join::<f64>().await.ok()
    }

    /// Get the current pitch (tilt)
    #[cfg(target_arch = "wasm32")]
    pub async fn get_pitch(&self) -> Option<f64> {
        let js = crate::interop::get_pitch_js(&self.map_id);
        document::eval(&js).join::<f64>().await.ok()
    }

    /// Get the current viewport bounds
    #[cfg(target_arch = "wasm32")]
    pub async fn get_bounds(&self) -> Option<Bounds> {
        let js = crate::interop::get_bounds_js(&self.map_id);
        document::eval(&js).join::<Bounds>().await.ok()
    }

    // No-op stubs for native targets
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_zoom(&self) -> Option<f64> {
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_center(&self) -> Option<LatLng> {
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_bearing(&self) -> Option<f64> {
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_pitch(&self) -> Option<f64> {
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_bounds(&self) -> Option<Bounds> {
        None
    }

    // =========================================================================
    // Escape Hatch
    // =========================================================================

    /// Execute arbitrary JavaScript against this map's instance
    ///
    /// The JS code receives `map` as a variable referencing the MapLibre map object.
    /// This is a fire-and-forget operation.
    pub fn eval(&self, js_code: &str) {
        let find = format!(
            r#"let map = window.__dioxus_maplibre_maps['{}']; if (!map) return;"#,
            self.map_id
        );
        let full_js = format!("(function() {{ {find} {js_code} }})();");
        self.eval_raw(&full_js);
    }

    /// Execute arbitrary JavaScript and return a deserialized result
    ///
    /// The JS code should return a value. It receives `map` as a variable.
    #[cfg(target_arch = "wasm32")]
    pub async fn eval_async<T: serde::de::DeserializeOwned>(&self, js_code: &str) -> Option<T> {
        let find = format!(
            r#"let map = window.__dioxus_maplibre_maps['{}']; if (!map) return null;"#,
            self.map_id
        );
        let full_js = format!("(function() {{ {find} {js_code} }})();");
        document::eval(&full_js).join::<T>().await.ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn eval_async<T: serde::de::DeserializeOwned>(&self, _js_code: &str) -> Option<T> {
        None
    }

    // =========================================================================
    // Internal
    // =========================================================================

    /// Fire-and-forget: spawn an async eval that we don't wait for
    #[cfg(target_arch = "wasm32")]
    fn fire_and_forget(&self, js_fn: impl FnOnce() -> String) {
        let js = js_fn();
        dioxus::prelude::spawn(async move {
            let _ = document::eval(&js).await;
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn fire_and_forget(&self, _js_fn: impl FnOnce() -> String) {
        // No-op on non-wasm targets
    }

    /// Execute raw JS without wrapping (for escape hatch)
    #[cfg(target_arch = "wasm32")]
    fn eval_raw(&self, js: &str) {
        let js = js.to_string();
        dioxus::prelude::spawn(async move {
            let _ = document::eval(&js).await;
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn eval_raw(&self, _js: &str) {}
}

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

fn control_position_str(pos: ControlPosition) -> &'static str {
    match pos {
        ControlPosition::TopLeft => "top-left",
        ControlPosition::TopRight => "top-right",
        ControlPosition::BottomLeft => "bottom-left",
        ControlPosition::BottomRight => "bottom-right",
    }
}
