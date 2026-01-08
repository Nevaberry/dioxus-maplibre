//! Marker component

use dioxus::prelude::*;

use crate::context::MapContext;
use crate::types::LatLng;

/// Props for the Marker component
#[derive(Props, Clone, PartialEq)]
pub struct MarkerProps {
    /// Geographic position of the marker
    pub position: LatLng,

    /// Unique marker ID (auto-generated if not provided)
    #[props(optional)]
    pub id: Option<String>,

    /// Emoji to display instead of default pin (e.g., "📷")
    #[props(optional)]
    pub emoji: Option<String>,

    /// Popup content (HTML string)
    #[props(optional)]
    pub popup: Option<String>,

    /// Child Popup component
    #[props(optional)]
    pub children: Element,
}

/// A marker on the map
#[component]
pub fn Marker(props: MarkerProps) -> Element {
    // Get map context from parent
    let ctx = use_context::<MapContext>();

    // Generate marker ID if not provided
    let marker_id = use_hook(|| {
        props.id.clone().unwrap_or_else(|| {
            format!("marker_{}", uuid::Uuid::new_v4().to_string().replace('-', ""))
        })
    });

    // Only do JS interop on wasm targets
    #[cfg(target_arch = "wasm32")]
    {
        use crate::interop::{add_marker_js, remove_marker_js, update_marker_position_js};
        use tracing::debug;

        let position = props.position;
        let popup = props.popup.clone();
        let emoji = props.emoji.clone();

        // Add marker when map is ready
        {
            let map_id = ctx.map_id.clone();
            let marker_id = marker_id.clone();
            let is_ready = ctx.is_ready;

            use_effect(move || {
                // Clone for async block
                let map_id = map_id.clone();
                let marker_id = marker_id.clone();
                let popup = popup.clone();
                let emoji = emoji.clone();

                if is_ready() {
                    debug!("Adding marker: {}", marker_id);
                    let js = add_marker_js(
                        &map_id,
                        &marker_id,
                        position.lat,
                        position.lng,
                        popup.as_deref(),
                        emoji.as_deref(),
                    );
                    spawn(async move {
                        let _ = document::eval(&js).await;
                    });
                }
            });
        }

        // Update position when it changes
        {
            let map_id = ctx.map_id.clone();
            let marker_id = marker_id.clone();
            let is_ready = ctx.is_ready;
            let mut prev_position = use_signal(|| position);

            use_effect(move || {
                let map_id = map_id.clone();
                let marker_id = marker_id.clone();

                if is_ready() && prev_position() != position {
                    debug!("Updating marker position: {}", marker_id);
                    let js = update_marker_position_js(
                        &map_id,
                        &marker_id,
                        position.lat,
                        position.lng,
                    );
                    spawn(async move {
                        let _ = document::eval(&js).await;
                    });
                    prev_position.set(position);
                }
            });
        }

        // Remove marker on unmount
        {
            let map_id = ctx.map_id.clone();
            let marker_id = marker_id.clone();

            use_drop(move || {
                debug!("Removing marker: {}", marker_id);
                let js = remove_marker_js(&map_id, &marker_id);
                spawn(async move {
                    let _ = document::eval(&js).await;
                });
            });
        }
    }

    // Marker doesn't render DOM - it's purely JS-side
    // Children (Popup) are handled via props
    rsx! {}
}
