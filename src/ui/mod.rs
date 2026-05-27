use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// User interface module
pub fn render(app: &App, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" oddatui 🦀 📊 ")
        .border_type(BorderType::Rounded);

    let info_text = if app.running {
        "Welcome to oddatui!\n\nApplication is running.\nPress 'q' or 'Esc' to quit."
    } else {
        "Quitting..."
    };

    let paragraph = Paragraph::new(info_text)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan));

    frame.render_widget(paragraph, frame.area());
}
