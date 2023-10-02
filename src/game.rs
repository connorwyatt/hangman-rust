use crate::game::guesses::prompt_for_guess;
use colored::Colorize;
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
                    "{}",
                    format!(
                        "Well done! You guessed the word with {} {} remaining!",
                        self.lives_remaining,
                        if self.lives_remaining != 1 {
                            "guesses"
                        } else {
                            "guess"
                        }
                    )
                    .on_bright_green()
                    .bright_white()
                );
                println!();
                self.output_current_word_state();
                println!();
            }
            GameStateResult::Lost => {
                println!(
                    "{}",
                    "Oh no! You ran out of lives! I'll tell you what is was though:"
                        .on_bright_red()
                        .bright_white()
                );
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

        println!(
            "{}",
            "Please guess a letter, and make it a good one!".bold()
        );
        println!();

        self.output_previous_guesses();
        println!();

        let guess = prompt_for_guess();
        println!();

        if self.is_already_guessed(&guess) {
            println!("{}", format!("You've already guessed \"{}\"!", guess).red());
            println!();
            return;
        }

        if self.is_letter_in_word(&guess) {
            println!(
                "{}",
                format!("\u{2713} Awesome! \"{}\" is in the word! Nice job!", &guess).green()
            );
            println!();
        } else {
            println!(
                "{}",
                format!("\u{2717} Sorry! \"{}\" is not in the word!", &guess).red()
            );
            println!();
            self.lives_remaining -= 1;
        }

        self.guesses.push(guess)
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

        println!("    {}", self.space_characters(&current_word_state).bold());
    }

    fn output_lives_remaining(&self) {
        println!(
            "You have {} lives remaining.",
            self.format_lives_remaining()
        );
    }

    fn format_lives_remaining(&self) -> String {
        match self.lives_remaining {
            1..=3 => self.lives_remaining.to_string().red().bold(),
            4..=6 => self.lives_remaining.to_string().yellow().bold(),
            _ => self.lives_remaining.to_string().green().bold(),
        }
        .to_string()
    }

    fn is_already_guessed(&self, letter: &char) -> bool {
        self.guesses.contains(letter)
    }

    fn is_letter_in_word(&self, letter: &char) -> bool {
        let word_chars = self.word.chars().collect::<Vec<char>>();

        word_chars.contains(letter)
    }

    fn output_previous_guesses(&self) {
        if self.guesses.is_empty() {
            println!("{}", "No previous guesses.".italic().dimmed());
            return;
        }

        println!(
            "Previous guesses: {}",
            self.guesses
                .iter()
                .map(|guess| {
                    format!(
                        "{} {}",
                        guess,
                        if self.is_letter_in_word(guess) {
                            "\u{2713}".green()
                        } else {
                            "\u{2717}".red()
                        }
                    )
                })
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
