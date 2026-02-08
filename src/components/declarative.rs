//! Declarative map object components built on top of `MapHandle`.

use dioxus::prelude::*;

use crate::handle::MapHandle;
use crate::options::{
    ControlPosition, GeoJsonSourceOptions, ImageSourceOptions, LayerOptions, MarkerOptions,
    PopupOptions, RasterDemSourceOptions, RasterSourceOptions, VectorSourceOptions,
};
use crate::types::LatLng;

use super::context::try_use_map_handle_signal;

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

#[derive(Debug, Clone, PartialEq)]
struct SourceState {
    id: String,
    source: MapSourceKind,
}

#[derive(Debug, Clone, PartialEq)]
struct LayerState {
    options: LayerOptions,
    register_click_events: bool,
    register_hover_events: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct MarkerState {
    id: String,
    position: LatLng,
    options: MarkerOptions,
}

#[derive(Debug, Clone, PartialEq)]
struct PopupState {
    id: String,
    position: LatLng,
    html: String,
    options: PopupOptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ControlState {
    kind: MapControlKind,
    position: ControlPosition,
}

fn add_source(map: &MapHandle, source: &SourceState) {
    match &source.source {
        MapSourceKind::GeoJson(options) => map.add_geojson_source(&source.id, options.clone()),
        MapSourceKind::Vector(options) => map.add_vector_source(&source.id, options.clone()),
        MapSourceKind::Raster(options) => map.add_raster_source(&source.id, options.clone()),
        MapSourceKind::RasterDem(options) => map.add_raster_dem_source(&source.id, options.clone()),
        MapSourceKind::Image(options) => map.add_image_source(&source.id, options.clone()),
    }
}

fn try_update_geojson_source(map: &MapHandle, previous: &SourceState, next: &SourceState) -> bool {
    if previous.id != next.id {
        return false;
    }
    let (MapSourceKind::GeoJson(previous_opts), MapSourceKind::GeoJson(next_opts)) =
        (&previous.source, &next.source)
    else {
        return false;
    };

    if previous_opts == next_opts {
        return true;
    }

    let unchanged_non_data_fields = previous_opts.cluster == next_opts.cluster
        && previous_opts.cluster_radius == next_opts.cluster_radius
        && previous_opts.cluster_max_zoom == next_opts.cluster_max_zoom
        && previous_opts.cluster_properties == next_opts.cluster_properties
        && previous_opts.generate_id == next_opts.generate_id
        && previous_opts.promote_id == next_opts.promote_id;

    if unchanged_non_data_fields {
        map.update_geojson_source(&next.id, next_opts.data.clone());
        return true;
    }

    false
}

fn remove_layer_bindings(map: &MapHandle, layer: &LayerState) {
    if layer.register_click_events {
        map.off_layer_click(&layer.options.id);
    }
    if layer.register_hover_events {
        map.off_layer_hover(&layer.options.id);
    }
    map.remove_layer(&layer.options.id);
}

fn add_layer_bindings(map: &MapHandle, layer: &LayerState) {
    map.add_layer(layer.options.clone());
    if layer.register_click_events {
        map.on_layer_click(&layer.options.id);
    }
    if layer.register_hover_events {
        map.on_layer_hover(&layer.options.id);
    }
}

fn remove_control(map: &MapHandle, control: ControlState) {
    match control.kind {
        MapControlKind::Navigation => map.remove_navigation_control(control.position),
        MapControlKind::Geolocate => map.remove_geolocate_control(control.position),
        MapControlKind::Scale => map.remove_scale_control(control.position),
        MapControlKind::Fullscreen => map.remove_fullscreen_control(control.position),
        MapControlKind::Attribution => map.remove_attribution_control(control.position),
    }
}

fn add_control(map: &MapHandle, control: ControlState) {
    match control.kind {
        MapControlKind::Navigation => map.add_navigation_control(control.position),
        MapControlKind::Geolocate => map.add_geolocate_control(control.position),
        MapControlKind::Scale => map.add_scale_control(control.position),
        MapControlKind::Fullscreen => map.add_fullscreen_control(control.position),
        MapControlKind::Attribution => map.add_attribution_control(control.position),
    }
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
    let handle_signal = try_use_map_handle_signal();
    let mut applied_source = use_signal(|| None::<SourceState>);

    let desired_source = SourceState {
        id: props.id.clone(),
        source: props.source.clone(),
    };

    use_effect(move || {
        let Some(handle_signal) = handle_signal else {
            return;
        };
        let Some(map) = handle_signal() else {
            return;
        };

        let previous = applied_source.peek().clone();
        if previous.as_ref() == Some(&desired_source) {
            return;
        }

        if let Some(previous) = &previous {
            if try_update_geojson_source(&map, previous, &desired_source) {
                applied_source.set(Some(desired_source.clone()));
                return;
            }
            map.remove_source(&previous.id);
        }

        add_source(&map, &desired_source);
        applied_source.set(Some(desired_source.clone()));
    });

    use_drop(move || {
        if let Some(handle_signal) = handle_signal
            && let Some(map) = handle_signal.peek().clone()
            && let Some(source) = applied_source.peek().as_ref()
        {
            map.remove_source(&source.id);
        }
    });

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
    let handle_signal = try_use_map_handle_signal();
    let mut applied_layer = use_signal(|| None::<LayerState>);

    let desired_layer = LayerState {
        options: props.options.clone(),
        register_click_events: props.register_click_events,
        register_hover_events: props.register_hover_events,
    };

    use_effect(move || {
        let Some(handle_signal) = handle_signal else {
            return;
        };
        let Some(map) = handle_signal() else {
            return;
        };

        let previous = applied_layer.peek().clone();
        if previous.as_ref() == Some(&desired_layer) {
            return;
        }

        if let Some(previous) = &previous {
            remove_layer_bindings(&map, previous);
        }

        add_layer_bindings(&map, &desired_layer);
        applied_layer.set(Some(desired_layer.clone()));
    });

    use_drop(move || {
        if let Some(handle_signal) = handle_signal
            && let Some(map) = handle_signal.peek().clone()
            && let Some(layer) = applied_layer.peek().as_ref()
        {
            remove_layer_bindings(&map, layer);
        }
    });

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
    let handle_signal = try_use_map_handle_signal();
    let mut applied_marker = use_signal(|| None::<MarkerState>);

    let desired_marker = MarkerState {
        id: props.id,
        position: props.position,
        options: props.options,
    };

    use_effect(move || {
        let Some(handle_signal) = handle_signal else {
            return;
        };
        let Some(map) = handle_signal() else {
            return;
        };

        let previous = applied_marker.peek().clone();
        if previous.as_ref() == Some(&desired_marker) {
            return;
        }

        if let Some(previous) = &previous {
            if previous.id == desired_marker.id && previous.options == desired_marker.options {
                map.update_marker_position(&desired_marker.id, desired_marker.position);
                applied_marker.set(Some(desired_marker.clone()));
                return;
            }
            map.remove_marker(&previous.id);
        }

        map.add_marker(
            &desired_marker.id,
            desired_marker.position,
            desired_marker.options.clone(),
        );
        applied_marker.set(Some(desired_marker.clone()));
    });

    use_drop(move || {
        if let Some(handle_signal) = handle_signal
            && let Some(map) = handle_signal.peek().clone()
            && let Some(marker) = applied_marker.peek().as_ref()
        {
            map.remove_marker(&marker.id);
        }
    });

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
    let handle_signal = try_use_map_handle_signal();
    let mut applied_popup = use_signal(|| None::<PopupState>);

    let desired_popup = PopupState {
        id: props.id,
        position: props.position,
        html: props.html,
        options: props.options,
    };

    use_effect(move || {
        let Some(handle_signal) = handle_signal else {
            return;
        };
        let Some(map) = handle_signal() else {
            return;
        };

        let previous = applied_popup.peek().clone();
        if previous.as_ref() == Some(&desired_popup) {
            return;
        }

        if let Some(previous) = &previous {
            map.remove_popup(&previous.id);
        }

        map.add_popup(
            &desired_popup.id,
            desired_popup.position,
            &desired_popup.html,
            desired_popup.options.clone(),
        );
        applied_popup.set(Some(desired_popup.clone()));
    });

    use_drop(move || {
        if let Some(handle_signal) = handle_signal
            && let Some(map) = handle_signal.peek().clone()
            && let Some(popup) = applied_popup.peek().as_ref()
        {
            map.remove_popup(&popup.id);
        }
    });

    rsx! {}
}

/// Declaratively add a control to the map.
#[derive(Props, Clone, PartialEq, Eq)]
pub struct MapControlProps {
    pub kind: MapControlKind,
    #[props(default)]
    pub position: ControlPosition,
}

#[component]
pub fn MapControl(props: MapControlProps) -> Element {
    let handle_signal = try_use_map_handle_signal();
    let mut applied_control = use_signal(|| None::<ControlState>);

    let desired_control = ControlState {
        kind: props.kind,
        position: props.position,
    };

    use_effect(move || {
        let Some(handle_signal) = handle_signal else {
            return;
        };
        let Some(map) = handle_signal() else {
            return;
        };

        let previous = *applied_control.peek();
        if previous == Some(desired_control) {
            return;
        }

        if let Some(previous) = previous {
            remove_control(&map, previous);
        }

        add_control(&map, desired_control);
        applied_control.set(Some(desired_control));
    });

    use_drop(move || {
        if let Some(handle_signal) = handle_signal
            && let Some(map) = handle_signal.peek().clone()
            && let Some(control) = applied_control.peek().as_ref().copied()
        {
            remove_control(&map, control);
        }
    });

    rsx! {}
}
