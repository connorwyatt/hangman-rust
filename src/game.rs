use crate::game::guesses::prompt_for_guess;
use words::get_random_word;

mod guesses;
mod words;

pub(crate) enum GameplayError {
    Unknown,
}

#[derive(PartialEq)]
pub(crate) enum GameStateResult {
    Won,
    Lost,
}

#[derive(PartialEq)]
enum GameState {
    Complete(GameStateResult),
    InProgress,
}

enum TryGuessSuccess {
    InWord(char),
    NotInWord(char),
}

enum TryGuessError {
    AlreadyGuessed(char),
}

const TOTAL_ALLOWED_GUESSES: usize = 12;

pub(crate) struct Game {
    word: String,
    guesses: Vec<char>,
}

impl Game {
    pub(crate) fn new() -> Self {
        Game {
            word: get_random_word().to_uppercase(),
            guesses: Vec::new(),
        }
    }

    pub(crate) fn play(&mut self) {
        let mut turn: usize = 0;

        let result = loop {
            let game_state = self.game_state();

            if let GameState::Complete(result) = game_state {
                break result;
            };

            turn += 1;

            self.play_guess_round(turn == 1);
        };

        match result {
            GameStateResult::Won => {
                println!(
                    "Well done! You guessed the word with {} guesses remaining!",
                    self.remaining_guesses()
                );
            }
            GameStateResult::Lost => {
                println!("Oh no! You didn't guess the word! I'll tell you what is was though:");
                self.output_current_word_state();
                println!("    {}", self.space_characters(&self.word));
            }
        };
    }

    fn game_state(&self) -> GameState {
        if self.unguessed_characters() == 0 {
            GameState::Complete(GameStateResult::Won)
        } else if self.remaining_guesses() == 0 {
            GameState::Complete(GameStateResult::Lost)
        } else {
            GameState::InProgress
        }
    }

    fn play_guess_round(&mut self, is_first_guess: bool) {
        if is_first_guess {
            println!("The word for you to guess is:");
        }

        self.output_current_word_state();

        self.output_remaining_guesses();

        println!("Please guess a letter, and make it a good one!");

        let guess = prompt_for_guess();

        self.output_guess(&guess);

        match self.check_guess(guess) {
            Ok(success) => {
                match success {
                    TryGuessSuccess::InWord(character) => {
                        println!("Awesome! \"{}\" is in the word! Nice job!", character)
                    }
                    TryGuessSuccess::NotInWord(character) => {
                        println!("Sorry! \"{}\" is not in the word!", character)
                    }
                }

                self.guesses.push(guess)
            }
            Err(error) => match error {
                TryGuessError::AlreadyGuessed(character) => {
                    println!("You've already guessed {}!", character)
                }
            },
        };
    }

    fn unguessed_characters(&self) -> usize {
        self.word
            .chars()
            .filter(|char| !self.guesses.contains(&char))
            .count()
    }

    fn output_current_word_state(&self) {
        let current_word_state = self
            .word
            .chars()
            .map(|char| {
                if self.guesses.contains(&char) {
                    char
                } else {
                    '?'
                }
            })
            .collect::<String>();

        println!("    {}", self.space_characters(&current_word_state));
    }

    fn remaining_guesses(&self) -> usize {
        TOTAL_ALLOWED_GUESSES - self.guesses.len()
    }

    fn output_remaining_guesses(&self) {
        println!("You have {} guesses remaining.", self.remaining_guesses());
    }

    fn output_guess(&self, guess: &char) {
        println!("Your guess is \"{}\"!", guess);
    }

    fn check_guess(&self, guess: char) -> Result<TryGuessSuccess, TryGuessError> {
        if self.guesses.contains(&guess) {
            return Err(TryGuessError::AlreadyGuessed(guess));
        }

        let word_chars = self.word.chars().collect::<Vec<char>>();

        if word_chars.contains(&guess) {
            Ok(TryGuessSuccess::InWord(guess))
        } else {
            Ok(TryGuessSuccess::NotInWord(guess))
        }
    }

    fn space_characters(&self, string: &String) -> String {
        let letters = string
            .chars()
            .map(|char| char.to_string())
            .collect::<Vec<String>>();

        letters.join(" ")
    }
}
