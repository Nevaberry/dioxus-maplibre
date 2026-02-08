//! Viewport padding JS bridge.

use super::find_map_js;
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
