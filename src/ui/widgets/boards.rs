use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::ui::app::App;

/// Renders the boards list view
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
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
