//! Camera, projection, interaction, and map-state JS bridge.

use super::find_map_js;
use super::js_escape::js_single_quoted;

fn call_number_setter_js(map_id: &str, method: &str, value: f64) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.{method}({value}); }})();")
}

fn call_number_getter_js(map_id: &str, method: &str) -> String {
    let find = find_map_js(map_id);
    format!("{find} return map.{method}();")
}

pub fn set_center_js(map_id: &str, lat: f64, lng: f64) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.setCenter([{lng}, {lat}]); }})();")
}

pub fn set_zoom_js(map_id: &str, value: f64) -> String {
    call_number_setter_js(map_id, "setZoom", value)
}

pub fn set_bearing_js(map_id: &str, value: f64) -> String {
    call_number_setter_js(map_id, "setBearing", value)
}

pub fn set_roll_js(map_id: &str, value: f64) -> String {
    call_number_setter_js(map_id, "setRoll", value)
}

pub fn set_center_elevation_js(map_id: &str, value: f64) -> String {
    call_number_setter_js(map_id, "setCenterElevation", value)
}

pub fn set_field_of_view_js(map_id: &str, value: f64) -> String {
    call_number_setter_js(map_id, "setVerticalFieldOfView", value)
}

pub fn set_min_zoom_js(map_id: &str, value: f64) -> String {
    call_number_setter_js(map_id, "setMinZoom", value)
}

pub fn set_max_zoom_js(map_id: &str, value: f64) -> String {
    call_number_setter_js(map_id, "setMaxZoom", value)
}

pub fn set_min_pitch_js(map_id: &str, value: f64) -> String {
    call_number_setter_js(map_id, "setMinPitch", value)
}

pub fn set_max_pitch_js(map_id: &str, value: f64) -> String {
    call_number_setter_js(map_id, "setMaxPitch", value)
}

pub fn get_roll_js(map_id: &str) -> String {
    call_number_getter_js(map_id, "getRoll")
}

pub fn get_center_elevation_js(map_id: &str) -> String {
    call_number_getter_js(map_id, "getCenterElevation")
}

pub fn get_field_of_view_js(map_id: &str) -> String {
    call_number_getter_js(map_id, "getVerticalFieldOfView")
}

pub fn get_min_zoom_js(map_id: &str) -> String {
    call_number_getter_js(map_id, "getMinZoom")
}

pub fn get_max_zoom_js(map_id: &str) -> String {
    call_number_getter_js(map_id, "getMaxZoom")
}

pub fn get_min_pitch_js(map_id: &str) -> String {
    call_number_getter_js(map_id, "getMinPitch")
}

pub fn get_max_pitch_js(map_id: &str) -> String {
    call_number_getter_js(map_id, "getMaxPitch")
}

pub fn set_projection_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.setProjection({options_json}); }})();")
}

pub fn get_projection_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!("{find} return map.getProjection() ?? null;")
}

pub fn set_center_clamped_to_ground_js(map_id: &str, enabled: bool) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.setCenterClampedToGround({enabled}); }})();")
}

pub fn get_center_clamped_to_ground_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!("{find} return map.getCenterClampedToGround();")
}

pub fn set_render_world_copies_js(map_id: &str, enabled: bool) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.setRenderWorldCopies({enabled}); }})();")
}

pub fn get_render_world_copies_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!("{find} return map.getRenderWorldCopies();")
}

pub fn set_max_bounds_js(map_id: &str, bounds_json: &str) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.setMaxBounds({bounds_json}); }})();")
}

pub fn project_js(map_id: &str, lat: f64, lng: f64) -> String {
    let find = find_map_js(map_id);
    format!(
        "{find} const point = map.project([{lng}, {lat}]); return {{ x: point.x, y: point.y }};"
    )
}

pub fn unproject_js(map_id: &str, x: f64, y: f64) -> String {
    let find = find_map_js(map_id);
    format!(
        "{find} const coordinate = map.unproject([{x}, {y}]); return {{ lat: coordinate.lat, lng: coordinate.lng }};"
    )
}

pub fn query_terrain_elevation_js(map_id: &str, lat: f64, lng: f64) -> String {
    let find = find_map_js(map_id);
    format!("{find} return map.queryTerrainElevation([{lng}, {lat}]) ?? null;")
}

pub fn set_global_state_property_js(map_id: &str, name: &str, value_json: &str) -> String {
    let find = find_map_js(map_id);
    let name_lit = js_single_quoted(name);
    format!("(function() {{ {find} map.setGlobalStateProperty({name_lit}, {value_json}); }})();")
}

pub fn get_global_state_property_js(map_id: &str, name: &str) -> String {
    let find = find_map_js(map_id);
    let name_lit = js_single_quoted(name);
    format!("{find} return map.getGlobalStateProperty({name_lit}) ?? null;")
}

pub fn set_interaction_enabled_js(map_id: &str, property: &str, enabled: bool) -> String {
    let find = find_map_js(map_id);
    let property_lit = js_single_quoted(property);
    let action = if enabled { "enable" } else { "disable" };
    format!(
        r#"(function() {{
            {find}
            const handler = map[{property_lit}];
            if (handler && typeof handler.{action} === 'function') handler.{action}();
        }})();"#
    )
}

pub fn is_interaction_enabled_js(map_id: &str, property: &str) -> String {
    let find = find_map_js(map_id);
    let property_lit = js_single_quoted(property);
    format!(
        "{find} const handler = map[{property_lit}]; return !!(handler && handler.isEnabled());"
    )
}

pub fn resize_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.resize(); }})();")
}

pub fn stop_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.stop(); }})();")
}

pub fn reset_north_pitch_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.resetNorthPitch(); }})();")
}

pub fn trigger_repaint_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!("(function() {{ {find} map.triggerRepaint(); }})();")
}
