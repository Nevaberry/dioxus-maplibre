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

pub fn add_navigation_control_with_options_js(
    map_id: &str,
    position: &str,
    options_json: &str,
) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("navigation", position),
        &format!("new maplibregl.NavigationControl({options_json})"),
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

pub fn add_geolocate_control_with_options_js(
    map_id: &str,
    position: &str,
    options_json: &str,
) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("geolocate", position),
        &format!("new maplibregl.GeolocateControl({options_json})"),
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

pub fn add_scale_control_with_options_js(
    map_id: &str,
    position: &str,
    options_json: &str,
) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("scale", position),
        &format!("new maplibregl.ScaleControl({options_json})"),
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

pub fn add_fullscreen_control_with_options_js(
    map_id: &str,
    position: &str,
    options_json: &str,
) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("fullscreen", position),
        &format!("new maplibregl.FullscreenControl({options_json})"),
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

pub fn add_attribution_control_with_options_js(
    map_id: &str,
    position: &str,
    options_json: &str,
) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("attribution", position),
        &format!("new maplibregl.AttributionControl({options_json})"),
    )
}

/// Generate JS to remove an attribution control
pub fn remove_attribution_control_js(map_id: &str, position: &str) -> String {
    remove_control_js(map_id, &control_key("attribution", position))
}

/// Generate JS to add MapLibre's globe projection toggle.
pub fn add_globe_control_js(map_id: &str, position: &str) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("globe", position),
        "new maplibregl.GlobeControl()",
    )
}

pub fn remove_globe_control_js(map_id: &str, position: &str) -> String {
    remove_control_js(map_id, &control_key("globe", position))
}

/// Generate JS to add MapLibre's optional logo control.
pub fn add_logo_control_js(map_id: &str, position: &str, options_json: &str) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("logo", position),
        &format!("new maplibregl.LogoControl({options_json})"),
    )
}

pub fn remove_logo_control_js(map_id: &str, position: &str) -> String {
    remove_control_js(map_id, &control_key("logo", position))
}

/// Generate JS to add MapLibre's terrain toggle control.
pub fn add_terrain_control_js(map_id: &str, position: &str, options_json: &str) -> String {
    add_control_js(
        map_id,
        position,
        &control_key("terrain", position),
        &format!("new maplibregl.TerrainControl({options_json})"),
    )
}

pub fn remove_terrain_control_js(map_id: &str, position: &str) -> String {
    remove_control_js(map_id, &control_key("terrain", position))
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
