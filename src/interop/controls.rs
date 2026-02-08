//! Control management JS bridge.

use super::find_map_js;
use super::js_escape::js_single_quoted;

fn control_key(kind: &str, position: &str) -> String {
    format!("{kind}:{position}")
}

/// Generate JS to add a navigation control
pub fn add_navigation_control_js(map_id: &str, position: &str) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("navigation", position),
        "new maplibregl.NavigationControl()",
    )
}

/// Generate JS to remove a navigation control
pub fn remove_navigation_control_js(map_id: &str, position: &str) -> String {
    remove_control_js(map_id, &control_key("navigation", position))
}

/// Generate JS to add a geolocate control
pub fn add_geolocate_control_js(map_id: &str, position: &str) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("geolocate", position),
        "new maplibregl.GeolocateControl({ positionOptions: { enableHighAccuracy: true }, trackUserLocation: true })",
    )
}

/// Generate JS to remove a geolocate control
pub fn remove_geolocate_control_js(map_id: &str, position: &str) -> String {
    remove_control_js(map_id, &control_key("geolocate", position))
}

/// Generate JS to add a scale control
pub fn add_scale_control_js(map_id: &str, position: &str) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("scale", position),
        "new maplibregl.ScaleControl()",
    )
}

/// Generate JS to remove a scale control
pub fn remove_scale_control_js(map_id: &str, position: &str) -> String {
    remove_control_js(map_id, &control_key("scale", position))
}

/// Generate JS to add a fullscreen control
pub fn add_fullscreen_control_js(map_id: &str, position: &str) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("fullscreen", position),
        "new maplibregl.FullscreenControl()",
    )
}

/// Generate JS to remove a fullscreen control
pub fn remove_fullscreen_control_js(map_id: &str, position: &str) -> String {
    remove_control_js(map_id, &control_key("fullscreen", position))
}

/// Generate JS to add an attribution control
pub fn add_attribution_control_js(map_id: &str, position: &str) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("attribution", position),
        "new maplibregl.AttributionControl({ compact: true })",
    )
}

/// Generate JS to remove an attribution control
pub fn remove_attribution_control_js(map_id: &str, position: &str) -> String {
    remove_control_js(map_id, &control_key("attribution", position))
}

fn add_control_js(map_id: &str, position: &str, control_key: &str, control_ctor: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let position_lit = js_single_quoted(position);
    let control_key_lit = js_single_quoted(control_key);
    format!(
        r#"
        (function() {{
            {find}
            const controlsRegistry = window.__dioxus_maplibre_controls && window.__dioxus_maplibre_controls[{map_id_lit}];
            if (!controlsRegistry) {{
                return;
            }}
            const existing = controlsRegistry[{control_key_lit}];
            if (existing) {{
                try {{
                    map.removeControl(existing);
                }} catch (_err) {{}}
            }}
            const control = {control_ctor};
            map.addControl(control, {position_lit});
            controlsRegistry[{control_key_lit}] = control;
        }})();
        "#
    )
}

fn remove_control_js(map_id: &str, control_key: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let control_key_lit = js_single_quoted(control_key);
    format!(
        r#"
        (function() {{
            {find}
            const controlsRegistry = window.__dioxus_maplibre_controls && window.__dioxus_maplibre_controls[{map_id_lit}];
            if (!controlsRegistry) {{
                return;
            }}
            const control = controlsRegistry[{control_key_lit}];
            if (!control) {{
                return;
            }}
            try {{
                map.removeControl(control);
            }} catch (_err) {{}}
            delete controlsRegistry[{control_key_lit}];
        }})();
        "#
    )
}
