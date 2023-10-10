use crate::game::{CompleteGameStatus, Game, GameStatus, MakeGuessResult};

pub(crate) struct App {
    pub(crate) game: Game,
    pub(crate) should_quit: bool,
    pub(crate) last_guess_result: Option<MakeGuessResult>,
    pub(crate) games_played: u16,
    pub(crate) games_won: u16,
    pub(crate) games_lost: u16,
}

impl App {
    pub(crate) fn new(game: Game) -> Self {
        App {
            game,
            should_quit: false,
            last_guess_result: None,
            games_played: 0,
            games_won: 0,
            games_lost: 0,
        }
    }

    pub(crate) fn make_guess(&mut self, guess: &str) {
        if let GameStatus::Complete(_) = self.game.status() {
            return;
        }

        self.last_guess_result = Some(self.game.make_guess(guess));

        if let GameStatus::Complete(complete_game_status) = self.game.status() {
            self.games_played += 1;
            match complete_game_status {
                CompleteGameStatus::Won => {
                    self.games_won += 1;
                }
                CompleteGameStatus::Lost => {
                    self.games_lost += 1;
                }
            }
        }
    }

    pub(crate) fn quit(&mut self) {
        self.should_quit = true;
    }
}
