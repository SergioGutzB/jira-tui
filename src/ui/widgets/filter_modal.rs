use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::ui::app::{App, FilterField};

use super::utils::centered_rect;

/// Renders the filter modal as a popup overlay
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let popup_area = centered_rect(70, 50, area);

    frame.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(" Configurar Filtros ")
        .style(Style::default().fg(Color::Cyan));

    let inner_area = popup_block.inner(popup_area);
    frame.render_widget(popup_block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(inner_area);

    let assignee_focused = app.filter_focused_field == FilterField::Assignee;
    let status_focused = app.filter_focused_field == FilterField::Status;
    let order_focused = app.filter_focused_field == FilterField::OrderBy;

    let assignee_style = if assignee_focused {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let status_style = if status_focused {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let order_style = if order_focused {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let assignee_text = Line::from(vec![
        Span::raw(" Asignado a: "),
        Span::styled(
            format!("< {} >", app.filter_assignee.label()),
            assignee_style,
        ),
    ]);

    let assignee_block = Paragraph::new(assignee_text)
        .block(Block::default().borders(Borders::ALL).title(" Asignado "));

    frame.render_widget(assignee_block, chunks[0]);

    let status_text = Line::from(vec![
        Span::raw(" Estado: "),
        Span::styled(format!("< {} >", app.filter_status.label()), status_style),
    ]);

    let status_block =
        Paragraph::new(status_text).block(Block::default().borders(Borders::ALL).title(" Estado "));

    frame.render_widget(status_block, chunks[1]);

    let order_text = Line::from(vec![
        Span::raw(" Ordenar por: "),
        Span::styled(format!("< {} >", app.filter_order_by.label()), order_style),
    ]);

    let order_block =
        Paragraph::new(order_text).block(Block::default().borders(Borders::ALL).title(" Orden "));

    frame.render_widget(order_block, chunks[2]);

    let help_text = Paragraph::new(
        " Tab/j/k: Cambiar campo | h/l/←/→: Cambiar valor | Enter: Aplicar | Esc: Cancelar ",
    )
    .style(Style::default().fg(Color::DarkGray))
    .alignment(Alignment::Center);

    frame.render_widget(help_text, chunks[4]);
}
