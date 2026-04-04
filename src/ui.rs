use std::ops::Index;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Offset, Rect, Rows},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, ToSpan},
    widgets::{Block, Borders, List, Padding, Paragraph, Row, Table, Tabs},
};
use throbber_widgets_tui::Throbber;
use tracing::info;

use crate::types::{PackageRef, Panel, SearchInputMode, Tab};

use crate::app::App;

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(7),
                Constraint::Fill(70),
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
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(packages_layout[0]);

        let package_search_area = packages_area_layout[0];
        let package_list_area = packages_area_layout[1];

        // Project panel
        let mut proj_widget = List::new(self.projects.iter().map(|p| p.project_name.clone()))
            .highlight_symbol("> ")
            .block(Block::new().borders(Borders::ALL).title_top("Projects"));
        if self.active_panel == Panel::Project {
            proj_widget = proj_widget.rapid_blink();
        }
        frame.render_widget(proj_widget, project_area);

        // Search panel
        let mut search_block = Block::new().borders(Borders::ALL);
        if self.active_panel == Panel::Search {
            search_block = search_block.border_style(Style::new().cyan());
        }

        let mut search_field = Paragraph::new(self.search_input.value()).block(search_block);

        if self.active_panel == Panel::Search {
            search_field = search_field.rapid_blink();
        }
        frame.render_widget(search_field, package_search_area);
        self.render_help_message(frame, package_search_area + Offset::new(2, 0));

        // Package list panel
        self.render_package_content(frame, package_list_area);
        self.render_package_tabs(frame, package_list_area + Offset::new(1, 0));

        // Package details panel
        frame.render_widget(
            Paragraph::new("package description here").block(Block::new().borders(Borders::ALL)),
            package_details_area,
        );

        // Hints panel
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

    fn render_help_message(&self, frame: &mut Frame, area: Rect) {
        let help_message = Line::from_iter(match self.search_state {
            SearchInputMode::Normal | SearchInputMode::Searching => vec![
                "Press ".to_span(),
                "/".bold(),
                " to start editing.".to_span(),
            ],
            SearchInputMode::Editing => vec![
                "Press ".to_span(),
                "Esc".bold(),
                " to stop editing, ".to_span(),
                "Enter".bold(),
                " to search".to_span(),
            ],
        });

        frame.render_widget(help_message, area);
    }

    pub fn render_package_tabs(&self, frame: &mut Frame, area: Rect) {
        let tabs = Tabs::new(vec!["Installed", "Search", "Upgrades"])
            .divider(symbols::DOT)
            .padding(" ", " ")
            .select(self.active_tab as usize);

        frame.render_widget(tabs, area);

        if self.search_state == SearchInputMode::Searching {
            let throbber = Throbber::default().throbber_set(throbber_widgets_tui::BRAILLE_SIX);
            frame.render_widget(throbber, area + Offset::new(40, 0));
        }
    }

    pub fn render_package_content(&self, frame: &mut Frame, area: Rect) {
        let rows = self
            .packages
            .iter()
            .map(|p| {
                if let Some(index) = self.selected_package_index {
                    if let Some(current_project) = self.projects.get(index)
                        && let Some(package_ref) = current_project
                            .package_refs
                            .iter()
                            .find(|cp| cp.package_id == p.id)
                    {
                        info!("current project: {:?}", package_ref);
                        return TableRow {
                            name: p.id.clone(),
                            latest_version: p
                                .versions
                                .iter()
                                .last()
                                .map(|f| f.version.clone())
                                .unwrap_or("".to_string()),
                            installed_version: Some(package_ref.version.clone()),
                        };
                    }
                }

                return TableRow {
                    name: p.id.clone(),
                    latest_version: p
                        .versions
                        .iter()
                        .last()
                        .map(|f| f.version.clone())
                        .unwrap_or("".to_string()),
                    installed_version: None,
                };
            })
            .collect::<Vec<_>>();

        match self.active_tab {
            Tab::Installed => {
                // TODO: Render installed packages
                App::render_package_table(frame, area, rows);
            }
            Tab::Upgrades => {
                // TODO: Find packages that has updates available
            }
            Tab::Search => {
                // TODO: Render the search results
                App::render_package_table(frame, area, rows);
            }
        }
    }

    fn render_package_table(frame: &mut Frame, area: Rect, rows: Vec<TableRow>) {
        let headers =
            Row::new(["Name", "Installed Version", "Latest Version"]).style(Style::new().bold());

        let widths = [
            Constraint::Fill(50),
            Constraint::Min(20),
            Constraint::Min(20),
        ];

        let rows = rows
            .into_iter()
            .map(|row| {
                Row::new(vec![
                    row.name,
                    row.installed_version.unwrap_or("".to_string()),
                    row.latest_version,
                ])
            })
            .collect::<Vec<_>>();

        let table = Table::new(rows, widths)
            .header(headers)
            .row_highlight_style(Style::new().on_magenta().green())
            .highlight_symbol(">>")
            .block(
                Block::new()
                    .borders(Borders::all())
                    .padding(Padding::new(1, 1, 0, 0)),
            );

        frame.render_widget(table, area);
    }
}

struct TableRow {
    name: String,
    latest_version: String,
    installed_version: Option<String>,
}
