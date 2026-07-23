//! Unit tests for option type serialization
#![allow(clippy::float_cmp)]

use dioxus_maplibre::{
    CanvasSourceOptions, ControlOptions, ControlPosition, EaseToOptions, FeatureIdentifier,
    FitBoundsOptions, FlyToOptions, FogOptions, GeoJsonSourceOptions, JumpToOptions, LatLng,
    LayerOptions, MapOptions, MarkerOptions, MissingImageOptions, Padding, PopupOptions,
    ProjectionOptions, QueryOptions, RasterDemSourceOptions, RasterSourceOptions, SkyOptions,
    TerrainControlOptions, TerrainOptions, VectorSourceOptions, VideoSourceOptions,
};
use serde_json::json;

#[test]
fn control_position_serializes_to_kebab_case() {
    let json = serde_json::to_string(&ControlPosition::TopLeft).unwrap();
    assert_eq!(json, r#""top-left""#);

    let json = serde_json::to_string(&ControlPosition::BottomRight).unwrap();
    assert_eq!(json, r#""bottom-right""#);
}

#[test]
fn padding_uniform() {
    let p = Padding::uniform(10.0);
    assert_eq!(p.top, 10.0);
    assert_eq!(p.bottom, 10.0);
    assert_eq!(p.left, 10.0);
    assert_eq!(p.right, 10.0);
}

#[test]
fn geojson_source_options_minimal() {
    let opts = GeoJsonSourceOptions {
        data: json!({"type": "FeatureCollection", "features": []}),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("FeatureCollection"));
    // Optional fields should not appear
    assert!(!json.contains("cluster"));
    assert!(!json.contains("generateId"));
}

#[test]
fn geojson_source_options_with_clustering() {
    let opts = GeoJsonSourceOptions {
        data: json!({"type": "FeatureCollection", "features": []}),
        cluster: Some(true),
        cluster_radius: Some(50),
        cluster_max_zoom: Some(14),
        generate_id: Some(true),
        max_zoom: Some(18),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains(r#""cluster":true"#));
    assert!(json.contains(r#""clusterRadius":50"#));
    assert!(json.contains(r#""clusterMaxZoom":14"#));
    assert!(json.contains(r#""generateId":true"#));
    assert!(json.contains(r#""maxzoom":18"#));
}

#[test]
fn vector_source_options_serialization() {
    let opts = VectorSourceOptions {
        url: Some("https://example.com/tiles.json".to_string()),
        min_zoom: Some(0),
        max_zoom: Some(14),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("example.com"));
    assert!(json.contains(r#""minzoom":0"#));
    assert!(json.contains(r#""maxzoom":14"#));
    assert!(!json.contains("minZoom"));
    assert!(!json.contains("maxZoom"));
}

#[test]
fn raster_source_options_serialization() {
    let opts = RasterSourceOptions {
        tiles: Some(vec!["https://example.com/{z}/{x}/{y}.png".to_string()]),
        tile_size: Some(256),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains(r#""tileSize":256"#));
}

#[test]
fn raster_dem_source_options_serialization() {
    let opts = RasterDemSourceOptions {
        url: Some("https://example.com/dem.json".to_string()),
        encoding: Some("terrarium".to_string()),
        max_zoom: Some(12),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("terrarium"));
    assert!(json.contains(r#""maxzoom":12"#));
}

#[test]
fn layer_options_builder_circle() {
    let layer = LayerOptions::circle("my-circles", "my-source")
        .paint(json!({"circle-radius": 6, "circle-color": "#3b82f6"}))
        .filter(json!(["==", ["get", "type"], "point"]));

    assert_eq!(layer.id, "my-circles");
    assert_eq!(layer.layer_type, "circle");
    assert_eq!(layer.source.as_deref(), Some("my-source"));
    assert!(layer.paint.is_some());
    assert!(layer.filter.is_some());
    assert!(layer.layout.is_none());
}

#[test]
fn layer_options_builder_fill() {
    let layer = LayerOptions::fill("my-fill", "polygons")
        .paint(json!({"fill-color": "#888", "fill-opacity": 0.4}))
        .min_zoom(5.0)
        .max_zoom(15.0);

    assert_eq!(layer.layer_type, "fill");
    assert_eq!(layer.min_zoom, Some(5.0));
    assert_eq!(layer.max_zoom, Some(15.0));
}

#[test]
fn layer_options_builder_line() {
    let layer = LayerOptions::line("route", "route-source")
        .paint(json!({"line-color": "#f00", "line-width": 3}))
        .layout(json!({"line-cap": "round", "line-join": "round"}));

    assert_eq!(layer.layer_type, "line");
    assert!(layer.layout.is_some());
}

#[test]
fn layer_options_background() {
    let layer = LayerOptions::background("bg").paint(json!({"background-color": "#000"}));

    assert!(layer.source.is_none());
    assert_eq!(layer.layer_type, "background");
}

#[test]
fn all_maplibre_layer_types_have_builders() {
    assert_eq!(LayerOptions::hillshade("h", "dem").layer_type, "hillshade");
    assert_eq!(
        LayerOptions::color_relief("r", "dem").layer_type,
        "color-relief"
    );
}

#[test]
fn map_and_projection_passthrough_options() {
    let map = MapOptions(json!({"maxPitch": 85, "rollEnabled": true}));
    let serialized = serde_json::to_value(map).unwrap();
    assert_eq!(serialized["maxPitch"], 85);
    assert_eq!(ProjectionOptions::globe().0["type"], "globe");
    assert_eq!(ProjectionOptions::mercator().0["type"], "mercator");
    assert_eq!(
        ProjectionOptions::vertical_perspective().0["type"],
        "vertical-perspective"
    );
}

#[test]
fn media_source_options_serialize() {
    let coordinates = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
    let video = VideoSourceOptions {
        urls: vec!["demo.webm".into()],
        coordinates,
    };
    let canvas = CanvasSourceOptions {
        canvas: "demo-canvas".into(),
        coordinates,
        animate: Some(false),
    };
    assert_eq!(serde_json::to_value(video).unwrap()["urls"][0], "demo.webm");
    assert_eq!(
        serde_json::to_value(canvas).unwrap()["canvas"],
        "demo-canvas"
    );
}

#[test]
fn control_and_missing_image_options_serialize() {
    assert_eq!(
        serde_json::to_string(&ControlOptions::default()).unwrap(),
        "{}"
    );
    let terrain = TerrainControlOptions {
        source: "dem".into(),
        exaggeration: Some(1.5),
    };
    assert_eq!(serde_json::to_value(terrain).unwrap()["source"], "dem");
    assert_eq!(MissingImageOptions::default().cell_size, 8);
}

#[test]
fn layer_options_with_source_layer() {
    let layer = LayerOptions::fill("countries", "openmaptiles").source_layer("boundary");

    assert_eq!(layer.source_layer.as_deref(), Some("boundary"));
}

#[test]
fn layer_options_serialization_camel_case() {
    let layer = LayerOptions::circle("test", "src")
        .source_layer("points")
        .min_zoom(3.0)
        .max_zoom(18.0);

    let json = serde_json::to_string(&layer).unwrap();
    assert!(json.contains(r#""source-layer":"points""#));
    assert!(json.contains(r#""minzoom":3"#));
    assert!(json.contains(r#""maxzoom":18"#));
    assert!(!json.contains("sourceLayer"));
    assert!(json.contains(r#""type":"circle""#));
}

#[test]
fn marker_options_default() {
    let opts = MarkerOptions::default();
    let json = serde_json::to_string(&opts).unwrap();
    // Default should serialize to empty object (all fields None)
    assert_eq!(json, "{}");
}

#[test]
fn marker_options_with_values() {
    let opts = MarkerOptions {
        color: Some("#ff0000".to_string()),
        draggable: Some(true),
        emoji: Some("📍".to_string()),
        popup_html: Some("<b>Hello</b>".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("#ff0000"));
    assert!(json.contains(r#""draggable":true"#));
}

#[test]
fn popup_options_serialization() {
    let opts = PopupOptions {
        offset: Some([0.0, -25.0]),
        close_button: Some(false),
        max_width: Some("300px".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains(r#""closeButton":false"#));
    assert!(json.contains(r#""maxWidth":"300px""#));
}

#[test]
fn fly_to_options_with_center() {
    let opts = FlyToOptions {
        center: Some(LatLng::new(60.17, 24.94)),
        zoom: Some(12.0),
        duration: Some(2000),
        essential: Some(true),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("60.17"));
    assert!(json.contains(r#""essential":true"#));
    assert!(json.contains(r#""duration":2000"#));
}

#[test]
fn ease_to_options_serialization() {
    let opts = EaseToOptions {
        bearing: Some(45.0),
        pitch: Some(60.0),
        duration: Some(1000),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains(r#""bearing":45"#));
    assert!(json.contains(r#""pitch":60"#));
}

#[test]
fn jump_to_options_serialization() {
    let opts = JumpToOptions {
        center: Some(LatLng::new(61.0, 24.0)),
        zoom: Some(8.0),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("61"));
}

#[test]
fn fit_bounds_options_with_padding() {
    let opts = FitBoundsOptions {
        padding: Some(Padding::uniform(50.0)),
        max_zoom: Some(15.0),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains(r#""maxZoom":15"#));
    assert!(json.contains(r#""top":50"#));
}

#[test]
fn terrain_options_serialization() {
    let opts = TerrainOptions {
        source: "dem".to_string(),
        exaggeration: Some(1.5),
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains(r#""source":"dem""#));
    assert!(json.contains(r#""exaggeration":1.5"#));
}

#[test]
fn sky_options_passthrough() {
    let opts = SkyOptions(json!({
        "sky-color": "#199EF3",
        "sky-horizon-blend": 0.5
    }));
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("#199EF3"));
}

#[test]
fn fog_options_passthrough() {
    let opts = FogOptions(json!({
        "color": "white",
        "horizon-blend": 0.1,
        "star-intensity": 0.5,
        "range": [0.5, 10.0]
    }));
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("white"));
    assert!(json.contains("horizon-blend"));
    assert!(json.contains("star-intensity"));
    assert!(json.contains("range"));
}

#[test]
fn feature_identifier_serialization() {
    let feat = FeatureIdentifier {
        source: "my-source".to_string(),
        id: 42.into(),
        source_layer: Some("points".to_string()),
    };
    let json = serde_json::to_string(&feat).unwrap();
    assert!(json.contains(r#""source":"my-source""#));
    assert!(json.contains(r#""id":42"#));
    assert!(json.contains(r#""sourceLayer":"points""#));
}

#[test]
fn feature_identifier_without_source_layer() {
    let feat = FeatureIdentifier {
        source: "geojson-src".to_string(),
        id: 7.into(),
        source_layer: None,
    };
    let json = serde_json::to_string(&feat).unwrap();
    assert!(!json.contains("sourceLayer"));
}

#[test]
fn feature_identifier_supports_string_ids() {
    let feature = FeatureIdentifier::new("vehicles", "tram-42");
    assert_eq!(serde_json::to_value(feature).unwrap()["id"], "tram-42");
}

#[test]
fn query_options_default_empty() {
    let opts = QueryOptions::default();
    let json = serde_json::to_string(&opts).unwrap();
    assert_eq!(json, "{}");
}

#[test]
fn query_options_with_layers_and_filter() {
    let opts = QueryOptions {
        layers: Some(vec!["circles".to_string(), "lines".to_string()]),
        filter: Some(json!(["==", ["get", "active"], true])),
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("circles"));
    assert!(json.contains("lines"));
    assert!(json.contains("active"));
}
