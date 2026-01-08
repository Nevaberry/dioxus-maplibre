//! Unit tests for event serialization/deserialization

use dioxus_maplibre::{MapClickEvent, MarkerClickEvent, MapMoveEvent, LatLng, Point};

#[test]
fn map_click_event_deserialize() {
    let json = r#"{
        "latlng": {"lat": 60.17, "lng": 24.94},
        "point": {"x": 100.0, "y": 200.0}
    }"#;

    let event: MapClickEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.latlng.lat, 60.17);
    assert_eq!(event.latlng.lng, 24.94);
    assert_eq!(event.point.x, 100.0);
    assert_eq!(event.point.y, 200.0);
}

#[test]
fn map_click_event_serialize() {
    let event = MapClickEvent {
        latlng: LatLng::new(60.17, 24.94),
        point: Point::new(100.0, 200.0),
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("60.17"));
    assert!(json.contains("24.94"));
}

#[test]
fn marker_click_event_deserialize() {
    let json = r#"{
        "marker_id": "marker_123",
        "latlng": {"lat": 60.17, "lng": 24.94}
    }"#;

    let event: MarkerClickEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.marker_id, "marker_123");
    assert_eq!(event.latlng.lat, 60.17);
}

#[test]
fn marker_click_event_serialize() {
    let event = MarkerClickEvent {
        marker_id: "marker_456".to_string(),
        latlng: LatLng::new(61.5, 23.79),
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("marker_456"));
    assert!(json.contains("61.5"));
}

#[test]
fn map_move_event_deserialize() {
    let json = r#"{
        "center": {"lat": 60.17, "lng": 24.94},
        "zoom": 12.5
    }"#;

    let event: MapMoveEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.center.lat, 60.17);
    assert_eq!(event.zoom, 12.5);
}

#[test]
fn map_move_event_serialize() {
    let event = MapMoveEvent {
        center: LatLng::new(60.17, 24.94),
        zoom: 10.0,
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("10"));
}

#[test]
fn latlng_json_roundtrip() {
    let original = LatLng::new(60.123456, 24.654321);
    let json = serde_json::to_string(&original).unwrap();
    let restored: LatLng = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

#[test]
fn point_json_roundtrip() {
    let original = Point::new(123.456, 789.012);
    let json = serde_json::to_string(&original).unwrap();
    let restored: Point = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}
