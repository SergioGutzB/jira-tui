use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use super::utils::centered_rect;

/// Renders a notification popup with title and message
pub fn render(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    title: &str,
    message: &str,
    is_success: bool,
) {
    let popup_area = centered_rect(60, 30, area);

    frame.render_widget(Clear, popup_area);

    let color = if is_success { Color::Green } else { Color::Red };

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title))
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD));

    let text = Paragraph::new(message)
        .block(popup_block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    frame.render_widget(text, popup_area);
}
