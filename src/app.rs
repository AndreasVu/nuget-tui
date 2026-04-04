use crossterm::event::EventStream;
use futures::StreamExt;
use throbber_widgets_tui::ThrobberState;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::info;
use tui_input::Input;

use crate::nuget::client::{NugetClient, Package};
use crate::projects::get_project_packages;
use crate::types::{Panel, Project, SearchInputMode, Tab};

#[derive(Debug)]
pub struct App {
    pub counter: i8,
    pub exit: bool,
    pub client: NugetClient,
    pub active_panel: Panel,
    pub active_tab: Tab,
    pub packages: Vec<Package>,
    pub search_input: Input,
    pub search_throbber_state: ThrobberState,
    pub search_state: SearchInputMode,
    pub selected: Option<usize>,
    pub selected_package_index: Option<usize>,
    pub projects: Vec<Project>,
    pub tx: Sender<AppEvent>,
    pub rx: Receiver<AppEvent>,
}

#[derive(Debug, Default)]
pub enum AppEvent {
    #[default]
    None,
    SearchResult(Vec<Package>),
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
            counter: 0,
            exit: false,
            packages: Vec::new(),
            search_input: Input::default(),
            selected: None,
            search_state: SearchInputMode::Normal,
            projects: Vec::new(),
            selected_package_index: None,
            search_throbber_state: ThrobberState::default(),
        }
    }
}

impl App {
    pub async fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        let mut events = EventStream::new();

        info!("Initializing application");
        self.initialize_application().await;

        while !self.exit {
            self.search_throbber_state.calc_next();
            terminal.draw(|frame| self.draw(frame))?;

            tokio::select! {
                Some(Ok(event)) = events.next() => self.handle_event(event)?,
                Some(app_event) = self.rx.recv() => self.handle_app_event(app_event)?,
            }
        }

        Ok(())
    }

    async fn initialize_application(&mut self) {
        self.projects = get_project_packages();
        if self.projects.len() > 0 {
            self.selected_package_index = Some(0);
        }
        self.get_packages_from_projects();
    }

    pub fn handle_app_event(&mut self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::SearchResult(result) => {
                self.packages = result;
                self.search_state = SearchInputMode::Normal;
                if !self.search_input.value().is_empty() {
                    self.active_tab = Tab::Search;
                }

                self.selected = if !self.packages.is_empty() {
                    Some(0)
                } else {
                    None
                };
            }
            AppEvent::Error(err) => {
                self.packages = Vec::new();
                self.selected = None;
            }
            _ => {}
        }
        Ok(())
    }
}
