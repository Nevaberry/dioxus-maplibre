//! Source management JS bridge.

use super::find_map_js;
use super::js_escape::js_single_quoted;

/// Generate JS to add a GeoJSON source
pub fn add_geojson_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "geojson", options_json)
}

/// Generate JS to add a vector tile source
pub fn add_vector_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "vector", options_json)
}

/// Generate JS to add a raster tile source
pub fn add_raster_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "raster", options_json)
}

/// Generate JS to add a raster DEM source
pub fn add_raster_dem_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "raster-dem", options_json)
}

/// Generate JS to add an image source
pub fn add_image_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "image", options_json)
}

/// Generate JS to add a video source.
pub fn add_video_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "video", options_json)
}

/// Generate JS to add a canvas source.
pub fn add_canvas_source_js(map_id: &str, source_id: &str, options_json: &str) -> String {
    add_source_js(map_id, source_id, "canvas", options_json)
}

/// Generate JS to add any MapLibre source type.
pub fn add_source_js(
    map_id: &str,
    source_id: &str,
    source_type: &str,
    options_json: &str,
) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let source_id_lit = js_single_quoted(source_id);
    let source_type_lit = js_single_quoted(source_type);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                const opts = {options_json};
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources[{map_id_lit}];
                if (sourceRegistry) {{
                    sourceRegistry[{source_id_lit}] = {{
                        type: {source_type_lit},
                        options: JSON.parse(JSON.stringify(opts))
                    }};
                }}
                if (map.getSource({source_id_lit})) return;
                map.addSource({source_id_lit}, {{
                    type: {source_type_lit},
                    ...opts
                }});
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to add source:', err);
            }}
        }})();
        "#
    )
}

/// Generate JS to update a GeoJSON source's data
pub fn update_geojson_source_js(map_id: &str, source_id: &str, data_json: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        r#"
        (function() {{
            {find}
            const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources[{map_id_lit}];
            if (sourceRegistry && sourceRegistry[{source_id_lit}]) {{
                sourceRegistry[{source_id_lit}].options = sourceRegistry[{source_id_lit}].options || {{}};
                sourceRegistry[{source_id_lit}].options.data = {data_json};
            }}
            const source = map.getSource({source_id_lit});
            if (source) {{
                try {{
                    source.setData({data_json});
                }} catch (err) {{
                    console.error('[dioxus-maplibre] Failed to update source:', err);
                }}
            }}
        }})();
        "#
    )
}

/// Generate JS to remove a source from the map
pub fn remove_source_js(map_id: &str, source_id: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        r#"
        (function() {{
            {find}
            try {{
                if (map.getSource({source_id_lit})) {{
                    map.removeSource({source_id_lit});
                }}
                const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources[{map_id_lit}];
                if (sourceRegistry) {{
                    delete sourceRegistry[{source_id_lit}];
                }}
            }} catch (err) {{
                console.error('[dioxus-maplibre] Failed to remove source:', err);
            }}
        }})();
        "#
    )
}

pub fn update_geojson_source_diff_js(map_id: &str, source_id: &str, diff_json: &str) -> String {
    let find = find_map_js(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        r#"
        (async function() {{
            {find}
            const source = map.getSource({source_id_lit});
            if (source && typeof source.updateData === 'function') {{
                await source.updateData({diff_json});
            }}
        }})();
        "#
    )
}

pub fn get_geojson_source_data_js(map_id: &str, source_id: &str) -> String {
    let find = find_map_js(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        "{find} const source = map.getSource({source_id_lit}); return source && source.getData ? await source.getData() : null;"
    )
}

pub fn get_cluster_expansion_zoom_js(map_id: &str, source_id: &str, cluster_id: u64) -> String {
    let find = find_map_js(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        "{find} const source = map.getSource({source_id_lit}); return source && source.getClusterExpansionZoom ? await source.getClusterExpansionZoom({cluster_id}) : null;"
    )
}

pub fn get_cluster_children_js(map_id: &str, source_id: &str, cluster_id: u64) -> String {
    let find = find_map_js(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        "{find} const source = map.getSource({source_id_lit}); return source && source.getClusterChildren ? await source.getClusterChildren({cluster_id}) : [];"
    )
}

pub fn get_cluster_leaves_js(
    map_id: &str,
    source_id: &str,
    cluster_id: u64,
    limit: u32,
    offset: u32,
) -> String {
    let find = find_map_js(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        "{find} const source = map.getSource({source_id_lit}); return source && source.getClusterLeaves ? await source.getClusterLeaves({cluster_id}, {limit}, {offset}) : [];"
    )
}

fn set_registry_source_option(
    map_id_lit: &str,
    source_id_lit: &str,
    key: &str,
    value: &str,
) -> String {
    let key_lit = js_single_quoted(key);
    format!(
        r#"const registry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources[{map_id_lit}];
            if (registry && registry[{source_id_lit}]) {{
                registry[{source_id_lit}].options[{key_lit}] = {value};
            }}"#
    )
}

pub fn set_source_coordinates_js(map_id: &str, source_id: &str, coordinates_json: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let source_id_lit = js_single_quoted(source_id);
    let update_registry =
        set_registry_source_option(&map_id_lit, &source_id_lit, "coordinates", coordinates_json);
    format!(
        r#"(function() {{
            {find}
            {update_registry}
            const source = map.getSource({source_id_lit});
            if (source && source.setCoordinates) source.setCoordinates({coordinates_json});
        }})();"#
    )
}

pub fn set_source_tiles_js(map_id: &str, source_id: &str, tiles_json: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let source_id_lit = js_single_quoted(source_id);
    let update_registry =
        set_registry_source_option(&map_id_lit, &source_id_lit, "tiles", tiles_json);
    format!(
        r#"(function() {{
            {find}
            {update_registry}
            const source = map.getSource({source_id_lit});
            if (source && source.setTiles) source.setTiles({tiles_json});
        }})();"#
    )
}

pub fn set_source_url_js(map_id: &str, source_id: &str, url: &str) -> String {
    let find = find_map_js(map_id);
    let map_id_lit = js_single_quoted(map_id);
    let source_id_lit = js_single_quoted(source_id);
    let url_lit = js_single_quoted(url);
    let update_registry = set_registry_source_option(&map_id_lit, &source_id_lit, "url", &url_lit);
    format!(
        r#"(function() {{
            {find}
            {update_registry}
            const source = map.getSource({source_id_lit});
            if (source && source.setUrl) source.setUrl({url_lit});
        }})();"#
    )
}

pub fn set_video_playing_js(map_id: &str, source_id: &str, playing: bool) -> String {
    let find = find_map_js(map_id);
    let source_id_lit = js_single_quoted(source_id);
    let operation = if playing {
        r#"
            if (!source) return;
            const video = typeof source.getVideo === 'function' ? source.getVideo() : null;
            if (video) {
                const result = video.play();
                if (result && typeof result.catch === 'function') result.catch(() => {});
            } else if (typeof source.play === 'function') {
                source.play();
            }
        "#
    } else {
        r#"
            if (!source) return;
            const video = typeof source.getVideo === 'function' ? source.getVideo() : null;
            if (video && !video.paused) {
                // A VideoSource starts playback while it is loading. Waiting for that
                // play promise prevents an immediate pause from becoming an unhandled
                // AbortError in browsers.
                const result = video.play();
                if (result && typeof result.then === 'function') {
                    result.then(() => video.pause(), () => video.pause());
                } else {
                    video.pause();
                }
            } else if (typeof source.pause === 'function') {
                source.pause();
            }
        "#
    };
    format!(
        r#"(function() {{
            {find}
            const source = map.getSource({source_id_lit});
            {operation}
        }})();"#
    )
}

pub fn set_raster_premultiply_alpha_js(map_id: &str, source_id: &str, enabled: bool) -> String {
    let find = find_map_js(map_id);
    let source_id_lit = js_single_quoted(source_id);
    format!(
        "(function() {{ {find} const source = map.getSource({source_id_lit}); if (source && source.setPremultiplyAlpha) source.setPremultiplyAlpha({enabled}); }})();"
    )
}

pub fn set_source_tile_lod_params_js(
    map_id: &str,
    max_zoom_levels_on_screen: f64,
    tile_count_max_min_ratio: f64,
    source_id: Option<&str>,
) -> String {
    let find = find_map_js(map_id);
    let source_argument = source_id
        .map(|id| format!(", {}", js_single_quoted(id)))
        .unwrap_or_default();
    format!(
        "(function() {{ {find} map.setSourceTileLodParams({max_zoom_levels_on_screen}, {tile_count_max_min_ratio}{source_argument}); }})();"
    )
}

#[cfg(test)]
mod tests {
    use super::set_video_playing_js;

    #[test]
    fn video_play_rejections_are_observed() {
        let js = set_video_playing_js("map", "video", true);
        assert!(js.contains("video.play()"));
        assert!(js.contains("result.catch"));
    }

    #[test]
    fn video_pause_waits_for_pending_playback() {
        let js = set_video_playing_js("map", "video", false);
        assert!(js.contains("result.then"));
        assert!(js.contains("video.pause()"));
    }
}
