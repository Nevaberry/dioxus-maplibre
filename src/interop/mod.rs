//! JavaScript interop module for MapLibre GL JS.
// These functions generate JS strings and are only called on wasm32 targets.
// On other targets they appear unused but we keep them available for tests.
#![allow(dead_code, unused_imports)]

mod controls;
mod core;
mod feature_state;
mod getters;
mod images;
mod js_escape;
mod layers;
mod lifecycle;
mod markers;
mod navigation;
mod padding;
mod popups;
mod queries;
mod sources;
mod style;
mod terrain_atmosphere;

pub(crate) use core::find_map_js;
pub use core::generate_map_id;

pub use controls::*;
pub use feature_state::*;
pub use getters::*;
pub use images::*;
pub use layers::*;
pub use lifecycle::*;
pub use markers::*;
pub use navigation::*;
pub use padding::*;
pub use popups::*;
pub use queries::*;
pub use sources::*;
pub use style::*;
pub use terrain_atmosphere::*;
