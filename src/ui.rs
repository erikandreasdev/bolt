use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Mode};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size()); // upgrade to f.area() when ratatui >= 0.27

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Min(1)])
        .split(chunks[0]);

    render_logo(f, header_chunks[0]);
    render_search_bar(f, app, header_chunks[1]);
    render_task_list(f, app, chunks[1]);
    render_footer(f, chunks[2]);

    if app.mode == Mode::ParamInput {
        render_param_input(f, app);
    }
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
                .title(Span::styled(
                    "Filter",
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )),
        );
    f.render_widget(search_bar, area);
}

fn render_task_list(f: &mut Frame, app: &mut App, area: Rect) {
    let tasks: Vec<ListItem> = app
        .filtered_indices
        .iter()
        .map(|&i| {
            let task = &app.tasks[i];
            let content = vec![
                Line::from(Span::styled(
                    format!(" {}", task.name),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    format!("  {}", task.description),
                    Style::default().fg(Color::White),
                )),
                Line::from(Span::styled(
                    format!("  $ {}", task.command),
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
                )),
                Line::from(""),
            ];
            ListItem::new(content)
        })
        .collect();

    let tasks_list = List::new(tasks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(Span::styled(
                    "Tasks",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                )),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▎");

    f.render_stateful_widget(tasks_list, area, &mut app.list_state);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let hint = Span::styled(
        "(Esc) Quit | (Up/Down) Navigate | (Enter) Run | (Type) Search",
        Style::default().fg(Color::Cyan),
    );
    let footer = Paragraph::new(Line::from(hint)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(footer, area);
}

fn render_param_input(f: &mut Frame, app: &App) {
    let frame_size = f.size();

    // 2 borders + blank + command line + blank + one row per filled param + current input + blank + hint
    let content_rows = 8 + app.param_index as u16;
    let height = content_rows.min(frame_size.height);
    let width = (frame_size.width * 70 / 100).max(50).min(frame_size.width);
    let area = centered_rect(width, height, frame_size);

    f.render_widget(Clear, area);

    let current_param = &app.param_names[app.param_index];

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  $ {}", app.command_template),
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
    ];

    for (name, value) in app.param_names.iter().zip(app.param_values.iter()) {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                name.as_str(),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" → ", Style::default().fg(Color::DarkGray)),
            Span::styled(value.as_str(), Style::default().fg(Color::White)),
        ]));
    }

    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(
            current_param.as_str(),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" → ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}_", app.param_input),
            Style::default().fg(Color::Yellow),
        ),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  (Enter) Confirm  (Esc) Cancel",
        Style::default().fg(Color::DarkGray),
    )));

    let title = format!(
        " Parameters: {} ({}/{}) ",
        app.selected_task_name,
        app.param_index + 1,
        app.param_names.len()
    );

    let popup = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(Span::styled(
                title,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )),
    );

    f.render_widget(popup, area);
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let x = r.x + r.width.saturating_sub(width) / 2;
    let y = r.y + r.height.saturating_sub(height) / 2;
    Rect {
        x,
        y,
        width: width.min(r.width),
        height: height.min(r.height),
    }
}
