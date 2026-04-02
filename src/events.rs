use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use crate::app::App;
use crate::types::{PanelChangeDirection, PANEL_ORDER};

impl App {
    pub fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Event::Key(ke) = event {
            if ke.kind == KeyEventKind::Press {
                self.handle_key_press(ke);
            }
        }
        Ok(())
    }

    pub fn handle_key_press(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Right => self.increment_counter(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Char('h') => self.handle_panel_change(PanelChangeDirection::Up),
            KeyCode::Char('l') => self.handle_panel_change(PanelChangeDirection::Down),
            _ => (),
        }
    }

    pub fn handle_panel_change(&mut self, direction: PanelChangeDirection) {
        let current_index = self.get_current_index();
        let next_index = match direction {
            PanelChangeDirection::Up => {
                if current_index == 0 { PANEL_ORDER.len() - 1 } else { current_index - 1 }
            }
            PanelChangeDirection::Down => {
                if current_index == PANEL_ORDER.len() - 1 { 0 } else { current_index + 1 }
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

    pub fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }
}
