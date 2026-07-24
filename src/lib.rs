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

mod events;
mod handle;
mod interop;
mod options;
mod types;

/// Map components
pub mod components;

// Re-export public API — Types
pub use types::{Bounds, LatLng, MapPosition, Point, QueryFeature};

// Re-export public API — Events
pub use events::{
    LayerClickEvent, LayerHoverEvent, LayerPressEvent, MapClickEvent, MapContextMenuEvent,
    MapDblClickEvent, MapErrorEvent, MapEvent, MapLifecycleEvent, MapMoveEvent, MapPitchEvent,
    MapReadyEvent, MapRollEvent, MapRotateEvent, MapZoomEvent, MarkerClickEvent,
    MarkerDragEndEvent, MarkerDragStartEvent, MarkerHoverEvent,
};

// Re-export public API — Options
pub use options::{
    CanvasSourceOptions, ControlOptions, ControlPosition, EaseToOptions, FeatureId,
    FeatureIdentifier, FitBoundsOptions, FlyToOptions, FogOptions, GeoJsonSourceOptions,
    ImageSourceOptions, JumpToOptions, LayerOptions, MapInteraction, MapOptions, MarkerOptions,
    MissingImageOptions, Padding, PopupOptions, ProjectionOptions, QueryOptions,
    RasterDemSourceOptions, RasterSourceOptions, SkyOptions, SourceOptions, TerrainControlOptions,
    TerrainOptions, VectorSourceOptions, VideoSourceOptions,
};

// Re-export public API — Handle & Component
pub use components::{
    Map, MapControl, MapControlKind, MapLayer, MapMarker, MapPopup, MapSource, MapSourceKind,
    use_map_handle,
};
pub use handle::MapHandle;
