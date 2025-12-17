use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph, Row, Table},
};

use super::utils::centered_rect;
use crate::ui::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let popup_area = centered_rect(90, 80, area);
    frame.render_widget(Clear, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(5),    // Table
            Constraint::Length(3), // Instructions
        ])
        .split(popup_area);

    render_title(frame, chunks[0], app);
    render_worklog_table(frame, chunks[1], app);
    render_instructions(frame, chunks[2]);
}

fn render_title(frame: &mut Frame, area: Rect, app: &App) {
    let title = if let Some(issue) = app.get_selected_issue() {
        format!(
            " Logged Time - {} ({}/{}) ",
            issue.key,
            app.worklogs.len(),
            app.total_worklogs
        )
    } else {
        " Logged Time ".to_string()
    };

    let block = Block::default().borders(Borders::ALL).title(title).style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_widget(block, area);
}

fn render_worklog_table(frame: &mut Frame, area: Rect, app: &App) {
    if app.worklogs.is_empty() {
        let empty_msg = Paragraph::new("No logged time entries")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(empty_msg, area);
        return;
    }

    let header = Row::new(vec!["Date/Time", "Duration", "Author", "Comment"])
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .worklogs
        .iter()
        .enumerate()
        .map(|(idx, worklog)| {
            let hours = worklog.time_spent_seconds / 3600;
            let minutes = (worklog.time_spent_seconds % 3600) / 60;
            let time_str = match (hours, minutes) {
                (0, m) => format!("{}m", m),
                (h, 0) => format!("{}h", h),
                (h, m) => format!("{}h {}m", h, m),
            };

            let started_local = worklog.started_at.with_timezone(&chrono::Local);
            let date_str = started_local.format("%d/%m/%Y %H:%M").to_string();

            let comment = worklog
                .comment
                .as_ref()
                .map(|c| {
                    if c.len() > 60 {
                        format!("{}...", &c[..57])
                    } else {
                        c.clone()
                    }
                })
                .unwrap_or_else(|| "-".to_string());

            let is_selected = idx == app.selected_worklog_index;
            let style = if is_selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Text::from(date_str),
                Text::from(time_str),
                Text::from(worklog.author.clone()),
                Text::from(comment),
            ])
            .style(style)
        })
        .collect();

    let widths = [
        Constraint::Length(16),     // Date/Time
        Constraint::Length(10),     // Duration
        Constraint::Min(15),        // Author (adaptive)
        Constraint::Percentage(50), // Comment (takes remaining space)
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" List "))
        .column_spacing(2);

    frame.render_widget(table, area);
}

fn render_instructions(frame: &mut Frame, area: Rect) {
    let instructions =
        Paragraph::new(" ↑/↓ or j/k: Navigate | Enter or 'e': Edit | 'd': Delete | Esc: Close ")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));

    frame.render_widget(instructions, area);
}
