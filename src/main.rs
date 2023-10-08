#[cfg(feature = "ratatui_game_runner")]
use crate::ratatui_game_runner::RatatuiGameRunner;
#[cfg(feature = "stdio_game_runner")]
use crate::stdio_game_runner::StdIOGameRunner;

mod feature_checks;
mod game;
#[cfg(feature = "ratatui_game_runner")]
mod ratatui_game_runner;
#[cfg(feature = "stdio_game_runner")]
mod stdio_game_runner;

fn main() {
    #[cfg(feature = "stdio_game_runner")]
    StdIOGameRunner::run(10, 4);

    #[cfg(feature = "ratatui_game_runner")]
    RatatuiGameRunner::run(10, 4);
}
