//! Image sprite JS bridge.

use super::find_map_js;
pub fn load_image_js(map_id: &str, image_id: &str, url: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (async function() {{
            {find}
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images['{map_id}'];
            if (imageRegistry) {{
                imageRegistry['{image_id}'] = '{url}';
            }}
            try {{
                const response = await map.loadImage('{url}');
                if (!map.hasImage('{image_id}')) {{
                    map.addImage('{image_id}', response.data);
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to load image:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to check if an image exists (returns boolean via dioxus.send)
pub fn has_image_js(map_id: &str, image_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            return map.hasImage('{image_id}');
        }})();
        "#
    )
}

/// Generate JS to load an image and return success via dioxus.send
pub fn load_image_async_js(map_id: &str, image_id: &str, url: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (async function() {{
            {find}
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images['{map_id}'];
            if (imageRegistry) {{
                imageRegistry['{image_id}'] = '{url}';
            }}
            try {{
                const response = await map.loadImage('{url}');
                if (!map.hasImage('{image_id}')) {{
                    map.addImage('{image_id}', response.data);
                }}
                return true;
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to load image:', err);
                return false;
            }}
        }})();
        "#
    )
}

/// Generate JS to remove an image from the map's sprite
pub fn remove_image_js(map_id: &str, image_id: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            if (map.hasImage('{image_id}')) {{
                map.removeImage('{image_id}');
            }}
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images['{map_id}'];
            if (imageRegistry) {{
                delete imageRegistry['{image_id}'];
            }}
        }})();
        "#
    )
}
