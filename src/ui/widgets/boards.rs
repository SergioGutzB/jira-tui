use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Row, Table, TableState},
    Frame,
};

use crate::ui::app::App;

/// Renders the boards list view
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    if app.boards.is_empty() {
        let empty_rows: Vec<Row> = vec![];
        let empty = Table::new(empty_rows, [Constraint::Percentage(100)])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Boards (Press 'b' to load) "),
            );
        frame.render_widget(empty, area);
        return;
    }

    let header = Row::new(vec!["ID", "Nombre", "Proyecto", "Tipo"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .boards
        .iter()
        .map(|board| {
            Row::new(vec![
                Text::from(board.id.to_string()),
                Text::from(board.name.clone()),
                Text::from(board.project_key.clone()),
                Text::from(board.board_type.clone()),
            ])
            .style(Style::default().fg(Color::White))
        })
        .collect();

    let widths = [
        Constraint::Length(8),  // ID
        Constraint::Percentage(40), // Nombre
        Constraint::Length(12), // Proyecto
        Constraint::Min(15),    // Tipo
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Boards "))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .column_spacing(2);

    let mut state = TableState::default();
    state.select(Some(app.selected_board_index));
    frame.render_stateful_widget(table, area, &mut state);
}
