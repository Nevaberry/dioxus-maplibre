//! Control management JS bridge.

use super::find_map_js;
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
