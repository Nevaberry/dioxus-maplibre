//! Source management JS bridge.

use super::find_map_js;
/// Generate JS to add a GeoJSON source
pub fn add_geojson_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources['{map_id}'];
                if (sourceRegistry) {{
                    sourceRegistry['{source_id}'] = {{
                        type: 'geojson',
                        options: JSON.parse(JSON.stringify(opts))
                    }};
                }}
                if (map.getSource('{source_id}')) return;
                map.addSource('{source_id}', {{
                    type: 'geojson',
                    ...opts
                }});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add GeoJSON source:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to add a vector tile source
pub fn add_vector_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources['{map_id}'];
                if (sourceRegistry) {{
                    sourceRegistry['{source_id}'] = {{
                        type: 'vector',
                        options: JSON.parse(JSON.stringify(opts))
                    }};
                }}
                if (map.getSource('{source_id}')) return;
                map.addSource('{source_id}', {{
                    type: 'vector',
                    ...opts
                }});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add vector source:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to add a raster tile source
pub fn add_raster_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources['{map_id}'];
                if (sourceRegistry) {{
                    sourceRegistry['{source_id}'] = {{
                        type: 'raster',
                        options: JSON.parse(JSON.stringify(opts))
                    }};
                }}
                if (map.getSource('{source_id}')) return;
                map.addSource('{source_id}', {{
                    type: 'raster',
                    ...opts
                }});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add raster source:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to add a raster DEM source
pub fn add_raster_dem_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources['{map_id}'];
                if (sourceRegistry) {{
                    sourceRegistry['{source_id}'] = {{
                        type: 'raster-dem',
                        options: JSON.parse(JSON.stringify(opts))
                    }};
                }}
                if (map.getSource('{source_id}')) return;
                map.addSource('{source_id}', {{
                    type: 'raster-dem',
                    ...opts
                }});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add raster-dem source:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to add an image source
pub fn add_image_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources['{map_id}'];
                if (sourceRegistry) {{
                    sourceRegistry['{source_id}'] = {{
                        type: 'image',
                        options: JSON.parse(JSON.stringify(opts))
                    }};
                }}
                if (map.getSource('{source_id}')) return;
                map.addSource('{source_id}', {{
                    type: 'image',
                    ...opts
                }});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add image source:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to update a GeoJSON source's data
pub fn update_geojson_source_js(map_id: &str, source_id: &str, data_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources['{map_id}'];
            if (sourceRegistry && sourceRegistry['{source_id}']) {{
                sourceRegistry['{source_id}'].options = sourceRegistry['{source_id}'].options || {{}};
                sourceRegistry['{source_id}'].options.data = {data_json};
            }}
            const source = map.getSource('{source_id}');
            if (source) {{
                try {{
                    source.setData({data_json});
                }} catch (err) {{
                    console.error('[dioxus-maplibre] Failed to update source:', err);
                }}
            }}
        }})();
        "#
    )
}

/// Generate JS to remove a source from the map
pub fn remove_source_js(map_id: &str, source_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                if (map.getSource('{source_id}')) {{
                    map.removeSource('{source_id}');
                }}
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources['{map_id}'];
                if (sourceRegistry) {{
                    delete sourceRegistry['{source_id}'];
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to remove source:', err);
            }}
        }})();
        "#
    )
}
