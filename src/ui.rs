use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &mut App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // List
            Constraint::Length(3), // Help footer
        ])
        .split(f.size());

    // Split header into Logo and Search
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(12), // Logo width " BOLT ⚡ " + borders
            Constraint::Min(1),     // Search bar
        ])
        .split(chunks[0]);

    render_logo(f, header_chunks[0]);
    render_search_bar(f, app, header_chunks[1]);
    render_task_list(f, app, chunks[1]);
    render_footer(f, chunks[2]);
}

fn render_logo(f: &mut Frame, area: Rect) {
    let logo = Paragraph::new(Span::styled(
        " BOLT ⚡",
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    f.render_widget(logo, area);
}

fn render_search_bar(f: &mut Frame, app: &App, area: Rect) {
    let search_text = format!("Search: {}", app.search_query);
    let search_bar = Paragraph::new(search_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled("Filter", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        );
    f.render_widget(search_bar, area);
}

fn render_task_list(f: &mut Frame, app: &mut App, area: Rect) {
    let tasks: Vec<ListItem> = app
        .filtered_tasks
        .iter()
        .map(|task| {
            let content = vec![
                Line::from(Span::styled(
                    format!(" {}", task.name), 
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                )),
                Line::from(Span::styled(
                    format!("  {}", task.description),
                    Style::default().fg(Color::White),
                )),
                Line::from(Span::styled(
                    format!("  $ {}", task.command),
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
                )),
                Line::from(""), // Spacing
            ];
            ListItem::new(content)
        })
        .collect();

    let tasks_list = List::new(tasks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(Span::styled("Tasks", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▎");

    // We need to manage the list state (selection)
    let mut state = ListState::default();
    state.select(Some(app.selected_index));

    f.render_stateful_widget(tasks_list, area, &mut state);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let current_keys_hint = Span::styled(
        "(Esc) Quit | (Up/Down) Navigate | (Enter) Run | (Type) Search",
        Style::default().fg(Color::Cyan),
    );

    let footer = Paragraph::new(Line::from(current_keys_hint))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        );
    f.render_widget(footer, area);
}
