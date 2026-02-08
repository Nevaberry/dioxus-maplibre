//! Source management JS bridge.

use super::find_map_js;
use super::js_escape::js_single_quoted;

/// Generate JS to add a GeoJSON source
pub fn add_geojson_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "geojson", options_json)
}

/// Generate JS to add a vector tile source
pub fn add_vector_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "vector", options_json)
}

/// Generate JS to add a raster tile source
pub fn add_raster_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "raster", options_json)
}

/// Generate JS to add a raster DEM source
pub fn add_raster_dem_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "raster-dem", options_json)
}

/// Generate JS to add an image source
pub fn add_image_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "image", options_json)
}

fn add_source_js(map_id: &str, source_id: &str, source_type: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let source_id_lit = js_single_quoted(source_id);
    let source_type_lit = js_single_quoted(source_type);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources[{map_id_lit}];
                if (sourceRegistry) {{
                    sourceRegistry[{source_id_lit}] = {{
                        type: {source_type_lit},
                        options: JSON.parse(JSON.stringify(opts))
                    }};
                }}
                if (map.getSource({source_id_lit})) return;
                map.addSource({source_id_lit}, {{
                    type: {source_type_lit},
                    ...opts
                }});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add source:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to update a GeoJSON source's data
pub fn update_geojson_source_js(map_id: &str, source_id: &str, data_json: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        r#"
        (function() {{
            {find}
            const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources[{map_id_lit}];
            if (sourceRegistry && sourceRegistry[{source_id_lit}]) {{
                sourceRegistry[{source_id_lit}].options = sourceRegistry[{source_id_lit}].options || {{}};
                sourceRegistry[{source_id_lit}].options.data = {data_json};
            }}
            const source = map.getSource({source_id_lit});
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
    let map_id_lit = js_single_quoted(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                if (map.getSource({source_id_lit})) {{
                    map.removeSource({source_id_lit});
                }}
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources[{map_id_lit}];
                if (sourceRegistry) {{
                    delete sourceRegistry[{source_id_lit}];
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to remove source:', err);
            }}
        }})();
        "#
    )
}
