//! Terrain, sky, and fog JS bridge.

use super::find_map_js;
pub fn set_terrain_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setTerrain({options_json});
            const terrainState = window.__dioxus_maplibre_terrain && window.__dioxus_maplibre_terrain['{map_id}'];
            if (terrainState) {{
                terrainState.hasValue = true;
                terrainState.value = JSON.parse(JSON.stringify({options_json}));
            }}
        }})();
        "#
    )
}

/// Generate JS to remove terrain
pub fn remove_terrain_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setTerrain(null);
            const terrainState = window.__dioxus_maplibre_terrain && window.__dioxus_maplibre_terrain['{map_id}'];
            if (terrainState) {{
                terrainState.hasValue = true;
                terrainState.value = null;
            }}
        }})();
        "#
    )
}

/// Generate JS to set sky
pub fn set_sky_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setSky({options_json});
            const skyState = window.__dioxus_maplibre_sky && window.__dioxus_maplibre_sky['{map_id}'];
            if (skyState) {{
                skyState.hasValue = true;
                skyState.value = JSON.parse(JSON.stringify({options_json}));
            }}
        }})();
        "#
    )
}

/// Generate JS to remove sky
pub fn remove_sky_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setSky(null);
            const skyState = window.__dioxus_maplibre_sky && window.__dioxus_maplibre_sky['{map_id}'];
            if (skyState) {{
                skyState.hasValue = true;
                skyState.value = null;
            }}
        }})();
        "#
    )
}

// =============================================================================
// Fog / Atmosphere
// =============================================================================

/// Generate JS to set fog/atmosphere
pub fn set_fog_js(map_id: &str, options_json: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setFog({options_json});
            const fogState = window.__dioxus_maplibre_fog && window.__dioxus_maplibre_fog['{map_id}'];
            if (fogState) {{
                fogState.hasValue = true;
                fogState.value = JSON.parse(JSON.stringify({options_json}));
            }}
        }})();
        "#
    )
}

/// Generate JS to remove fog/atmosphere
pub fn remove_fog_js(map_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            map.setFog(null);
            const fogState = window.__dioxus_maplibre_fog && window.__dioxus_maplibre_fog['{map_id}'];
            if (fogState) {{
                fogState.hasValue = true;
                fogState.value = null;
            }}
        }})();
        "#
    )
}
