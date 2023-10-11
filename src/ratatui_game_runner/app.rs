use crate::game::{CompleteGameStatus, Game, GameStatus, MakeGuessResult};

#[derive(Eq, PartialEq)]
pub(crate) enum PlayAgain {
    Yes,
    No,
}

#[derive(Eq, PartialEq)]
pub(crate) struct GameCompleteState {
    pub(crate) currently_selected: PlayAgain,
}

#[derive(Eq, PartialEq)]
pub(crate) enum CurrentView {
    GameInProgress,
    GameComplete(GameCompleteState),
}

pub(crate) struct App {
    pub(crate) current_view: CurrentView,
    pub(crate) lives: usize,
    pub(crate) minimum_word_size: usize,
    pub(crate) current_game: Game,
    pub(crate) last_guess_result: Option<MakeGuessResult>,
    pub(crate) games_played: u16,
    pub(crate) games_won: u16,
    pub(crate) games_lost: u16,
    pub(crate) should_quit: bool,
}

impl App {
    pub(crate) fn new(lives: usize, minimum_word_size: usize) -> Self {
        App {
            current_view: CurrentView::GameInProgress,
            lives,
            minimum_word_size,
            current_game: Game::new(lives, minimum_word_size),
            last_guess_result: None,
            games_played: 0,
            games_won: 0,
            games_lost: 0,
            should_quit: false,
        }
    }

    pub(crate) fn start_new_game(&mut self) {
        self.current_game = Game::new(self.lives, self.minimum_word_size);
        self.current_view = CurrentView::GameInProgress;
    }

    pub(crate) fn make_guess(&mut self, guess: &str) {
        if let GameStatus::Complete(_) = self.current_game.status() {
            return;
        };

        self.last_guess_result = Some(self.current_game.make_guess(guess));

        if let GameStatus::Complete(complete_game_status) = self.current_game.status() {
            self.games_played += 1;
            self.current_view = CurrentView::GameComplete(GameCompleteState {
                currently_selected: PlayAgain::Yes,
            });
            match complete_game_status {
                CompleteGameStatus::Won => {
                    self.games_won += 1;
                }
                CompleteGameStatus::Lost => {
                    self.games_lost += 1;
                }
            };
        };
    }

    pub(crate) fn quit(&mut self) {
        self.should_quit = true;
    }
}
