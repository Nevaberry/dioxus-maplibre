//! Layer management JS bridge.

use super::find_map_js;
use super::js_escape::js_single_quoted;

/// Generate JS to add a layer from serialized LayerOptions
pub fn add_layer_js(map_id: &str, layer_json: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const layerDef = {layer_json};
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers[{map_id_lit}];
                const layerOrder = window.__dioxus_maplibre_layer_order && window.__dioxus_maplibre_layer_order[{map_id_lit}];
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
    let map_id_lit = js_single_quoted(map_id);
    let layer_id_lit = js_single_quoted(layer_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const mapHandlers = window.__dioxus_maplibre_layer_handlers && window.__dioxus_maplibre_layer_handlers[{map_id_lit}];
                const handlers = mapHandlers && mapHandlers[{layer_id_lit}];
                if (handlers) {{
                    if (handlers.click) {{
                        map.off('click', {layer_id_lit}, handlers.click);
                    }}
                    if (handlers.mouseenter) {{
                        map.off('mouseenter', {layer_id_lit}, handlers.mouseenter);
                    }}
                    if (handlers.mouseleave) {{
                        map.off('mouseleave', {layer_id_lit}, handlers.mouseleave);
                    }}
                    delete mapHandlers[{layer_id_lit}];
                }}

                if (map.getLayer({layer_id_lit})) {{
                    map.removeLayer({layer_id_lit});
                }}
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers[{map_id_lit}];
                if (layerRegistry) {{
                    delete layerRegistry[{layer_id_lit}];
                }}
                const layerOrder = window.__dioxus_maplibre_layer_order && window.__dioxus_maplibre_layer_order[{map_id_lit}];
                if (layerOrder) {{
                    const idx = layerOrder.indexOf({layer_id_lit});
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
    let map_id_lit = js_single_quoted(map_id);
    let layer_id_lit = js_single_quoted(layer_id);
    let name_lit = js_single_quoted(name);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers[{map_id_lit}];
                if (layerRegistry && layerRegistry[{layer_id_lit}]) {{
                    if (!layerRegistry[{layer_id_lit}].paint) {{
                        layerRegistry[{layer_id_lit}].paint = {{}};
                    }}
                    layerRegistry[{layer_id_lit}].paint[{name_lit}] = {value_json};
                }}
                if (map.getLayer({layer_id_lit})) {{
                    map.setPaintProperty({layer_id_lit}, {name_lit}, {value_json});
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
    let map_id_lit = js_single_quoted(map_id);
    let layer_id_lit = js_single_quoted(layer_id);
    let name_lit = js_single_quoted(name);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers[{map_id_lit}];
                if (layerRegistry && layerRegistry[{layer_id_lit}]) {{
                    if (!layerRegistry[{layer_id_lit}].layout) {{
                        layerRegistry[{layer_id_lit}].layout = {{}};
                    }}
                    layerRegistry[{layer_id_lit}].layout[{name_lit}] = {value_json};
                }}
                if (map.getLayer({layer_id_lit})) {{
                    map.setLayoutProperty({layer_id_lit}, {name_lit}, {value_json});
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
    let map_id_lit = js_single_quoted(map_id);
    let layer_id_lit = js_single_quoted(layer_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers[{map_id_lit}];
                if (layerRegistry && layerRegistry[{layer_id_lit}]) {{
                    layerRegistry[{layer_id_lit}].filter = {filter_json};
                }}
                if (map.getLayer({layer_id_lit})) {{
                    map.setFilter({layer_id_lit}, {filter_json});
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
    let map_id_lit = js_single_quoted(map_id);
    let layer_id_lit = js_single_quoted(layer_id);
    format!(
        r#"
        (function() {{
            {find}
            const handlersRoot = window.__dioxus_maplibre_layer_handlers;
            if (!handlersRoot) {{
                return;
            }}
            if (!handlersRoot[{map_id_lit}]) {{
                handlersRoot[{map_id_lit}] = {{}};
            }}
            const mapHandlers = handlersRoot[{map_id_lit}];
            if (!mapHandlers[{layer_id_lit}]) {{
                mapHandlers[{layer_id_lit}] = {{}};
            }}
            const handlers = mapHandlers[{layer_id_lit}];
            if (handlers.click) {{
                return;
            }}
            const onClick = function(e) {{
                if (e.features && e.features.length > 0) {{
                    e.originalEvent.stopPropagation();
                    const feature = e.features[0];
                    if (window.__dioxus_maplibre_sendEvent) {{
                        window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                            type: 'layer_click',
                            layer_id: {layer_id_lit},
                            feature_id: feature.id !== undefined ? feature.id : null,
                            properties: feature.properties || {{}},
                            latlng: {{ lat: e.lngLat.lat, lng: e.lngLat.lng }}
                        }}));
                    }}
                }}
            }};
            handlers.click = onClick;
            map.on('click', {layer_id_lit}, onClick);
        }})();
        "#
    )
}

/// Generate JS to unregister click handlers on a layer
pub fn unregister_layer_click_js(map_id: &str, layer_id: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let layer_id_lit = js_single_quoted(layer_id);
    format!(
        r#"
        (function() {{
            {find}
            const mapHandlers = window.__dioxus_maplibre_layer_handlers && window.__dioxus_maplibre_layer_handlers[{map_id_lit}];
            const handlers = mapHandlers && mapHandlers[{layer_id_lit}];
            if (!handlers || !handlers.click) {{
                return;
            }}
            map.off('click', {layer_id_lit}, handlers.click);
            delete handlers.click;
            if (!handlers.mouseenter && !handlers.mouseleave) {{
                delete mapHandlers[{layer_id_lit}];
            }}
        }})();
        "#
    )
}

/// Generate JS to register hover handlers on a layer (separate from layer creation)
pub fn register_layer_hover_js(map_id: &str, layer_id: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let layer_id_lit = js_single_quoted(layer_id);
    format!(
        r#"
        (function() {{
            {find}
            const handlersRoot = window.__dioxus_maplibre_layer_handlers;
            if (!handlersRoot) {{
                return;
            }}
            if (!handlersRoot[{map_id_lit}]) {{
                handlersRoot[{map_id_lit}] = {{}};
            }}
            const mapHandlers = handlersRoot[{map_id_lit}];
            if (!mapHandlers[{layer_id_lit}]) {{
                mapHandlers[{layer_id_lit}] = {{}};
            }}
            const handlers = mapHandlers[{layer_id_lit}];

            if (!handlers.mouseenter) {{
                const onMouseEnter = function(e) {{
                    map.getCanvas().style.cursor = 'pointer';
                    if (e.features && e.features.length > 0) {{
                        const feature = e.features[0];
                        if (window.__dioxus_maplibre_sendEvent) {{
                            window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                                type: 'layer_hover',
                                layer_id: {layer_id_lit},
                                feature_id: feature.id !== undefined ? feature.id : null,
                                properties: feature.properties || {{}},
                                latlng: {{ lat: e.lngLat.lat, lng: e.lngLat.lng }},
                                cursor_x: e.originalEvent.clientX,
                                cursor_y: e.originalEvent.clientY
                            }}));
                        }}
                    }}
                }};
                handlers.mouseenter = onMouseEnter;
                map.on('mouseenter', {layer_id_lit}, onMouseEnter);
            }}

            if (!handlers.mouseleave) {{
                const onMouseLeave = function() {{
                    map.getCanvas().style.cursor = '';
                    if (window.__dioxus_maplibre_sendEvent) {{
                        window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                            type: 'layer_hover',
                            layer_id: {layer_id_lit},
                            feature_id: null,
                            properties: null,
                            latlng: {{ lat: 0, lng: 0 }},
                            cursor_x: 0,
                            cursor_y: 0
                        }}));
                    }}
                }};
                handlers.mouseleave = onMouseLeave;
                map.on('mouseleave', {layer_id_lit}, onMouseLeave);
            }}
        }})();
        "#
    )
}

/// Generate JS to unregister hover handlers on a layer
pub fn unregister_layer_hover_js(map_id: &str, layer_id: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let layer_id_lit = js_single_quoted(layer_id);
    format!(
        r#"
        (function() {{
            {find}
            const mapHandlers = window.__dioxus_maplibre_layer_handlers && window.__dioxus_maplibre_layer_handlers[{map_id_lit}];
            const handlers = mapHandlers && mapHandlers[{layer_id_lit}];
            if (!handlers) {{
                return;
            }}
            if (handlers.mouseenter) {{
                map.off('mouseenter', {layer_id_lit}, handlers.mouseenter);
                delete handlers.mouseenter;
            }}
            if (handlers.mouseleave) {{
                map.off('mouseleave', {layer_id_lit}, handlers.mouseleave);
                delete handlers.mouseleave;
            }}
            if (!handlers.click && !handlers.mouseenter && !handlers.mouseleave) {{
                delete mapHandlers[{layer_id_lit}];
            }}
        }})();
        "#
    )
}

/// Generate JS to move a layer before another layer (or to top if before_id is None)
pub fn move_layer_js(map_id: &str, layer_id: &str, before_id: Option<&str>) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let layer_id_lit = js_single_quoted(layer_id);
    let before_arg = before_id.map_or_else(|| "undefined".to_string(), js_single_quoted);
    let before_id_lit = before_id.map_or_else(|| "null".to_string(), js_single_quoted);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                if (map.getLayer({layer_id_lit})) {{
                    map.moveLayer({layer_id_lit}, {before_arg});
                }}
                const layerOrder = window.__dioxus_maplibre_layer_order && window.__dioxus_maplibre_layer_order[{map_id_lit}];
                if (layerOrder) {{
                    const idx = layerOrder.indexOf({layer_id_lit});
                    if (idx >= 0) {{
                        layerOrder.splice(idx, 1);
                    }}
                    const beforeId = {before_id_lit};
                    if (beforeId != null) {{
                        const beforeIdx = layerOrder.indexOf(beforeId);
                        if (beforeIdx >= 0) {{
                            layerOrder.splice(beforeIdx, 0, {layer_id_lit});
                        }} else {{
                            layerOrder.push({layer_id_lit});
                        }}
                    }} else {{
                        layerOrder.push({layer_id_lit});
                    }}
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to move layer:', err);
            }}
        }})();
        "#
    )
}
