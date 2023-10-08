use crate::ratatui_game_runner::{app::App, tui::Frame};
use ratatui::{
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub(crate) fn render(app: &App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Press `Esc` or `Ctrl-C` to stop running.")
            .block(
                Block::default()
                    .title("Hangman")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center),
        frame.size(),
    )
}
