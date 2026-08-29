use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Offset, Rect},
    style::{Style, Stylize},
    symbols,
    text::{Line, ToSpan},
    widgets::{Block, Borders, List, Padding, Paragraph, Row, Table, Tabs, Widget},
};
use throbber_widgets_tui::Throbber;
use tracing::info;

use crate::types::{Panel, SearchInputMode, Tab};

use crate::app::App;

impl App {
    pub fn draw(&mut self, frame: &mut Frame) {
        // TODO: expand the project section and show list when active

        // -- Layouts
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(3),
                Constraint::Fill(70),
                Constraint::Min(1),
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

        // -- Panels

        // Project panel
        self.render_project_area(frame, project_area);

        // Search panel
        self.render_search_box(frame, package_search_area);
        self.render_help_message(frame, package_search_area + Offset::new(2, 0));

        // Package list panel
        self.render_package_content(frame, package_list_area);
        self.render_package_tabs(frame, package_list_area + Offset::new(2, 0));
        self.render_search_throbber(frame, package_list_area);

        // Package details panel
        self.render_package_description(frame, package_details_area);

        // Hints panel
        self.render_hints_panel(frame, hints_area);
    }

    fn render_search_box(&self, frame: &mut Frame<'_>, package_search_area: Rect) {
        let mut search_block = Block::new().borders(Borders::ALL);
        if self.active_panel == Panel::Search {
            search_block = search_block.border_style(Style::new().cyan());
        }

        let search_field =
            Paragraph::new(self.search_state.search_input.value()).block(search_block);
        frame.render_widget(search_field, package_search_area);
    }

    fn render_search_throbber(&self, frame: &mut Frame<'_>, package_list_area: Rect) {
        if self.search_state.input_mode == SearchInputMode::Searching {
            let throbber = Throbber::default().throbber_set(throbber_widgets_tui::BRAILLE_SIX);
            frame.render_widget(throbber, package_list_area + Offset::new(1, 0));
        }
    }

    fn render_project_area(&self, frame: &mut Frame<'_>, project_area: Rect) {
        let mut project_widget_block = Block::new().borders(Borders::ALL).title_top("Projects");
        if self.active_panel == Panel::Project {
            project_widget_block = project_widget_block.border_style(Style::new().cyan());
        }
        let proj_widget = List::new(self.projects.iter().map(|p| p.project_name.clone()))
            .highlight_symbol("> ")
            .block(project_widget_block);
        frame.render_widget(proj_widget, project_area);
    }

    fn render_help_message(&self, frame: &mut Frame, area: Rect) {
        let help_message = Line::from_iter(match self.search_state.input_mode {
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
            .select(self.active_tab as usize);

        frame.render_widget(tabs, area);
    }

    pub fn render_package_content(&mut self, frame: &mut Frame, area: Rect) {
        // TODO: Optimize this to not re-render the entire table on every update

        let rows = self
            .packages
            .iter()
            .map(|p| {
                if let Some(index) = self.package_list_state.selected() {
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
                self.render_package_table(frame, area, rows);
            }
            Tab::Upgrades => {
                // TODO: Find packages that has updates available
            }
            Tab::Search => {
                // TODO: Render the search results
                self.render_package_table(frame, area, rows);
            }
        }
    }

    fn render_package_table(&mut self, frame: &mut Frame, area: Rect, rows: Vec<TableRow>) {
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

        let mut block = Block::new()
            .borders(Borders::all())
            .padding(Padding::new(1, 1, 0, 0));

        if self.active_panel == Panel::Packages {
            block = block.border_style(Style::new().cyan());
        }

        let mut table = Table::new(rows, widths)
            .header(headers)
            .row_highlight_style(Style::new().bold().cyan())
            .highlight_symbol("> ")
            .block(block);

        if self.active_panel != Panel::Packages {
            table = table
                .row_highlight_style(Style::new().dim())
                .style(Style::new().dim())
        }

        frame.render_stateful_widget(table, area, &mut self.package_list_state);
    }

    fn render_hints_panel(&self, frame: &mut Frame<'_>, hints_area: Rect) {
        let mut lines = vec![
            " ".to_span(),
            "Quit: ".to_span(),
            "q ".bold(),
            " | ".to_span(),
            "Navigate panel: ".to_span(),
            "j/k ".bold(),
            " | ".to_span(),
            "Change panel: ".to_span(),
            "h/l".bold(),
        ];

        match self.active_panel {
            Panel::Project => {
                lines.append(
                    vec![
                        " | ".to_span(),
                        "Select project: ".to_span(),
                        "<space>".bold(),
                    ]
                    .as_mut(),
                );
            }
            Panel::Packages => {
                lines.append(
                    vec![
                        " | ".to_span(),
                        "<space> ".bold(),
                        "- Select package".to_span(),
                        " | ".to_span(),
                        "i ".bold(),
                        "- Install package".to_span(),
                    ]
                    .as_mut(),
                );
            }
            _ => {}
        }

        frame.render_widget(Line::from_iter(lines), hints_area);
    }

    // TODO: Split the are and show description with relevant info on top (Author, description, title, id)
    // Show readme on bottom
    fn render_package_description(&self, frame: &mut Frame<'_>, package_details_area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(package_details_area);

        let package_information_area = layout[0];
        let readme_area = layout[1];

        if let Some(selected_package_id) = self.package_list_state.selected() {
            if let Some(_) = self.packages.get(selected_package_id) {
                if let Some(readme) = &self.current_readme {
                    info!("Trying to render markdown file: {}", readme);
                    frame.render_widget(
                        Paragraph::new(readme.clone()).block(Block::new().borders(Borders::ALL)),
                        readme_area,
                    );
                    return;
                }
            }
        }
        frame.render_widget(
            Paragraph::new("Loading readme").block(Block::new().borders(Borders::ALL)),
            readme_area,
        );
    }
}

struct TableRow {
    name: String,
    latest_version: String,
    installed_version: Option<String>,
}
