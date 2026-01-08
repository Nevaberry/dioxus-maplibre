//! # dioxus-maplibre
//!
//! A MapLibre GL JS wrapper for Dioxus 0.7+
//!
//! This crate provides Dioxus components for integrating MapLibre GL JS maps
//! into your Dioxus applications.
//!
//! ## Example
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_maplibre::{Map, Marker, Popup, LatLng};
//!
//! fn App() -> Element {
//!     rsx! {
//!         Map {
//!             style: "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json",
//!             center: LatLng::new(60.17, 24.94),
//!             zoom: 10.0,
//!
//!             Marker {
//!                 position: LatLng::new(60.17, 24.94),
//!                 Popup { content: "Hello Helsinki!" }
//!             }
//!         }
//!     }
//! }
//! ```

mod types;
mod events;
mod context;
mod interop;

/// Map components (Map, Marker, Popup)
pub mod components;

// Re-export public API
pub use types::{LatLng, MapPosition, Bounds, Point};
pub use events::{MapClickEvent, MarkerClickEvent, MapMoveEvent};
pub use context::MapContext;
pub use components::{Map, Marker, Popup};
pub use components::map::{MarkerHoverEvent, fly_to, pan_by};
