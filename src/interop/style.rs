//! Style and runtime style replay JS bridge.

use super::find_map_js;
pub fn set_move_event_throttle_js(map_id: &str, throttle_ms: u32) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const value = Number({throttle_ms});
            map.__dioxusMoveEventThrottleMs = Number.isFinite(value) && value >= 0 ? value : 80;
        }})();
        "#
    )
}

/// Generate JS to set the map style
pub fn set_style_js(map_id: &str, style_url: &str) -> String {
    let find = find_map_js(map_id);
    format!(
        r#"
        (function() {{
            {find}
            const sourceRegistry = window.__dioxus_maplibre_sources && window.__dioxus_maplibre_sources['{map_id}'];
            const layerRegistry = window.__dioxus_maplibre_layers && window.__dioxus_maplibre_layers['{map_id}'];
            const layerOrder = window.__dioxus_maplibre_layer_order && window.__dioxus_maplibre_layer_order['{map_id}'];
            const imageRegistry = window.__dioxus_maplibre_images && window.__dioxus_maplibre_images['{map_id}'];
            const terrainState = window.__dioxus_maplibre_terrain && window.__dioxus_maplibre_terrain['{map_id}'];
            const skyState = window.__dioxus_maplibre_sky && window.__dioxus_maplibre_sky['{map_id}'];
            const fogState = window.__dioxus_maplibre_fog && window.__dioxus_maplibre_fog['{map_id}'];

            if (!window.__dioxus_maplibre_style_switch_tokens) {{
                window.__dioxus_maplibre_style_switch_tokens = {{}};
            }}
            const styleSwitchToken = `${{Date.now()}}_${{Math.random().toString(36).slice(2)}}`;
            window.__dioxus_maplibre_style_switch_tokens['{map_id}'] = styleSwitchToken;

            let replayed = false;
            let replayTimeoutId = null;
            let awaitingNewStyle = false;
            const trackedSourceIds = sourceRegistry ? Object.keys(sourceRegistry) : [];
            const trackedLayerIds = layerRegistry ? Object.keys(layerRegistry) : [];
            let sawStyleData = false;

            const replayReadiness = function() {{
                const styleLoaded = map.isStyleLoaded();
                const sourcesGone = trackedSourceIds.every((id) => !map.getSource(id));
                const layersGone = trackedLayerIds.every((id) => !map.getLayer(id));
                return {{ styleLoaded, sourcesGone, layersGone }};
            }};

            const replayRuntimeState = function(trigger) {{
                const activeToken = window.__dioxus_maplibre_style_switch_tokens
                    && window.__dioxus_maplibre_style_switch_tokens['{map_id}'];
                if (activeToken !== styleSwitchToken) return;
                if (replayed) {{
                    return;
                }}
                replayed = true;
                awaitingNewStyle = false;
                try {{
                    if (sourceRegistry) {{
                        for (const [sourceId, sourceDef] of Object.entries(sourceRegistry)) {{
                            try {{
                                if (!sourceDef || !sourceDef.type) continue;
                                if (map.getSource(sourceId)) continue;
                                const options = sourceDef.options
                                    ? JSON.parse(JSON.stringify(sourceDef.options))
                                    : {{}};
                                map.addSource(sourceId, {{
                                    type: sourceDef.type,
                                    ...options
                                }});
                            }} catch (err) {{
                                console.error('[dioxus-maplibre] Failed replaying source:', sourceId, err);
                            }}
                        }}
                    }}

                    const orderedLayerIds =
                        layerOrder && layerOrder.length > 0
                            ? layerOrder.slice()
                            : Object.keys(layerRegistry || {{}});
                    for (const layerId of orderedLayerIds) {{
                        const layerDef = layerRegistry && layerRegistry[layerId];
                        if (!layerDef) continue;
                        try {{
                            if (map.getLayer(layerId)) continue;
                            const layerToAdd = JSON.parse(JSON.stringify(layerDef));
                            map.addLayer(layerToAdd);
                        }} catch (err) {{
                            console.error('[dioxus-maplibre] Failed replaying layer:', layerId, err);
                        }}
                    }}

                    if (terrainState && terrainState.hasValue) {{
                        try {{
                            map.setTerrain(terrainState.value);
                        }} catch (err) {{
                            console.error('[dioxus-maplibre] Failed replaying terrain state:', err);
                        }}
                    }}

                    if (skyState && skyState.hasValue) {{
                        try {{
                            map.setSky(skyState.value);
                        }} catch (err) {{
                            console.error('[dioxus-maplibre] Failed replaying sky state:', err);
                        }}
                    }}

                    if (fogState && fogState.hasValue) {{
                        try {{
                            map.setFog(fogState.value);
                        }} catch (err) {{
                            console.error('[dioxus-maplibre] Failed replaying fog state:', err);
                        }}
                    }}

                    if (imageRegistry) {{
                        for (const [imageId, url] of Object.entries(imageRegistry)) {{
                            if (!url) continue;
                            map.loadImage(url).then((response) => {{
                                if (!response || !response.data) return;
                                if (!map.hasImage(imageId)) {{
                                    map.addImage(imageId, response.data);
                                }}
                            }}).catch((err) => {{
                                console.error('[dioxus-maplibre] Failed replaying image:', imageId, err);
                            }});
                        }}
                    }}
                }} catch (err) {{
                    console.error('[dioxus-maplibre] Runtime replay failed for map {map_id}:', err);
                }} finally {{
                    if (replayTimeoutId != null) {{
                        clearTimeout(replayTimeoutId);
                    }}
                    map.off('style.load', onStyleLoad);
                    map.off('styledata', onStyleData);
                }}
            }};

            const maybeReplay = function(trigger, force) {{
                if (!awaitingNewStyle) {{
                    return;
                }}
                const readiness = replayReadiness();
                const canReplayWithoutLoaded = sawStyleData && readiness.sourcesGone && readiness.layersGone;
                const canReplayNormally = readiness.styleLoaded && readiness.sourcesGone && readiness.layersGone;
                if (!force && !canReplayNormally && !canReplayWithoutLoaded) return;
                replayRuntimeState(force ? `${{trigger}}+forced` : trigger);
            }};

            const onStyleLoad = function() {{
                maybeReplay('style.load', false);
            }};

            const onStyleData = function(e) {{
                if (!awaitingNewStyle) {{
                    return;
                }}
                if (e && e.dataType && e.dataType !== 'style') {{
                    return;
                }}
                sawStyleData = true;
                maybeReplay('styledata', false);
            }};

            map.on('style.load', onStyleLoad);
            map.on('styledata', onStyleData);
            awaitingNewStyle = true;
            map.setStyle('{style_url}');

            setTimeout(function() {{
                if (!awaitingNewStyle || replayed) {{
                    return;
                }}
                const readiness = replayReadiness();
                // Some styles emit validation errors and never reach isStyleLoaded=true.
                // If style transition occurred (styledata seen) and old custom objects are gone,
                // we can safely replay immediately.
                if (sawStyleData && readiness.sourcesGone && readiness.layersGone) {{
                    replayRuntimeState('post-setStyle-styledata-transition');
                    return;
                }}
                maybeReplay('post-setStyle-check', false);
            }}, 0);

            replayTimeoutId = setTimeout(function() {{
                maybeReplay('timeout', true);
            }}, 6000);
        }})();
        "#
    )
}
