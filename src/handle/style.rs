//! Style-related MapHandle methods.

use super::MapHandle;

impl MapHandle {
    /// Change the map's style URL
    pub fn set_style(&self, url: &str) {
        self.fire_and_forget(|| crate::interop::set_style_js(&self.map_id, url));
    }
}
