//! Circle Layer component for MapLibre

use dioxus::prelude::*;

use crate::context::MapContext;

/// Props for the CircleLayer component
#[derive(Props, Clone, PartialEq)]
pub struct CircleLayerProps {
    /// Unique layer ID
    pub id: String,

    /// Source ID to get data from
    pub source: String,

    /// Paint properties (circle-radius, circle-color, circle-stroke-width, etc.)
    /// See: https://maplibre.org/maplibre-style-spec/layers/#circle
    #[props(default)]
    pub paint: Option<serde_json::Value>,

    /// Layout properties (visibility, etc.)
    #[props(default)]
    pub layout: Option<serde_json::Value>,
}

/// A circle layer that renders points from a GeoJSON source as circles
///
/// # Example
/// ```rust,ignore
/// CircleLayer {
///     id: "cameras-circles",
///     source: "cameras",
///     paint: serde_json::json!({
///         "circle-radius": 6,
///         "circle-color": "#3b82f6",
///         "circle-stroke-width": 2,
///         "circle-stroke-color": "#000000"
///     }),
/// }
/// ```
#[component]
pub fn CircleLayer(props: CircleLayerProps) -> Element {
    // Get map context from parent
    let ctx = use_context::<MapContext>();

    // Track whether layer has been added
    let mut layer_added = use_signal(|| false);

    // Only do JS interop on wasm targets
    #[cfg(target_arch = "wasm32")]
    {
        use crate::interop::{add_layer_js, remove_layer_js};
        use tracing::debug;

        let layer_id = props.id.clone();
        let source_id = props.source.clone();

        // Default paint properties for circles
        let paint = props.paint.clone().unwrap_or(serde_json::json!({
            "circle-radius": 6,
            "circle-color": "#3b82f6",
            "circle-stroke-width": 2,
            "circle-stroke-color": "#000000"
        }));

        let layout = props.layout.clone().unwrap_or(serde_json::json!({}));

        // Add layer when map is ready
        {
            let map_id = ctx.map_id.clone();
            let layer_id = layer_id.clone();
            let source_id = source_id.clone();
            let paint = paint.clone();
            let layout = layout.clone();
            let is_ready = ctx.is_ready;

            use_effect(move || {
                let map_id = map_id.clone();
                let layer_id = layer_id.clone();
                let source_id = source_id.clone();
                let paint = paint.clone();
                let layout = layout.clone();

                if is_ready() && !layer_added() {
                    debug!("Adding circle layer: {}", layer_id);
                    layer_added.set(true);

                    let js = add_layer_js(
                        &map_id,
                        &layer_id,
                        "circle",
                        &source_id,
                        &paint.to_string(),
                        &layout.to_string(),
                    );
                    spawn(async move {
                        let _ = document::eval(&js).await;
                    });
                }
            });
        }

        // Remove layer on unmount
        {
            let map_id = ctx.map_id.clone();
            let layer_id = layer_id.clone();

            use_drop(move || {
                debug!("Removing circle layer: {}", layer_id);
                let js = remove_layer_js(&map_id, &layer_id);
                spawn(async move {
                    let _ = document::eval(&js).await;
                });
            });
        }
    }

    // Layer doesn't render DOM - it's purely JS-side
    rsx! {}
}
