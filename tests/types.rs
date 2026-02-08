//! Unit tests for geographic types
#![allow(clippy::float_cmp)]

use dioxus_maplibre::{Bounds, LatLng, MapPosition, Point, QueryFeature};

#[test]
fn latlng_new() {
    let pos = LatLng::new(60.17, 24.94);
    assert_eq!(pos.lat, 60.17);
    assert_eq!(pos.lng, 24.94);
}

#[test]
fn latlng_helsinki() {
    let pos = LatLng::helsinki();
    assert!((pos.lat - 60.1699).abs() < 0.0001);
    assert!((pos.lng - 24.9384).abs() < 0.0001);
}

#[test]
fn latlng_to_array() {
    let pos = LatLng::new(60.17, 24.94);
    let arr = pos.to_array();
    // MapLibre uses [lng, lat] order
    assert_eq!(arr, [24.94, 60.17]);
}

#[test]
fn latlng_from_array() {
    // MapLibre uses [lng, lat] order
    let pos = LatLng::from_array([24.94, 60.17]);
    assert_eq!(pos.lat, 60.17);
    assert_eq!(pos.lng, 24.94);
}

#[test]
fn latlng_default() {
    let pos = LatLng::default();
    assert_eq!(pos.lat, 0.0);
    assert_eq!(pos.lng, 0.0);
}

#[test]
fn latlng_equality() {
    let a = LatLng::new(60.17, 24.94);
    let b = LatLng::new(60.17, 24.94);
    let c = LatLng::new(60.18, 24.94);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn latlng_clone() {
    let a = LatLng::new(60.17, 24.94);
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn map_position_new() {
    let center = LatLng::new(60.17, 24.94);
    let pos = MapPosition::new(center, 10.0);
    assert_eq!(pos.center.lat, 60.17);
    assert_eq!(pos.zoom, 10.0);
}

#[test]
fn map_position_default() {
    let pos = MapPosition::default();
    // Default uses Helsinki
    assert!((pos.center.lat - 60.1699).abs() < 0.0001);
    assert_eq!(pos.zoom, 10.0);
}

#[test]
fn bounds_new() {
    let sw = LatLng::new(60.0, 24.0);
    let ne = LatLng::new(61.0, 25.0);
    let bounds = Bounds::new(sw, ne);
    assert_eq!(bounds.sw.lat, 60.0);
    assert_eq!(bounds.ne.lat, 61.0);
}

#[test]
fn bounds_contains_inside() {
    let sw = LatLng::new(60.0, 24.0);
    let ne = LatLng::new(61.0, 25.0);
    let bounds = Bounds::new(sw, ne);

    let inside = LatLng::new(60.5, 24.5);
    assert!(bounds.contains(&inside));
}

#[test]
fn bounds_contains_outside() {
    let sw = LatLng::new(60.0, 24.0);
    let ne = LatLng::new(61.0, 25.0);
    let bounds = Bounds::new(sw, ne);

    let outside = LatLng::new(59.0, 24.5);
    assert!(!bounds.contains(&outside));
}

#[test]
fn bounds_contains_edge() {
    let sw = LatLng::new(60.0, 24.0);
    let ne = LatLng::new(61.0, 25.0);
    let bounds = Bounds::new(sw, ne);

    // Edges should be included
    let on_edge = LatLng::new(60.0, 24.5);
    assert!(bounds.contains(&on_edge));
}

#[test]
fn bounds_center() {
    let sw = LatLng::new(60.0, 24.0);
    let ne = LatLng::new(62.0, 26.0);
    let bounds = Bounds::new(sw, ne);

    let center = bounds.center();
    assert_eq!(center.lat, 61.0);
    assert_eq!(center.lng, 25.0);
}

#[test]
fn point_new() {
    let p = Point::new(100.0, 200.0);
    assert_eq!(p.x, 100.0);
    assert_eq!(p.y, 200.0);
}

#[test]
fn point_default() {
    let p = Point::default();
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
}

#[test]
fn point_equality() {
    let a = Point::new(100.0, 200.0);
    let b = Point::new(100.0, 200.0);
    let c = Point::new(100.0, 201.0);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn query_feature_deserialize() {
    let json = r#"{
        "id": 42,
        "geometry": {"type": "Point", "coordinates": [24.94, 60.17]},
        "properties": {"name": "Helsinki"},
        "source": "cities",
        "sourceLayer": "places"
    }"#;
    let feature: QueryFeature = serde_json::from_str(json).unwrap();
    assert_eq!(feature.id, Some(42));
    assert_eq!(feature.source, "cities");
    assert_eq!(feature.source_layer.as_deref(), Some("places"));
}

#[test]
fn query_feature_without_optional_fields() {
    let json = r#"{
        "geometry": {"type": "Point", "coordinates": [24.94, 60.17]},
        "properties": {},
        "source": "my-source"
    }"#;
    let feature: QueryFeature = serde_json::from_str(json).unwrap();
    assert_eq!(feature.id, None);
    assert_eq!(feature.source_layer, None);
    assert_eq!(feature.source, "my-source");
}
