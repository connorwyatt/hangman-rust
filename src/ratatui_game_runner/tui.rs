use crate::ratatui_game_runner::{app::App, event_handler::EventHandler, ui};
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::panic;

pub type Frame<'a> = ratatui::Frame<'a, ratatui::backend::CrosstermBackend<std::io::Stderr>>;
pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

pub(crate) struct Tui {
    terminal: CrosstermTerminal,
    pub event_handler: EventHandler,
}

impl Tui {
    pub(crate) fn new(terminal: CrosstermTerminal, event_handler: EventHandler) -> Self {
        Self {
            terminal,
            event_handler,
        }
    }

    pub(crate) fn initialize(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub(crate) fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    pub(crate) fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}
