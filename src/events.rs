use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use tokio::sync::mpsc::Sender;
use tracing::error;
use tui_input::backend::crossterm::EventHandler;

use crate::app::{App, AppEvent};
use crate::nuget::client::NugetClient;
use crate::types::{PANEL_ORDER, Panel, PanelChangeDirection, SearchInputMode, Tab};

impl App {
    pub fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Event::Key(ke) = event {
            if ke.kind == KeyEventKind::Press {
                match self.active_panel {
                    Panel::Search => self.handle_search_keys(event),
                    Panel::Packages => self.handle_package_keys(ke),
                    _ => self.handle_navigation_keys(ke),
                }
            }
        }
        Ok(())
    }

    fn handle_package_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('j') => {
                self.selected_package_changed_handler();
                self.package_list_state.select_next();
            },
            KeyCode::Char('k') => {
                self.selected_package_changed_handler();
                self.package_list_state.select_previous();
            },
            _ => self.handle_navigation_keys(key_event),
        }
    }

    fn handle_navigation_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') => self.handle_panel_change(PanelChangeDirection::Up),
            KeyCode::Char('l') => self.handle_panel_change(PanelChangeDirection::Down),
            _ => (),
        }
    }

    fn handle_search_keys(&mut self, event: Event) {
        if !event.is_key_press() {
            return;
        }

        let key_event = event.as_key_press_event().unwrap();
        if self.search_state.input_mode == SearchInputMode::Normal {
            match key_event.code {
                KeyCode::Char('/') => {
                    if self.search_state.input_mode == SearchInputMode::Normal {
                        self.search_state.input_mode = SearchInputMode::Editing;
                    }
                }
                _ => self.handle_navigation_keys(key_event),
            }

            return;
        }

        match key_event.code {
            KeyCode::Esc => {
                self.search_state.input_mode = SearchInputMode::Normal;
            }
            KeyCode::Enter => {
                let tx = self.tx.clone();
                let search_value = self.search_state.search_input.value().to_string();

                let client = self.client.clone();

                // TODO: Handle scrolling and pagination for searching
                tokio::spawn(async move { search_packages(&client, &tx, search_value).await });

                self.search_state.input_mode = SearchInputMode::Searching;
            }
            _ => {
                if self.search_state.input_mode == SearchInputMode::Editing {
                    self.search_state.search_input.handle_event(&event);
                }
            }
        }
    }

    pub fn handle_panel_change(&mut self, direction: PanelChangeDirection) {
        let current_index = self.get_current_index();
        let next_index = match direction {
            PanelChangeDirection::Up => {
                if current_index == 0 {
                    PANEL_ORDER.len() - 1
                } else {
                    current_index - 1
                }
            }
            PanelChangeDirection::Down => {
                if current_index == PANEL_ORDER.len() - 1 {
                    0
                } else {
                    current_index + 1
                }
            }
        };
        self.active_panel = *PANEL_ORDER.get(next_index).unwrap_or(&self.active_panel);
    }

    pub fn get_current_index(&self) -> usize {
        PANEL_ORDER
            .iter()
            .position(|p| p == &self.active_panel)
            .unwrap_or(0)
    }
}

async fn search_packages(client: &NugetClient, tx: &Sender<AppEvent>, search_value: String) {
    let packages = client.search(&search_value, 50, 0).await;
    match packages {
        Ok(packages) => {
            let message = AppEvent::SearchResult(packages);
            match tx.send(message).await {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Failed to send search result: {}", e);
                }
            }
        }
        Err(error) => {
            error!("Failed to fetch packages from NuGet: {}", error);
            let message = AppEvent::Error(error);
            match tx.send(message).await {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Failed to send error: {}", e);
                }
            }
        }
    }
}
