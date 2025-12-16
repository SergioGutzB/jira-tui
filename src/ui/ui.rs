use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
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
        CurrentScreen::BoardsList => " Boards List | 'b' Load | Enter to Select | 'q' Quit ",
        CurrentScreen::Backlog => " Backlog | Enter to View Details | 'b' Back to Boards ",
        CurrentScreen::IssueDetail => " Issue Details | Scroll with Up/Down | Esc to Back ",
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
        CurrentScreen::IssueDetail => {
            render_issue_detail(frame, area, app);
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

// ... (render_boards se mantiene igual) ...
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

// ... (render_backlog se mantiene igual) ...
fn render_backlog(frame: &mut Frame, area: Rect, app: &App) {
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

/// Renders the detailed view of a single issue.
fn render_issue_detail(frame: &mut Frame, area: Rect, app: &App) {
    if let Some(issue) = app.get_selected_issue() {
        // Layout: Top (Metadata) / Bottom (Description)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Metadata height
                Constraint::Min(1),    // Description remaining
            ])
            .split(area);

        // --- Metadata Block ---
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

        // --- Description Block ---
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
            .wrap(Wrap { trim: true }) // Enable text wrapping
            .scroll((app.vertical_scroll, 0)); // Enable scrolling from App state

        frame.render_widget(desc_block, chunks[1]);
    } else {
        // Fallback if no issue selected
        let block = Block::default().borders(Borders::ALL).title(" Error ");
        let p = Paragraph::new("No issue selected.").block(block);
        frame.render_widget(p, area);
    }
}

// ... (render_loading y centered_rect se mantienen igual) ...
fn render_loading(frame: &mut Frame, area: Rect) {
    let loading_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));
    let loading_text = Paragraph::new(" Loading data from Jira... ")
        .block(loading_block)
        .alignment(Alignment::Center);
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
