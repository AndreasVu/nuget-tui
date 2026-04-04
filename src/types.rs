use std::path::PathBuf;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Panel {
    #[default]
    Project,
    Search,
    Details,
    Packages,
}

#[derive(Debug, Default, PartialEq)]
pub enum SearchState {
    Active,
    #[default]
    Inactive,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum Tab {
    #[default]
    Installed = 0,
    Upgrades = 1,
    Search = 2,
}

pub enum PanelChangeDirection {
    Up,
    Down,
}

#[derive(Debug, Default)]
pub struct Project {
    pub project_name: String,
    pub project_path: PathBuf,
    pub unloaded_packages: Vec<PackageRef>,
}

#[derive(Debug, Default)]
pub struct PackageRef {
    pub package_id: String,
    pub version: String,
}

pub static PANEL_ORDER: &[Panel] = &[
    Panel::Project,
    Panel::Search,
    Panel::Details,
    Panel::Packages,
];
