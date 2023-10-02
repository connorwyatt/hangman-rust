use crate::game::guesses::prompt_for_guess;
use std::io;
use words::get_random_word;

mod guesses;
mod words;

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

const TOTAL_LIVES: usize = 9;

pub(crate) struct Game {
    word: String,
    guesses: Vec<char>,
    lives_remaining: usize,
}

impl Game {
    pub(crate) fn new() -> io::Result<Self> {
        Ok(Game {
            word: get_random_word().to_uppercase(),
            guesses: Vec::new(),
            lives_remaining: TOTAL_LIVES,
        })
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
                    self.lives_remaining,
                );
                println!();
                self.output_current_word_state();
                println!();
            }
            GameStateResult::Lost => {
                println!("Oh no! You ran out of lives! I'll tell you what is was though:");
                println!();
                self.output_current_word_state();
                println!();
                println!("    {}", self.space_characters(&self.word));
                println!();
            }
        };
    }

    fn game_state(&self) -> GameState {
        if self.unknown_characters() == 0 {
            GameState::Complete(GameStateResult::Won)
        } else if self.lives_remaining == 0 {
            GameState::Complete(GameStateResult::Lost)
        } else {
            GameState::InProgress
        }
    }

    fn play_guess_round(&mut self, is_first_guess: bool) {
        if is_first_guess {
            println!("The word for you to guess is:");
            println!();
        }

        self.output_current_word_state();
        println!();

        self.output_lives_remaining();
        println!();

        println!("Please guess a letter, and make it a good one!");
        println!();

        self.output_previous_guesses();
        println!();

        let guess = prompt_for_guess();
        println!();

        self.output_guess(&guess);
        println!();

        match self.check_guess(guess) {
            Ok(success) => {
                match success {
                    TryGuessSuccess::InWord(character) => {
                        println!("Awesome! \"{}\" is in the word! Nice job!", character);
                        println!();
                    }
                    TryGuessSuccess::NotInWord(character) => {
                        println!("Sorry! \"{}\" is not in the word!", character);
                        println!();

                        self.lives_remaining -= 1;
                    }
                }

                self.guesses.push(guess)
            }
            Err(error) => match error {
                TryGuessError::AlreadyGuessed(character) => {
                    println!("You've already guessed \"{}\"!", character);
                }
            },
        };
    }

    fn unknown_characters(&self) -> usize {
        self.word
            .chars()
            .filter(|char| !self.guesses.contains(char))
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
                    '_'
                }
            })
            .collect::<String>();

        println!("    {}", self.space_characters(&current_word_state));
    }

    fn output_lives_remaining(&self) {
        println!("You have {} lives remaining.", self.lives_remaining);
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

    fn output_previous_guesses(&self) {
        println!(
            "Previous guesses: {}",
            self.guesses
                .iter()
                .map(|guess| guess.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    fn space_characters(&self, string: &str) -> String {
        let letters = string
            .chars()
            .map(|char| char.to_string())
            .collect::<Vec<String>>();

        letters.join(" ")
    }
}
