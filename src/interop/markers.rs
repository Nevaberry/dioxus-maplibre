//! Marker management JS bridge.

use super::find_map_js;
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
