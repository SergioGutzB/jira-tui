use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::domain::models::IssueStatus;
use crate::ui::app::App;

/// Renders the detailed view of a single issue
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    if let Some(issue) = app.get_selected_issue() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6), Constraint::Min(1)])
            .split(area);

        let status_color = match issue.status {
            IssueStatus::Todo => Color::Gray,
            IssueStatus::InProgress => Color::Yellow,
            IssueStatus::Done => Color::Green,
            _ => Color::Magenta,
        };

        let meta_text = vec![
            Line::from(vec![
                Span::styled("KEY: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&issue.key),
                Span::raw("  |  "),
                Span::styled("STATUS: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("{:?}", issue.status),
                    Style::default().fg(status_color),
                ),
            ]),
            Line::from(vec![
                Span::styled("SUMMARY: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&issue.summary),
            ]),
            Line::from(vec![
                Span::styled("ASSIGNEE: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(issue.assignee.as_deref().unwrap_or("Unassigned")),
                Span::raw("  |  "),
                Span::styled("PRIORITY: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(issue.priority.as_deref().unwrap_or("None")),
            ]),
            Line::from(vec![
                Span::styled("UPDATED: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(issue.updated_at.format("%Y-%m-%d %H:%M").to_string()),
            ]),
        ];

        let meta_block = Paragraph::new(meta_text)
            .block(Block::default().borders(Borders::ALL).title(" Issue Info "))
            .alignment(Alignment::Left);

        frame.render_widget(meta_block, chunks[0]);

        let desc_text = issue
            .description
            .as_deref()
            .unwrap_or("No description provided.");

        let desc_block = Paragraph::new(desc_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Description "),
            )
            .wrap(Wrap { trim: true })
            .scroll((app.vertical_scroll, 0));

        frame.render_widget(desc_block, chunks[1]);
    } else {
        let block = Block::default().borders(Borders::ALL).title(" Error ");
        let p = Paragraph::new("No issue selected.").block(block);
        frame.render_widget(p, area);
    }
}
