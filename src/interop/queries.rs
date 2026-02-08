//! Feature query JS bridge.

use super::find_map_js;
use super::js_escape::js_single_quoted;

pub fn query_rendered_features_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const hasOptions = !!(opts && Object.keys(opts).length > 0);
                const requestedLayers = opts && Array.isArray(opts.layers) ? opts.layers : [];

                let features = hasOptions
                    ? map.queryRenderedFeatures(undefined, opts)
                    : map.queryRenderedFeatures();

                if (hasOptions && requestedLayers.length > 0 && features.length === 0) {{
                    const layerSet = new Set(requestedLayers);
                    features = map
                        .queryRenderedFeatures()
                        .filter(f => f && f.layer && layerSet.has(f.layer.id));
                }}

                return features.map(f => ({{
                    id: Number.isFinite(f.id) ? Math.trunc(f.id) : null,
                    geometry: f.geometry,
                    properties: f.properties || {{}},
                    source: f.source,
                    sourceLayer: f.sourceLayer || null
                }}));
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to query rendered features:', err);
                return [];
            }}
        }})();
        "#
    )
}

/// Generate JS to query rendered features at a screen point
pub fn query_rendered_features_at_js(map_id: &str, x: f64, y: f64, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const requestedLayers = opts && Array.isArray(opts.layers) ? opts.layers : [];

                let features = map.queryRenderedFeatures([{x}, {y}], opts);

                if (requestedLayers.length > 0 && features.length === 0) {{
                    const layerSet = new Set(requestedLayers);
                    features = map
                        .queryRenderedFeatures([{x}, {y}])
                        .filter(f => f && f.layer && layerSet.has(f.layer.id));
                }}

                return features.map(f => ({{
                    id: Number.isFinite(f.id) ? Math.trunc(f.id) : null,
                    geometry: f.geometry,
                    properties: f.properties || {{}},
                    source: f.source,
                    sourceLayer: f.sourceLayer || null
                }}));
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to query rendered features at point:', err);
                return [];
            }}
        }})();
        "#
    )
}

/// Generate JS to query source features
pub fn query_source_features_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const features = map.querySourceFeatures({source_id_lit}, opts);

                return features.map(f => ({{
                    id: Number.isFinite(f.id) ? Math.trunc(f.id) : null,
                    geometry: f.geometry,
                    properties: f.properties || {{}},
                    source: {source_id_lit},
                    sourceLayer: f.sourceLayer || null
                }}));
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to query source features:', err);
                return [];
            }}
        }})();
        "#
    )
}
