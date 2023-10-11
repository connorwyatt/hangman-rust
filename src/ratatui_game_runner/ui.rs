mod game_complete;
mod game_in_progress;
mod shared;

use crate::ratatui_game_runner::{app::App, app::CurrentView, tui::Frame};

pub(crate) fn render(app: &mut App, frame: &mut Frame) {
    game_in_progress::render(app, frame);

    if let CurrentView::GameComplete(_) = app.current_view {
        game_complete::render(app, frame);
    }
}
