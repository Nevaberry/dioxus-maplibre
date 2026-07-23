//! Feature-state MapHandle methods.
#![allow(clippy::needless_pass_by_value, clippy::unused_async)]

use super::MapHandle;
use crate::options::FeatureIdentifier;
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

impl MapHandle {
    /// Set feature state for styling (hover effects, selection, etc.)
    pub fn set_feature_state(&self, feature: &FeatureIdentifier, state: serde_json::Value) {
        self.fire_and_forget(|| {
            let json = serde_json::to_string(&state).unwrap_or_default();
            let feature_id = serde_json::to_string(&feature.id).unwrap_or_else(|_| "null".into());
            crate::interop::set_feature_state_js(
                &self.map_id,
                &feature.source,
                &feature_id,
                feature.source_layer.as_deref(),
                &json,
            )
        });
    }

    /// Remove all feature state
    pub fn remove_feature_state(&self, feature: &FeatureIdentifier) {
        self.fire_and_forget(|| {
            let feature_id = serde_json::to_string(&feature.id).unwrap_or_else(|_| "null".into());
            crate::interop::remove_feature_state_js(
                &self.map_id,
                &feature.source,
                &feature_id,
                feature.source_layer.as_deref(),
            )
        });
    }

    /// Remove one named feature-state property.
    pub fn remove_feature_state_property(&self, feature: &FeatureIdentifier, property: &str) {
        self.fire_and_forget(|| {
            let feature_id = serde_json::to_string(&feature.id).unwrap_or_else(|_| "null".into());
            crate::interop::remove_feature_state_property_js(
                &self.map_id,
                &feature.source,
                &feature_id,
                feature.source_layer.as_deref(),
                property,
            )
        });
    }

    /// Read all state for a feature.
    #[cfg(target_arch = "wasm32")]
    pub async fn get_feature_state(
        &self,
        feature: &FeatureIdentifier,
    ) -> Option<serde_json::Value> {
        let feature_id = serde_json::to_string(&feature.id).ok()?;
        let js = crate::interop::get_feature_state_js(
            &self.map_id,
            &feature.source,
            &feature_id,
            feature.source_layer.as_deref(),
        );
        document::eval(&js).join::<serde_json::Value>().await.ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_feature_state(
        &self,
        _feature: &FeatureIdentifier,
    ) -> Option<serde_json::Value> {
        None
    }
}
