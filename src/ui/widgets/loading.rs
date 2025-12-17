use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use super::utils::centered_rect;

/// Renders a loading indicator as a centered popup
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
    let loading_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let loading_text = Paragraph::new(" Loading data from Jira... ")
        .block(loading_block)
        .alignment(Alignment::Center);

    let popup_area = centered_rect(60, 20, area);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(loading_text, popup_area);
}
