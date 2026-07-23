//! Option types for MapLibre GL JS operations.

mod atmosphere;
mod controls;
mod images;
mod layers;
mod map;
mod navigation;
mod overlays;
mod queries;
mod sources;

pub use atmosphere::{FogOptions, SkyOptions, TerrainOptions};
pub use controls::{ControlOptions, ControlPosition, Padding, TerrainControlOptions};
pub use images::MissingImageOptions;
pub use layers::LayerOptions;
pub use map::{MapInteraction, MapOptions, ProjectionOptions};
pub use navigation::{EaseToOptions, FitBoundsOptions, FlyToOptions, JumpToOptions};
pub use overlays::{MarkerOptions, PopupOptions};
pub use queries::{FeatureId, FeatureIdentifier, QueryOptions};
pub use sources::{
    CanvasSourceOptions, GeoJsonSourceOptions, ImageSourceOptions, RasterDemSourceOptions,
    RasterSourceOptions, SourceOptions, VectorSourceOptions, VideoSourceOptions,
};
