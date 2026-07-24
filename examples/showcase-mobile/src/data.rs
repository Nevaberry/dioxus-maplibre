use dioxus_maplibre::LatLng;
use serde_json::{Value, json};

use crate::state::{Connectivity, SceneKind};

pub const ONLINE_DARK_STYLE: &str =
    "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json";
pub const ONLINE_LIGHT_STYLE: &str =
    "https://basemaps.cartocdn.com/gl/positron-gl-style/style.json";
pub const TERRAIN_TILES: &str =
    "https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png";
pub const OFFLINE_TERRAIN_TILE: &str = "/offline/terrain-tile.png";

pub const fn scene_style(scene: SceneKind, connectivity: Connectivity) -> &'static str {
    match connectivity {
        Connectivity::Online => ONLINE_DARK_STYLE,
        Connectivity::Offline => scene.offline_style(),
    }
}

pub fn scene_center(scene: SceneKind) -> LatLng {
    match scene {
        SceneKind::Helsinki | SceneKind::Buildings => LatLng::new(60.1699, 24.9384),
        SceneKind::Matterhorn => LatLng::new(45.9763, 7.6586),
        SceneKind::Tokyo => LatLng::new(35.6812, 139.7671),
    }
}

pub fn helsinki_places() -> Value {
    json!({
        "type": "FeatureCollection",
        "features": [
            point("senate", "Senate Square", "Landmark", 24.9524, 60.1695, 18),
            point("station", "Central Station", "Transit", 24.9414, 60.1719, 12),
            point("market", "Market Square", "Market", 24.9554, 60.1677, 9),
            point("library", "Oodi Library", "Culture", 24.9386, 60.1730, 8),
            point("temppeliaukio", "Rock Church", "Landmark", 24.9259, 60.1729, 7),
            point("kaivopuisto", "Kaivopuisto", "Park", 24.9557, 60.1554, 6),
            point("suomenlinna", "Suomenlinna", "Island", 24.9870, 60.1450, 5),
            point("kallio", "Kallio", "District", 24.9509, 60.1843, 11),
            point("töölö", "Töölö Bay", "Nature", 24.9325, 60.1811, 4),
            point("design", "Design Museum", "Culture", 24.9459, 60.1631, 3),
            point("hakaniemi", "Hakaniemi", "Market", 24.9501, 60.1798, 10),
            point("punavuori", "Punavuori", "District", 24.9368, 60.1605, 6)
        ]
    })
}

fn point(id: &str, name: &str, category: &str, lng: f64, lat: f64, weight: u8) -> Value {
    json!({
        "type": "Feature",
        "id": id,
        "properties": { "name": name, "category": category, "weight": weight },
        "geometry": { "type": "Point", "coordinates": [lng, lat] }
    })
}

pub fn building_data() -> Value {
    json!({
        "type": "FeatureCollection",
        "features": [
            building("cathedral", "Helsinki Cathedral", 62, "Landmark", [[
                [24.9504, 60.1698], [24.9533, 60.1698], [24.9533, 60.1711],
                [24.9504, 60.1711], [24.9504, 60.1698]
            ]]),
            building("station", "Central Station", 48, "Transit", [[
                [24.9393, 60.1708], [24.9432, 60.1708], [24.9432, 60.1727],
                [24.9393, 60.1727], [24.9393, 60.1708]
            ]]),
            building("oodi", "Oodi Library", 32, "Culture", [[
                [24.9368, 60.1724], [24.9400, 60.1724], [24.9400, 60.1737],
                [24.9368, 60.1737], [24.9368, 60.1724]
            ]]),
            building("museum", "Design Museum", 26, "Culture", [[
                [24.9447, 60.1624], [24.9471, 60.1624], [24.9471, 60.1637],
                [24.9447, 60.1637], [24.9447, 60.1624]
            ]]),
            building("tower", "Harbour Tower", 84, "Office", [[
                [24.9580, 60.1647], [24.9600, 60.1647], [24.9600, 60.1662],
                [24.9580, 60.1662], [24.9580, 60.1647]
            ]])
        ]
    })
}

fn building(
    id: &str,
    name: &str,
    height: u16,
    category: &str,
    coordinates: [[[f64; 2]; 5]; 1],
) -> Value {
    json!({
        "type": "Feature",
        "id": id,
        "properties": {
            "name": name,
            "height": height,
            "base_height": 0,
            "category": category
        },
        "geometry": { "type": "Polygon", "coordinates": coordinates }
    })
}

pub fn tokyo_heat_data() -> Value {
    let center = scene_center(SceneKind::Tokyo);
    let mut features = Vec::with_capacity(180);

    for index in 0..180 {
        let angle = f64::from(index) * 2.399_963;
        let ring = f64::from(index).sqrt() * 0.001_15;
        let wave = 0.55 + f64::from(index % 9) / 12.0;
        features.push(json!({
            "type": "Feature",
            "id": index,
            "properties": { "weight": wave, "name": format!("Signal {}", index + 1) },
            "geometry": {
                "type": "Point",
                "coordinates": [
                    center.lng + ring * angle.cos(),
                    center.lat + ring * angle.sin()
                ]
            }
        }));
    }

    json!({ "type": "FeatureCollection", "features": features })
}

pub fn interaction_areas() -> Value {
    json!({
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "id": "harbour",
                "properties": { "name": "South Harbour", "category": "Waterfront" },
                "geometry": { "type": "Polygon", "coordinates": [[
                    [24.946, 60.164], [24.958, 60.164], [24.958, 60.170],
                    [24.946, 60.170], [24.946, 60.164]
                ]] }
            },
            {
                "type": "Feature",
                "id": "center",
                "properties": { "name": "City Centre", "category": "District" },
                "geometry": { "type": "Polygon", "coordinates": [[
                    [24.932, 60.168], [24.946, 60.168], [24.946, 60.176],
                    [24.932, 60.176], [24.932, 60.168]
                ]] }
            }
        ]
    })
}

pub fn terrain_contours() -> Value {
    let mut features = Vec::new();
    for (index, offset) in [-0.035_f64, -0.024, -0.014, -0.004, 0.008, 0.020]
        .into_iter()
        .enumerate()
    {
        features.push(json!({
            "type": "Feature",
            "properties": { "elevation": 2200 + index * 280 },
            "geometry": {
                "type": "LineString",
                "coordinates": [
                    [7.60, 45.94 + offset], [7.625, offset.mul_add(0.4, 45.955)],
                    [7.65, offset.mul_add(0.2, 45.968)], [7.675, offset.mul_add(0.5, 45.96)],
                    [7.705, 45.945 + offset]
                ]
            }
        }));
    }
    json!({ "type": "FeatureCollection", "features": features })
}

pub fn archipelago_selection() -> Value {
    json!({
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "id": "download-area",
            "properties": { "name": "Selected area" },
            "geometry": { "type": "Polygon", "coordinates": [[
                [24.70, 59.96], [25.18, 59.96], [25.18, 60.18],
                [24.70, 60.18], [24.70, 59.96]
            ]] }
        }]
    })
}
