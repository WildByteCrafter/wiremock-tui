use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn title_paragraph(text: &str) -> Paragraph<'_> {
    Paragraph::new(text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL))
}
