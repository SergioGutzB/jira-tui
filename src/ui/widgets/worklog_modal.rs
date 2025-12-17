use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::app::{App, WorklogField};

use super::utils::centered_rect;

/// Renders the worklog modal as a popup overlay
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let popup_area = centered_rect(70, 50, area);

    frame.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(" Registrar Tiempo ")
        .style(Style::default().fg(Color::Cyan));

    let inner_area = popup_block.inner(popup_area);
    frame.render_widget(popup_block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(inner_area);

    let hours_focused = app.worklog_focused_field == WorklogField::Hours;
    let minutes_focused = app.worklog_focused_field == WorklogField::Minutes;
    let comment_focused = app.worklog_focused_field == WorklogField::Comment;

    let hours_style = if hours_focused {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let minutes_style = if minutes_focused {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let comment_style = if comment_focused {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let hours_text = Line::from(vec![
        Span::raw(" Horas: "),
        Span::styled(format!("{}", app.worklog_hours), hours_style),
    ]);

    let hours_block = Paragraph::new(hours_text)
        .block(Block::default().borders(Borders::ALL).title(" Horas "));

    frame.render_widget(hours_block, chunks[0]);

    let minutes_text = Line::from(vec![
        Span::raw(" Minutos: "),
        Span::styled(format!("{}", app.worklog_minutes), minutes_style),
    ]);

    let minutes_block = Paragraph::new(minutes_text)
        .block(Block::default().borders(Borders::ALL).title(" Minutos "));

    frame.render_widget(minutes_block, chunks[1]);

    let comment_text = if app.worklog_comment.is_empty() {
        Line::from(vec![Span::styled(
            " (Opcional) Descripción del trabajo...",
            Style::default().fg(Color::DarkGray),
        )])
    } else {
        Line::from(vec![Span::styled(
            format!(" {}", app.worklog_comment),
            comment_style,
        )])
    };

    let comment_block = Paragraph::new(comment_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Comentario "),
    );

    frame.render_widget(comment_block, chunks[2]);

    let help_text = Paragraph::new(
        " Tab/j/k: Cambiar campo | 0-9: Editar número | a-z: Editar texto | Enter: Guardar | Esc: Cancelar ",
    )
    .style(Style::default().fg(Color::DarkGray))
    .alignment(Alignment::Center);

    frame.render_widget(help_text, chunks[4]);
}
