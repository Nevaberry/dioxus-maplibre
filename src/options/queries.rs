//! Query and feature-state option models.

use serde::{Deserialize, Serialize};
/// Options for querying rendered or source features
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryOptions {
    /// Restrict query to specific layer IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,

    /// Filter expression to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

/// Identifies a feature for feature state operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureIdentifier {
    /// Source ID
    pub source: String,

    /// Feature ID (must be numeric for MapLibre feature state)
    pub id: i64,

    /// Source layer (required for vector tile sources)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer: Option<String>,
}
