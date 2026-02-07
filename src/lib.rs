//! # dioxus-maplibre
//!
//! A MapLibre GL JS wrapper for Dioxus 0.7+
//!
//! This crate provides a `Map` component and `MapHandle` API for integrating
//! MapLibre GL JS maps into your Dioxus applications.
//!
//! ## Example
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_maplibre::{Map, MapHandle, LatLng, FlyToOptions};
//!
//! fn App() -> Element {
//!     let mut map = use_signal(|| None::<MapHandle>);
//!
//!     rsx! {
//!         Map {
//!             style: "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json",
//!             center: LatLng::new(60.17, 24.94),
//!             zoom: 12.0,
//!             on_ready: move |handle: MapHandle| {
//!                 map.set(Some(handle));
//!             },
//!         }
//!     }
//! }
//! ```

mod types;
mod events;
mod options;
mod handle;
mod interop;

/// Map components
pub mod components;

// Re-export public API — Types
pub use types::{LatLng, MapPosition, Bounds, Point};

// Re-export public API — Events
pub use events::{
    MapClickEvent, MapDblClickEvent, MapContextMenuEvent,
    MarkerClickEvent, MarkerHoverEvent,
    MapMoveEvent, MapZoomEvent, MapRotateEvent, MapPitchEvent,
    LayerClickEvent, LayerHoverEvent,
};

// Re-export public API — Options
pub use options::{
    ControlPosition, Padding,
    GeoJsonSourceOptions, VectorSourceOptions, RasterSourceOptions,
    RasterDemSourceOptions, ImageSourceOptions,
    LayerOptions, MarkerOptions, PopupOptions,
    FlyToOptions, EaseToOptions, JumpToOptions, FitBoundsOptions,
    TerrainOptions, SkyOptions, FeatureIdentifier,
};

// Re-export public API — Handle & Component
pub use handle::MapHandle;
pub use components::Map;
