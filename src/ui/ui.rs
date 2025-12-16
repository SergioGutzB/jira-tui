use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::ui::app::{App, CurrentScreen};

/// Renders the user interface based on the current application state.
pub fn render(app: &App, frame: &mut Frame) {
    // Split screen: Top bar (Title) and Main Body
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title bar
            Constraint::Min(1),    // Main content
        ])
        .split(frame.size());

    render_title(frame, chunks[0], app);
    render_body(frame, chunks[1], app);
}

fn render_title(frame: &mut Frame, area: Rect, app: &App) {
    let title_text = format!(" Rust Jira TUI - Screen: {:?} ", app.current_screen);

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
        _ => {
            // Placeholder for future screens
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Work in Progress ");
            frame.render_widget(block, area);
        }
    }

    // Overlay for loading state
    if app.is_loading {
        let loading_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow));
        let loading_text = Paragraph::new(" Loading data from Jira... ")
            .block(loading_block)
            .alignment(ratatui::layout::Alignment::Center);

        // Centered popup area
        let popup_area = centered_rect(60, 20, area);
        frame.render_widget(ratatui::widgets::Clear, popup_area); // Clear background
        frame.render_widget(loading_text, popup_area);
    }
}

fn render_boards(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .boards
        .iter()
        .map(|b| {
            let content = format!(" {} | {} ({}) ", b.id, b.name, b.board_type);
            ListItem::new(Line::from(content))
        })
        .collect();

    let title = if app.boards.is_empty() {
        " Boards (No data or Press 'b' to load) "
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
        )
        .highlight_symbol(">> ");

    // We use a predefined state here for simplicity in this step.
    // In a full implementation, we would pass `&mut ListState`.
    // For now, we manually simulate selection by recreating the widget logic or
    // simpler: just render the list.
    // *Note*: Ratatui lists require a `ListState` to render the selection.
    // To keep `ui.rs` stateless (pure function), we should store `ListState` in `App`.
    // However, to fix the compilation for this step without refactoring `App` too much,
    // we will rely on visual feedback or add `ListState` later.

    // Quick fix to show selection visually without ListState for this specific step:
    // We will render it, but selection highlighting requires `frame.render_stateful_widget`.
    // Let's do a basic render first.

    // Create a temporary state for rendering the selection
    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected_board_index));

    frame.render_stateful_widget(list, area, &mut state);
}

/// Helper to center a rect (useful for popups)
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
