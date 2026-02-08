//! Navigation and camera JS bridge.

use super::find_map_js;
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
