use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, Paragraph},
};

use crate::types::Panel;

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
            .title_top(self.search_input.clone());
        if self.active_panel == Panel::Search {
            search_block = search_block.border_style(Style::new().cyan());
        }

        let mut search_list = List::new(
            self.packages
                .iter()
                .map(|p| p.title.clone())
                .collect::<Vec<_>>(),
        )
        .block(search_block);

        if self.active_panel == Panel::Search {
            search_list = search_list.rapid_blink();
        }
        frame.render_widget(search_list, package_search_area);

        // Package list panel
        frame.render_widget(
            Paragraph::new("List of packages shown here").block(Block::new().borders(Borders::ALL)),
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
}
