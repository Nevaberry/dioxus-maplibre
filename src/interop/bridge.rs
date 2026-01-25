//! JavaScript bridge for MapLibre GL JS
//!
//! This module generates the JavaScript code needed to interact with MapLibre GL JS.
//! Maps are stored in `window.__dioxus_maplibre_maps[map_id]` for lifecycle management.

// These functions generate JS strings and are only called on wasm32 targets.
// On other targets they appear unused but we keep them available for testing.
#![allow(dead_code)]

use uuid::Uuid;

/// Generate a unique map ID
pub fn generate_map_id() -> String {
    format!("map_{}", Uuid::new_v4().to_string().replace('-', ""))
}

/// Generate JS to initialize the global maps registry
pub fn init_registry_js() -> String {
    r#"
    if (!window.__dioxus_maplibre_maps) {
        window.__dioxus_maplibre_maps = {};
        window.__dioxus_maplibre_markers = {};
        window.__dioxus_maplibre_sources = {};
        window.__dioxus_maplibre_layers = {};
    }
    "#.to_string()
}

/// Generate JS to initialize a MapLibre map
/// Includes polling wait for MapLibre to load since document::Script is async
pub fn init_map_js(
    container_id: &str,
    map_id: &str,
    style: &str,
    center_lng: f64,
    center_lat: f64,
    zoom: f64,
) -> String {
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

            // Wait for container to be in DOM - try specific ID first, then fall back to any map container
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
                    // Find a div inside map-container that looks like our container
                    const candidates = mapContainerParent.querySelectorAll('div[id^="map_"][id$="_container"]');
                    for (const candidate of candidates) {{
                        // Check if this container doesn't already have a map
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

            // Use the actual container ID for map storage (may differ from map_id due to remounting)
            const actualContainerId = container.id;
            console.log(`[dioxus-maplibre] Container found: ${{actualContainerId}} ${{container.offsetWidth}}x${{container.offsetHeight}}`);

            // Ensure registry exists
            if (!window.__dioxus_maplibre_maps) {{
                window.__dioxus_maplibre_maps = {{}};
                window.__dioxus_maplibre_markers = {{}};
                window.__dioxus_maplibre_sources = {{}};
                window.__dioxus_maplibre_layers = {{}};
            }}

            // Check if this container already has a map (by checking for canvas)
            if (container.querySelector('canvas.maplibregl-canvas')) {{
                console.log('[dioxus-maplibre] Container already has a map, skipping init');
                dioxus.send(JSON.stringify({{ type: 'ready' }}));
                return 'already_exists';
            }}

            // Check if map exists by the container ID
            if (window.__dioxus_maplibre_maps[actualContainerId]) {{
                console.log('[dioxus-maplibre] Map already registered for this container');
                dioxus.send(JSON.stringify({{ type: 'ready' }}));
                return 'already_exists';
            }}

            try {{
                // Create the map using the actual container element
                const map = new maplibregl.Map({{
                    container: container, // Use actual DOM element, not ID string
                    style: '{style}',
                    center: [{center_lng}, {center_lat}],
                    zoom: {zoom},
                    attributionControl: true
                }});

                console.log('[dioxus-maplibre] Map instance created for container:', actualContainerId);

                // Store map reference using the actual container ID
                window.__dioxus_maplibre_maps[actualContainerId] = map;
                window.__dioxus_maplibre_markers[actualContainerId] = {{}};
                window.__dioxus_maplibre_sources[actualContainerId] = {{}};
                window.__dioxus_maplibre_layers[actualContainerId] = [];

                // Also store under the original map_id for lookups
                // (handles case where Dioxus remounts component with new ID)
                window.__dioxus_maplibre_maps['{map_id}'] = map;
                window.__dioxus_maplibre_markers['{map_id}'] = window.__dioxus_maplibre_markers[actualContainerId];
                window.__dioxus_maplibre_sources['{map_id}'] = window.__dioxus_maplibre_sources[actualContainerId];
                window.__dioxus_maplibre_layers['{map_id}'] = window.__dioxus_maplibre_layers[actualContainerId];

                // Set up global event sender for markers to use
                // (markers run in separate eval contexts and can't use dioxus.send directly)
                window.__dioxus_maplibre_sendEvent = function(eventJson) {{
                    dioxus.send(eventJson);
                }};

                // Set up click event listener
                map.on('click', function(e) {{
                    dioxus.send(JSON.stringify({{
                        type: 'click',
                        latlng: {{ lat: e.lngLat.lat, lng: e.lngLat.lng }},
                        point: {{ x: e.point.x, y: e.point.y }}
                    }}));
                }});

                // Set up moveend event listener for tracking position changes
                map.on('moveend', function() {{
                    const center = map.getCenter();
                    dioxus.send(JSON.stringify({{
                        type: 'move',
                        center: {{ lat: center.lat, lng: center.lng }},
                        zoom: map.getZoom()
                    }}));
                }});

                // Signal that map is ready
                map.on('load', function() {{
                    console.log('[dioxus-maplibre] Map loaded, sending ready event');
                    dioxus.send(JSON.stringify({{
                        type: 'ready'
                    }}));
                }});

                // Also handle error events
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

/// Generate JS to add a marker to the map
pub fn add_marker_js(
    map_id: &str,
    marker_id: &str,
    lat: f64,
    lng: f64,
    popup_html: Option<&str>,
    emoji: Option<&str>,
) -> String {
    let popup_code = match popup_html {
        Some(html) => format!(
            r#"
            const popup = new maplibregl.Popup({{ offset: 25 }})
                .setHTML(`{}`);
            marker.setPopup(popup);
            "#,
            html.replace('`', "\\`").replace('\\', "\\\\")
        ),
        None => String::new(),
    };

    let element_code = match emoji {
        Some(e) => format!(
            r#"
            const el = document.createElement('div');
            el.className = 'maplibre-marker-emoji';
            el.innerHTML = '{e}';
            el.style.fontSize = '28px';
            el.style.cursor = 'pointer';
            el.style.filter = 'drop-shadow(0 2px 4px rgba(0,0,0,0.5))';
            "#
        ),
        None => String::new(),
    };

    let marker_options = if emoji.is_some() {
        "{ element: el }"
    } else {
        "{ color: '#3b82f6' }"
    };

    format!(
        r#"
        (function() {{
            // Try exact map_id first, then fall back to any available map
            let map = window.__dioxus_maplibre_maps['{map_id}'];
            let actualMapId = '{map_id}';

            if (!map) {{
                // Fallback: find any map in the registry (handles ID mismatch from component remount)
                const mapKeys = Object.keys(window.__dioxus_maplibre_maps || {{}});
                if (mapKeys.length > 0) {{
                    actualMapId = mapKeys[0];
                    map = window.__dioxus_maplibre_maps[actualMapId];
                    console.log('[dioxus-maplibre] Marker using fallback map:', actualMapId);
                }}
            }}

            if (!map) {{
                console.error('[dioxus-maplibre] No map found for marker');
                return;
            }}

            {element_code}

            const marker = new maplibregl.Marker({marker_options})
                .setLngLat([{lng}, {lat}])
                .addTo(map);

            {popup_code}

            // Store marker reference (ensure markers object exists for this map)
            if (!window.__dioxus_maplibre_markers[actualMapId]) {{
                window.__dioxus_maplibre_markers[actualMapId] = {{}};
            }}
            window.__dioxus_maplibre_markers[actualMapId]['{marker_id}'] = marker;

            // Click handler for marker (use global sender since this runs in different eval context)
            marker.getElement().addEventListener('click', function(e) {{
                e.stopPropagation();
                console.log('[dioxus-maplibre] Marker clicked:', '{marker_id}');
                if (window.__dioxus_maplibre_sendEvent) {{
                    window.__dioxus_maplibre_sendEvent(JSON.stringify({{
                        type: 'marker_click',
                        marker_id: '{marker_id}',
                        latlng: {{ lat: {lat}, lng: {lng} }}
                    }}));
                }}
            }});

            // Mouseenter handler for hover preview
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

            // Mouseleave handler
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

/// Generate JS to remove a marker from the map
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

/// Generate JS to fly to a location
pub fn fly_to_js(map_id: &str, lat: f64, lng: f64, zoom: Option<f64>) -> String {
    let zoom_param = zoom.map(|z| format!(", zoom: {z}")).unwrap_or_default();
    format!(
        r#"
        (function() {{
            const map = window.__dioxus_maplibre_maps['{map_id}'];
            if (map) {{
                map.flyTo({{
                    center: [{lng}, {lat}],
                    essential: true{zoom_param}
                }});
            }}
        }})();
        "#
    )
}

/// Generate JS to pan the map by pixel offset (instant, no animation)
pub fn pan_by_js(x: i32, y: i32) -> String {
    format!(
        r#"
        (function() {{
            // Find any available map
            const mapKeys = Object.keys(window.__dioxus_maplibre_maps || {{}});
            if (mapKeys.length > 0) {{
                const map = window.__dioxus_maplibre_maps[mapKeys[0]];
                if (map) {{
                    map.panBy([{x}, {y}], {{ duration: 0 }});
                }}
            }}
        }})();
        "#
    )
}

/// Generate JS to destroy a map and clean up resources
pub fn destroy_map_js(map_id: &str) -> String {
    format!(
        r#"
        (function() {{
            // Remove all markers first
            const markers = window.__dioxus_maplibre_markers['{map_id}'];
            if (markers) {{
                Object.values(markers).forEach(marker => marker.remove());
                delete window.__dioxus_maplibre_markers['{map_id}'];
            }}

            // Remove the map
            const map = window.__dioxus_maplibre_maps['{map_id}'];
            if (map) {{
                map.remove();
                delete window.__dioxus_maplibre_maps['{map_id}'];
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
// GeoJSON Source and Layer Functions
// =============================================================================

/// Generate JS to add a GeoJSON source to the map
pub fn add_geojson_source_js(map_id: &str, source_id: &str, geojson: &str) -> String {
    format!(
        r#"
        (function() {{
            let map = window.__dioxus_maplibre_maps['{map_id}'];
            if (!map) {{
                const mapKeys = Object.keys(window.__dioxus_maplibre_maps || {{}});
                if (mapKeys.length > 0) {{
                    map = window.__dioxus_maplibre_maps[mapKeys[0]];
                }}
            }}
            if (!map) {{
                console.error('[dioxus-maplibre] No map found for source');
                return;
            }}

            // Check if source already exists
            if (map.getSource('{source_id}')) {{
                console.log('[dioxus-maplibre] Source already exists:', '{source_id}');
                return;
            }}

            try {{
                const data = {geojson};
                map.addSource('{source_id}', {{
                    type: 'geojson',
                    data: data
                }});
                console.log(`[dioxus-maplibre] Added GeoJSON source: {source_id} with ${{data.features?.length || 0}} features`);

                // Track in registry
                if (!window.__dioxus_maplibre_sources['{map_id}']) {{
                    window.__dioxus_maplibre_sources['{map_id}'] = {{}};
                }}
                window.__dioxus_maplibre_sources['{map_id}']['{source_id}'] = true;
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add source:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to update a GeoJSON source's data
pub fn update_geojson_source_js(map_id: &str, source_id: &str, geojson: &str) -> String {
    format!(
        r#"
        (function() {{
            let map = window.__dioxus_maplibre_maps['{map_id}'];
            if (!map) {{
                const mapKeys = Object.keys(window.__dioxus_maplibre_maps || {{}});
                if (mapKeys.length > 0) {{
                    map = window.__dioxus_maplibre_maps[mapKeys[0]];
                }}
            }}
            if (!map) return;

            const source = map.getSource('{source_id}');
            if (source) {{
                try {{
                    const data = {geojson};
                    source.setData(data);
                    console.log(`[dioxus-maplibre] Updated source: {source_id} with ${{data.features?.length || 0}} features`);
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
    format!(
        r#"
        (function() {{
            let map = window.__dioxus_maplibre_maps['{map_id}'];
            if (!map) {{
                const mapKeys = Object.keys(window.__dioxus_maplibre_maps || {{}});
                if (mapKeys.length > 0) {{
                    map = window.__dioxus_maplibre_maps[mapKeys[0]];
                }}
            }}
            if (!map) return;

            try {{
                if (map.getSource('{source_id}')) {{
                    map.removeSource('{source_id}');
                    console.log('[dioxus-maplibre] Removed source:', '{source_id}');
                }}
                // Clean up registry
                if (window.__dioxus_maplibre_sources['{map_id}']) {{
                    delete window.__dioxus_maplibre_sources['{map_id}']['{source_id}'];
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to remove source:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to add a layer to the map with click and hover event handlers
pub fn add_layer_js(
    map_id: &str,
    layer_id: &str,
    layer_type: &str,
    source_id: &str,
    paint: &str,
    layout: &str,
) -> String {
    format!(
        r#"
        (function() {{
            let map = window.__dioxus_maplibre_maps['{map_id}'];
            if (!map) {{
                const mapKeys = Object.keys(window.__dioxus_maplibre_maps || {{}});
                if (mapKeys.length > 0) {{
                    map = window.__dioxus_maplibre_maps[mapKeys[0]];
                }}
            }}
            if (!map) {{
                console.error('[dioxus-maplibre] No map found for layer');
                return;
            }}

            // Check if layer already exists
            if (map.getLayer('{layer_id}')) {{
                console.log('[dioxus-maplibre] Layer already exists:', '{layer_id}');
                return;
            }}

            // Wait for source to be available
            if (!map.getSource('{source_id}')) {{
                console.error('[dioxus-maplibre] Source not found for layer:', '{source_id}');
                return;
            }}

            try {{
                map.addLayer({{
                    id: '{layer_id}',
                    type: '{layer_type}',
                    source: '{source_id}',
                    paint: {paint},
                    layout: {layout}
                }});
                console.log(`[dioxus-maplibre] Added layer: {layer_id} type: {layer_type}`);

                // Track in registry
                if (!window.__dioxus_maplibre_layers['{map_id}']) {{
                    window.__dioxus_maplibre_layers['{map_id}'] = [];
                }}
                window.__dioxus_maplibre_layers['{map_id}'].push('{layer_id}');

                // Attach click handler
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

                // Attach hover handlers
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

            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add layer:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to remove a layer from the map
pub fn remove_layer_js(map_id: &str, layer_id: &str) -> String {
    format!(
        r#"
        (function() {{
            let map = window.__dioxus_maplibre_maps['{map_id}'];
            if (!map) {{
                const mapKeys = Object.keys(window.__dioxus_maplibre_maps || {{}});
                if (mapKeys.length > 0) {{
                    map = window.__dioxus_maplibre_maps[mapKeys[0]];
                }}
            }}
            if (!map) return;

            try {{
                if (map.getLayer('{layer_id}')) {{
                    // Remove event listeners first (MapLibre handles this automatically on removeLayer)
                    map.removeLayer('{layer_id}');
                    console.log('[dioxus-maplibre] Removed layer:', '{layer_id}');
                }}
                // Clean up registry
                if (window.__dioxus_maplibre_layers['{map_id}']) {{
                    const idx = window.__dioxus_maplibre_layers['{map_id}'].indexOf('{layer_id}');
                    if (idx > -1) {{
                        window.__dioxus_maplibre_layers['{map_id}'].splice(idx, 1);
                    }}
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to remove layer:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to update a layer's paint properties
pub fn update_layer_paint_js(map_id: &str, layer_id: &str, property: &str, value: &str) -> String {
    format!(
        r#"
        (function() {{
            let map = window.__dioxus_maplibre_maps['{map_id}'];
            if (!map) {{
                const mapKeys = Object.keys(window.__dioxus_maplibre_maps || {{}});
                if (mapKeys.length > 0) {{
                    map = window.__dioxus_maplibre_maps[mapKeys[0]];
                }}
            }}
            if (!map) return;

            try {{
                if (map.getLayer('{layer_id}')) {{
                    map.setPaintProperty('{layer_id}', '{property}', {value});
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to update layer paint:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to set feature state (for hover highlighting, etc.)
pub fn set_feature_state_js(
    map_id: &str,
    source_id: &str,
    feature_id: i64,
    state: &str,
) -> String {
    format!(
        r#"
        (function() {{
            let map = window.__dioxus_maplibre_maps['{map_id}'];
            if (!map) {{
                const mapKeys = Object.keys(window.__dioxus_maplibre_maps || {{}});
                if (mapKeys.length > 0) {{
                    map = window.__dioxus_maplibre_maps[mapKeys[0]];
                }}
            }}
            if (!map) return;

            try {{
                map.setFeatureState(
                    {{ source: '{source_id}', id: {feature_id} }},
                    {state}
                );
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to set feature state:', err);
            }}
        }})();
        "#
    )
}
