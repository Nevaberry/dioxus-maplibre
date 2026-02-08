//! Shared map-handle context and hook helpers.

use dioxus::prelude::{Signal, try_use_context};

use crate::handle::MapHandle;

pub(crate) type MapHandleSignal = Signal<Option<MapHandle>>;

pub(crate) fn try_use_map_handle_signal() -> Option<MapHandleSignal> {
    try_use_context::<MapHandleSignal>()
}

/// Access the nearest `Map` handle from context.
///
/// Returns `None` when called outside a `Map` subtree or before map initialization.
pub fn use_map_handle() -> Option<MapHandle> {
    try_use_map_handle_signal().and_then(|signal| signal())
}
