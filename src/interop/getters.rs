//! Getter JS bridge methods returning values.

use super::find_map_js;
pub fn get_zoom_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        {find}
        return map.getZoom();
        "#
    )
}

/// Generate JS to get center as {{ lat, lng }}
pub fn get_center_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        {find}
        const c = map.getCenter();
        return {{ lat: c.lat, lng: c.lng }};
        "#
    )
}

/// Generate JS to get bearing
pub fn get_bearing_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        {find}
        return map.getBearing();
        "#
    )
}

/// Generate JS to get pitch
pub fn get_pitch_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        {find}
        return map.getPitch();
        "#
    )
}

/// Generate JS to get bounds as {{ sw: {{ lat, lng }}, ne: {{ lat, lng }} }}
pub fn get_bounds_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        {find}
        const b = map.getBounds();
        return {{
            sw: {{ lat: b.getSouth(), lng: b.getWest() }},
            ne: {{ lat: b.getNorth(), lng: b.getEast() }}
        }};
        "#
    )
}
