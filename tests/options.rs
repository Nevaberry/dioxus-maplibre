//! Unit tests for option type serialization

use dioxus_maplibre::{
    ControlPosition, Padding,
    GeoJsonSourceOptions, VectorSourceOptions, RasterSourceOptions,
    RasterDemSourceOptions, LayerOptions, MarkerOptions, PopupOptions,
    FlyToOptions, EaseToOptions, JumpToOptions, FitBoundsOptions,
    TerrainOptions, SkyOptions, FeatureIdentifier, QueryOptions,
    LatLng,
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
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains(r#""cluster":true"#));
    assert!(json.contains(r#""clusterRadius":50"#));
    assert!(json.contains(r#""clusterMaxZoom":14"#));
    assert!(json.contains(r#""generateId":true"#));
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
    assert!(json.contains(r#""minZoom":0"#));
    assert!(json.contains(r#""maxZoom":14"#));
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
        ..Default::default()
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("terrarium"));
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
    let layer = LayerOptions::background("bg")
        .paint(json!({"background-color": "#000"}));

    assert!(layer.source.is_none());
    assert_eq!(layer.layer_type, "background");
}

#[test]
fn layer_options_with_source_layer() {
    let layer = LayerOptions::fill("countries", "openmaptiles")
        .source_layer("boundary");

    assert_eq!(layer.source_layer.as_deref(), Some("boundary"));
}

#[test]
fn layer_options_serialization_camel_case() {
    let layer = LayerOptions::circle("test", "src")
        .source_layer("points")
        .min_zoom(3.0)
        .max_zoom(18.0);

    let json = serde_json::to_string(&layer).unwrap();
    assert!(json.contains(r#""sourceLayer":"points""#));
    assert!(json.contains(r#""minZoom":3"#));
    assert!(json.contains(r#""maxZoom":18"#));
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
fn feature_identifier_serialization() {
    let feat = FeatureIdentifier {
        source: "my-source".to_string(),
        id: 42,
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
        id: 7,
        source_layer: None,
    };
    let json = serde_json::to_string(&feat).unwrap();
    assert!(!json.contains("sourceLayer"));
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
    };
    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("circles"));
    assert!(json.contains("lines"));
    assert!(json.contains("active"));
}
