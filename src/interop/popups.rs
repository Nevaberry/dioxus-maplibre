//! Popup management JS bridge.

use super::find_map_js;
use super::js_escape::{js_single_quoted, js_template_literal};
pub fn add_popup_js(
    map_id: &str,
    popup_id: &str,
    lat: f64,
    lng: f64,
    html: &str,
    options_json: &str,
) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let popup_id_lit = js_single_quoted(popup_id);
    let escaped_html = js_template_literal(html);
    format!(
        r#"
        (function() {{
            {find}
            const opts = {options_json};
            const popupOpts = {{}};
            if (opts.offset) popupOpts.offset = opts.offset;
            if (opts.anchor) popupOpts.anchor = opts.anchor;
            if (opts.closeButton != null) popupOpts.closeButton = opts.closeButton;
            if (opts.closeOnClick != null) popupOpts.closeOnClick = opts.closeOnClick;
            if (opts.maxWidth) popupOpts.maxWidth = opts.maxWidth;
            if (opts.className) popupOpts.className = opts.className;

            const popup = new maplibregl.Popup(popupOpts)
                .setLngLat([{lng}, {lat}])
                .setHTML(`{escaped_html}`)
                .addTo(map);

            // Store popup reference
            if (!window.__dioxus_maplibre_popups) window.__dioxus_maplibre_popups = {{}};
            if (!window.__dioxus_maplibre_popups[{map_id_lit}]) window.__dioxus_maplibre_popups[{map_id_lit}] = {{}};
            window.__dioxus_maplibre_popups[{map_id_lit}][{popup_id_lit}] = popup;
        }})();
        "#
    )
}

/// Generate JS to remove a popup
pub fn remove_popup_js(map_id: &str, popup_id: &str) -> String {
    let map_id_lit = js_single_quoted(map_id);
    let popup_id_lit = js_single_quoted(popup_id);
    format!(
        r#"
        (function() {{
            const popups = window.__dioxus_maplibre_popups && window.__dioxus_maplibre_popups[{map_id_lit}];
            if (popups && popups[{popup_id_lit}]) {{
                popups[{popup_id_lit}].remove();
                delete popups[{popup_id_lit}];
            }}
        }})();
        "#
    )
}

#[cfg(test)]
mod tests {
    use super::add_popup_js;

    #[test]
    fn add_popup_js_escapes_html_and_ids() {
        let js = add_popup_js("map'1", "popup'1", 60.0, 24.0, "<div>${`x`}</div>", "{}");
        assert!(js.contains("'map\\'1'"));
        assert!(js.contains("'popup\\'1'"));
        assert!(js.contains("<div>\\${\\`x\\`}</div>"));
    }
}
