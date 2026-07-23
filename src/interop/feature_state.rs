//! Feature-state JS bridge.

use super::find_map_js;
use super::js_escape::js_single_quoted;

pub fn set_feature_state_js(
    map_id: &str,
    source: &str,
    feature_id_json: &str,
    source_layer: Option<&str>,
    state_json: &str,
) -> String {
    let find = find_map_js(map_id);
    let source_lit = js_single_quoted(source);
    let source_layer_prop = source_layer
        .map(|layer| format!(", sourceLayer: {}", js_single_quoted(layer)))
        .unwrap_or_default();
    format!(
        r#"
        (function() {{
            {find}
            try {{
                map.setFeatureState(
                    {{ source: {source_lit}, id: {feature_id_json}{source_layer_prop} }},
                    {state_json}
                );
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to set feature state:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to remove feature state
pub fn remove_feature_state_js(
    map_id: &str,
    source: &str,
    feature_id_json: &str,
    source_layer: Option<&str>,
) -> String {
    let find = find_map_js(map_id);
    let source_lit = js_single_quoted(source);
    let source_layer_prop = source_layer
        .map(|layer| format!(", sourceLayer: {}", js_single_quoted(layer)))
        .unwrap_or_default();
    format!(
        r#"
        (function() {{
            {find}
            try {{
                map.removeFeatureState(
                    {{ source: {source_lit}, id: {feature_id_json}{source_layer_prop} }}
                );
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to remove feature state:', err);
            }}
        }})();
        "#
    )
}

pub fn remove_feature_state_property_js(
    map_id: &str,
    source: &str,
    feature_id_json: &str,
    source_layer: Option<&str>,
    property: &str,
) -> String {
    let find = find_map_js(map_id);
    let source_lit = js_single_quoted(source);
    let property_lit = js_single_quoted(property);
    let source_layer_prop = source_layer
        .map(|layer| format!(", sourceLayer: {}", js_single_quoted(layer)))
        .unwrap_or_default();
    format!(
        r#"
        (function() {{
            {find}
            map.removeFeatureState(
                {{ source: {source_lit}, id: {feature_id_json}{source_layer_prop} }},
                {property_lit}
            );
        }})();
        "#
    )
}

pub fn get_feature_state_js(
    map_id: &str,
    source: &str,
    feature_id_json: &str,
    source_layer: Option<&str>,
) -> String {
    let find = find_map_js(map_id);
    let source_lit = js_single_quoted(source);
    let source_layer_prop = source_layer
        .map(|layer| format!(", sourceLayer: {}", js_single_quoted(layer)))
        .unwrap_or_default();
    format!(
        "{find} return map.getFeatureState({{ source: {source_lit}, id: {feature_id_json}{source_layer_prop} }});"
    )
}
