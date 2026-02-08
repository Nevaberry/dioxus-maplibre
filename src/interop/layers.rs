//! Layer management JS bridge.

use super::find_map_js;
/// Generate JS to add a layer from serialized LayerOptions
pub fn add_layer_js(map_id: &str, layer_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const layerDef = {layer_json};
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers['{map_id}'];
                const layerOrder = window.__dioxus_maplibre_layer_order && window.__dioxus_maplibre_layer_order['{map_id}'];
                if (layerRegistry) {{
                    layerRegistry[layerDef.id] = JSON.parse(JSON.stringify(layerDef));
                }}
                if (layerOrder && !layerOrder.includes(layerDef.id)) {{
                    layerOrder.push(layerDef.id);
                }}
                if (map.getLayer(layerDef.id)) return;
                map.addLayer(layerDef);
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add layer:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to remove a layer
pub fn remove_layer_js(map_id: &str, layer_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                if (map.getLayer('{layer_id}')) {{
                    map.removeLayer('{layer_id}');
                }}
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers['{map_id}'];
                if (layerRegistry) {{
                    delete layerRegistry['{layer_id}'];
                }}
                const layerOrder = window.__dioxus_maplibre_layer_order && window.__dioxus_maplibre_layer_order['{map_id}'];
                if (layerOrder) {{
                    const idx = layerOrder.indexOf('{layer_id}');
                    if (idx >= 0) {{
                        layerOrder.splice(idx, 1);
                    }}
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to remove layer:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to set a paint property on a layer
pub fn set_paint_property_js(map_id: &str, layer_id: &str, name: &str, value_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers['{map_id}'];
                if (layerRegistry && layerRegistry['{layer_id}']) {{
                    if (!layerRegistry['{layer_id}'].paint) {{
                        layerRegistry['{layer_id}'].paint = {{}};
                    }}
                    layerRegistry['{layer_id}'].paint['{name}'] = {value_json};
                }}
                if (map.getLayer('{layer_id}')) {{
                    map.setPaintProperty('{layer_id}', '{name}', {value_json});
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to set paint property:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to set a layout property on a layer
pub fn set_layout_property_js(
    map_id: &str,
    layer_id: &str,
    name: &str,
    value_json: &str,
) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers['{map_id}'];
                if (layerRegistry && layerRegistry['{layer_id}']) {{
                    if (!layerRegistry['{layer_id}'].layout) {{
                        layerRegistry['{layer_id}'].layout = {{}};
                    }}
                    layerRegistry['{layer_id}'].layout['{name}'] = {value_json};
                }}
                if (map.getLayer('{layer_id}')) {{
                    map.setLayoutProperty('{layer_id}', '{name}', {value_json});
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to set layout property:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to set a filter on a layer
pub fn set_filter_js(map_id: &str, layer_id: &str, filter_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers['{map_id}'];
                if (layerRegistry && layerRegistry['{layer_id}']) {{
                    layerRegistry['{layer_id}'].filter = {filter_json};
                }}
                if (map.getLayer('{layer_id}')) {{
                    map.setFilter('{layer_id}', {filter_json});
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to set filter:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to register a click handler on a layer (separate from layer creation)
pub fn register_layer_click_js(map_id: &str, layer_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.on('click', '{layer_id}', function(e) {{
                if (e.features && e.features.length > 0) {{
                    e.originalEvent.stopPropagation();
                    const feature = e.features[0];
                    if (window.__dioxus_maplibre_sendEvent) {{
                        window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                            type: 'layer_click',
                            layer_id: '{layer_id}',
                            feature_id: feature.id !== undefined ? feature.id : null,
                            properties: feature.properties || {{}},
                            latlng: {{ lat: e.lngLat.lat, lng: e.lngLat.lng }}
                        }}));
                    }}
                }}
            }});
        }})();
        "#
    )
}

/// Generate JS to register hover handlers on a layer (separate from layer creation)
pub fn register_layer_hover_js(map_id: &str, layer_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.on('mouseenter', '{layer_id}', function(e) {{
                map.getCanvas().style.cursor = 'pointer';
                if (e.features && e.features.length > 0) {{
                    const feature = e.features[0];
                    if (window.__dioxus_maplibre_sendEvent) {{
                        window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                            type: 'layer_hover',
                            layer_id: '{layer_id}',
                            feature_id: feature.id !== undefined ? feature.id : null,
                            properties: feature.properties || {{}},
                            latlng: {{ lat: e.lngLat.lat, lng: e.lngLat.lng }},
                            cursor_x: e.originalEvent.clientX,
                            cursor_y: e.originalEvent.clientY
                        }}));
                    }}
                }}
            }});

            map.on('mouseleave', '{layer_id}', function() {{
                map.getCanvas().style.cursor = '';
                if (window.__dioxus_maplibre_sendEvent) {{
                    window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                        type: 'layer_hover',
                        layer_id: '{layer_id}',
                        feature_id: null,
                        properties: null,
                        latlng: {{ lat: 0, lng: 0 }},
                        cursor_x: 0,
                        cursor_y: 0
                    }}));
                }}
            }});
        }})();
        "#
    )
}

/// Generate JS to move a layer before another layer (or to top if before_id is None)
pub fn move_layer_js(map_id: &str, layer_id: &str, before_id: Option<&str>) -> String {
    let find = find_map_js(map_id);
    let before_arg = before_id.map_or_else(|| "undefined".to_string(), |id| format!("'{id}'"));
    let before_id_lit = before_id.map_or_else(|| "null".to_string(), |id| format!("'{id}'"));
    format!(
        r#"
        (function() {{
            {find}
            try {{
                if (map.getLayer('{layer_id}')) {{
                    map.moveLayer('{layer_id}', {before_arg});
                }}
                const layerOrder = window.__dioxus_maplibre_layer_order && window.__dioxus_maplibre_layer_order['{map_id}'];
                if (layerOrder) {{
                    const idx = layerOrder.indexOf('{layer_id}');
                    if (idx >= 0) {{
                        layerOrder.splice(idx, 1);
                    }}
                    const beforeId = {before_id_lit};
                    if (beforeId != null) {{
                        const beforeIdx = layerOrder.indexOf(beforeId);
                        if (beforeIdx >= 0) {{
                            layerOrder.splice(beforeIdx, 0, '{layer_id}');
                        }} else {{
                            layerOrder.push('{layer_id}');
                        }}
                    }} else {{
                        layerOrder.push('{layer_id}');
                    }}
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to move layer:', err);
            }}
        }})();
        "#
    )
}
