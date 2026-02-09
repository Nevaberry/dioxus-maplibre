//! Image sprite JS bridge.

use super::find_map_js;
use super::js_escape::js_single_quoted;

pub fn load_image_js(map_id: &str, image_id: &str, url: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let image_id_lit = js_single_quoted(image_id);
    let url_lit = js_single_quoted(url);
    format!(
        r#"
        (async function() {{
            {find}
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images[{map_id_lit}];
            if (imageRegistry) {{
                imageRegistry[{image_id_lit}] = {url_lit};
            }}
            try {{
                const response = await map.loadImage({url_lit});
                if (!map.hasImage({image_id_lit})) {{
                    map.addImage({image_id_lit}, response.data);
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
    let image_id_lit = js_single_quoted(image_id);
    format!(
        r#"
        {find}
        return map.hasImage({image_id_lit});
        "#
    )
}

/// Generate JS to load an image and return success via dioxus.send
pub fn load_image_async_js(map_id: &str, image_id: &str, url: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let image_id_lit = js_single_quoted(image_id);
    let url_lit = js_single_quoted(url);
    format!(
        r#"
        {find}
        const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images[{map_id_lit}];
        if (imageRegistry) {{
            imageRegistry[{image_id_lit}] = {url_lit};
        }}
        try {{
            const response = await map.loadImage({url_lit});
            if (!map.hasImage({image_id_lit})) {{
                map.addImage({image_id_lit}, response.data);
            }}
            return true;
        }} catch (err) {{
            console.error('[dioxus-maplibre] Failed to load image:', err);
            return false;
        }}
        "#
    )
}

/// Generate JS to remove an image from the map's sprite
pub fn remove_image_js(map_id: &str, image_id: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let image_id_lit = js_single_quoted(image_id);
    format!(
        r#"
        (function() {{
            {find}
            if (map.hasImage({image_id_lit})) {{
                map.removeImage({image_id_lit});
            }}
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images[{map_id_lit}];
            if (imageRegistry) {{
                delete imageRegistry[{image_id_lit}];
            }}
        }})();
        "#
    )
}
