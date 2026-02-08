//! Unit tests for event serialization/deserialization
#![allow(clippy::float_cmp, clippy::unreadable_literal)]

use dioxus_maplibre::{
    LatLng, MapClickEvent, MapContextMenuEvent, MapDblClickEvent, MapErrorEvent, MapEvent,
    MapMoveEvent, MapPitchEvent, MapRotateEvent, MapZoomEvent, MarkerClickEvent,
    MarkerDragEndEvent, MarkerDragStartEvent, MarkerHoverEvent, Point,
};

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
fn map_dblclick_event_roundtrip() {
    let event = MapDblClickEvent {
        latlng: LatLng::new(61.5, 23.79),
        point: Point::new(50.0, 75.0),
    };
    let json = serde_json::to_string(&event).unwrap();
    let restored: MapDblClickEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.latlng.lat, 61.5);
    assert_eq!(restored.point.x, 50.0);
}

#[test]
fn map_contextmenu_event_roundtrip() {
    let event = MapContextMenuEvent {
        latlng: LatLng::new(62.0, 25.0),
        point: Point::new(200.0, 300.0),
    };
    let json = serde_json::to_string(&event).unwrap();
    let restored: MapContextMenuEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.latlng.lng, 25.0);
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
fn marker_hover_event_roundtrip() {
    let event = MarkerHoverEvent {
        marker_id: "m1".to_string(),
        latlng: LatLng::new(60.0, 24.0),
        hover: true,
        cursor_x: 100.0,
        cursor_y: 200.0,
    };
    let json = serde_json::to_string(&event).unwrap();
    let restored: MarkerHoverEvent = serde_json::from_str(&json).unwrap();
    assert!(restored.hover);
    assert_eq!(restored.cursor_x, 100.0);
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
        bounds: None,
        phase: None,
    };

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("10"));
}

#[test]
fn map_event_ready_deserialize() {
    let event: MapEvent = serde_json::from_str(r#"{ "type": "ready" }"#).unwrap();
    assert!(matches!(event, MapEvent::Ready));
}

#[test]
fn map_event_error_deserialize() {
    let event: MapEvent =
        serde_json::from_str(r#"{ "type": "error", "message": "MapLibre GL JS not loaded" }"#)
            .unwrap();
    let MapEvent::Error(MapErrorEvent { message }) = event else {
        panic!("expected error variant");
    };
    assert_eq!(message.as_deref(), Some("MapLibre GL JS not loaded"));
}

#[test]
fn map_zoom_event_roundtrip() {
    let event = MapZoomEvent { zoom: 15.5 };
    let json = serde_json::to_string(&event).unwrap();
    let restored: MapZoomEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.zoom, 15.5);
}

#[test]
fn map_rotate_event_roundtrip() {
    let event = MapRotateEvent { bearing: 45.0 };
    let json = serde_json::to_string(&event).unwrap();
    let restored: MapRotateEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.bearing, 45.0);
}

#[test]
fn map_pitch_event_roundtrip() {
    let event = MapPitchEvent { pitch: 60.0 };
    let json = serde_json::to_string(&event).unwrap();
    let restored: MapPitchEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.pitch, 60.0);
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

#[test]
fn marker_dragstart_event_roundtrip() {
    let event = MarkerDragStartEvent {
        marker_id: "drag-1".to_string(),
        latlng: LatLng::new(60.17, 24.94),
    };
    let json = serde_json::to_string(&event).unwrap();
    let restored: MarkerDragStartEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.marker_id, "drag-1");
    assert_eq!(restored.latlng.lat, 60.17);
}

#[test]
fn marker_dragend_event_roundtrip() {
    let event = MarkerDragEndEvent {
        marker_id: "drag-2".to_string(),
        latlng: LatLng::new(61.5, 23.79),
    };
    let json = serde_json::to_string(&event).unwrap();
    let restored: MarkerDragEndEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.marker_id, "drag-2");
    assert_eq!(restored.latlng.lng, 23.79);
}

#[test]
fn marker_dragend_from_js_format() {
    let json = r#"{
        "marker_id": "m42",
        "latlng": {"lat": 60.2, "lng": 24.8}
    }"#;
    let event: MarkerDragEndEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.marker_id, "m42");
    assert_eq!(event.latlng.lat, 60.2);
    assert_eq!(event.latlng.lng, 24.8);
}
