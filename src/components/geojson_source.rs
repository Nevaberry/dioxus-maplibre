//! GeoJSON Source component for MapLibre layers

use dioxus::prelude::*;

use crate::context::MapContext;

/// Props for the GeoJsonSource component
#[derive(Props, Clone, PartialEq)]
pub struct GeoJsonSourceProps {
    /// Unique source ID
    pub id: String,

    /// GeoJSON data as a string (FeatureCollection)
    pub data: String,

    /// Child layer components (CircleLayer, SymbolLayer, etc.)
    pub children: Element,
}

/// A GeoJSON data source that can feed one or more layers
///
/// # Example
/// ```rust,ignore
/// GeoJsonSource {
///     id: "cameras",
///     data: geojson_string,
///     CircleLayer {
///         id: "cameras-circles",
///         source: "cameras",
///     }
/// }
/// ```
#[component]
pub fn GeoJsonSource(props: GeoJsonSourceProps) -> Element {
    // Get map context from parent
    #[allow(unused_variables)] // only used on wasm32
    let ctx = use_context::<MapContext>();

    // Track whether source has been added
    #[allow(unused_variables, unused_mut)] // only used on wasm32
    let mut source_added = use_signal(|| false);

    // Track previous data for change detection
    #[allow(unused_variables, unused_mut)] // only used on wasm32
    let mut prev_data = use_signal(String::new);

    // Only do JS interop on wasm targets
    #[cfg(target_arch = "wasm32")]
    {
        use crate::interop::{add_geojson_source_js, update_geojson_source_js, remove_source_js};
        use tracing::debug;

        let source_id = props.id.clone();
        let data = props.data.clone();

        // Add source when map is ready
        {
            let map_id = ctx.map_id.clone();
            let source_id = source_id.clone();
            let data = data.clone();
            let is_ready = ctx.is_ready;

            use_effect(move || {
                let map_id = map_id.clone();
                let source_id = source_id.clone();
                let data = data.clone();

                if is_ready() && !source_added() {
                    debug!("Adding GeoJSON source: {}", source_id);
                    source_added.set(true);
                    prev_data.set(data.clone());

                    let js = add_geojson_source_js(&map_id, &source_id, &data);
                    spawn(async move {
                        let _ = document::eval(&js).await;
                    });
                }
            });
        }

        // Update source when data changes
        {
            let map_id = ctx.map_id.clone();
            let source_id = source_id.clone();
            let data = data.clone();
            let is_ready = ctx.is_ready;

            use_effect(move || {
                let map_id = map_id.clone();
                let source_id = source_id.clone();
                let data = data.clone();

                if is_ready() && source_added() && prev_data() != data {
                    debug!("Updating GeoJSON source: {}", source_id);
                    prev_data.set(data.clone());

                    let js = update_geojson_source_js(&map_id, &source_id, &data);
                    spawn(async move {
                        let _ = document::eval(&js).await;
                    });
                }
            });
        }

        // Remove source on unmount
        {
            let map_id = ctx.map_id.clone();
            let source_id = source_id.clone();

            use_drop(move || {
                debug!("Removing GeoJSON source: {}", source_id);
                let js = remove_source_js(&map_id, &source_id);
                spawn(async move {
                    let _ = document::eval(&js).await;
                });
            });
        }
    }

    // Render children (layer components)
    rsx! { {props.children} }
}
