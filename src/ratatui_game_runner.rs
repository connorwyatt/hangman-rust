mod app;
mod event_handler;
mod tui;
mod ui;
mod update;

use crate::ratatui_game_runner::{app::App, event_handler::EventHandler, tui::Tui, update::update};
use ratatui::{backend::CrosstermBackend, Terminal};

pub(crate) struct RatatuiGameRunner;

impl RatatuiGameRunner {
    pub(crate) fn run(lives: usize, minimum_word_size: usize) {
        let mut app = App::new(lives, minimum_word_size);

        let backend = CrosstermBackend::new(std::io::stderr());
        let terminal = Terminal::new(backend).expect("failed to create Terminal");
        let events = EventHandler::new(250);
        let mut tui = Tui::new(terminal, events);
        tui.initialize().expect("failed to initialize UI");

        while !app.should_quit {
            tui.draw(&mut app).expect("failed to draw");

            update(
                &mut app,
                tui.event_handler.next().expect("failed to read next event"),
            );
        }

        tui.exit().expect("tui failed to exit");
    }
}
