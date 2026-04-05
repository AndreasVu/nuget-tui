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
pub enum SearchInputMode {
    Editing,
    #[default]
    Normal,
    Searching,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum Tab {
    #[default]
    Installed = 0,
    Search = 1,
    Upgrades = 2,
}

pub enum PanelChangeDirection {
    Up,
    Down,
}

#[derive(Debug, Default, Clone)]
pub struct Project {
    pub project_name: String,
    pub project_path: PathBuf,
    pub package_refs: Vec<PackageRef>,
}

#[derive(Debug, Default, Clone)]
pub struct PackageRef {
    pub package_id: String,
    pub version: String,
}

pub static PANEL_ORDER: &[Panel] = &[Panel::Project, Panel::Search, Panel::Packages];
