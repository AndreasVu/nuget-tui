use crossterm::event::EventStream;
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::nuget::client::{NugetClient, Package};
use crate::projects::get_project_packages;
use crate::types::{Panel, Project, SearchState, Tab};

#[derive(Debug)]
pub struct App {
    pub counter: i8,
    pub exit: bool,
    pub client: NugetClient,
    pub active_panel: Panel,
    pub active_tab: Tab,
    pub search_state: SearchState,
    pub packages: Vec<Package>,
    pub search_input: String,
    pub selected: Option<usize>,
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
            search_input: String::new(),
            selected: None,
            search_state: SearchState::Inactive,
            projects: Vec::new(),
        }
    }
}

impl App {
    pub async fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        let mut events = EventStream::new();

        self.initialize_application().await;

        while !self.exit {
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
        self.packages = self.get_packages_from_projects().await;
        // Fetch packages in current project.
        // fetch all packages installed
    }

    pub fn handle_app_event(&mut self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::SearchResult(result) => {
                self.packages = result;
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
