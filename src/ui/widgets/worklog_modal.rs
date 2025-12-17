use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::ui::app::{App, WorklogField};

use super::utils::centered_rect;

/// Renders the worklog modal as a popup overlay
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let popup_area = centered_rect(80, 70, area);

    frame.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(" Log Time ")
        .style(Style::default().fg(Color::Cyan));

    let inner_area = popup_block.inner(popup_area);
    frame.render_widget(popup_block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Date
            Constraint::Length(3), // Time
            Constraint::Length(3), // Time spent
            Constraint::Length(5), // Comment
            Constraint::Length(1),
            Constraint::Length(3), // Help
        ])
        .split(inner_area);

    // Date
    let date_focused = matches!(
        app.worklog_focused_field,
        WorklogField::Day | WorklogField::Month | WorklogField::Year
    );
    let date_border_color = if date_focused {
        Color::Yellow
    } else {
        Color::White
    };

    let day_style = if app.worklog_focused_field == WorklogField::Day {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let month_style = if app.worklog_focused_field == WorklogField::Month {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let year_style = if app.worklog_focused_field == WorklogField::Year {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let date_text = Line::from(vec![
        Span::raw(" Date: "),
        Span::styled(format!("{:02}", app.worklog_day), day_style),
        Span::raw(" / "),
        Span::styled(format!("{:02}", app.worklog_month), month_style),
        Span::raw(" / "),
        Span::styled(format!("{:04}", app.worklog_year), year_style),
    ]);

    let date_block = Paragraph::new(date_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Date (DD/MM/YYYY) ")
            .border_style(Style::default().fg(date_border_color)),
    );

    frame.render_widget(date_block, chunks[0]);

    // Time
    let time_focused = matches!(
        app.worklog_focused_field,
        WorklogField::Hour | WorklogField::Minute
    );
    let time_border_color = if time_focused {
        Color::Yellow
    } else {
        Color::White
    };

    let hour_style = if app.worklog_focused_field == WorklogField::Hour {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let minute_style = if app.worklog_focused_field == WorklogField::Minute {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let time_text = Line::from(vec![
        Span::raw(" Time: "),
        Span::styled(format!("{:02}", app.worklog_hour), hour_style),
        Span::raw(" : "),
        Span::styled(format!("{:02}", app.worklog_minute), minute_style),
    ]);

    let time_block = Paragraph::new(time_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Time (HH:MM) ")
            .border_style(Style::default().fg(time_border_color)),
    );

    frame.render_widget(time_block, chunks[1]);

    // Time spent
    let duration_focused = matches!(
        app.worklog_focused_field,
        WorklogField::TimeHours | WorklogField::TimeMinutes
    );
    let duration_border_color = if duration_focused {
        Color::Yellow
    } else {
        Color::White
    };

    let time_hours_style = if app.worklog_focused_field == WorklogField::TimeHours {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let time_minutes_style = if app.worklog_focused_field == WorklogField::TimeMinutes {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let duration_text = Line::from(vec![
        Span::raw(" Time: "),
        Span::styled(format!("{}", app.worklog_time_hours), time_hours_style),
        Span::raw(" h "),
        Span::styled(format!("{}", app.worklog_time_minutes), time_minutes_style),
        Span::raw(" m"),
    ]);

    let duration_block = Paragraph::new(duration_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Time Spent ")
            .border_style(Style::default().fg(duration_border_color)),
    );

    frame.render_widget(duration_block, chunks[2]);

    // Comment
    let comment_focused = app.worklog_focused_field == WorklogField::Comment;
    let comment_border_color = if comment_focused {
        Color::Yellow
    } else {
        Color::White
    };

    let comment_text = if app.worklog_comment.is_empty() {
        Line::from(vec![Span::styled(
            " (Optional) Work description...",
            Style::default().fg(Color::DarkGray),
        )])
    } else {
        Line::from(vec![Span::raw(format!(" {}", app.worklog_comment))])
    };

    let comment_block = Paragraph::new(comment_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Comment ")
            .border_style(Style::default().fg(comment_border_color)),
    );

    frame.render_widget(comment_block, chunks[3]);

    let help_text = Paragraph::new(
        " Tab/j/k: Switch field | 0-9: Edit | Backspace: Delete | Enter: Save | Esc: Cancel ",
    )
    .style(Style::default().fg(Color::DarkGray))
    .alignment(Alignment::Center);

    frame.render_widget(help_text, chunks[5]);
}
