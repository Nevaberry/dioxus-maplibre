//! Shared helpers for JS bridge modules.

use uuid::Uuid;

use super::js_escape::js_single_quoted;

/// Generate a unique map ID.
pub fn generate_map_id() -> String {
    format!("map_{}", Uuid::new_v4().to_string().replace('-', ""))
}

/// JS snippet to find a map by ID with fallback to any available map.
pub(crate) fn find_map_js(map_id: &str) -> String {
    let map_id_lit = js_single_quoted(map_id);
    format!(
        r#"let mapRegistry = window.__dioxus_maplibre_maps || {{}};
            let map = mapRegistry[{map_id_lit}];
            if (!map) {{
                const mapKeys = Object.keys(mapRegistry);
                if (mapKeys.length === 1) {{
                    map = mapRegistry[mapKeys[0]];
                }} else {{
                    console.error('[dioxus-maplibre] map id not found', {map_id_lit}, 'available maps:', mapKeys);
                    return;
                }}
            }}"#
    )
}

#[cfg(test)]
mod tests {
    use super::find_map_js;

    #[test]
    fn find_map_js_escapes_map_id_literal() {
        let js = find_map_js("map'with-quote");
        assert!(js.contains("'map\\'with-quote'"));
        assert!(!js.contains("map'with-quote"));
    }
}
