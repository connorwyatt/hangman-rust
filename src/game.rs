use crate::game::MakeGuessSuccess::{Correct, Incorrect};
use words::random_word;

mod words;

pub(crate) struct Game {
    status: GameStatus,
    word: String,
    guesses: Vec<(String, GuessStatus)>,
    lives_remaining: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GameStatus {
    InProgress,
    Complete(CompleteGameStatus),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CompleteGameStatus {
    Won,
    Lost,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GuessStatus {
    Correct,
    Incorrect,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MakeGuessSuccess {
    Correct,
    Incorrect,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MakeGuessError {
    Empty,
    TooLong,
    Invalid,
    AlreadyGuessed,
    GameComplete,
}

impl Game {
    pub(crate) fn new(lives: usize, minimum_word_size: usize) -> Self {
        Game {
            status: GameStatus::InProgress,
            word: random_word(minimum_word_size).to_uppercase(),
            guesses: Vec::new(),
            lives_remaining: lives,
        }
    }

    pub(crate) fn make_guess(&mut self, guess: &str) -> Result<MakeGuessSuccess, MakeGuessError> {
        let guess = &guess.to_uppercase();

        if self.status != GameStatus::InProgress {
            return Err(MakeGuessError::GameComplete);
        }

        self.validate_guess(guess)?;

        if self.is_already_guessed(guess) {
            return Err(MakeGuessError::AlreadyGuessed);
        }

        let is_correct = self.is_letter_in_word(guess);

        self.guesses.push((
            guess.clone(),
            if is_correct {
                GuessStatus::Correct
            } else {
                GuessStatus::Incorrect
            },
        ));

        if !is_correct {
            self.lives_remaining -= 1;
        }

        self.update_status();

        if is_correct {
            Ok(Correct)
        } else {
            Ok(Incorrect)
        }
    }

    pub(crate) fn status(&self) -> GameStatus {
        self.status.clone()
    }

    pub(crate) fn lives_remaining(&self) -> usize {
        self.lives_remaining
    }

    pub(crate) fn guesses(&self) -> Vec<(String, GuessStatus)> {
        self.guesses.clone()
    }

    pub(crate) fn guess_letters(&self) -> Vec<String> {
        self.guesses
            .iter()
            .map(|(guess, _)| guess.clone())
            .collect::<Vec<String>>()
    }

    pub(crate) fn letters(&self) -> Vec<String> {
        self.word
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    pub(crate) fn blanked_out_letters(&self) -> Vec<String> {
        self.letters()
            .iter()
            .map(|x| {
                if self.guess_letters().contains(x) {
                    x.clone()
                } else {
                    String::from('_')
                }
            })
            .collect::<Vec<String>>()
    }

    fn update_status(&mut self) {
        self.status = if self.unknown_letters_count() == 0 {
            GameStatus::Complete(CompleteGameStatus::Won)
        } else if self.lives_remaining == 0 {
            GameStatus::Complete(CompleteGameStatus::Lost)
        } else {
            GameStatus::InProgress
        }
    }

    fn validate_guess(&self, guess: &String) -> Result<(), MakeGuessError> {
        if guess.is_empty() {
            return Err(MakeGuessError::Empty);
        } else if guess.len() > 1 {
            return Err(MakeGuessError::TooLong);
        }

        if let Some(char) = guess.chars().next() {
            if !char.is_ascii_alphabetic() {
                return Err(MakeGuessError::Invalid);
            }
        }

        Ok(())
    }

    fn unknown_letters_count(&self) -> usize {
        self.letters()
            .iter()
            .filter(|letter| !self.guess_letters().contains(letter))
            .count()
    }

    fn is_already_guessed(&self, letter: &String) -> bool {
        self.guess_letters().contains(letter)
    }
    fn is_letter_in_word(&self, letter: &String) -> bool {
        let letters = self.letters();

        letters.contains(letter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::MakeGuessError::{AlreadyGuessed, Empty, GameComplete, Invalid, TooLong};

    #[test]
    fn when_starting_the_game_the_status_is_in_progress() {
        let game = Game::new(1, 1);

        assert_eq!(game.status, GameStatus::InProgress);
    }

    #[test]
    fn when_guessing_a_valid_and_correct_letter_then_a_correct_result_is_returned_and_a_guess_is_added_and_no_life_is_lost(
    ) {
        let mut game = create_game(String::from("test"));

        let initial_lives_remaining = game.lives_remaining();

        let result = game.make_guess(&String::from("t"));

        assert_eq!(result, Ok(Correct));
        assert_eq!(
            game.guesses(),
            vec![(String::from("T"), GuessStatus::Correct)]
        );
        assert_eq!(game.lives_remaining(), initial_lives_remaining);
    }

    #[test]
    fn when_guessing_a_valid_but_incorrect_letter_then_an_incorrect_result_is_returned_and_a_guess_is_added_and_a_life_is_lost(
    ) {
        let mut game = create_single_life_game(String::from("test"));

        let result = game.make_guess(&String::from("a"));

        assert_eq!(result, Ok(Incorrect));
        assert_eq!(game.lives_remaining(), 0);
        assert_eq!(
            game.status(),
            GameStatus::Complete(CompleteGameStatus::Lost)
        );
    }

    #[test]
    fn when_guessing_the_final_letter_then_a_correct_result_is_returned_and_the_game_is_completed_as_a_win(
    ) {
        let mut game = create_game(String::from("win"));

        let initial_lives_remaining = game.lives_remaining();

        let _ = game.make_guess(&String::from("w"));
        let _ = game.make_guess(&String::from("i"));

        let result = game.make_guess(&String::from("n"));

        assert_eq!(result, Ok(Correct));
        assert_eq!(game.lives_remaining(), initial_lives_remaining);
        assert_eq!(game.status(), GameStatus::Complete(CompleteGameStatus::Won))
    }

    #[test]
    fn when_guessing_a_valid_but_incorrect_letter_with_one_life_left_then_an_incorrect_result_is_returned_and_the_game_is_completed_as_a_loss(
    ) {
        let mut game = create_game(String::from("test"));

        let initial_lives_remaining = game.lives_remaining();

        let result = game.make_guess(&String::from("a"));

        assert_eq!(result, Ok(Incorrect));
        assert_eq!(
            game.guesses(),
            vec![(String::from("A"), GuessStatus::Incorrect)]
        );
        assert_eq!(game.lives_remaining(), initial_lives_remaining - 1);
    }

    #[test]
    fn when_guessing_a_valid_and_correct_duplicate_letter_then_an_error_result_is_returned_and_a_life_is_not_lost_and_a_guess_is_not_added(
    ) {
        let mut game = create_game(String::from("test"));

        let initial_lives_remaining = game.lives_remaining();

        let _ = game.make_guess(&String::from("t"));
        let result = game.make_guess(&String::from("t"));

        assert_eq!(result, Err(AlreadyGuessed));
        assert_eq!(
            game.guesses(),
            vec![(String::from("T"), GuessStatus::Correct)]
        );
        assert_eq!(game.lives_remaining(), initial_lives_remaining);
    }

    #[test]
    fn when_guessing_a_valid_but_incorrect_duplicate_letter_then_an_error_result_is_returned_and_a_life_is_not_lost_and_a_guess_is_not_added(
    ) {
        let mut game = create_game(String::from("test"));

        let initial_lives_remaining = game.lives_remaining();

        let _ = game.make_guess(&String::from("a"));
        let result = game.make_guess(&String::from("a"));

        assert_eq!(result, Err(AlreadyGuessed));
        assert_eq!(
            game.guesses(),
            vec![(String::from("A"), GuessStatus::Incorrect)]
        );
        assert_eq!(game.lives_remaining(), initial_lives_remaining - 1);
    }

    #[test]
    fn when_guessing_an_empty_string_then_an_error_result_is_returned_and_a_life_is_not_lost_and_a_guess_is_not_added(
    ) {
        let mut game = create_game(String::from("test"));

        let initial_lives_remaining = game.lives_remaining();

        let result = game.make_guess(&String::from(""));

        assert_eq!(result, Err(Empty));
        assert_eq!(game.guesses(), vec![]);
        assert_eq!(game.lives_remaining(), initial_lives_remaining);
    }

    #[test]
    fn when_guessing_a_string_longer_than_one_letter_then_an_error_result_is_returned_and_a_life_is_not_lost_and_a_guess_is_not_added(
    ) {
        let mut game = create_game(String::from("test"));

        let initial_lives_remaining = game.lives_remaining();

        let result = game.make_guess(&String::from("aa"));

        assert_eq!(result, Err(TooLong));
        assert_eq!(game.guesses(), vec![]);
        assert_eq!(game.lives_remaining(), initial_lives_remaining);
    }

    #[test]
    fn when_guessing_an_invalid_character_then_an_error_result_is_returned_and_a_life_is_not_lost_and_a_guess_is_not_added(
    ) {
        let mut game = create_game(String::from("test"));

        let initial_lives_remaining = game.lives_remaining();

        let result = game.make_guess(&String::from("1"));

        assert_eq!(result, Err(Invalid));
        assert_eq!(game.guesses(), vec![]);
        assert_eq!(game.lives_remaining(), initial_lives_remaining);
    }

    #[test]
    fn when_guessing_a_valid_letter_when_the_game_is_complete_then_an_error_result_is_returned_and_a_life_is_not_lost_and_a_guess_is_not_added(
    ) {
        let mut game = create_single_life_game(String::from("test"));

        let _ = game.make_guess(&String::from("a"));
        let result = game.make_guess(&String::from("b"));

        assert_eq!(result, Err(GameComplete));
        assert_eq!(
            game.guesses(),
            vec![(String::from("A"), GuessStatus::Incorrect)]
        );
        assert_eq!(game.lives_remaining(), 0);
    }

    fn create_game(word: String) -> Game {
        Game {
            status: GameStatus::InProgress,
            word: word.to_uppercase(),
            guesses: Vec::new(),
            lives_remaining: 10,
        }
    }

    fn create_single_life_game(word: String) -> Game {
        Game {
            status: GameStatus::InProgress,
            word: word.to_uppercase(),
            guesses: Vec::new(),
            lives_remaining: 1,
        }
    }
}
