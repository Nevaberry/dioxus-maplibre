//! Map initialization and teardown JS bridge.

use super::js_escape::js_single_quoted;

/// Generate JS to initialize a MapLibre map.
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
    move_event_throttle_ms: u32,
) -> String {
    let container_id_lit = js_single_quoted(container_id);
    let map_id_lit = js_single_quoted(map_id);
    let style_lit = js_single_quoted(style);

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
            // Wait for next animation frame to ensure DOM is flushed
            await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));

            // Wait for MapLibre GL JS to load (max 10 seconds)
            let attempts = 0;
            const maxAttempts = 100;
            while (typeof maplibregl === 'undefined' && attempts < maxAttempts) {{
                await new Promise(resolve => setTimeout(resolve, 100));
                attempts++;
            }}

            if (typeof maplibregl === 'undefined') {{
                console.error('[dioxus-maplibre] MapLibre GL JS not loaded after 10 seconds!');
                dioxus.send(JSON.stringify({{ type: 'error', message: 'MapLibre GL JS not loaded' }}));
                return 'error';
            }}

            // Wait for container to be in DOM - try specific ID first, then fall back
            let container = document.getElementById({container_id_lit});
            let containerAttempts = 0;

            while (!container && containerAttempts < 50) {{
                await new Promise(resolve => requestAnimationFrame(resolve));
                container = document.getElementById({container_id_lit});
                containerAttempts++;
            }}

            // Fallback: find any map container div that doesn't already have a map
            if (!container) {{
                const mapContainerParent = document.querySelector('.map-container');
                if (mapContainerParent) {{
                    const candidates = mapContainerParent.querySelectorAll('div[id^="map_"][id$="_container"]');
                    for (const candidate of candidates) {{
                        if (!candidate.querySelector('canvas.maplibregl-canvas')) {{
                            container = candidate;
                            break;
                        }}
                    }}
                }}
            }}

            if (!container) {{
                console.error('[dioxus-maplibre] Container not found by ID or fallback:', {container_id_lit});
                dioxus.send(JSON.stringify({{ type: 'error', message: 'Container not found' }}));
                return 'error';
            }}

            const actualContainerId = container.id;

            // Ensure registries exist
            if (!window.__dioxus_maplibre_maps) {{
                window.__dioxus_maplibre_maps = {{}};
            }}
            if (!window.__dioxus_maplibre_markers) {{
                window.__dioxus_maplibre_markers = {{}};
            }}
            if (!window.__dioxus_maplibre_popups) {{
                window.__dioxus_maplibre_popups = {{}};
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
            if (!window.__dioxus_maplibre_layer_handlers) {{
                window.__dioxus_maplibre_layer_handlers = {{}};
            }}
            if (!window.__dioxus_maplibre_controls) {{
                window.__dioxus_maplibre_controls = {{}};
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

            // Check if this container already has a map
            if (container.querySelector('canvas.maplibregl-canvas')) {{
                dioxus.send(JSON.stringify({{ type: 'ready' }}));
                return 'already_exists';
            }}

            if (window.__dioxus_maplibre_maps[actualContainerId]) {{
                dioxus.send(JSON.stringify({{ type: 'ready' }}));
                return 'already_exists';
            }}

            try {{
                const map = new maplibregl.Map({{
                    container,
                    style: {style_lit},
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

                const initialMoveEventThrottleMs = Number({move_event_throttle_ms});
                map.__dioxusMoveEventThrottleMs =
                    Number.isFinite(initialMoveEventThrottleMs) && initialMoveEventThrottleMs >= 0
                        ? initialMoveEventThrottleMs
                        : 80;

                // Store map reference under both actual container ID and map_id.
                window.__dioxus_maplibre_maps[actualContainerId] = map;
                window.__dioxus_maplibre_markers[actualContainerId] = {{}};
                window.__dioxus_maplibre_popups[actualContainerId] = {{}};
                window.__dioxus_maplibre_sources[actualContainerId] = {{}};
                window.__dioxus_maplibre_layers[actualContainerId] = {{}};
                window.__dioxus_maplibre_layer_order[actualContainerId] = [];
                window.__dioxus_maplibre_layer_handlers[actualContainerId] = {{}};
                window.__dioxus_maplibre_controls[actualContainerId] = {{}};
                window.__dioxus_maplibre_images[actualContainerId] = {{}};
                window.__dioxus_maplibre_terrain[actualContainerId] = {{ hasValue: false, value: null }};
                window.__dioxus_maplibre_sky[actualContainerId] = {{ hasValue: false, value: null }};
                window.__dioxus_maplibre_fog[actualContainerId] = {{ hasValue: false, value: null }};

                window.__dioxus_maplibre_maps[{map_id_lit}] = map;
                window.__dioxus_maplibre_markers[{map_id_lit}] = window.__dioxus_maplibre_markers[actualContainerId];
                window.__dioxus_maplibre_popups[{map_id_lit}] = window.__dioxus_maplibre_popups[actualContainerId];
                window.__dioxus_maplibre_sources[{map_id_lit}] = window.__dioxus_maplibre_sources[actualContainerId];
                window.__dioxus_maplibre_layers[{map_id_lit}] = window.__dioxus_maplibre_layers[actualContainerId];
                window.__dioxus_maplibre_layer_order[{map_id_lit}] = window.__dioxus_maplibre_layer_order[actualContainerId];
                window.__dioxus_maplibre_layer_handlers[{map_id_lit}] = window.__dioxus_maplibre_layer_handlers[actualContainerId];
                window.__dioxus_maplibre_controls[{map_id_lit}] = window.__dioxus_maplibre_controls[actualContainerId];
                window.__dioxus_maplibre_images[{map_id_lit}] = window.__dioxus_maplibre_images[actualContainerId];
                window.__dioxus_maplibre_terrain[{map_id_lit}] = window.__dioxus_maplibre_terrain[actualContainerId];
                window.__dioxus_maplibre_sky[{map_id_lit}] = window.__dioxus_maplibre_sky[actualContainerId];
                window.__dioxus_maplibre_fog[{map_id_lit}] = window.__dioxus_maplibre_fog[actualContainerId];

                // Global event sender for cross-eval communication.
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

                const emitMoveEvent = function(eventName) {{
                    const center = map.getCenter();
                    const bounds = map.getBounds();
                    dioxus.send(JSON.stringify({{
                        type: 'move',
                        phase: eventName,
                        center: {{ lat: center.lat, lng: center.lng }},
                        zoom: map.getZoom(),
                        bounds: {{
                            sw: {{ lat: bounds.getSouth(), lng: bounds.getWest() }},
                            ne: {{ lat: bounds.getNorth(), lng: bounds.getEast() }}
                        }}
                    }}));
                }};

                const DEFAULT_MOVE_EVENT_THROTTLE_MS = 80;
                let moveRafId = null;
                let movePending = false;
                let lastMoveEmitAt = 0;

                const getMoveEventThrottleMs = function() {{
                    const value = Number(map.__dioxusMoveEventThrottleMs);
                    if (!Number.isFinite(value) || value < 0) {{
                        return DEFAULT_MOVE_EVENT_THROTTLE_MS;
                    }}
                    return value;
                }};

                const scheduleMoveEmit = function() {{
                    if (moveRafId !== null) {{
                        return;
                    }}
                    moveRafId = requestAnimationFrame(() => {{
                        moveRafId = null;
                        if (!movePending) {{
                            return;
                        }}
                        movePending = false;
                        const now = (typeof performance !== 'undefined' && performance.now)
                            ? performance.now()
                            : Date.now();
                        const throttleMs = getMoveEventThrottleMs();
                        if ((now - lastMoveEmitAt) < throttleMs) {{
                            movePending = true;
                            scheduleMoveEmit();
                            return;
                        }}
                        lastMoveEmitAt = now;
                        emitMoveEvent('move');
                    }});
                }};

                map.on('move', function() {{
                    movePending = true;
                    scheduleMoveEmit();
                }});

                map.on('moveend', function() {{
                    movePending = false;
                    if (moveRafId !== null) {{
                        cancelAnimationFrame(moveRafId);
                        moveRafId = null;
                    }}
                    emitMoveEvent('moveend');
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
                    dioxus.send(JSON.stringify({{ type: 'ready' }}));
                    emitMoveEvent('move_load');
                }});

                map.on('error', function(e) {{
                    const message = e && e.error && e.error.message
                        ? String(e.error.message)
                        : (e && e.error ? String(e.error) : 'Map error');
                    console.error('[dioxus-maplibre] Map error:', e && e.error ? e.error : e);
                    dioxus.send(JSON.stringify({{ type: 'error', message }}));
                }});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to create map:', err);
                dioxus.send(JSON.stringify({{ type: 'error', message: err && err.message ? err.message : String(err) }}));
                return 'error';
            }}

            return 'ok';
        }})();
        "#
    )
}

/// Generate JS to destroy a map and clean up resources.
pub fn destroy_map_js(map_id: &str) -> String {
    let map_id_lit = js_single_quoted(map_id);
    format!(
        r#"
        (function() {{
            const map = window.__dioxus_maplibre_maps && window.__dioxus_maplibre_maps[{map_id_lit}];

            const markers = window.__dioxus_maplibre_markers && window.__dioxus_maplibre_markers[{map_id_lit}];
            if (markers) {{
                Object.values(markers).forEach(marker => marker.remove());
                delete window.__dioxus_maplibre_markers[{map_id_lit}];
            }}

            const popups = window.__dioxus_maplibre_popups && window.__dioxus_maplibre_popups[{map_id_lit}];
            if (popups) {{
                Object.values(popups).forEach(popup => popup.remove());
                delete window.__dioxus_maplibre_popups[{map_id_lit}];
            }}

            const controls = window.__dioxus_maplibre_controls && window.__dioxus_maplibre_controls[{map_id_lit}];
            if (map && controls) {{
                for (const control of Object.values(controls)) {{
                    try {{
                        map.removeControl(control);
                    }} catch (_err) {{}}
                }}
                delete window.__dioxus_maplibre_controls[{map_id_lit}];
            }}

            const mapHandlers = window.__dioxus_maplibre_layer_handlers && window.__dioxus_maplibre_layer_handlers[{map_id_lit}];
            if (map && mapHandlers) {{
                for (const [layerId, handlers] of Object.entries(mapHandlers)) {{
                    if (handlers && handlers.click) {{
                        map.off('click', layerId, handlers.click);
                    }}
                    if (handlers && handlers.mouseenter) {{
                        map.off('mouseenter', layerId, handlers.mouseenter);
                    }}
                    if (handlers && handlers.mouseleave) {{
                        map.off('mouseleave', layerId, handlers.mouseleave);
                    }}
                }}
                delete window.__dioxus_maplibre_layer_handlers[{map_id_lit}];
            }}

            if (map) {{
                map.remove();
            }}

            const keysToDelete = [];
            const registries = [
                '__dioxus_maplibre_maps',
                '__dioxus_maplibre_markers',
                '__dioxus_maplibre_popups',
                '__dioxus_maplibre_sources',
                '__dioxus_maplibre_layers',
                '__dioxus_maplibre_layer_order',
                '__dioxus_maplibre_layer_handlers',
                '__dioxus_maplibre_controls',
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
            keysToDelete.push({map_id_lit});

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
