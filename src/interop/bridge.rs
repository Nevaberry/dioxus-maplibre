//! JavaScript bridge for MapLibre GL JS
//!
//! This module generates the JavaScript code needed to interact with MapLibre GL JS.
//! Maps are stored in `window.__dioxus_maplibre_maps[map_id]` for lifecycle management.
//!
//! **Why global `sendEvent`?** Each `document::eval()` call creates an isolated JS context
//! with its own `dioxus.send()`. Markers, layers, and other objects added via separate evals
//! need a shared callback to route events back to the map's event channel.

// These functions generate JS strings and are only called on wasm32 targets.
// On other targets they appear unused but we keep them available for testing.
#![allow(dead_code)]

use uuid::Uuid;

/// Generate a unique map ID
pub fn generate_map_id() -> String {
    format!("map_{}", Uuid::new_v4().to_string().replace('-', ""))
}

// =============================================================================
// Map Initialization & Lifecycle
// =============================================================================

/// Generate JS to initialize a MapLibre map
///
/// Sets up the map with polling for MapLibre GL JS, container finding with
/// hot-reload fallback, and event listeners for all supported event types.
#[allow(clippy::too_many_arguments)]
pub fn init_map_js(
    container_id: &str,
    map_id: &str,
    style: &str,
    center_lng: f64,
    center_lat: f64,
    zoom: f64,
    bearing: f64,
    pitch: f64,
    min_zoom: Option<f64>,
    max_zoom: Option<f64>,
    max_bounds: Option<&str>,
    cooperative_gestures: Option<bool>,
) -> String {
    let min_zoom_param = min_zoom
        .map(|z| format!("minZoom: {z},"))
        .unwrap_or_default();
    let max_zoom_param = max_zoom
        .map(|z| format!("maxZoom: {z},"))
        .unwrap_or_default();
    let max_bounds_param = max_bounds
        .map(|b| format!("maxBounds: {b},"))
        .unwrap_or_default();
    let cooperative_gestures_param = cooperative_gestures
        .map(|v| format!("cooperativeGestures: {v},"))
        .unwrap_or_default();

    format!(
        r#"
        (async function() {{
            console.log('[dioxus-maplibre] Initializing map:', '{map_id}');

            // Wait for next animation frame to ensure DOM is flushed
            await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));

            // Wait for MapLibre GL JS to load (max 10 seconds)
            let attempts = 0;
            const maxAttempts = 100;
            while (typeof maplibregl === 'undefined' && attempts < maxAttempts) {{
                if (attempts % 10 === 0) console.log(`[dioxus-maplibre] Waiting for maplibregl... attempt ${{attempts + 1}}`);
                await new Promise(resolve => setTimeout(resolve, 100));
                attempts++;
            }}

            if (typeof maplibregl === 'undefined') {{
                console.error('[dioxus-maplibre] MapLibre GL JS not loaded after 10 seconds!');
                dioxus.send(JSON.stringify({{ type: 'error', message: 'MapLibre GL JS not loaded' }}));
                return 'error';
            }}

            console.log('[dioxus-maplibre] MapLibre loaded, version:', maplibregl.version || 'unknown');

            // Wait for container to be in DOM - try specific ID first, then fall back
            let container = document.getElementById('{container_id}');
            let containerAttempts = 0;

            while (!container && containerAttempts < 50) {{
                if (containerAttempts % 10 === 0) console.log(`[dioxus-maplibre] Waiting for container by ID... ${{containerAttempts}}`);
                await new Promise(resolve => requestAnimationFrame(resolve));
                container = document.getElementById('{container_id}');
                containerAttempts++;
            }}

            // Fallback: find any map container div that doesn't already have a map
            if (!container) {{
                console.log('[dioxus-maplibre] ID not found, trying fallback selector...');
                const mapContainerParent = document.querySelector('.map-container');
                if (mapContainerParent) {{
                    const candidates = mapContainerParent.querySelectorAll('div[id^="map_"][id$="_container"]');
                    for (const candidate of candidates) {{
                        if (!candidate.querySelector('canvas.maplibregl-canvas')) {{
                            container = candidate;
                            console.log('[dioxus-maplibre] Found fallback container:', candidate.id);
                            break;
                        }}
                    }}
                }}
            }}

            if (!container) {{
                console.error('[dioxus-maplibre] Container not found by ID or fallback:', '{container_id}');
                dioxus.send(JSON.stringify({{ type: 'error', message: 'Container not found' }}));
                return 'error';
            }}

            const actualContainerId = container.id;
            console.log(`[dioxus-maplibre] Container found: ${{actualContainerId}} ${{container.offsetWidth}}x${{container.offsetHeight}}`);

            // Ensure registry exists
            if (!window.__dioxus_maplibre_maps) {{
                window.__dioxus_maplibre_maps = {{}};
            }}
            if (!window.__dioxus_maplibre_markers) {{
                window.__dioxus_maplibre_markers = {{}};
            }}
            if (!window.__dioxus_maplibre_sources) {{
                window.__dioxus_maplibre_sources = {{}};
            }}
            if (!window.__dioxus_maplibre_layers) {{
                window.__dioxus_maplibre_layers = {{}};
            }}
            if (!window.__dioxus_maplibre_layer_order) {{
                window.__dioxus_maplibre_layer_order = {{}};
            }}
            if (!window.__dioxus_maplibre_images) {{
                window.__dioxus_maplibre_images = {{}};
            }}
            if (!window.__dioxus_maplibre_terrain) {{
                window.__dioxus_maplibre_terrain = {{}};
            }}
            if (!window.__dioxus_maplibre_sky) {{
                window.__dioxus_maplibre_sky = {{}};
            }}
            if (!window.__dioxus_maplibre_fog) {{
                window.__dioxus_maplibre_fog = {{}};
            }}
            if (!window.__dioxus_maplibre_debug) {{
                window.__dioxus_maplibre_debug = {{
                    patch: 'runtime-replay-v5',
                    setStyleCalls: 0,
                    replayRuns: 0,
                    replaySourceAdds: 0,
                    replayLayerAdds: 0,
                    lastMapId: null,
                    lastStyle: null,
                    lastError: null
                }};
            }}
            window.__dioxus_maplibre_debug.lastMapId = '{map_id}';

            // Check if this container already has a map
            if (container.querySelector('canvas.maplibregl-canvas')) {{
                console.log('[dioxus-maplibre] Container already has a map, skipping init');
                dioxus.send(JSON.stringify({{ type: 'ready' }}));
                return 'already_exists';
            }}

            if (window.__dioxus_maplibre_maps[actualContainerId]) {{
                console.log('[dioxus-maplibre] Map already registered for this container');
                dioxus.send(JSON.stringify({{ type: 'ready' }}));
                return 'already_exists';
            }}

            try {{
                const map = new maplibregl.Map({{
                    container: container,
                    style: '{style}',
                    center: [{center_lng}, {center_lat}],
                    zoom: {zoom},
                    bearing: {bearing},
                    pitch: {pitch},
                    {min_zoom_param}
                    {max_zoom_param}
                    {max_bounds_param}
                    {cooperative_gestures_param}
                    attributionControl: true
                }});

                console.log('[dioxus-maplibre] Map instance created for container:', actualContainerId);

                // Store map reference under both actual container ID and map_id
                window.__dioxus_maplibre_maps[actualContainerId] = map;
                window.__dioxus_maplibre_markers[actualContainerId] = {{}};
                window.__dioxus_maplibre_sources[actualContainerId] = {{}};
                window.__dioxus_maplibre_layers[actualContainerId] = {{}};
                window.__dioxus_maplibre_layer_order[actualContainerId] = [];
                window.__dioxus_maplibre_images[actualContainerId] = {{}};
                window.__dioxus_maplibre_terrain[actualContainerId] = {{ hasValue: false, value: null }};
                window.__dioxus_maplibre_sky[actualContainerId] = {{ hasValue: false, value: null }};
                window.__dioxus_maplibre_fog[actualContainerId] = {{ hasValue: false, value: null }};

                window.__dioxus_maplibre_maps['{map_id}'] = map;
                window.__dioxus_maplibre_markers['{map_id}'] = window.__dioxus_maplibre_markers[actualContainerId];
                window.__dioxus_maplibre_sources['{map_id}'] = window.__dioxus_maplibre_sources[actualContainerId];
                window.__dioxus_maplibre_layers['{map_id}'] = window.__dioxus_maplibre_layers[actualContainerId];
                window.__dioxus_maplibre_layer_order['{map_id}'] = window.__dioxus_maplibre_layer_order[actualContainerId];
                window.__dioxus_maplibre_images['{map_id}'] = window.__dioxus_maplibre_images[actualContainerId];
                window.__dioxus_maplibre_terrain['{map_id}'] = window.__dioxus_maplibre_terrain[actualContainerId];
                window.__dioxus_maplibre_sky['{map_id}'] = window.__dioxus_maplibre_sky[actualContainerId];
                window.__dioxus_maplibre_fog['{map_id}'] = window.__dioxus_maplibre_fog[actualContainerId];

                // Global event sender for cross-eval communication
                window.__dioxus_maplibre_sendEvent = function(eventJson) {{
                    dioxus.send(eventJson);
                }};

                // --- Event listeners ---

                map.on('click', function(e) {{
                    dioxus.send(JSON.stringify({{
                        type: 'click',
                        latlng: {{ lat: e.lngLat.lat, lng: e.lngLat.lng }},
                        point: {{ x: e.point.x, y: e.point.y }}
                    }}));
                }});

                map.on('dblclick', function(e) {{
                    dioxus.send(JSON.stringify({{
                        type: 'dblclick',
                        latlng: {{ lat: e.lngLat.lat, lng: e.lngLat.lng }},
                        point: {{ x: e.point.x, y: e.point.y }}
                    }}));
                }});

                map.on('contextmenu', function(e) {{
                    dioxus.send(JSON.stringify({{
                        type: 'contextmenu',
                        latlng: {{ lat: e.lngLat.lat, lng: e.lngLat.lng }},
                        point: {{ x: e.point.x, y: e.point.y }}
                    }}));
                }});

                map.on('moveend', function() {{
                    const center = map.getCenter();
                    const bounds = map.getBounds();
                    dioxus.send(JSON.stringify({{
                        type: 'move',
                        center: {{ lat: center.lat, lng: center.lng }},
                        zoom: map.getZoom(),
                        bounds: {{
                            sw: {{ lat: bounds.getSouth(), lng: bounds.getWest() }},
                            ne: {{ lat: bounds.getNorth(), lng: bounds.getEast() }}
                        }}
                    }}));
                }});

                map.on('zoomend', function() {{
                    dioxus.send(JSON.stringify({{
                        type: 'zoom',
                        zoom: map.getZoom()
                    }}));
                }});

                map.on('rotateend', function() {{
                    dioxus.send(JSON.stringify({{
                        type: 'rotate',
                        bearing: map.getBearing()
                    }}));
                }});

                map.on('pitchend', function() {{
                    dioxus.send(JSON.stringify({{
                        type: 'pitch',
                        pitch: map.getPitch()
                    }}));
                }});

                map.on('load', function() {{
                    console.log('[dioxus-maplibre] Map loaded, sending ready event');
                    dioxus.send(JSON.stringify({{ type: 'ready' }}));
                    map.fire('moveend');
                }});

                map.on('error', function(e) {{
                    console.error('[dioxus-maplibre] Map error:', e.error);
                }});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to create map:', err);
                dioxus.send(JSON.stringify({{ type: 'error', message: err.message }}));
                return 'error';
            }}

            return 'ok';
        }})();
        "#
    )
}

/// Generate JS to destroy a map and clean up resources
pub fn destroy_map_js(map_id: &str) -> String {
    format!(
        r#"
        (function() {{
            const map = window.__dioxus_maplibre_maps && window.__dioxus_maplibre_maps['{map_id}'];

            const markers = window.__dioxus_maplibre_markers['{map_id}'];
            if (markers) {{
                Object.values(markers).forEach(marker => marker.remove());
                delete window.__dioxus_maplibre_markers['{map_id}'];
            }}

            if (map) {{
                map.remove();
            }}

            const keysToDelete = [];
            const registries = [
                '__dioxus_maplibre_maps',
                '__dioxus_maplibre_markers',
                '__dioxus_maplibre_sources',
                '__dioxus_maplibre_layers',
                '__dioxus_maplibre_layer_order',
                '__dioxus_maplibre_images',
                '__dioxus_maplibre_terrain',
                '__dioxus_maplibre_sky',
                '__dioxus_maplibre_fog',
            ];

            const mapRegistry = window.__dioxus_maplibre_maps || {{}};
            if (map) {{
                for (const [key, value] of Object.entries(mapRegistry)) {{
                    if (value === map) {{
                        keysToDelete.push(key);
                    }}
                }}
            }}
            keysToDelete.push('{map_id}');

            for (const key of keysToDelete) {{
                for (const registryName of registries) {{
                    const registry = window[registryName];
                    if (registry && Object.prototype.hasOwnProperty.call(registry, key)) {{
                        delete registry[key];
                    }}
                }}
            }}
        }})();
        "#
    )
}

// =============================================================================
// Helper: find map by ID with fallback
// =============================================================================

/// JS snippet to find a map by ID with fallback to any available map
fn find_map_js(map_id: &str) -> String {
    format!(
        r#"let mapRegistry = window.__dioxus_maplibre_maps || {{}};
            let map = mapRegistry['{map_id}'];
            if (!map) {{
                const mapKeys = Object.keys(mapRegistry);
                if (mapKeys.length === 1) {{
                    map = mapRegistry[mapKeys[0]];
                    console.warn('[dioxus-maplibre] map id not found, using sole map instance', '{map_id}', '=>', mapKeys[0]);
                }} else {{
                    console.error('[dioxus-maplibre] map id not found', '{map_id}', 'available maps:', mapKeys);
                    return;
                }}
            }}"#
    )
}

// =============================================================================
// Sources
// =============================================================================

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
                console.log('[dioxus-maplibre] Added GeoJSON source:', '{source_id}');
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
                console.log('[dioxus-maplibre] Added vector source:', '{source_id}');
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
                console.log('[dioxus-maplibre] Added raster source:', '{source_id}');
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
                console.log('[dioxus-maplibre] Added raster-dem source:', '{source_id}');
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
                console.log('[dioxus-maplibre] Added image source:', '{source_id}');
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
                    console.log('[dioxus-maplibre] Updated GeoJSON source:', '{source_id}');
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
                console.log('[dioxus-maplibre] Removed source:', '{source_id}');
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to remove source:', err);
            }}
        }})();
        "#
    )
}

// =============================================================================
// Layers
// =============================================================================

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
                console.log('[dioxus-maplibre] Added layer:', layerDef.id);
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
                console.log('[dioxus-maplibre] Removed layer:', '{layer_id}');
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
                console.log('[dioxus-maplibre] Set paint property:', '{layer_id}', '{name}');
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
                console.log('[dioxus-maplibre] Set layout property:', '{layer_id}', '{name}');
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
                console.log('[dioxus-maplibre] Set filter on layer:', '{layer_id}');
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

// =============================================================================
// Controls
// =============================================================================

/// Generate JS to add a navigation control
pub fn add_navigation_control_js(map_id: &str, position: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.addControl(new maplibregl.NavigationControl(), '{position}');
        }})();
        "#
    )
}

/// Generate JS to add a geolocate control
pub fn add_geolocate_control_js(map_id: &str, position: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.addControl(new maplibregl.GeolocateControl({{
                positionOptions: {{ enableHighAccuracy: true }},
                trackUserLocation: true
            }}), '{position}');
        }})();
        "#
    )
}

/// Generate JS to add a scale control
pub fn add_scale_control_js(map_id: &str, position: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.addControl(new maplibregl.ScaleControl(), '{position}');
        }})();
        "#
    )
}

/// Generate JS to add a fullscreen control
pub fn add_fullscreen_control_js(map_id: &str, position: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.addControl(new maplibregl.FullscreenControl(), '{position}');
        }})();
        "#
    )
}

/// Generate JS to add an attribution control
pub fn add_attribution_control_js(map_id: &str, position: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.addControl(new maplibregl.AttributionControl({{ compact: true }}), '{position}');
        }})();
        "#
    )
}

// =============================================================================
// Markers
// =============================================================================

/// Generate JS to add a marker
pub fn add_marker_js(
    map_id: &str,
    marker_id: &str,
    lat: f64,
    lng: f64,
    options_json: &str,
) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}

            const opts = {options_json};

            let markerOpts = {{}};
            let el = null;

            if (opts.emoji) {{
                el = document.createElement('div');
                el.className = 'maplibre-marker-emoji';
                el.innerHTML = opts.emoji;
                el.style.fontSize = '28px';
                el.style.cursor = 'pointer';
                el.style.filter = 'drop-shadow(0 2px 4px rgba(0,0,0,0.5))';
                markerOpts.element = el;
            }} else {{
                markerOpts.color = opts.color || '#3b82f6';
            }}

            if (opts.draggable) markerOpts.draggable = true;
            if (opts.rotation != null) markerOpts.rotation = opts.rotation;
            if (opts.scale != null) markerOpts.scale = opts.scale;

            const marker = new maplibregl.Marker(markerOpts)
                .setLngLat([{lng}, {lat}])
                .addTo(map);

            if (opts.popupHtml) {{
                const popup = new maplibregl.Popup({{ offset: 25 }})
                    .setHTML(opts.popupHtml);
                marker.setPopup(popup);
            }}

            // Store marker reference
            if (!window.__dioxus_maplibre_markers['{map_id}']) {{
                window.__dioxus_maplibre_markers['{map_id}'] = {{}};
            }}
            window.__dioxus_maplibre_markers['{map_id}']['{marker_id}'] = marker;

            // Click handler
            marker.getElement().addEventListener('click', function(e) {{
                e.stopPropagation();
                if (window.__dioxus_maplibre_sendEvent) {{
                    window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                        type: 'marker_click',
                        marker_id: '{marker_id}',
                        latlng: {{ lat: {lat}, lng: {lng} }}
                    }}));
                }}
            }});

            // Drag handlers (only if marker is draggable)
            if (opts.draggable) {{
                marker.on('dragstart', function() {{
                    const lngLat = marker.getLngLat();
                    if (window.__dioxus_maplibre_sendEvent) {{
                        window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                            type: 'marker_dragstart',
                            marker_id: '{marker_id}',
                            latlng: {{ lat: lngLat.lat, lng: lngLat.lng }}
                        }}));
                    }}
                }});
                marker.on('dragend', function() {{
                    const lngLat = marker.getLngLat();
                    if (window.__dioxus_maplibre_sendEvent) {{
                        window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                            type: 'marker_dragend',
                            marker_id: '{marker_id}',
                            latlng: {{ lat: lngLat.lat, lng: lngLat.lng }}
                        }}));
                    }}
                }});
            }}

            // Hover handlers
            marker.getElement().addEventListener('mouseenter', function(e) {{
                if (window.__dioxus_maplibre_sendEvent) {{
                    window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                        type: 'marker_hover',
                        marker_id: '{marker_id}',
                        latlng: {{ lat: {lat}, lng: {lng} }},
                        hover: true,
                        cursor_x: e.clientX,
                        cursor_y: e.clientY
                    }}));
                }}
            }});

            marker.getElement().addEventListener('mouseleave', function(e) {{
                if (window.__dioxus_maplibre_sendEvent) {{
                    window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                        type: 'marker_hover',
                        marker_id: '{marker_id}',
                        latlng: {{ lat: {lat}, lng: {lng} }},
                        hover: false,
                        cursor_x: e.clientX,
                        cursor_y: e.clientY
                    }}));
                }}
            }});
        }})();
        "#
    )
}

/// Generate JS to remove a marker
pub fn remove_marker_js(map_id: &str, marker_id: &str) -> String {
    format!(
        r#"
        (function() {{
            const markers = window.__dioxus_maplibre_markers['{map_id}'];
            if (markers && markers['{marker_id}']) {{
                markers['{marker_id}'].remove();
                delete markers['{marker_id}'];
            }}
        }})();
        "#
    )
}

/// Generate JS to update marker position
pub fn update_marker_position_js(map_id: &str, marker_id: &str, lat: f64, lng: f64) -> String {
    format!(
        r#"
        (function() {{
            const markers = window.__dioxus_maplibre_markers['{map_id}'];
            if (markers && markers['{marker_id}']) {{
                markers['{marker_id}'].setLngLat([{lng}, {lat}]);
            }}
        }})();
        "#
    )
}

// =============================================================================
// Popups
// =============================================================================

/// Generate JS to add a standalone popup at a location
pub fn add_popup_js(
    map_id: &str,
    popup_id: &str,
    lat: f64,
    lng: f64,
    html: &str,
    options_json: &str,
) -> String {
    let find = find_map_js(map_id);
    let escaped_html = html.replace('\\', "\\\\").replace('`', "\\`");
    format!(
        r#"
        (function() {{
            {find}
            const opts = {options_json};
            const popupOpts = {{}};
            if (opts.offset) popupOpts.offset = opts.offset;
            if (opts.anchor) popupOpts.anchor = opts.anchor;
            if (opts.closeButton != null) popupOpts.closeButton = opts.closeButton;
            if (opts.closeOnClick != null) popupOpts.closeOnClick = opts.closeOnClick;
            if (opts.maxWidth) popupOpts.maxWidth = opts.maxWidth;
            if (opts.className) popupOpts.className = opts.className;

            const popup = new maplibregl.Popup(popupOpts)
                .setLngLat([{lng}, {lat}])
                .setHTML(`{escaped_html}`)
                .addTo(map);

            // Store popup reference
            if (!window.__dioxus_maplibre_popups) window.__dioxus_maplibre_popups = {{}};
            if (!window.__dioxus_maplibre_popups['{map_id}']) window.__dioxus_maplibre_popups['{map_id}'] = {{}};
            window.__dioxus_maplibre_popups['{map_id}']['{popup_id}'] = popup;
        }})();
        "#
    )
}

/// Generate JS to remove a popup
pub fn remove_popup_js(map_id: &str, popup_id: &str) -> String {
    format!(
        r#"
        (function() {{
            const popups = window.__dioxus_maplibre_popups && window.__dioxus_maplibre_popups['{map_id}'];
            if (popups && popups['{popup_id}']) {{
                popups['{popup_id}'].remove();
                delete popups['{popup_id}'];
            }}
        }})();
        "#
    )
}

// =============================================================================
// Navigation
// =============================================================================

/// Generate JS for flyTo
pub fn fly_to_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const opts = {options_json};
            // Convert LatLng to MapLibre [lng, lat] format
            if (opts.center && opts.center.lat !== undefined) {{
                opts.center = [opts.center.lng, opts.center.lat];
            }}
            map.flyTo(opts);
        }})();
        "#
    )
}

/// Generate JS for easeTo
pub fn ease_to_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const opts = {options_json};
            if (opts.center && opts.center.lat !== undefined) {{
                opts.center = [opts.center.lng, opts.center.lat];
            }}
            map.easeTo(opts);
        }})();
        "#
    )
}

/// Generate JS for jumpTo
pub fn jump_to_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const opts = {options_json};
            if (opts.center && opts.center.lat !== undefined) {{
                opts.center = [opts.center.lng, opts.center.lat];
            }}
            map.jumpTo(opts);
        }})();
        "#
    )
}

/// Generate JS for fitBounds
pub fn fit_bounds_js(
    map_id: &str,
    sw_lng: f64,
    sw_lat: f64,
    ne_lng: f64,
    ne_lat: f64,
    options_json: &str,
) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const opts = {options_json};
            map.fitBounds([[{sw_lng}, {sw_lat}], [{ne_lng}, {ne_lat}]], opts);
        }})();
        "#
    )
}

/// Generate JS for panTo
pub fn pan_to_js(map_id: &str, lat: f64, lng: f64) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.panTo([{lng}, {lat}]);
        }})();
        "#
    )
}

/// Generate JS for panBy
pub fn pan_by_js(map_id: &str, x: i32, y: i32) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.panBy([{x}, {y}], {{ duration: 0 }});
        }})();
        "#
    )
}

/// Generate JS for zoomTo
pub fn zoom_to_js(map_id: &str, zoom: f64) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.zoomTo({zoom});
        }})();
        "#
    )
}

/// Generate JS for zoomIn
pub fn zoom_in_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.zoomIn();
        }})();
        "#
    )
}

/// Generate JS for zoomOut
pub fn zoom_out_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.zoomOut();
        }})();
        "#
    )
}

/// Generate JS for rotateTo (setBearing)
pub fn rotate_to_js(map_id: &str, bearing: f64) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.rotateTo({bearing});
        }})();
        "#
    )
}

/// Generate JS for setPitch
pub fn set_pitch_js(map_id: &str, pitch: f64) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setPitch({pitch});
        }})();
        "#
    )
}

/// Generate JS for resetNorth
pub fn reset_north_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.resetNorth();
        }})();
        "#
    )
}

// =============================================================================
// Feature State
// =============================================================================

/// Generate JS to set feature state
pub fn set_feature_state_js(
    map_id: &str,
    source: &str,
    feature_id: i64,
    source_layer: Option<&str>,
    state_json: &str,
) -> String {
    let find = find_map_js(map_id);
    let source_layer_prop = source_layer
        .map(|sl| format!(", sourceLayer: '{sl}'"))
        .unwrap_or_default();
    format!(
        r#"
        (function() {{
            {find}
            try {{
                map.setFeatureState(
                    {{ source: '{source}', id: {feature_id}{source_layer_prop} }},
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
    feature_id: i64,
    source_layer: Option<&str>,
) -> String {
    let find = find_map_js(map_id);
    let source_layer_prop = source_layer
        .map(|sl| format!(", sourceLayer: '{sl}'"))
        .unwrap_or_default();
    format!(
        r#"
        (function() {{
            {find}
            try {{
                map.removeFeatureState(
                    {{ source: '{source}', id: {feature_id}{source_layer_prop} }}
                );
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to remove feature state:', err);
            }}
        }})();
        "#
    )
}

// =============================================================================
// Images
// =============================================================================

/// Generate JS to load an image and add it to the map's sprite
pub fn load_image_js(map_id: &str, image_id: &str, url: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (async function() {{
            {find}
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images['{map_id}'];
            if (imageRegistry) {{
                imageRegistry['{image_id}'] = '{url}';
            }}
            try {{
                const response = await map.loadImage('{url}');
                if (!map.hasImage('{image_id}')) {{
                    map.addImage('{image_id}', response.data);
                }}
                console.log('[dioxus-maplibre] Loaded image:', '{image_id}');
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to load image:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to check if an image exists (returns boolean via dioxus.send)
pub fn has_image_js(map_id: &str, image_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            return map.hasImage('{image_id}');
        }})();
        "#
    )
}

/// Generate JS to load an image and return success via dioxus.send
pub fn load_image_async_js(map_id: &str, image_id: &str, url: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (async function() {{
            {find}
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images['{map_id}'];
            if (imageRegistry) {{
                imageRegistry['{image_id}'] = '{url}';
            }}
            try {{
                const response = await map.loadImage('{url}');
                if (!map.hasImage('{image_id}')) {{
                    map.addImage('{image_id}', response.data);
                }}
                console.log('[dioxus-maplibre] Loaded image (async):', '{image_id}');
                return true;
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to load image:', err);
                return false;
            }}
        }})();
        "#
    )
}

/// Generate JS to remove an image from the map's sprite
pub fn remove_image_js(map_id: &str, image_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            if (map.hasImage('{image_id}')) {{
                map.removeImage('{image_id}');
            }}
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images['{map_id}'];
            if (imageRegistry) {{
                delete imageRegistry['{image_id}'];
            }}
            console.log('[dioxus-maplibre] Removed image:', '{image_id}');
        }})();
        "#
    )
}

// =============================================================================
// Style
// =============================================================================

/// Generate JS to set the map style
pub fn set_style_js(map_id: &str, style_url: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources['{map_id}'];
            const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers['{map_id}'];
            const layerOrder = window.__dioxus_maplibre_layer_order && window.__dioxus_maplibre_layer_order['{map_id}'];
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images['{map_id}'];
            const terrainState = window.__dioxus_maplibre_terrain && window.__dioxus_maplibre_terrain['{map_id}'];
            const skyState = window.__dioxus_maplibre_sky && window.__dioxus_maplibre_sky['{map_id}'];
            const fogState = window.__dioxus_maplibre_fog && window.__dioxus_maplibre_fog['{map_id}'];
            const debugState = window.__dioxus_maplibre_debug || (window.__dioxus_maplibre_debug = {{
                patch: 'runtime-replay-v5',
                setStyleCalls: 0,
                replayRuns: 0,
                replaySourceAdds: 0,
                replayLayerAdds: 0,
                lastMapId: null,
                lastStyle: null,
                lastError: null
            }});
            debugState.patch = 'runtime-replay-v5';
            debugState.setStyleCalls += 1;
            debugState.lastMapId = '{map_id}';
            debugState.lastStyle = '{style_url}';
            debugState.lastReplayTrigger = null;
            debugState.lastReplayStats = null;
            debugState.lastError = null;

            if (!window.__dioxus_maplibre_style_switch_tokens) {{
                window.__dioxus_maplibre_style_switch_tokens = {{}};
            }}
            const styleSwitchToken = `${{Date.now()}}_${{Math.random().toString(36).slice(2)}}`;
            window.__dioxus_maplibre_style_switch_tokens['{map_id}'] = styleSwitchToken;

            console.log('[dioxus-maplibre] setStyle requested for map {map_id}:', '{style_url}');

            let replayed = false;
            let replayTimeoutId = null;
            let awaitingNewStyle = false;
            const trackedSourceIds = sourceRegistry ? Object.keys(sourceRegistry) : [];
            const trackedLayerIds = layerRegistry ? Object.keys(layerRegistry) : [];
            let lastWaitState = '';
            let sawStyleData = false;

            const replayReadiness = function() {{
                const styleLoaded = map.isStyleLoaded();
                const sourcesGone = trackedSourceIds.every((id) => !map.getSource(id));
                const layersGone = trackedLayerIds.every((id) => !map.getLayer(id));
                return {{ styleLoaded, sourcesGone, layersGone }};
            }};

            const replayRuntimeState = function(trigger) {{
                const activeToken = window.__dioxus_maplibre_style_switch_tokens
                    && window.__dioxus_maplibre_style_switch_tokens['{map_id}'];
                if (activeToken !== styleSwitchToken) {{
                    console.log('[dioxus-maplibre] Skipping stale style replay for map {map_id}');
                    return;
                }}
                if (replayed) {{
                    return;
                }}
                replayed = true;
                awaitingNewStyle = false;
                debugState.lastReplayTrigger = trigger;
                try {{
                    debugState.replayRuns += 1;
                    const sourceCount = sourceRegistry ? Object.keys(sourceRegistry).length : 0;
                    const layerCount = layerRegistry ? Object.keys(layerRegistry).length : 0;
                    const imageCount = imageRegistry ? Object.keys(imageRegistry).length : 0;
                    debugState.lastReplayStats = {{
                        sourceCount,
                        layerCount,
                        imageCount,
                        orderedLayerCount: layerOrder ? layerOrder.length : 0
                    }};
                    console.log(
                        '[dioxus-maplibre] Replaying runtime state after style change for map {map_id}'
                        + ' trigger=' + trigger
                        + ' sources=' + sourceCount
                        + ' layers=' + layerCount
                        + ' images=' + imageCount
                        + ' ordered_layers=' + (layerOrder ? layerOrder.length : 0)
                    );

                    if (sourceRegistry) {{
                        for (const [sourceId, sourceDef] of Object.entries(sourceRegistry)) {{
                            try {{
                                if (!sourceDef || !sourceDef.type) continue;
                                if (map.getSource(sourceId)) continue;
                                const options = sourceDef.options
                                    ? JSON.parse(JSON.stringify(sourceDef.options))
                                    : {{}};
                                map.addSource(sourceId, {{
                                    type: sourceDef.type,
                                    ...options
                                }});
                                debugState.replaySourceAdds += 1;
                                console.log('[dioxus-maplibre] Replayed source:', sourceId, sourceDef.type);
                            }} catch (err) {{
                                debugState.lastError = String(err);
                                console.error('[dioxus-maplibre] Failed replaying source:', sourceId, err);
                            }}
                        }}
                    }}

                    const orderedLayerIds =
                        layerOrder && layerOrder.length > 0
                            ? layerOrder.slice()
                            : Object.keys(layerRegistry || {{}});
                    for (const layerId of orderedLayerIds) {{
                        const layerDef = layerRegistry && layerRegistry[layerId];
                        if (!layerDef) continue;
                        try {{
                            if (map.getLayer(layerId)) continue;
                            const layerToAdd = JSON.parse(JSON.stringify(layerDef));
                            map.addLayer(layerToAdd);
                            debugState.replayLayerAdds += 1;
                            console.log('[dioxus-maplibre] Replayed layer:', layerId);
                        }} catch (err) {{
                            debugState.lastError = String(err);
                            console.error('[dioxus-maplibre] Failed replaying layer:', layerId, err);
                        }}
                    }}

                    if (terrainState && terrainState.hasValue) {{
                        try {{
                            map.setTerrain(terrainState.value);
                            console.log('[dioxus-maplibre] Replayed terrain state');
                        }} catch (err) {{
                            debugState.lastError = String(err);
                            console.error('[dioxus-maplibre] Failed replaying terrain state:', err);
                        }}
                    }}

                    if (skyState && skyState.hasValue) {{
                        try {{
                            map.setSky(skyState.value);
                            console.log('[dioxus-maplibre] Replayed sky state');
                        }} catch (err) {{
                            debugState.lastError = String(err);
                            console.error('[dioxus-maplibre] Failed replaying sky state:', err);
                        }}
                    }}

                    if (fogState && fogState.hasValue) {{
                        try {{
                            map.setFog(fogState.value);
                            console.log('[dioxus-maplibre] Replayed fog state');
                        }} catch (err) {{
                            debugState.lastError = String(err);
                            console.error('[dioxus-maplibre] Failed replaying fog state:', err);
                        }}
                    }}

                    if (imageRegistry) {{
                        for (const [imageId, url] of Object.entries(imageRegistry)) {{
                            if (!url) continue;
                            map.loadImage(url).then((response) => {{
                                if (!response || !response.data) return;
                                if (!map.hasImage(imageId)) {{
                                    map.addImage(imageId, response.data);
                                    console.log('[dioxus-maplibre] Replayed image:', imageId);
                                }}
                            }}).catch((err) => {{
                                debugState.lastError = String(err);
                                console.error('[dioxus-maplibre] Failed replaying image:', imageId, err);
                            }});
                        }}
                    }}

                    console.log('[dioxus-maplibre] Runtime replay complete for map {map_id}');
                }} catch (err) {{
                    debugState.lastError = String(err);
                    console.error('[dioxus-maplibre] Runtime replay failed for map {map_id}:', err);
                }} finally {{
                    if (replayTimeoutId != null) {{
                        clearTimeout(replayTimeoutId);
                    }}
                    map.off('style.load', onStyleLoad);
                    map.off('styledata', onStyleData);
                }}
            }};

            const maybeReplay = function(trigger, force) {{
                if (!awaitingNewStyle) {{
                    return;
                }}
                const readiness = replayReadiness();
                const canReplayWithoutLoaded = sawStyleData && readiness.sourcesGone && readiness.layersGone;
                const canReplayNormally = readiness.styleLoaded && readiness.sourcesGone && readiness.layersGone;
                if (!force && !canReplayNormally && !canReplayWithoutLoaded) {{
                    const waitState = `${{readiness.styleLoaded}}|${{readiness.sourcesGone}}|${{readiness.layersGone}}|${{sawStyleData}}`;
                    if (waitState !== lastWaitState) {{
                        lastWaitState = waitState;
                        console.log(
                            '[dioxus-maplibre] Waiting replay for map {map_id}'
                            + ' trigger=' + trigger
                            + ' styleLoaded=' + readiness.styleLoaded
                            + ' sourcesGone=' + readiness.sourcesGone
                            + ' layersGone=' + readiness.layersGone
                            + ' sawStyleData=' + sawStyleData
                        );
                    }}
                    return;
                }}
                replayRuntimeState(force ? `${{trigger}}+forced` : trigger);
            }};

            const onStyleLoad = function() {{
                maybeReplay('style.load', false);
            }};

            const onStyleData = function(e) {{
                if (!awaitingNewStyle) {{
                    return;
                }}
                if (e && e.dataType && e.dataType !== 'style') {{
                    return;
                }}
                sawStyleData = true;
                maybeReplay('styledata', false);
            }};

            map.on('style.load', onStyleLoad);
            map.on('styledata', onStyleData);
            awaitingNewStyle = true;
            map.setStyle('{style_url}');

            setTimeout(function() {{
                if (!awaitingNewStyle || replayed) {{
                    return;
                }}
                const readiness = replayReadiness();
                // Some styles emit validation errors and never reach isStyleLoaded=true.
                // If style transition occurred (styledata seen) and old custom objects are gone,
                // we can safely replay immediately.
                if (sawStyleData && readiness.sourcesGone && readiness.layersGone) {{
                    replayRuntimeState('post-setStyle-styledata-transition');
                    return;
                }}
                maybeReplay('post-setStyle-check', false);
            }}, 0);

            replayTimeoutId = setTimeout(function() {{
                console.warn('[dioxus-maplibre] Timed out waiting for style.load, forcing replay for map {map_id}');
                maybeReplay('timeout', true);
            }}, 6000);
        }})();
        "#
    )
}

// =============================================================================
// Terrain & Sky
// =============================================================================

/// Generate JS to set terrain
pub fn set_terrain_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setTerrain({options_json});
            const terrainState = window.__dioxus_maplibre_terrain && window.__dioxus_maplibre_terrain['{map_id}'];
            if (terrainState) {{
                terrainState.hasValue = true;
                terrainState.value = JSON.parse(JSON.stringify({options_json}));
            }}
            console.log('[dioxus-maplibre] Set terrain for map {map_id}');
        }})();
        "#
    )
}

/// Generate JS to remove terrain
pub fn remove_terrain_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setTerrain(null);
            const terrainState = window.__dioxus_maplibre_terrain && window.__dioxus_maplibre_terrain['{map_id}'];
            if (terrainState) {{
                terrainState.hasValue = true;
                terrainState.value = null;
            }}
            console.log('[dioxus-maplibre] Removed terrain for map {map_id}');
        }})();
        "#
    )
}

/// Generate JS to set sky
pub fn set_sky_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setSky({options_json});
            const skyState = window.__dioxus_maplibre_sky && window.__dioxus_maplibre_sky['{map_id}'];
            if (skyState) {{
                skyState.hasValue = true;
                skyState.value = JSON.parse(JSON.stringify({options_json}));
            }}
            console.log('[dioxus-maplibre] Set sky for map {map_id}');
        }})();
        "#
    )
}

/// Generate JS to remove sky
pub fn remove_sky_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setSky(null);
            const skyState = window.__dioxus_maplibre_sky && window.__dioxus_maplibre_sky['{map_id}'];
            if (skyState) {{
                skyState.hasValue = true;
                skyState.value = null;
            }}
            console.log('[dioxus-maplibre] Removed sky for map {map_id}');
        }})();
        "#
    )
}

// =============================================================================
// Fog / Atmosphere
// =============================================================================

/// Generate JS to set fog/atmosphere
pub fn set_fog_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setFog({options_json});
            const fogState = window.__dioxus_maplibre_fog && window.__dioxus_maplibre_fog['{map_id}'];
            if (fogState) {{
                fogState.hasValue = true;
                fogState.value = JSON.parse(JSON.stringify({options_json}));
            }}
            console.log('[dioxus-maplibre] Set fog for map {map_id}');
        }})();
        "#
    )
}

/// Generate JS to remove fog/atmosphere
pub fn remove_fog_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setFog(null);
            const fogState = window.__dioxus_maplibre_fog && window.__dioxus_maplibre_fog['{map_id}'];
            if (fogState) {{
                fogState.hasValue = true;
                fogState.value = null;
            }}
            console.log('[dioxus-maplibre] Removed fog for map {map_id}');
        }})();
        "#
    )
}

// =============================================================================
// Viewport Padding
// =============================================================================

/// Generate JS to set viewport padding
pub fn set_padding_js(map_id: &str, padding_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setPadding({padding_json});
        }})();
        "#
    )
}

/// Generate JS to get viewport padding
pub fn get_padding_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const p = map.getPadding();
            return {{ top: p.top, bottom: p.bottom, left: p.left, right: p.right }};
        }})();
        "#
    )
}

// =============================================================================
// Layer Ordering
// =============================================================================

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
                console.log('[dioxus-maplibre] Moved layer:', '{layer_id}', 'before', {before_id_lit});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to move layer:', err);
            }}
        }})();
        "#
    )
}

// =============================================================================
// Getters (return values via eval)
// =============================================================================

/// Generate JS to get zoom level
pub fn get_zoom_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            return map.getZoom();
        }})();
        "#
    )
}

/// Generate JS to get center as {{ lat, lng }}
pub fn get_center_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const c = map.getCenter();
            return {{ lat: c.lat, lng: c.lng }};
        }})();
        "#
    )
}

/// Generate JS to get bearing
pub fn get_bearing_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            return map.getBearing();
        }})();
        "#
    )
}

/// Generate JS to get pitch
pub fn get_pitch_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            return map.getPitch();
        }})();
        "#
    )
}

/// Generate JS to get bounds as {{ sw: {{ lat, lng }}, ne: {{ lat, lng }} }}
pub fn get_bounds_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const b = map.getBounds();
            return {{
                sw: {{ lat: b.getSouth(), lng: b.getWest() }},
                ne: {{ lat: b.getNorth(), lng: b.getEast() }}
            }};
        }})();
        "#
    )
}

// =============================================================================
// Feature Queries
// =============================================================================

/// Generate JS to query rendered features (entire viewport)
pub fn query_rendered_features_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const features = map.queryRenderedFeatures(opts);
                return features.map(f => ({{
                    id: f.id !== undefined ? f.id : null,
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
                const features = map.queryRenderedFeatures([{x}, {y}], opts);
                return features.map(f => ({{
                    id: f.id !== undefined ? f.id : null,
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
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const features = map.querySourceFeatures('{source_id}', opts);
                return features.map(f => ({{
                    id: f.id !== undefined ? f.id : null,
                    geometry: f.geometry,
                    properties: f.properties || {{}},
                    source: '{source_id}',
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
