use crossterm::event::EventStream;
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::nuget::client::{NugetClient, Package};
use crate::types::{Panel, Project, Tab};

#[derive(Debug)]
pub struct App {
    pub counter: i8,
    pub exit: bool,
    pub client: NugetClient,
    pub active_panel: Panel,
    pub active_tab: Tab,
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
            projects: Vec::new(),
        }
    }
}

impl App {
    pub async fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        let mut events = EventStream::new();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            tokio::select! {
                Some(Ok(event)) = events.next() => self.handle_event(event)?,
                Some(app_event) = self.rx.recv() => self.handle_app_event(app_event)?,
            }
        }

        Ok(())
    }

    pub fn handle_app_event(&mut self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::SearchResult(result) => {
                self.packages = result;
                self.selected = if !self.packages.is_empty() { Some(0) } else { None };
            }
            _ => {}
        }
        Ok(())
    }
}
