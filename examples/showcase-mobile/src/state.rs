use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTab {
    Scenes,
    Lab,
    Offline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connectivity {
    Online,
    Offline,
}

impl Connectivity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Online => "Online",
            Self::Offline => "Offline",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneKind {
    Helsinki,
    Buildings,
    Matterhorn,
    Tokyo,
}

impl SceneKind {
    pub const ALL: [Self; 4] = [
        Self::Helsinki,
        Self::Buildings,
        Self::Matterhorn,
        Self::Tokyo,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Helsinki => "Helsinki",
            Self::Buildings => "3D",
            Self::Matterhorn => "Terrain",
            Self::Tokyo => "Tokyo",
        }
    }

    pub const fn eyebrow(self) -> &'static str {
        match self {
            Self::Helsinki => "Clusters & symbols",
            Self::Buildings => "Buildings & query",
            Self::Matterhorn => "Terrain & atmosphere",
            Self::Tokyo => "Heatmap & live data",
        }
    }

    pub const fn offline_style(self) -> &'static str {
        match self {
            Self::Helsinki | Self::Buildings => "/offline/helsinki-style.json",
            Self::Matterhorn => "/offline/matterhorn-style.json",
            Self::Tokyo => "/offline/tokyo-style.json",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabPanel {
    Home,
    Layers,
    Interaction,
    Camera,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackId {
    Helsinki,
    Matterhorn,
    Archipelago,
}

impl PackId {
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Helsinki => "helsinki",
            Self::Matterhorn => "matterhorn",
            Self::Archipelago => "archipelago",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Helsinki => "Helsinki",
            Self::Matterhorn => "Matterhorn",
            Self::Archipelago => "Archipelago",
        }
    }

    pub const fn size_mb(self) -> u16 {
        match self {
            Self::Helsinki => 42,
            Self::Matterhorn => 138,
            Self::Archipelago => 86,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineView {
    Packs,
    SelectArea,
    Downloading,
    Ready,
}

impl OfflineView {
    pub const fn shows_connectivity_toggle(self) -> bool {
        matches!(self, Self::Packs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct OfflineState {
    pub view: OfflineView,
    pub selected_pack: PackId,
    pub runtime_supported: bool,
    pub helsinki_ready: bool,
    pub matterhorn_ready: bool,
    pub archipelago_ready: bool,
    pub progress: u8,
    pub component: String,
    pub paused: bool,
    pub error: Option<String>,
}

impl Default for OfflineState {
    fn default() -> Self {
        Self {
            view: OfflineView::Packs,
            selected_pack: PackId::Helsinki,
            runtime_supported: false,
            helsinki_ready: true,
            matterhorn_ready: false,
            archipelago_ready: false,
            progress: 0,
            component: "Style".into(),
            paused: false,
            error: None,
        }
    }
}

impl OfflineState {
    pub const fn is_ready(&self, pack: PackId) -> bool {
        match pack {
            PackId::Helsinki => self.helsinki_ready,
            PackId::Matterhorn => self.matterhorn_ready,
            PackId::Archipelago => self.archipelago_ready,
        }
    }

    pub fn set_ready(&mut self, pack: PackId, ready: bool) {
        match pack {
            PackId::Helsinki => self.helsinki_ready = ready,
            PackId::Matterhorn => self.matterhorn_ready = ready,
            PackId::Archipelago => self.archipelago_ready = ready,
        }
    }

    pub const fn storage_used_mb(&self) -> u16 {
        (if self.helsinki_ready {
            PackId::Helsinki.size_mb()
        } else {
            0
        }) + (if self.matterhorn_ready {
            PackId::Matterhorn.size_mb()
        } else {
            0
        }) + (if self.archipelago_ready {
            PackId::Archipelago.size_mb()
        } else {
            0
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MobileContext {
    pub runtime_ready: Signal<bool>,
    pub active_tab: Signal<AppTab>,
    pub connectivity: Signal<Connectivity>,
    pub scene: Signal<SceneKind>,
    pub lab_panel: Signal<LabPanel>,
    pub offline: Signal<OfflineState>,
}

impl MobileContext {
    pub fn new() -> Self {
        Self {
            runtime_ready: Signal::new(false),
            active_tab: Signal::new(AppTab::Scenes),
            connectivity: Signal::new(Connectivity::Online),
            scene: Signal::new(SceneKind::Helsinki),
            lab_panel: Signal::new(LabPanel::Home),
            offline: Signal::new(OfflineState::default()),
        }
    }

    pub fn set_tab(mut self, tab: AppTab) {
        self.active_tab.set(tab);
    }

    pub fn set_connectivity(mut self, connectivity: Connectivity) {
        self.connectivity.set(connectivity);
    }

    pub fn set_scene(mut self, scene: SceneKind) {
        self.scene.set(scene);
    }

    pub fn set_lab_panel(mut self, panel: LabPanel) {
        self.lab_panel.set(panel);
    }

    pub fn active_tab(self) -> AppTab {
        (self.active_tab)()
    }

    pub fn connectivity(self) -> Connectivity {
        (self.connectivity)()
    }

    pub fn scene(self) -> SceneKind {
        (self.scene)()
    }

    pub fn lab_panel(self) -> LabPanel {
        (self.lab_panel)()
    }

    pub fn offline_view(self) -> OfflineView {
        (self.offline)().view
    }

    pub fn offline(self) -> OfflineState {
        (self.offline)()
    }

    pub fn runtime_ready(self) -> bool {
        (self.runtime_ready)()
    }
}
