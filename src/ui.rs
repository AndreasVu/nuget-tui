use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
};

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

        frame.render_widget(
            Paragraph::new("csproj selection here").block(
                Block::new()
                    .borders(Borders::ALL)
                    .title_top("Selected project"),
            ),
            project_area,
        );
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .title_top("Search packages"),
            package_search_area,
        );
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
                .title_top("h/l - Navigate panel")
                .slow_blink(),
            hints_area,
        );
    }
}
