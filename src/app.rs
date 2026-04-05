use crossterm::event::EventStream;
use futures::StreamExt;
use ratatui::widgets::{ListState, TableState};
use throbber_widgets_tui::ThrobberState;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info};
use tui_input::Input;

use crate::nuget::client::{NugetClient, Package};
use crate::projects::get_project_packages;
use crate::types::{Panel, Project, SearchInputMode, Tab};

#[derive(Debug)]
pub struct App {
    pub exit: bool,
    pub client: NugetClient,
    pub active_panel: Panel,
    pub active_tab: Tab,
    pub packages: Vec<Package>,
    pub search_state: SearchState,
    pub selected_project_index: Option<usize>,
    pub projects: Vec<Project>,
    pub tx: Sender<AppEvent>,
    pub rx: Receiver<AppEvent>,
    pub current_readme: Option<String>,
    pub package_list_state: TableState,
}

#[derive(Debug, Default)]
pub struct SearchState {
    pub search_input: Input,
    pub search_throbber_state: ThrobberState,
    pub input_mode: SearchInputMode,
}

#[derive(Debug, Default)]
pub enum AppEvent {
    #[default]
    None,
    SearchResult(Vec<Package>),
    ReadmeResult(Option<String>),
    Error(anyhow::Error),
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(32);
        Self {
            rx,
            tx,
            active_panel: Panel::default(),
            active_tab: Tab::default(),
            client: NugetClient::default(),
            exit: false,
            packages: Vec::new(),
            search_state: SearchState::default(),
            projects: Vec::new(),
            selected_project_index: None,
            current_readme: None,
            package_list_state: TableState::default(),
        }
    }
}

impl App {
    pub async fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        let mut events = EventStream::new();

        info!("Initializing application");
        self.initialize_application().await;

        while !self.exit {
            self.search_state.search_throbber_state.calc_next();
            tokio::select! {
                Some(Ok(event)) = events.next() => self.handle_event(event)?,
                Some(app_event) = self.rx.recv() => self.handle_app_event(app_event)?,
            }

            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    async fn initialize_application(&mut self) {
        self.projects = get_project_packages();
        if self.projects.len() > 0 {
            self.selected_project_index = Some(0);
        }
        self.get_packages_from_projects();
    }

    pub fn selected_package_changed_handler(&mut self) {
        if let Some(index) = self.package_list_state.selected() {
            if let Some(package) = self.packages.get(index) {
                let client = self.client.clone();
                let pakcage_id = package.id.clone();
                let package_version = package.version.clone();
                let tx = self.tx.clone();

                tokio::spawn(async move {
                    let result = client.get_readme(&pakcage_id, &package_version).await;
                    if let Err(e) = result {
                        eprintln!("Failed to get readme: {}", e);
                        return;
                    }

                    if let Err(e) = tx.send(AppEvent::ReadmeResult(result.unwrap())).await {
                        eprintln!("Failed to send readme loading: {}", e);
                    }
                });
            }
        }
    }

    pub fn handle_app_event(&mut self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::SearchResult(result) => {
                self.packages = result;
                self.search_state.input_mode = SearchInputMode::Normal;
                if !self.search_state.search_input.value().is_empty() {
                    self.active_tab = Tab::Search;
                }

                if !self.packages.is_empty() {
                    self.package_list_state.select_first();
                }
                self.selected_package_changed_handler();
            }
            AppEvent::ReadmeResult(result) => {
                self.current_readme = result;
            }
            AppEvent::Error(err) => {
                error!("Unexpected error: {}", err);
                self.packages = Vec::new();
            }
            _ => {}
        }

        Ok(())
    }
}
