use crate::stdio_game_runner::StdIOGameRunner;

mod game;
mod stdio_game_runner;

fn main() {
    StdIOGameRunner::run(10, 4);
}
