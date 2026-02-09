use dioxus::prelude::*;
use dioxus_maplibre::{GeoJsonSourceOptions, LayerOptions, LatLng, Map, MapErrorEvent, MapHandle};
use serde_json::json;

const MAX_POINTS_PER_SHAPE: u32 = 1_000_000;
const DEFAULT_CIRCLE_POINTS: u32 = 100;
const POINT_STEP: u32 = 10_000;
const DEFAULT_CIRCLE_SIZE_MULTIPLIER: f64 = 1.0;

const CIRCLE_SOURCE: &str = "stress-circle-source";
const SQUARE_SOURCE: &str = "stress-square-source";
const TRIANGLE_SOURCE: &str = "stress-triangle-source";
const DIAMOND_SOURCE: &str = "stress-diamond-source";

const CIRCLE_LAYER: &str = "stress-circle-layer";
const SQUARE_LAYER: &str = "stress-square-layer";
const TRIANGLE_LAYER: &str = "stress-triangle-layer";
const DIAMOND_LAYER: &str = "stress-diamond-layer";
const SQUARE_FAST_LAYER: &str = "stress-square-fast-layer";
const TRIANGLE_FAST_LAYER: &str = "stress-triangle-fast-layer";
const DIAMOND_FAST_LAYER: &str = "stress-diamond-fast-layer";

const SQUARE_ICON: &str = "stress-icon-square";
const TRIANGLE_ICON: &str = "stress-icon-triangle";
const DIAMOND_ICON: &str = "stress-icon-diamond";

const GENERATE_JS_TEMPLATE: &str = r#"
const specs = __SPECS_JSON__;

const FINLAND_POLYGON = [
  [20.10, 59.50],
  [22.00, 59.70],
  [24.40, 60.00],
  [27.20, 60.20],
  [30.30, 60.60],
  [31.60, 62.10],
  [31.50, 64.20],
  [31.40, 66.80],
  [30.50, 68.80],
  [29.00, 69.90],
  [26.50, 70.20],
  [23.90, 69.80],
  [22.10, 68.80],
  [21.20, 67.10],
  [20.70, 65.20],
  [20.20, 62.70],
  [20.00, 60.50]
];

function insidePolygon(lng, lat, polygon) {
  let inside = false;
  for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i, i += 1) {
    const xi = polygon[i][0];
    const yi = polygon[i][1];
    const xj = polygon[j][0];
    const yj = polygon[j][1];
    const crosses = ((yi > lat) !== (yj > lat))
      && (lng < ((xj - xi) * (lat - yi)) / ((yj - yi) || 1e-9) + xi);
    if (crosses) inside = !inside;
  }
  return inside;
}

function nextUnit(state) {
  state.value = (state.value * 1664525 + 1013904223) >>> 0;
  return state.value / 4294967296;
}

function randomFinlandPoint(state) {
  while (true) {
    const lng = 20.0 + nextUnit(state) * 11.6;
    const lat = 59.5 + nextUnit(state) * 10.7;
    if (insidePolygon(lng, lat, FINLAND_POLYGON)) {
      return [lng, lat];
    }
  }
}

function buildCollection(count, seed) {
  const features = new Array(count);
  const state = { value: seed >>> 0 };
  for (let i = 0; i < count; i += 1) {
    const coordinates = randomFinlandPoint(state);
    features[i] = {
      type: "Feature",
      geometry: { type: "Point", coordinates }
    };
  }
  return { type: "FeatureCollection", features };
}

const startedAt = performance.now();
for (const spec of specs) {
  const source = map.getSource(spec.sourceId);
  if (!source) continue;
  source.setData(buildCollection(spec.count, spec.seed));
}

let totalPoints = 0;
for (const spec of specs) {
  totalPoints += spec.count;
}

return {
  elapsedMs: performance.now() - startedAt,
  totalPoints
};
"#;

fn install_shape_icons_js() -> String {
    format!(
        r##"
        const specs = [
            {{ id: "{square_icon}", kind: "square", fill: "#22d3ee", stroke: "#ecfeff" }},
            {{ id: "{triangle_icon}", kind: "triangle", fill: "#f97316", stroke: "#ffedd5" }},
            {{ id: "{diamond_icon}", kind: "diamond", fill: "#facc15", stroke: "#fef3c7" }}
        ];

        const size = 48;
        const out = {{
            ok: true,
            listImagesBefore: (typeof map.listImages === "function") ? map.listImages() : null,
            perIcon: []
        }};

        try {{
            for (const spec of specs) {{
                const iconOut = {{ id: spec.id, kind: spec.kind, ok: true }};
                try {{
                    const canvas = document.createElement("canvas");
                    canvas.width = size;
                    canvas.height = size;
                    const ctx = canvas.getContext("2d");
                    if (!ctx) {{
                        throw new Error("canvas.getContext('2d') returned null");
                    }}

                    ctx.clearRect(0, 0, size, size);
                    ctx.fillStyle = spec.fill;
                    ctx.strokeStyle = spec.stroke;
                    ctx.lineWidth = 4;

                    if (spec.kind === "square") {{
                        ctx.beginPath();
                        ctx.rect(9, 9, size - 18, size - 18);
                        ctx.fill();
                        ctx.stroke();
                    }} else if (spec.kind === "triangle") {{
                        ctx.beginPath();
                        ctx.moveTo(size / 2, 6);
                        ctx.lineTo(size - 8, size - 9);
                        ctx.lineTo(8, size - 9);
                        ctx.closePath();
                        ctx.fill();
                        ctx.stroke();
                    }} else if (spec.kind === "diamond") {{
                        ctx.beginPath();
                        ctx.moveTo(size / 2, 5);
                        ctx.lineTo(size - 6, size / 2);
                        ctx.lineTo(size / 2, size - 5);
                        ctx.lineTo(6, size / 2);
                        ctx.closePath();
                        ctx.fill();
                        ctx.stroke();
                    }}

                    if (map.hasImage(spec.id)) {{
                        map.removeImage(spec.id);
                        iconOut.removedExisting = true;
                    }}

                    map.addImage(spec.id, ctx.getImageData(0, 0, size, size), {{ pixelRatio: 2 }});
                    iconOut.hasImageAfterAdd = map.hasImage(spec.id);
                }} catch (err) {{
                    iconOut.ok = false;
                    iconOut.error = (err && err.message) ? err.message : String(err);
                    out.ok = false;
                }}

                out.perIcon.push(iconOut);
            }}

            out.listImagesAfter = (typeof map.listImages === "function") ? map.listImages() : null;
            return out;
        }} catch (fatal) {{
            console.error("[dioxus-maplibre][stress] failed to install shape icons", fatal);
            return {{
                ok: false,
                error: (fatal && fatal.message) ? fatal.message : String(fatal),
                perIcon: out.perIcon
            }};
        }}
        "##,
        square_icon = SQUARE_ICON,
        triangle_icon = TRIANGLE_ICON,
        diamond_icon = DIAMOND_ICON
    )
}

fn diagnostics_snapshot_js() -> String {
    format!(
        r##"
        try {{
            const safeError = (err) => (err && err.message) ? err.message : String(err);

            const safeValue = (value) => {{
                try {{
                    return JSON.parse(JSON.stringify(value));
                }} catch (err) {{
                    return {{
                        unserializable: true,
                        error: safeError(err)
                    }};
                }}
            }};

            const safeLayoutProperty = (id, prop) => {{
                try {{
                    return safeValue(map.getLayoutProperty(id, prop));
                }} catch (err) {{
                    return {{ error: safeError(err) }};
                }}
            }};

            const safePaintProperty = (id, prop) => {{
                try {{
                    return safeValue(map.getPaintProperty(id, prop));
                }} catch (err) {{
                    return {{ error: safeError(err) }};
                }}
            }};

            const inspectSource = (id) => {{
                let src = null;
                try {{
                    src = map.getSource(id);
                }} catch (err) {{
                    return {{ id, exists: false, error: safeError(err) }};
                }}
                if (!src) return {{ id, exists: false }};
                const info = {{ id, exists: true, type: src.type || null }};
                try {{
                    if (src && src._data && Array.isArray(src._data.features)) {{
                        info.featureCount = src._data.features.length;
                    }}
                }} catch (e) {{
                    info.featureCountError = safeError(e);
                }}
                return info;
            }};

            const inspectLayer = (id) => {{
                let layer = null;
                try {{
                    layer = map.getLayer(id);
                }} catch (err) {{
                    return {{ id, exists: false, error: safeError(err) }};
                }}
                if (!layer) return {{ id, exists: false }};
                return {{
                    id,
                    exists: true,
                    type: layer.type,
                    source: layer.source || null,
                    iconImage: safeLayoutProperty(id, "icon-image"),
                    textField: safeLayoutProperty(id, "text-field"),
                    textSize: safeLayoutProperty(id, "text-size"),
                    iconSize: safeLayoutProperty(id, "icon-size"),
                    visibility: safeLayoutProperty(id, "visibility"),
                    circleRadius: safePaintProperty(id, "circle-radius")
                }};
            }};

            const out = {{
                ok: true,
                styleLoaded: (typeof map.isStyleLoaded === "function") ? map.isStyleLoaded() : null,
                zoom: (typeof map.getZoom === "function") ? map.getZoom() : null,
                center: (typeof map.getCenter === "function")
                    ? (() => {{
                        const c = map.getCenter();
                        return {{ lng: c.lng, lat: c.lat }};
                    }})()
                    : null,
                listImages: safeValue((typeof map.listImages === "function") ? map.listImages() : null),
                hasImage: {{
                    square: (typeof map.hasImage === "function") ? map.hasImage("{square_icon}") : null,
                    triangle: (typeof map.hasImage === "function") ? map.hasImage("{triangle_icon}") : null,
                    diamond: (typeof map.hasImage === "function") ? map.hasImage("{diamond_icon}") : null
                }},
                sources: {{
                    circle: inspectSource("{circle_source}"),
                    square: inspectSource("{square_source}"),
                    triangle: inspectSource("{triangle_source}"),
                    diamond: inspectSource("{diamond_source}")
                }},
                layers: {{
                    circle: inspectLayer("{circle_layer}"),
                    square: inspectLayer("{square_layer}"),
                    triangle: inspectLayer("{triangle_layer}"),
                    diamond: inspectLayer("{diamond_layer}"),
                    squareFast: inspectLayer("{square_fast_layer}"),
                    triangleFast: inspectLayer("{triangle_fast_layer}"),
                    diamondFast: inspectLayer("{diamond_fast_layer}")
                }}
            }};

            try {{
                if (typeof map.addImage === "function") {{
                    const name = "__stress_diag_temp";
                    if (map.hasImage(name)) map.removeImage(name);
                    const tiny = {{
                        width: 2,
                        height: 2,
                        data: new Uint8Array([
                            255, 0, 0, 255,
                            255, 0, 0, 255,
                            255, 0, 0, 255,
                            255, 0, 0, 255
                        ])
                    }};
                    map.addImage(name, tiny);
                    out.tempImageAdd = {{
                        ok: true,
                        hasAfterAdd: map.hasImage(name)
                    }};
                    map.removeImage(name);
                    out.tempImageAdd.hasAfterRemove = map.hasImage(name);
                }}
            }} catch (err) {{
                out.tempImageAdd = {{
                    ok: false,
                    error: safeError(err)
                }};
            }}

            return safeValue(out);
        }} catch (fatal) {{
            return {{
                ok: false,
                fatalError: (fatal && fatal.message) ? fatal.message : String(fatal)
            }};
        }}
        "##,
        square_icon = SQUARE_ICON,
        triangle_icon = TRIANGLE_ICON,
        diamond_icon = DIAMOND_ICON,
        circle_source = CIRCLE_SOURCE,
        square_source = SQUARE_SOURCE,
        triangle_source = TRIANGLE_SOURCE,
        diamond_source = DIAMOND_SOURCE,
        circle_layer = CIRCLE_LAYER,
        square_layer = SQUARE_LAYER,
        triangle_layer = TRIANGLE_LAYER,
        diamond_layer = DIAMOND_LAYER,
        square_fast_layer = SQUARE_FAST_LAYER,
        triangle_fast_layer = TRIANGLE_FAST_LAYER,
        diamond_fast_layer = DIAMOND_FAST_LAYER
    )
}

fn empty_collection() -> serde_json::Value {
    json!({
        "type": "FeatureCollection",
        "features": []
    })
}

fn parse_point_count(raw: &str) -> Option<u32> {
    raw.parse::<u32>()
        .ok()
        .map(|count| count.min(MAX_POINTS_PER_SHAPE))
}

fn parse_size_multiplier(raw: &str) -> Option<f64> {
    raw.parse::<f64>()
        .ok()
        .map(|multiplier| multiplier.clamp(0.2, 4.0))
}

fn circle_radius_expression(size_multiplier: f64) -> serde_json::Value {
    json!([
        "interpolate",
        ["exponential", 1.35],
        ["zoom"],
        2, 1.5 * size_multiplier,
        4, 3.0 * size_multiplier,
        6, 6.5 * size_multiplier,
        8, 12.0 * size_multiplier,
        10, 20.0 * size_multiplier,
        12, 30.0 * size_multiplier
    ])
}

fn shape_icon_size_expression() -> serde_json::Value {
    json!([
        "interpolate",
        ["exponential", 1.3],
        ["zoom"],
        2, 0.12,
        4, 0.2,
        6, 0.32,
        8, 0.48,
        10, 0.7,
        12, 0.95
    ])
}

fn fast_shape_circle_radius_expression() -> serde_json::Value {
    json!(["interpolate", ["linear"], ["zoom"], 2, 0.6, 6, 1.1, 10, 2.0, 12, 2.6])
}

fn build_generation_script(
    circle_count: u32,
    square_count: u32,
    triangle_count: u32,
    diamond_count: u32,
) -> String {
    let specs = json!([
        { "sourceId": CIRCLE_SOURCE, "count": circle_count, "seed": 11 },
        { "sourceId": SQUARE_SOURCE, "count": square_count, "seed": 29 },
        { "sourceId": TRIANGLE_SOURCE, "count": triangle_count, "seed": 47 },
        { "sourceId": DIAMOND_SOURCE, "count": diamond_count, "seed": 83 }
    ]);

    let specs_json = serde_json::to_string(&specs).unwrap_or_else(|_| "[]".to_string());
    GENERATE_JS_TEMPLATE.replace("__SPECS_JSON__", &specs_json)
}

fn add_stress_layers(handle: &MapHandle, circle_size_multiplier: f64) {
    handle.add_geojson_source(
        CIRCLE_SOURCE,
        GeoJsonSourceOptions {
            data: empty_collection(),
            ..Default::default()
        },
    );
    handle.add_geojson_source(
        SQUARE_SOURCE,
        GeoJsonSourceOptions {
            data: empty_collection(),
            ..Default::default()
        },
    );
    handle.add_geojson_source(
        TRIANGLE_SOURCE,
        GeoJsonSourceOptions {
            data: empty_collection(),
            ..Default::default()
        },
    );
    handle.add_geojson_source(
        DIAMOND_SOURCE,
        GeoJsonSourceOptions {
            data: empty_collection(),
            ..Default::default()
        },
    );

    handle.add_layer(LayerOptions::circle(CIRCLE_LAYER, CIRCLE_SOURCE).paint(json!({
        "circle-color": "#22d3ee",
        "circle-opacity": ["interpolate", ["linear"], ["zoom"], 2, 0.72, 8, 0.92],
        "circle-radius": circle_radius_expression(circle_size_multiplier),
        "circle-stroke-width": ["interpolate", ["linear"], ["zoom"], 2, 0.8, 10, 1.8],
        "circle-stroke-color": "#ecfeff"
    })));

}

fn add_shape_icon_layers(handle: &MapHandle) {
    let icon_size = shape_icon_size_expression();

    handle.add_layer(LayerOptions::symbol(SQUARE_LAYER, SQUARE_SOURCE).layout(json!({
        "icon-image": SQUARE_ICON,
        "icon-size": icon_size,
        "icon-allow-overlap": true,
        "icon-ignore-placement": true
    })));

    handle.add_layer(LayerOptions::symbol(TRIANGLE_LAYER, TRIANGLE_SOURCE).layout(json!({
        "icon-image": TRIANGLE_ICON,
        "icon-size": shape_icon_size_expression(),
        "icon-allow-overlap": true,
        "icon-ignore-placement": true
    })));

    handle.add_layer(LayerOptions::symbol(DIAMOND_LAYER, DIAMOND_SOURCE).layout(json!({
        "icon-image": DIAMOND_ICON,
        "icon-size": shape_icon_size_expression(),
        "icon-allow-overlap": true,
        "icon-ignore-placement": true
    })));
}

fn force_shape_icon_layouts(handle: &MapHandle) {
    let icon_size = shape_icon_size_expression();

    handle.set_layout_property(SQUARE_LAYER, "icon-image", json!(SQUARE_ICON));
    handle.set_layout_property(SQUARE_LAYER, "icon-size", icon_size.clone());
    handle.set_layout_property(SQUARE_LAYER, "icon-allow-overlap", json!(true));
    handle.set_layout_property(SQUARE_LAYER, "icon-ignore-placement", json!(true));
    handle.set_layout_property(SQUARE_LAYER, "text-field", json!(""));
    handle.set_layout_property(SQUARE_LAYER, "text-size", json!(0));

    handle.set_layout_property(TRIANGLE_LAYER, "icon-image", json!(TRIANGLE_ICON));
    handle.set_layout_property(TRIANGLE_LAYER, "icon-size", icon_size.clone());
    handle.set_layout_property(TRIANGLE_LAYER, "icon-allow-overlap", json!(true));
    handle.set_layout_property(TRIANGLE_LAYER, "icon-ignore-placement", json!(true));
    handle.set_layout_property(TRIANGLE_LAYER, "text-field", json!(""));
    handle.set_layout_property(TRIANGLE_LAYER, "text-size", json!(0));

    handle.set_layout_property(DIAMOND_LAYER, "icon-image", json!(DIAMOND_ICON));
    handle.set_layout_property(DIAMOND_LAYER, "icon-size", icon_size);
    handle.set_layout_property(DIAMOND_LAYER, "icon-allow-overlap", json!(true));
    handle.set_layout_property(DIAMOND_LAYER, "icon-ignore-placement", json!(true));
    handle.set_layout_property(DIAMOND_LAYER, "text-field", json!(""));
    handle.set_layout_property(DIAMOND_LAYER, "text-size", json!(0));
}

fn add_shape_fast_layers(handle: &MapHandle) {
    let radius = fast_shape_circle_radius_expression();
    let hidden = json!("none");

    handle.add_layer(
        LayerOptions::circle(SQUARE_FAST_LAYER, SQUARE_SOURCE)
            .paint(json!({
                "circle-color": "#22d3ee",
                "circle-opacity": 0.88,
                "circle-radius": radius.clone()
            }))
            .layout(json!({ "visibility": hidden })),
    );

    handle.add_layer(
        LayerOptions::circle(TRIANGLE_FAST_LAYER, TRIANGLE_SOURCE)
            .paint(json!({
                "circle-color": "#f97316",
                "circle-opacity": 0.88,
                "circle-radius": radius.clone()
            }))
            .layout(json!({ "visibility": hidden })),
    );

    handle.add_layer(
        LayerOptions::circle(DIAMOND_FAST_LAYER, DIAMOND_SOURCE)
            .paint(json!({
                "circle-color": "#facc15",
                "circle-opacity": 0.88,
                "circle-radius": radius
            }))
            .layout(json!({ "visibility": hidden })),
    );
}

fn apply_shape_render_mode(handle: &MapHandle, fast_render_mode: bool) {
    let (symbol_vis, fast_vis) = if fast_render_mode {
        (json!("none"), json!("visible"))
    } else {
        (json!("visible"), json!("none"))
    };

    handle.set_layout_property(SQUARE_LAYER, "visibility", symbol_vis.clone());
    handle.set_layout_property(TRIANGLE_LAYER, "visibility", symbol_vis.clone());
    handle.set_layout_property(DIAMOND_LAYER, "visibility", symbol_vis);

    handle.set_layout_property(SQUARE_FAST_LAYER, "visibility", fast_vis.clone());
    handle.set_layout_property(TRIANGLE_FAST_LAYER, "visibility", fast_vis.clone());
    handle.set_layout_property(DIAMOND_FAST_LAYER, "visibility", fast_vis);
}

fn add_shape_text_layers_fallback(handle: &MapHandle) {
    handle.add_layer(
        LayerOptions::symbol(SQUARE_LAYER, SQUARE_SOURCE)
            .layout(json!({
                "text-field": "S",
                "text-size": ["interpolate", ["linear"], ["zoom"], 2, 10, 10, 16],
                "text-allow-overlap": true,
                "text-ignore-placement": true
            }))
            .paint(json!({
                "text-color": "#22d3ee",
                "text-halo-color": "#0f172a",
                "text-halo-width": 1.0
            })),
    );

    handle.add_layer(
        LayerOptions::symbol(TRIANGLE_LAYER, TRIANGLE_SOURCE)
            .layout(json!({
                "text-field": "T",
                "text-size": ["interpolate", ["linear"], ["zoom"], 2, 10, 10, 16],
                "text-allow-overlap": true,
                "text-ignore-placement": true
            }))
            .paint(json!({
                "text-color": "#f97316",
                "text-halo-color": "#0f172a",
                "text-halo-width": 1.0
            })),
    );

    handle.add_layer(
        LayerOptions::symbol(DIAMOND_LAYER, DIAMOND_SOURCE)
            .layout(json!({
                "text-field": "D",
                "text-size": ["interpolate", ["linear"], ["zoom"], 2, 10, 10, 16],
                "text-allow-overlap": true,
                "text-ignore-placement": true
            }))
            .paint(json!({
                "text-color": "#facc15",
                "text-halo-color": "#0f172a",
                "text-halo-width": 1.0
            })),
    );
}

#[component]
pub fn Stress() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut circle_count = use_signal(|| DEFAULT_CIRCLE_POINTS);
    let mut circle_size_multiplier = use_signal(|| DEFAULT_CIRCLE_SIZE_MULTIPLIER);
    let mut square_count = use_signal(|| 0_u32);
    let mut triangle_count = use_signal(|| 0_u32);
    let mut diamond_count = use_signal(|| 0_u32);

    let mut circle_enabled = use_signal(|| true);
    let mut square_enabled = use_signal(|| false);
    let mut triangle_enabled = use_signal(|| false);
    let mut diamond_enabled = use_signal(|| false);

    let mut is_generating = use_signal(|| false);
    let mut generation_status =
        use_signal(|| format!("Preparing default {} circles...", DEFAULT_CIRCLE_POINTS));
    let mut last_elapsed_ms = use_signal(|| None::<f64>);
    let mut last_total_points = use_signal(|| 0_u64);
    let mut map_error = use_signal(|| None::<String>);
    let mut shape_render_mode = use_signal(|| "Initializing".to_string());
    let mut icon_install_debug = use_signal(|| "No icon install attempt yet.".to_string());
    let mut diagnostics_json = use_signal(|| "No diagnostics collected yet.".to_string());
    let mut debug_log = use_signal(Vec::<String>::new);
    let mut fast_render_mode = use_signal(|| false);

    let style: Signal<String> = use_context();

    let selected_circle_count = if circle_enabled() { circle_count() } else { 0 };
    let selected_square_count = if square_enabled() { square_count() } else { 0 };
    let selected_triangle_count = if triangle_enabled() {
        triangle_count()
    } else {
        0
    };
    let selected_diamond_count = if diamond_enabled() { diamond_count() } else { 0 };

    let planned_total_points = u64::from(selected_circle_count)
        + u64::from(selected_square_count)
        + u64::from(selected_triangle_count)
        + u64::from(selected_diamond_count);
    let fast_mode_label = if fast_render_mode() { "ON" } else { "OFF" };

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                Map {
                    style: style(),
                    center: LatLng::new(64.6, 26.5),
                    zoom: 4.6,
                    on_ready: move |handle: MapHandle| {
                        add_stress_layers(&handle, circle_size_multiplier());
                        map_handle.set(Some(handle.clone()));
                        debug_log.write().push("on_ready: map + base stress layers initialized".to_string());

                        let circles = if circle_enabled() { circle_count() } else { 0 };
                        is_generating.set(true);
                        generation_status
                            .set(format!("Generating default {circles} circle points..."));
                        last_elapsed_ms.set(None);
                        map_error.set(None);

                        let map = handle.clone();
                        spawn(async move {
                            let icon_install_result = map
                                .eval_async::<serde_json::Value>(&install_shape_icons_js())
                                .await;
                            if let Some(ref value) = icon_install_result {
                                icon_install_debug.set(
                                    serde_json::to_string_pretty(value)
                                        .unwrap_or_else(|_| value.to_string()),
                                );
                            } else {
                                icon_install_debug.set(
                                    "install_shape_icons_js returned None (eval_async deserialize/join failed)"
                                        .to_string(),
                                );
                            }
                            let icons_ready = icon_install_result
                                .as_ref()
                                .and_then(|value| value.get("ok"))
                                .and_then(serde_json::Value::as_bool)
                                .unwrap_or(false);
                            let icon_error = icon_install_result
                                .as_ref()
                                .and_then(|value| value.get("error"))
                                .and_then(serde_json::Value::as_str)
                                .map(ToString::to_string);

                            if icons_ready {
                                add_shape_icon_layers(&map);
                                force_shape_icon_layouts(&map);
                                add_shape_fast_layers(&map);
                                apply_shape_render_mode(&map, fast_render_mode());
                                shape_render_mode.set("Icons".to_string());
                                debug_log.write().push(
                                    "icon install: OK, added icon-based symbol layers + forced icon layouts"
                                        .to_string(),
                                );
                            } else {
                                add_shape_text_layers_fallback(&map);
                                add_shape_fast_layers(&map);
                                apply_shape_render_mode(&map, fast_render_mode());
                                if let Some(error) = icon_error {
                                    debug_log.write().push(format!(
                                        "icon install: FAILED, using text fallback ({error})"
                                    ));
                                    shape_render_mode.set(format!("Text fallback ({error})"));
                                } else {
                                    debug_log.write().push(
                                        "icon install: FAILED without explicit error, using text fallback"
                                            .to_string(),
                                    );
                                    shape_render_mode.set("Text fallback".to_string());
                                }
                            }

                            if let Some(snapshot) =
                                map.eval_async::<serde_json::Value>(&diagnostics_snapshot_js()).await
                            {
                                diagnostics_json.set(
                                    serde_json::to_string_pretty(&snapshot)
                                        .unwrap_or_else(|_| snapshot.to_string()),
                                );
                                debug_log.write().push(
                                    "post-install diagnostics snapshot collected".to_string(),
                                );
                            } else {
                                diagnostics_json.set(
                                    "post-install diagnostics snapshot returned None".to_string(),
                                );
                                debug_log.write().push(
                                    "post-install diagnostics snapshot failed (None)".to_string(),
                                );
                            }

                            let js = build_generation_script(circles, 0, 0, 0);
                            if let Some(result) = map.eval_async::<serde_json::Value>(&js).await {
                                let elapsed_ms = result
                                    .get("elapsedMs")
                                    .and_then(serde_json::Value::as_f64)
                                    .unwrap_or_default();
                                let total_points = result
                                    .get("totalPoints")
                                    .and_then(serde_json::Value::as_u64)
                                    .unwrap_or(u64::from(circles));

                                generation_status.set(format!(
                                    "Rendered {total_points} points in {:.2}s",
                                    elapsed_ms / 1000.0
                                ));
                                last_elapsed_ms.set(Some(elapsed_ms));
                                last_total_points.set(total_points);
                                debug_log.write().push(format!(
                                    "default generation complete: {total_points} points, {:.1}ms",
                                    elapsed_ms
                                ));
                            } else {
                                generation_status.set(
                                    "Default dataset generation command submitted.".to_string(),
                                );
                                last_total_points.set(u64::from(circles));
                                debug_log.write().push(
                                    "default generation eval returned None".to_string(),
                                );
                            }

                            if let Some(snapshot) =
                                map.eval_async::<serde_json::Value>(&diagnostics_snapshot_js()).await
                            {
                                diagnostics_json.set(
                                    serde_json::to_string_pretty(&snapshot)
                                        .unwrap_or_else(|_| snapshot.to_string()),
                                );
                                debug_log
                                    .write()
                                    .push("post-generation diagnostics snapshot collected".to_string());
                            }
                            is_generating.set(false);
                        });
                    },
                    on_error: move |event: MapErrorEvent| {
                        let message = event.message.unwrap_or_else(|| "Unknown map error".to_string());
                        map_error.set(Some(message));
                    },
                }
            }
            div { style: "width: 360px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px; overflow-y: auto;",
                h3 { style: "margin: 0 0 12px 0;", "Stress Test (Finland)" }
                p { style: "margin: 0 0 10px 0; color: #b8c1d9;", "Generate random point datasets inside Finland and render them at once." }
                p { style: "margin: 0 0 12px 0; color: #8ea0c2;", "Each slider goes to 1,000,000 points per shape." }

                div { style: "margin-top: 8px; padding: 10px; border: 1px solid #334155; border-radius: 6px; background: #111827;",
                    button {
                        style: "padding: 6px 8px; border-radius: 4px; border: none; cursor: pointer; background: #06b6d4; color: white;",
                        onclick: move |_| circle_enabled.set(!circle_enabled()),
                        if circle_enabled() { "Disable circles" } else { "Enable circles" }
                    }
                    p { style: "margin: 0 0 6px 0; color: #9ec9ff;", "Circle markers: {circle_count()}" }
                    input {
                        style: "width: 100%;",
                        r#type: "range",
                        min: "0",
                        max: "{MAX_POINTS_PER_SHAPE}",
                        step: "{POINT_STEP}",
                        value: "{circle_count}",
                        oninput: move |event| {
                            if let Some(count) = parse_point_count(&event.value()) {
                                circle_count.set(count);
                            }
                        }
                    }
                    input {
                        style: "margin-top: 6px; width: 100%; padding: 4px; border-radius: 4px; border: 1px solid #475569; background: #0f172a; color: #e0e0e0;",
                        r#type: "number",
                        min: "0",
                        max: "{MAX_POINTS_PER_SHAPE}",
                        value: "{circle_count}",
                        oninput: move |event| {
                            if let Some(count) = parse_point_count(&event.value()) {
                                circle_count.set(count);
                            }
                        }
                    }
                }

                div { style: "margin-top: 12px; padding: 10px; border: 1px solid #334155; border-radius: 6px; background: #111827;",
                    button {
                        style: "padding: 6px 8px; border-radius: 4px; border: none; cursor: pointer; background: #0ea5e9; color: white;",
                        onclick: move |_| square_enabled.set(!square_enabled()),
                        if square_enabled() { "Disable squares" } else { "Enable squares" }
                    }
                    p { style: "margin: 8px 0 6px 0; color: #9ec9ff;", "Squares: {square_count()}" }
                    input {
                        style: "width: 100%;",
                        r#type: "range",
                        min: "0",
                        max: "{MAX_POINTS_PER_SHAPE}",
                        step: "{POINT_STEP}",
                        value: "{square_count}",
                        oninput: move |event| {
                            if let Some(count) = parse_point_count(&event.value()) {
                                square_count.set(count);
                            }
                        }
                    }
                    input {
                        style: "margin-top: 6px; width: 100%; padding: 4px; border-radius: 4px; border: 1px solid #475569; background: #0f172a; color: #e0e0e0;",
                        r#type: "number",
                        min: "0",
                        max: "{MAX_POINTS_PER_SHAPE}",
                        value: "{square_count}",
                        oninput: move |event| {
                            if let Some(count) = parse_point_count(&event.value()) {
                                square_count.set(count);
                            }
                        }
                    }
                }

                div { style: "margin-top: 12px; padding: 10px; border: 1px solid #334155; border-radius: 6px; background: #111827;",
                    button {
                        style: "padding: 6px 8px; border-radius: 4px; border: none; cursor: pointer; background: #f97316; color: white;",
                        onclick: move |_| triangle_enabled.set(!triangle_enabled()),
                        if triangle_enabled() { "Disable triangles" } else { "Enable triangles" }
                    }
                    p { style: "margin: 8px 0 6px 0; color: #9ec9ff;", "Triangles: {triangle_count()}" }
                    input {
                        style: "width: 100%;",
                        r#type: "range",
                        min: "0",
                        max: "{MAX_POINTS_PER_SHAPE}",
                        step: "{POINT_STEP}",
                        value: "{triangle_count}",
                        oninput: move |event| {
                            if let Some(count) = parse_point_count(&event.value()) {
                                triangle_count.set(count);
                            }
                        }
                    }
                    input {
                        style: "margin-top: 6px; width: 100%; padding: 4px; border-radius: 4px; border: 1px solid #475569; background: #0f172a; color: #e0e0e0;",
                        r#type: "number",
                        min: "0",
                        max: "{MAX_POINTS_PER_SHAPE}",
                        value: "{triangle_count}",
                        oninput: move |event| {
                            if let Some(count) = parse_point_count(&event.value()) {
                                triangle_count.set(count);
                            }
                        }
                    }
                }

                div { style: "margin-top: 12px; padding: 10px; border: 1px solid #334155; border-radius: 6px; background: #111827;",
                    button {
                        style: "padding: 6px 8px; border-radius: 4px; border: none; cursor: pointer; background: #facc15; color: #111827;",
                        onclick: move |_| diamond_enabled.set(!diamond_enabled()),
                        if diamond_enabled() { "Disable diamonds" } else { "Enable diamonds" }
                    }
                    p { style: "margin: 8px 0 6px 0; color: #9ec9ff;", "Diamonds: {diamond_count()}" }
                    input {
                        style: "width: 100%;",
                        r#type: "range",
                        min: "0",
                        max: "{MAX_POINTS_PER_SHAPE}",
                        step: "{POINT_STEP}",
                        value: "{diamond_count}",
                        oninput: move |event| {
                            if let Some(count) = parse_point_count(&event.value()) {
                                diamond_count.set(count);
                            }
                        }
                    }
                    input {
                        style: "margin-top: 6px; width: 100%; padding: 4px; border-radius: 4px; border: 1px solid #475569; background: #0f172a; color: #e0e0e0;",
                        r#type: "number",
                        min: "0",
                        max: "{MAX_POINTS_PER_SHAPE}",
                        value: "{diamond_count}",
                        oninput: move |event| {
                            if let Some(count) = parse_point_count(&event.value()) {
                                diamond_count.set(count);
                            }
                        }
                    }
                }

                div { style: "margin-top: 12px; padding: 10px; border: 1px solid #334155; border-radius: 6px; background: #111827;",
                    p { style: "margin: 0 0 6px 0; color: #9ec9ff;", "Circle size multiplier (zoom-scaled): {circle_size_multiplier():.1}x" }
                    input {
                        style: "width: 100%;",
                        r#type: "range",
                        min: "0.2",
                        max: "4.0",
                        step: "0.1",
                        value: "{circle_size_multiplier}",
                        oninput: move |event| {
                            if let Some(multiplier) = parse_size_multiplier(&event.value()) {
                                circle_size_multiplier.set(multiplier);
                                if let Some(ref map) = *map_handle.read() {
                                    map.set_paint_property(
                                        CIRCLE_LAYER,
                                        "circle-radius",
                                        circle_radius_expression(multiplier),
                                    );
                                }
                            }
                        }
                    }
                }

                div { style: "margin-top: 12px; padding: 10px; border: 1px solid #334155; border-radius: 6px; background: #111827;",
                    p { style: "margin: 0 0 6px 0; color: #9ec9ff;", "Render mode for square/triangle/diamond" }
                    button {
                        style: "padding: 8px; border-radius: 4px; border: none; cursor: pointer; background: #0f766e; color: white;",
                        onclick: move |_| {
                            let new_mode = !fast_render_mode();
                            fast_render_mode.set(new_mode);
                            debug_log.write().push(format!(
                                "fast render mode {}",
                                if new_mode { "enabled" } else { "disabled" }
                            ));

                            if let Some(ref map) = *map_handle.read() {
                                apply_shape_render_mode(map, new_mode);
                                let map = map.clone();
                                spawn(async move {
                                    if let Some(snapshot) =
                                        map.eval_async::<serde_json::Value>(&diagnostics_snapshot_js()).await
                                    {
                                        diagnostics_json.set(
                                            serde_json::to_string_pretty(&snapshot)
                                                .unwrap_or_else(|_| snapshot.to_string()),
                                        );
                                        debug_log.write().push(
                                            "post-mode-switch diagnostics snapshot collected"
                                                .to_string(),
                                        );
                                    }
                                });
                            }
                        },
                        if fast_render_mode() {
                            "Fast mode: ON (color circles)"
                        } else {
                            "Fast mode: OFF (shape icons)"
                        }
                    }
                    if planned_total_points >= 150_000 {
                        p { style: "margin-top: 8px; color: #facc15;",
                            "Recommendation: enable fast mode for smoother pan/zoom above ~150k points."
                        }
                    }
                }

                p { style: "margin-top: 12px; color: #b8c1d9;", "Planned total points: {planned_total_points}" }

                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 10px;",
                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 9px; border-radius: 4px; border: none; background: #3b82f6; color: white; cursor: pointer;",
                                    disabled: is_generating(),
                                    onclick: move |_| {
                                        if is_generating() {
                                            return;
                                        }

                                        let circles = if circle_enabled() { circle_count() } else { 0 };
                                        let squares = if square_enabled() { square_count() } else { 0 };
                                        let triangles = if triangle_enabled() { triangle_count() } else { 0 };
                                        let diamonds = if diamond_enabled() { diamond_count() } else { 0 };
                                        let total = u64::from(circles)
                                            + u64::from(squares)
                                            + u64::from(triangles)
                                            + u64::from(diamonds);

                                        let js = build_generation_script(circles, squares, triangles, diamonds);
                                        generation_status.set(format!("Generating {total} points..."));
                                        last_elapsed_ms.set(None);
                                        map_error.set(None);
                                        is_generating.set(true);
                                        debug_log.write().push(format!(
                                            "manual generation requested: circles={circles} (enabled={}), squares={squares}, triangles={triangles}, diamonds={diamonds}",
                                            circle_enabled()
                                        ));

                                        let map = map.clone();
                                        spawn(async move {
                                            if let Some(result) = map.eval_async::<serde_json::Value>(&js).await {
                                                let elapsed_ms = result
                                                    .get("elapsedMs")
                                                    .and_then(serde_json::Value::as_f64)
                                                    .unwrap_or_default();
                                                let total_points = result
                                                    .get("totalPoints")
                                                    .and_then(serde_json::Value::as_u64)
                                                    .unwrap_or(total);

                                                generation_status.set(format!(
                                                    "Rendered {total_points} points in {:.2}s",
                                                    elapsed_ms / 1000.0
                                                ));
                                                last_elapsed_ms.set(Some(elapsed_ms));
                                                last_total_points.set(total_points);
                                                debug_log.write().push(format!(
                                                    "manual generation complete: {total_points} points, {:.1}ms",
                                                    elapsed_ms
                                                ));
                                            } else {
                                                generation_status.set("Generation command submitted.".to_string());
                                                last_total_points.set(total);
                                                debug_log.write().push(
                                                    "manual generation eval returned None".to_string(),
                                                );
                                            }

                                            if let Some(snapshot) =
                                                map.eval_async::<serde_json::Value>(&diagnostics_snapshot_js()).await
                                            {
                                                diagnostics_json.set(
                                                    serde_json::to_string_pretty(&snapshot)
                                                        .unwrap_or_else(|_| snapshot.to_string()),
                                                );
                                                debug_log.write().push(
                                                    "manual post-generation diagnostics snapshot collected"
                                                        .to_string(),
                                                );
                                            } else {
                                                diagnostics_json.set(
                                                    "manual post-generation diagnostics snapshot returned None"
                                                        .to_string(),
                                                );
                                                debug_log.write().push(
                                                    "manual post-generation diagnostics snapshot failed (None)"
                                                        .to_string(),
                                                );
                                            }
                                            is_generating.set(false);
                                        });
                                    },
                                    if is_generating() { "Generating..." } else { "Generate / Refresh dataset" }
                                }
                            }
                        }

                        button {
                            style: "padding: 8px; border-radius: 4px; border: none; background: #1d4ed8; color: white; cursor: pointer;",
                            disabled: is_generating(),
                            onclick: move |_| {
                                circle_enabled.set(true);
                                circle_count.set(MAX_POINTS_PER_SHAPE);
                                square_enabled.set(false);
                                triangle_enabled.set(false);
                                diamond_enabled.set(false);
                                square_count.set(0);
                                triangle_count.set(0);
                                diamond_count.set(0);
                            },
                            "Preset: 1M circles"
                        }

                        button {
                            style: "padding: 8px; border-radius: 4px; border: none; background: #2563eb; color: white; cursor: pointer;",
                            disabled: is_generating(),
                            onclick: move |_| {
                                circle_enabled.set(true);
                                circle_count.set(10_000);
                                square_enabled.set(true);
                                triangle_enabled.set(true);
                                diamond_enabled.set(true);
                                square_count.set(10_000);
                                triangle_count.set(10_000);
                                diamond_count.set(10_000);
                            },
                            "Preset: 10k each shape"
                        }

                        button {
                            style: "padding: 8px; border-radius: 4px; border: none; background: #475569; color: white; cursor: pointer;",
                            disabled: is_generating(),
                            onclick: move |_| {
                                circle_enabled.set(false);
                                circle_count.set(0);
                                square_count.set(0);
                                triangle_count.set(0);
                                diamond_count.set(0);
                                square_enabled.set(false);
                                triangle_enabled.set(false);
                                diamond_enabled.set(false);
                            },
                            "Clear all shapes"
                        }

                        {
                            let map = map.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px; border-radius: 4px; border: none; background: #0f766e; color: white; cursor: pointer;",
                                    onclick: move |_| {
                                        debug_log.write().push("manual diagnostics requested".to_string());
                                        let map = map.clone();
                                        spawn(async move {
                                            if let Some(snapshot) =
                                                map.eval_async::<serde_json::Value>(&diagnostics_snapshot_js()).await
                                            {
                                                diagnostics_json.set(
                                                    serde_json::to_string_pretty(&snapshot)
                                                        .unwrap_or_else(|_| snapshot.to_string()),
                                                );
                                                debug_log.write().push(
                                                    "manual diagnostics snapshot collected".to_string(),
                                                );
                                            } else {
                                                diagnostics_json.set(
                                                    "manual diagnostics snapshot returned None".to_string(),
                                                );
                                                debug_log.write().push(
                                                    "manual diagnostics snapshot failed (None)".to_string(),
                                                );
                                            }
                                        });
                                    },
                                    "Run Deep Diagnostics"
                                }
                            }
                        }

                        button {
                            style: "padding: 8px; border-radius: 4px; border: none; background: #334155; color: white; cursor: pointer;",
                            onclick: move |_| {
                                debug_log.set(Vec::new());
                            },
                            "Clear Debug Log"
                        }
                    }
                }

                p { style: "margin-top: 12px; color: #9ec9ff;", "{generation_status}" }
                p { style: "margin-top: 6px; color: #93c5fd;", "Shape renderer: {shape_render_mode}" }
                p { style: "margin-top: 6px; color: #93c5fd;", "Fast render mode: {fast_mode_label}" }
                p { style: "margin-top: 6px; color: #a5b4fc;", "Last rendered points: {last_total_points}" }

                if let Some(ms) = last_elapsed_ms() {
                    p { style: "margin-top: 4px; color: #93c5fd;", "Last generation time: {ms:.0} ms" }
                }

                if let Some(error) = map_error() {
                    p { style: "margin-top: 8px; color: #fca5a5;", "Map error: {error}" }
                }

                p { style: "margin-top: 12px; color: #93c5fd; font-weight: 600;", "Icon Install Payload" }
                pre {
                    "data-testid": "stress-icon-install-debug",
                    style: "margin-top: 6px; background: #020617; padding: 8px; border-radius: 4px; font-size: 11px; max-height: 140px; overflow-y: auto; white-space: pre-wrap;",
                    "{icon_install_debug}"
                }

                p { style: "margin-top: 12px; color: #93c5fd; font-weight: 600;", "Runtime Diagnostics Snapshot" }
                pre {
                    "data-testid": "stress-runtime-diagnostics",
                    style: "margin-top: 6px; background: #020617; padding: 8px; border-radius: 4px; font-size: 11px; max-height: 220px; overflow-y: auto; white-space: pre-wrap;",
                    "{diagnostics_json}"
                }

                p { style: "margin-top: 12px; color: #93c5fd; font-weight: 600;", "Debug Event Log" }
                pre {
                    "data-testid": "stress-debug-log",
                    style: "margin-top: 6px; background: #020617; padding: 8px; border-radius: 4px; font-size: 11px; max-height: 160px; overflow-y: auto; white-space: pre-wrap;",
                    {debug_log.read().join("\n")}
                }
            }
        }
    }
}
