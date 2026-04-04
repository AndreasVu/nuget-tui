use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, Paragraph, Tabs},
};

use crate::types::{Panel, Tab};

use crate::app::App;

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(70),
                Constraint::Percentage(10),
            ])
            .split(frame.area());

        let packages_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(layout[1]);

        let project_area = layout[0];
        let hints_area = layout[2];
        let package_details_area = packages_layout[1];

        let packages_area_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(packages_layout[0]);

        let package_search_area = packages_area_layout[0];
        let package_list_area = packages_area_layout[1];

        // Project panel
        let mut proj_widget = Paragraph::new("csproj selection here").block(
            Block::new()
                .borders(Borders::ALL)
                .title_top("Selected project"),
        );
        if self.active_panel == Panel::Project {
            proj_widget = proj_widget.rapid_blink();
        }
        frame.render_widget(proj_widget, project_area);

        // Search panel
        let mut search_block = Block::new()
            .borders(Borders::ALL)
            .title_top("Search packages");
        if self.active_panel == Panel::Search {
            search_block = search_block.border_style(Style::new().cyan());
        }

        let mut search_field = Paragraph::new(self.search_input.clone()).block(search_block);

        if self.active_panel == Panel::Search {
            search_field = search_field.rapid_blink();
        }
        frame.render_widget(search_field, package_search_area);

        // Package list panel
        frame.render_widget(
            List::new(self.get_package_names()).block(Block::new().borders(Borders::ALL)),
            package_list_area,
        );
        frame.render_widget(
            Paragraph::new("package description here").block(Block::new().borders(Borders::ALL)),
            package_details_area,
        );
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .title_top("q - Quit")
                .title_top("")
                .title_top("j/k - Navigate within panel")
                .title_top("")
                .title_top("h/l - Navigate panel"),
            hints_area,
        );
    }

    pub fn render_package_tabs(&mut self, frame: &mut Frame, area: Rect) {
        let tabs = Tabs::new(vec!["Installed", "Search", "Upgrades"])
            .style(Color::DarkGray)
            .select(self.active_tab as usize);

        frame.render_widget(tabs, area);
    }

    pub fn render_package_content(&mut self, frame: &mut Frame, area: Rect) {
        match self.active_tab {
            Tab::Installed => {
                // TODO: Render installed packages
            }
            Tab::Upgrades => {
                // TODO: Find packages that has updates available
            }
            Tab::Search => {
                // TODO: Render the search results
                // Do default query if no search term is provided
            }
        }
    }
}
