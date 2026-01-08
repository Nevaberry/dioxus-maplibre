//! Popup component
//!
//! For MVP, popups are created declaratively via the Marker's popup prop.
//! This component exists for future expansion and API consistency.

use dioxus::prelude::*;

/// A popup attached to a marker
///
/// Currently, this is a declarative placeholder. The actual popup is created
/// by the Marker component using the `popup` prop. This component is here
/// for API consistency and future expansion.
#[component]
pub fn Popup(
    /// HTML content to display in the popup
    content: String,
) -> Element {
    // Popups are currently created via Marker's JS code
    // This component exists for API consistency
    rsx! {}
}
