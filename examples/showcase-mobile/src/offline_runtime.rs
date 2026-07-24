use dioxus::prelude::*;
use serde::Deserialize;

use crate::state::{Connectivity, MobileContext, OfflineView, PackId};

#[derive(Debug, Deserialize)]
struct RuntimeSnapshot {
    mode: Option<String>,
    packs: Vec<String>,
    #[serde(default)]
    supported: bool,
}

#[derive(Debug, Deserialize)]
struct DownloadEvent {
    kind: String,
    progress: Option<u8>,
    component: Option<String>,
    message: Option<String>,
}

pub fn initialize(state: MobileContext) {
    spawn(async move {
        let snapshot = document::eval(
            r"
            const started = Date.now();
            while (!window.dioxusMaplibreOffline && Date.now() - started < 5000) {
                await new Promise((resolve) => setTimeout(resolve, 50));
            }
            if (!window.dioxusMaplibreOffline) {
                return { mode: null, packs: [], supported: false };
            }
            return window.dioxusMaplibreOffline.snapshot();
            ",
        )
        .join::<RuntimeSnapshot>()
        .await;

        let mut state = state;
        if let Ok(snapshot) = snapshot {
            if snapshot.mode.as_deref() == Some("offline") {
                state.set_connectivity(Connectivity::Offline);
            }

            {
                let mut offline = state.offline.write();
                offline.runtime_supported = snapshot.supported;
                for pack in snapshot.packs {
                    match pack.as_str() {
                        "matterhorn" => offline.matterhorn_ready = true,
                        "archipelago" => offline.archipelago_ready = true,
                        _ => {}
                    }
                }
            }
        }
        state.runtime_ready.set(true);
    });
}

pub fn persist_connectivity(connectivity: Connectivity) {
    let mode = match connectivity {
        Connectivity::Online => "online",
        Connectivity::Offline => "offline",
    };
    spawn(async move {
        let script = format!(
            r"
            if (window.dioxusMaplibreOffline) {{
                window.dioxusMaplibreOffline.setMode('{mode}');
            }} else {{
                localStorage.setItem('dioxus-maplibre-mode', '{mode}');
            }}
            "
        );
        let _ = document::eval(&script).await;
    });
}

pub fn download_pack(pack: PackId, mut state: MobileContext) {
    let slug = pack.slug();
    dioxus_core::spawn_forever(async move {
        let script = format!(
            r"
            if (!window.dioxusMaplibreOffline) {{
                dioxus.send({{ kind: 'error', message: 'Offline runtime is unavailable' }});
                return;
            }}
            await window.dioxusMaplibreOffline.downloadPack('{slug}', (event) => dioxus.send(event));
            "
        );
        let mut evaluator = document::eval(&script);

        while let Ok(event) = evaluator.recv::<DownloadEvent>().await {
            let mut offline = state.offline.write();
            match event.kind.as_str() {
                "progress" => {
                    offline.progress = event.progress.unwrap_or(offline.progress);
                    if let Some(component) = event.component {
                        offline.component = component;
                    }
                }
                "paused" => offline.paused = true,
                "resumed" => offline.paused = false,
                "cancelled" => {
                    offline.view = OfflineView::Packs;
                    offline.progress = 0;
                    offline.paused = false;
                    break;
                }
                "complete" => {
                    offline.set_ready(pack, true);
                    offline.progress = 100;
                    offline.paused = false;
                    offline.view = OfflineView::Ready;
                    break;
                }
                "error" => {
                    offline.error = event.message;
                    offline.view = OfflineView::Packs;
                    break;
                }
                _ => {}
            }
        }
    });
}

pub fn pause_or_resume(paused: bool) {
    let method = if paused { "resume" } else { "pause" };
    spawn(async move {
        let script = format!("window.dioxusMaplibreOffline?.{method}Download();");
        let _ = document::eval(&script).await;
    });
}

pub fn cancel_download() {
    spawn(async move {
        let _ = document::eval("window.dioxusMaplibreOffline?.cancelDownload();").await;
    });
}
