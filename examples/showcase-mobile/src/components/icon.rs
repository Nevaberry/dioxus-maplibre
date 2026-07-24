use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    ArrowLeft,
    Camera,
    Check,
    ChevronRight,
    Compass,
    Download,
    Events,
    Globe,
    Layers,
    Map,
    Pause,
}

#[component]
pub fn AppIcon(kind: IconKind) -> Element {
    let paths: Element = match kind {
        IconKind::ArrowLeft => rsx! { path { d: "M15 18l-6-6 6-6" } },
        IconKind::Camera => rsx! {
            path { d: "M4 8.5h3l1.5-2h7l1.5 2h3v9H4z" }
            circle { cx: "12", cy: "13", r: "3" }
        },
        IconKind::Check => rsx! { path { d: "M5 12.5l4 4L19 7" } },
        IconKind::ChevronRight => rsx! { path { d: "M9 6l6 6-6 6" } },
        IconKind::Compass => rsx! {
            circle { cx: "12", cy: "12", r: "9" }
            path { d: "M15.5 8.5l-2 5-5 2 2-5z" }
        },
        IconKind::Download => rsx! {
            path { d: "M12 3v12m0 0l-4-4m4 4l4-4" }
            path { d: "M5 20h14" }
        },
        IconKind::Events => rsx! {
            path { d: "M5 4l12 8-6 2-2 6z" }
            path { d: "M15 16l3 3" }
        },
        IconKind::Globe => rsx! {
            circle { cx: "12", cy: "12", r: "9" }
            path { d: "M3 12h18M12 3c3 3 3 15 0 18M12 3c-3 3-3 15 0 18" }
        },
        IconKind::Layers => rsx! {
            path { d: "M3 8l9-5 9 5-9 5z" }
            path { d: "M5 12l7 4 7-4M5 16l7 4 7-4" }
        },
        IconKind::Map => rsx! {
            path { d: "M3 6l6-3 6 3 6-3v15l-6 3-6-3-6 3z" }
            path { d: "M9 3v15M15 6v15" }
        },
        IconKind::Pause => rsx! { path { d: "M8 5v14M16 5v14" } },
    };

    rsx! {
        svg {
            class: "app-icon",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.8",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            {paths}
        }
    }
}
