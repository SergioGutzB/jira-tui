use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::domain::models::IssueStatus;
use crate::ui::app::App;

/// Renders the backlog/issues list view
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let items: Vec<ListItem> = app
        .issues
        .iter()
        .map(|i| {
            let status_style = match i.status {
                IssueStatus::Todo => Style::default().fg(Color::Gray),
                IssueStatus::InProgress => Style::default().fg(Color::Yellow),
                IssueStatus::Done => Style::default().fg(Color::Green),
                _ => Style::default().fg(Color::Magenta),
            };
            let status_str = format!("{:?}", i.status);
            let priority = i.priority.as_deref().unwrap_or("-");
            let content = Line::from(vec![
                Span::styled(
                    format!("{:<10}", i.key),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("[{:<12}] ", status_str), status_style),
                Span::raw(format!("({:^8}) ", priority)),
                Span::raw(&i.summary),
            ]);
            ListItem::new(content)
        })
        .collect();

    let title = " Backlog / Issues ";
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = ListState::default();
    state.select(Some(app.selected_issue_index));
    frame.render_stateful_widget(list, area, &mut state);
}
