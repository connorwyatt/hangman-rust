use crate::game::Game;

pub(crate) struct App {
    game: Game,
    should_quit: bool,
}

impl App {
    pub(crate) fn new(game: Game) -> Self {
        App {
            game,
            should_quit: false,
        }
    }

    pub(crate) fn quit(&mut self) {
        self.should_quit = true;
    }

    pub(crate) fn should_quit(&self) -> bool {
        self.should_quit
    }
}
