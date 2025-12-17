use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::app::{App, CurrentScreen};
use crate::ui::widgets;

/// Main render function - entry point for all UI rendering
pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(frame.size());

    render_title(frame, chunks[0], app);
    render_body(frame, chunks[1], app);
}

/// Renders the title bar with context-specific help text
fn render_title(frame: &mut Frame, area: Rect, app: &App) {
    let title_text = match app.current_screen {
        CurrentScreen::BoardsList => " Boards List | 'b' Load | Enter to Select | 'q' Quit ",
        CurrentScreen::Backlog => " Backlog | 'f' Filter | Enter View Details | 'b' Back ",
        CurrentScreen::IssueDetail => " Issue Details | 'w' Log Time | Up/Down Scroll | Esc Back ",
        CurrentScreen::FilterModal => {
            " Filter Modal | Tab to Switch | Left/Right to Change | Enter to Apply "
        }
        CurrentScreen::WorklogModal => {
            " Log Time | Tab Switch Field | Type to Edit | Enter Save | Esc Cancel "
        }
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

/// Renders the main body content based on current screen
fn render_body(frame: &mut Frame, area: Rect, app: &App) {
    match app.current_screen {
        CurrentScreen::Dashboard | CurrentScreen::BoardsList => {
            widgets::boards::render(frame, area, app);
        }
        CurrentScreen::Backlog => {
            widgets::backlog::render(frame, area, app);
        }
        CurrentScreen::IssueDetail => {
            widgets::issue_detail::render(frame, area, app);
        }
        CurrentScreen::FilterModal => {
            if let Some(prev_screen) = &app.previous_screen {
                match prev_screen {
                    CurrentScreen::Backlog => widgets::backlog::render(frame, area, app),
                    _ => {}
                }
            }
            widgets::filter_modal::render(frame, area, app);
        }
        CurrentScreen::WorklogModal => {
            if let Some(prev_screen) = &app.previous_screen {
                match prev_screen {
                    CurrentScreen::IssueDetail => widgets::issue_detail::render(frame, area, app),
                    _ => {}
                }
            }
            widgets::worklog_modal::render(frame, area, app);
        }
        _ => {
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Work in Progress ");
            frame.render_widget(block, area);
        }
    }

    if app.is_loading {
        widgets::loading::render(frame, area);
    }
}
