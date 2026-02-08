//! Declarative map object components built on top of `MapHandle`.

use dioxus::prelude::*;

use crate::options::{
    ControlPosition, GeoJsonSourceOptions, ImageSourceOptions, LayerOptions, MarkerOptions,
    PopupOptions, RasterDemSourceOptions, RasterSourceOptions, VectorSourceOptions,
};
use crate::types::LatLng;

use super::context::use_map_handle;

#[derive(Debug, Clone, PartialEq)]
pub enum MapSourceKind {
    GeoJson(GeoJsonSourceOptions),
    Vector(VectorSourceOptions),
    Raster(RasterSourceOptions),
    RasterDem(RasterDemSourceOptions),
    Image(ImageSourceOptions),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapControlKind {
    Navigation,
    Geolocate,
    Scale,
    Fullscreen,
    Attribution,
}

/// Declaratively mount a source and remove it on unmount.
#[derive(Props, Clone, PartialEq)]
pub struct MapSourceProps {
    pub id: String,
    pub source: MapSourceKind,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn MapSource(props: MapSourceProps) -> Element {
    let handle = use_map_handle();
    let mut installed = use_signal(|| false);

    {
        let handle = handle.clone();
        let id = props.id.clone();
        let source = props.source.clone();
        use_effect(move || {
            if installed() {
                return;
            }
            let Some(map) = handle.clone() else {
                return;
            };
            match &source {
                MapSourceKind::GeoJson(options) => map.add_geojson_source(&id, options.clone()),
                MapSourceKind::Vector(options) => map.add_vector_source(&id, options.clone()),
                MapSourceKind::Raster(options) => map.add_raster_source(&id, options.clone()),
                MapSourceKind::RasterDem(options) => {
                    map.add_raster_dem_source(&id, options.clone())
                }
                MapSourceKind::Image(options) => map.add_image_source(&id, options.clone()),
            }
            installed.set(true);
        });
    }

    {
        let handle = handle.clone();
        let id = props.id.clone();
        use_drop(move || {
            if let Some(map) = handle {
                map.remove_source(&id);
            }
        });
    }

    rsx! {{props.children}}
}

/// Declaratively mount a layer and remove it on unmount.
#[derive(Props, Clone, PartialEq)]
pub struct MapLayerProps {
    pub options: LayerOptions,
    #[props(default = false)]
    pub register_click_events: bool,
    #[props(default = false)]
    pub register_hover_events: bool,
}

#[component]
pub fn MapLayer(props: MapLayerProps) -> Element {
    let handle = use_map_handle();
    let mut installed = use_signal(|| false);

    {
        let handle = handle.clone();
        let options = props.options.clone();
        let register_click = props.register_click_events;
        let register_hover = props.register_hover_events;
        use_effect(move || {
            if installed() {
                return;
            }
            let Some(map) = handle.clone() else {
                return;
            };
            let layer_id = options.id.clone();
            map.add_layer(options.clone());
            if register_click {
                map.on_layer_click(&layer_id);
            }
            if register_hover {
                map.on_layer_hover(&layer_id);
            }
            installed.set(true);
        });
    }

    {
        let handle = handle.clone();
        let layer_id = props.options.id.clone();
        use_drop(move || {
            if let Some(map) = handle {
                map.remove_layer(&layer_id);
            }
        });
    }

    rsx! {}
}

/// Declaratively mount a marker and remove it on unmount.
#[derive(Props, Clone, PartialEq)]
pub struct MapMarkerProps {
    pub id: String,
    pub position: LatLng,
    #[props(default)]
    pub options: MarkerOptions,
}

#[component]
pub fn MapMarker(props: MapMarkerProps) -> Element {
    let handle = use_map_handle();
    let mut installed = use_signal(|| false);
    let mut last_position = use_signal(|| props.position);

    {
        let handle = handle.clone();
        let id = props.id.clone();
        let options = props.options.clone();
        let position = props.position;
        use_effect(move || {
            let Some(map) = handle.clone() else {
                return;
            };
            if !installed() {
                map.add_marker(&id, position, options.clone());
                last_position.set(position);
                installed.set(true);
                return;
            }
            if last_position() != position {
                map.update_marker_position(&id, position);
                last_position.set(position);
            }
        });
    }

    {
        let handle = handle.clone();
        let id = props.id.clone();
        use_drop(move || {
            if let Some(map) = handle {
                map.remove_marker(&id);
            }
        });
    }

    rsx! {}
}

/// Declaratively mount a popup and remove it on unmount.
#[derive(Props, Clone, PartialEq)]
pub struct MapPopupProps {
    pub id: String,
    pub position: LatLng,
    pub html: String,
    #[props(default)]
    pub options: PopupOptions,
}

#[component]
pub fn MapPopup(props: MapPopupProps) -> Element {
    let handle = use_map_handle();
    let mut installed = use_signal(|| false);

    {
        let handle = handle.clone();
        let id = props.id.clone();
        let position = props.position;
        let html = props.html.clone();
        let options = props.options.clone();
        use_effect(move || {
            if installed() {
                return;
            }
            let Some(map) = handle.clone() else {
                return;
            };
            map.add_popup(&id, position, &html, options.clone());
            installed.set(true);
        });
    }

    {
        let handle = handle.clone();
        let id = props.id.clone();
        use_drop(move || {
            if let Some(map) = handle {
                map.remove_popup(&id);
            }
        });
    }

    rsx! {}
}

/// Declaratively add a control to the map.
#[derive(Props, Clone, PartialEq)]
pub struct MapControlProps {
    pub kind: MapControlKind,
    #[props(default)]
    pub position: ControlPosition,
}

#[component]
pub fn MapControl(props: MapControlProps) -> Element {
    let handle = use_map_handle();
    let mut installed = use_signal(|| false);

    {
        let handle = handle.clone();
        let kind = props.kind;
        let position = props.position;
        use_effect(move || {
            if installed() {
                return;
            }
            let Some(map) = handle.clone() else {
                return;
            };
            match kind {
                MapControlKind::Navigation => map.add_navigation_control(position),
                MapControlKind::Geolocate => map.add_geolocate_control(position),
                MapControlKind::Scale => map.add_scale_control(position),
                MapControlKind::Fullscreen => map.add_fullscreen_control(position),
                MapControlKind::Attribution => map.add_attribution_control(position),
            }
            installed.set(true);
        });
    }

    rsx! {}
}
