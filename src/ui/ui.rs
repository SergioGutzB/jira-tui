use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::domain::models::IssueStatus;
use crate::ui::app::{App, CurrentScreen};

pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(frame.size());

    render_title(frame, chunks[0], app);
    render_body(frame, chunks[1], app);
}

fn render_title(frame: &mut Frame, area: Rect, app: &App) {
    let title_text = match app.current_screen {
        CurrentScreen::BoardsList => " Select a Board (Up/Down + Enter) | 'q' to Quit ",
        CurrentScreen::Backlog => " Backlog Issues (Up/Down) | 'b' to Back to Boards ",
        _ => " Rust Jira TUI ",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(title_text)
        .block(block)
        .style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(paragraph, area);
}

fn render_body(frame: &mut Frame, area: Rect, app: &App) {
    match app.current_screen {
        CurrentScreen::Dashboard | CurrentScreen::BoardsList => {
            render_boards(frame, area, app);
        }
        CurrentScreen::Backlog => {
            render_backlog(frame, area, app);
        }
        _ => {
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Work in Progress ");
            frame.render_widget(block, area);
        }
    }

    if app.is_loading {
        render_loading(frame, area);
    }
}

fn render_boards(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .boards
        .iter()
        .map(|b| {
            ListItem::new(Line::from(format!(
                " {} | {} ({}) ",
                b.id, b.name, b.board_type
            )))
        })
        .collect();

    let title = if app.boards.is_empty() {
        " Boards (Press 'b' to load) "
    } else {
        " Boards "
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = ListState::default();
    state.select(Some(app.selected_board_index));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_backlog(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .issues
        .iter()
        .map(|i| {
            // Color code the status
            let status_style = match i.status {
                IssueStatus::Todo => Style::default().fg(Color::Gray),
                IssueStatus::InProgress => Style::default().fg(Color::Yellow),
                IssueStatus::Done => Style::default().fg(Color::Green),
                _ => Style::default().fg(Color::Magenta),
            };

            let status_str = format!("{:?}", i.status); // e.g., "InProgress"
            let priority = i.priority.as_deref().unwrap_or("-");

            // Format: PROJ-123 [Status] (Priority) Summary...
            let content = Line::from(vec![
                ratatui::text::Span::styled(
                    format!("{:<10}", i.key),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                ratatui::text::Span::styled(format!("[{:<12}] ", status_str), status_style),
                ratatui::text::Span::raw(format!("({:^8}) ", priority)),
                ratatui::text::Span::raw(&i.summary),
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

fn render_loading(frame: &mut Frame, area: Rect) {
    let loading_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));
    let loading_text = Paragraph::new(" Loading data from Jira... ")
        .block(loading_block)
        .alignment(ratatui::layout::Alignment::Center);

    let popup_area = centered_rect(60, 20, area);
    frame.render_widget(ratatui::widgets::Clear, popup_area);
    frame.render_widget(loading_text, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
