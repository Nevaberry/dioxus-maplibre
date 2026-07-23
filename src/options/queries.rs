//! Query and feature-state option models.

use serde::{Deserialize, Serialize};

/// A MapLibre feature ID, which may be numeric or textual.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FeatureId {
    Number(i64),
    String(String),
}

impl From<i64> for FeatureId {
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for FeatureId {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for FeatureId {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
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

    /// Images available while evaluating filters and expressions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_images: Option<Vec<String>>,

    /// Validate the filter before querying.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,

    /// Vector source layer used by `query_source_features`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer: Option<String>,
}

/// Identifies a feature for feature state operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureIdentifier {
    /// Source ID
    pub source: String,

    /// Numeric or string feature ID.
    pub id: FeatureId,

    /// Source layer (required for vector tile sources)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer: Option<String>,
}

impl FeatureIdentifier {
    pub fn new(source: impl Into<String>, id: impl Into<FeatureId>) -> Self {
        Self {
            source: source.into(),
            id: id.into(),
            source_layer: None,
        }
    }

    #[must_use]
    pub fn source_layer(mut self, source_layer: impl Into<String>) -> Self {
        self.source_layer = Some(source_layer.into());
        self
    }
}
