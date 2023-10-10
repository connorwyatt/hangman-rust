use crate::game::{Game, MakeGuessResult};

pub(crate) struct App {
    game: Game,
    should_quit: bool,
    last_guess_result: Option<MakeGuessResult>,
}

impl App {
    pub(crate) fn new(game: Game) -> Self {
        App {
            game,
            should_quit: false,
            last_guess_result: None,
        }
    }

    pub(crate) fn game(&self) -> &Game {
        &self.game
    }

    pub(crate) fn make_guess(&mut self, guess: &str) {
        self.last_guess_result = Some(self.game.make_guess(guess));
    }

    pub(crate) fn last_guess_result(&self) -> Option<MakeGuessResult> {
        self.last_guess_result.clone()
    }

    pub(crate) fn quit(&mut self) {
        self.should_quit = true;
    }

    pub(crate) fn should_quit(&self) -> bool {
        self.should_quit
    }
}
