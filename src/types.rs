use std::path::PathBuf;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Panel {
    #[default]
    Project,
    Search,
    Details,
    Packages,
}

#[derive(Debug, Default)]
pub enum Tab {
    #[default]
    Installed,
    Upgrades,
    Search,
}

pub enum PanelChangeDirection {
    Up,
    Down,
}

#[derive(Debug, Default)]
pub struct Project {
    pub project_name: String,
    pub project_path: PathBuf,
    pub loaded_packages: Vec<UnLoadedPackage>,
}

#[derive(Debug, Default)]
pub struct UnLoadedPackage {
    pub package_id: String,
    pub version: String,
}

pub static PANEL_ORDER: &[Panel] = &[
    Panel::Project,
    Panel::Search,
    Panel::Details,
    Panel::Packages,
];
