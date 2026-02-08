//! Feature query MapHandle methods.

use super::MapHandle;
use crate::options::QueryOptions;
use crate::types::QueryFeature;
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::document;

impl MapHandle {
    /// Query rendered features in the entire viewport
    #[cfg(target_arch = "wasm32")]
    pub async fn query_rendered_features(&self, options: QueryOptions) -> Vec<QueryFeature> {
        let json = serde_json::to_string(&options).unwrap_or_default();
        let js = crate::interop::query_rendered_features_js(&self.map_id, &json);
        document::eval(&js)
            .join::<Vec<QueryFeature>>()
            .await
            .unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn query_rendered_features(&self, _options: QueryOptions) -> Vec<QueryFeature> {
        Vec::new()
    }

    /// Query rendered features at a screen point
    #[cfg(target_arch = "wasm32")]
    pub async fn query_rendered_features_at(
        &self,
        point: crate::types::Point,
        options: QueryOptions,
    ) -> Vec<QueryFeature> {
        let json = serde_json::to_string(&options).unwrap_or_default();
        let js =
            crate::interop::query_rendered_features_at_js(&self.map_id, point.x, point.y, &json);
        document::eval(&js)
            .join::<Vec<QueryFeature>>()
            .await
            .unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn query_rendered_features_at(
        &self,
        _point: crate::types::Point,
        _options: QueryOptions,
    ) -> Vec<QueryFeature> {
        Vec::new()
    }

    /// Query all features in a source
    #[cfg(target_arch = "wasm32")]
    pub async fn query_source_features(
        &self,
        source_id: &str,
        options: QueryOptions,
    ) -> Vec<QueryFeature> {
        let json = serde_json::to_string(&options).unwrap_or_default();
        let js = crate::interop::query_source_features_js(&self.map_id, source_id, &json);
        document::eval(&js)
            .join::<Vec<QueryFeature>>()
            .await
            .unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn query_source_features(
        &self,
        _source_id: &str,
        _options: QueryOptions,
    ) -> Vec<QueryFeature> {
        Vec::new()
    }
}
