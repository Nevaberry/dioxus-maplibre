use dioxus::prelude::*;
use dioxus_maplibre::{
    CanvasSourceOptions, ImageSourceOptions, LatLng, LayerOptions, Map, MapErrorEvent, MapHandle,
    VideoSourceOptions,
};
use serde_json::json;

#[component]
pub fn MediaSources() -> Element {
    let mut map_handle = use_signal(|| None::<MapHandle>);
    let mut video_playing = use_signal(|| true);
    let mut status = use_signal(|| "waiting".to_string());
    let style: Signal<String> = use_context();

    rsx! {
        div { style: "display: flex; height: 100%;",
            div { style: "flex: 1; position: relative;",
                canvas {
                    id: "showcase-source-canvas",
                    width: "256",
                    height: "256",
                    style: "display: none;",
                }
                Map {
                    style: style(),
                    center: LatLng::new(37.563, -122.5145),
                    zoom: 15.5,
                    bearing: -60.0,
                    on_ready: move |handle: MapHandle| {
                        handle.eval(r#"
                            const canvas = document.getElementById('showcase-source-canvas');
                            const ctx = canvas && canvas.getContext('2d');
                            if (ctx) {
                                const gradient = ctx.createLinearGradient(0, 0, 256, 256);
                                gradient.addColorStop(0, '#2563eb');
                                gradient.addColorStop(1, '#ec4899');
                                ctx.fillStyle = gradient;
                                ctx.fillRect(0, 0, 256, 256);
                                ctx.fillStyle = '#ffffff';
                                ctx.font = 'bold 32px sans-serif';
                                ctx.fillText('Canvas', 65, 135);
                            }
                        "#);
                        handle.add_canvas_source("canvas-media", CanvasSourceOptions {
                            canvas: "showcase-source-canvas".into(),
                            animate: Some(false),
                            coordinates: [
                                [-122.5200, 37.5660], [-122.5165, 37.5660],
                                [-122.5165, 37.5635], [-122.5200, 37.5635],
                            ],
                        });
                        handle.add_layer(LayerOptions::raster("canvas-media-layer", "canvas-media")
                            .paint(json!({ "raster-opacity": 0.9, "raster-fade-duration": 0 }))
                        );

                        handle.add_image_source("image-media", ImageSourceOptions {
                            url: "https://maplibre.org/maplibre-gl-js/docs/assets/radar.gif".into(),
                            coordinates: [
                                [-122.5160, 37.5660], [-122.5125, 37.5660],
                                [-122.5125, 37.5635], [-122.5160, 37.5635],
                            ],
                        });
                        handle.add_layer(LayerOptions::raster("image-media-layer", "image-media")
                            .paint(json!({ "raster-opacity": 0.82, "raster-fade-duration": 0 }))
                        );

                        handle.add_video_source("video-media", VideoSourceOptions {
                            urls: vec![
                                "https://static-assets.mapbox.com/mapbox-gl-js/drone.mp4".into(),
                                "https://static-assets.mapbox.com/mapbox-gl-js/drone.webm".into(),
                            ],
                            coordinates: [
                                [-122.51596391201019, 37.56238816766053],
                                [-122.51467645168304, 37.56410183312965],
                                [-122.51309394836426, 37.563391708549425],
                                [-122.51423120498657, 37.56161849366671],
                            ],
                        });
                        handle.add_layer(LayerOptions::raster("video-media-layer", "video-media")
                            .paint(json!({ "raster-opacity": 0.9, "raster-fade-duration": 0 }))
                        );
                        status.set("canvas + image + video registered".into());
                        map_handle.set(Some(handle));
                    },
                    on_error: move |event: MapErrorEvent| {
                        if let Some(message) = event.message {
                            status.set(format!("source error: {message}"));
                        }
                    },
                }
            }
            div { style: "width: 300px; background: #16213e; color: #e0e0e0; padding: 16px; font-size: 13px;",
                h3 { style: "margin: 0 0 12px 0;", "Media Sources" }
                p { "Canvas, image, and video sources rendered through raster layers." }
                p { "data-testid": "media-status", "{status}" }
                if let Some(ref map) = *map_handle.read() {
                    div { style: "display: grid; gap: 8px; margin-top: 16px;",
                        {
                            let map = map.clone();
                            let playing = video_playing();
                            rsx! { button {
                                "data-testid": "toggle-video",
                                onclick: move |_| {
                                    if playing { map.pause_video_source("video-media"); }
                                    else { map.play_video_source("video-media"); }
                                    video_playing.set(!playing);
                                },
                                if playing { "Pause video" } else { "Play video" }
                            } }
                        }
                        {
                            let map = map.clone();
                            rsx! { button {
                                onclick: move |_| map.set_source_coordinates("canvas-media", [
                                    [-122.5210, 37.5670], [-122.5170, 37.5670],
                                    [-122.5170, 37.5640], [-122.5210, 37.5640],
                                ]),
                                "Move canvas corners"
                            } }
                        }
                    }
                }
            }
        }
    }
}
