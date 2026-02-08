//! Feature-state MapHandle methods.

use super::MapHandle;
use crate::options::FeatureIdentifier;

impl MapHandle {
    /// Set feature state for styling (hover effects, selection, etc.)
    pub fn set_feature_state(&self, feature: &FeatureIdentifier, state: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&state).unwrap_or_default();
            crate::interop::set_feature_state_js(
                &self.map_id,
                &feature.source,
                feature.id,
                feature.source_layer.as_deref(),
                &json,
            )
        });
    }

    /// Remove all feature state
    pub fn remove_feature_state(&self, feature: &FeatureIdentifier) {
        self.fire_and_forget(|| {
            crate::interop::remove_feature_state_js(
                &self.map_id,
                &feature.source,
                feature.id,
                feature.source_layer.as_deref(),
            )
        });
    }
}
