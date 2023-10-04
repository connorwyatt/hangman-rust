use crate::game::{
    CompleteGameStatus, Game, GameStatus, GuessStatus, MakeGuessError, MakeGuessSuccess,
};
use colored::Colorize;
use std::io::{stdin, Error};

pub(crate) struct StdIOGameRunner;

impl StdIOGameRunner {
    pub(crate) fn run(lives: usize, minimum_word_size: usize) {
        Self::print_intro();

        loop {
            let mut game = Game::new(lives, minimum_word_size);

            while game.status() == GameStatus::InProgress {
                Self::play_guess_round(&mut game);
            }

            match game.status() {
                GameStatus::InProgress => {
                    unreachable!("The loop above only ends when the game is complete.")
                }
                GameStatus::Complete(complete_game_status) => {
                    Self::handle_complete_game(&game, &complete_game_status)
                }
            }

            Self::prompt_for_new_game();
        }
    }

    fn print_intro() {
        println!("Welcome to");
        println!();

        let lines = [
            "===============================================",
            " _   _                                         ",
            "| | | | __ _ _ __   __ _ _ __ ___   __ _ _ __  ",
            "| |_| |/ _` | '_ \\ / _` | '_ ` _ \\ / _` | '_ \\ ",
            "|  _  | (_| | | | | (_| | | | | | | (_| | | | |",
            "|_| |_|\\__,_|_| |_|\\__, |_| |_| |_|\\__,_|_| |_|",
            "                   |___/                       ",
            "===============================================",
        ];

        let formatted_lines = lines
            .iter()
            .map(|&line| line.cyan().bold().to_string())
            .collect::<Vec<String>>();

        for formatted_line in formatted_lines {
            println!("{}", formatted_line);
        }

        println!();
    }

    fn play_guess_round(game: &mut Game) {
        println!("The word for you to guess is:");
        println!();

        Self::output_current_word_state(game);
        println!();

        Self::output_lives_remaining(game);
        println!();

        println!(
            "{}",
            "Please guess a letter, and make it a good one!".bold()
        );
        println!();

        Self::output_previous_guesses(game);
        println!();

        let guess = Self::read_input();
        println!();

        Self::handle_make_guess_result(&guess, game.make_guess(&guess));
    }

    fn output_current_word_state(game: &Game) {
        let letters = game.blanked_out_letters();

        println!("    {}", letters.join(" ").bold());
    }

    fn output_unblanked_word(game: &Game) {
        let letters = game.letters();

        println!("    {}", letters.join(" ").bold());
    }

    fn output_lives_remaining(game: &Game) {
        println!(
            "You have {} lives remaining.",
            Self::format_lives_remaining(game.lives_remaining())
        );
    }

    fn format_lives_remaining(lives_remaining: usize) -> String {
        match lives_remaining {
            1..=3 => lives_remaining.to_string().red().bold(),
            4..=6 => lives_remaining.to_string().yellow().bold(),
            _ => lives_remaining.to_string().green().bold(),
        }
        .to_string()
    }

    fn output_previous_guesses(game: &Game) {
        let guesses = game.guesses();

        if guesses.is_empty() {
            println!("{}", "No previous guesses.".italic().dimmed());
            return;
        }

        println!(
            "Previous guesses: {}",
            guesses
                .iter()
                .map(|(guess, status)| {
                    format!(
                        "{} {}",
                        guess,
                        match status {
                            GuessStatus::Correct => "\u{2713}".green(),
                            GuessStatus::Incorrect => "\u{2717}".red(),
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    fn handle_make_guess_result(
        guess: &String,
        make_guess_result: Result<MakeGuessSuccess, MakeGuessError>,
    ) {
        match make_guess_result {
            Ok(result) => match result {
                MakeGuessSuccess::Correct => {
                    println!(
                        "{}",
                        format!("\u{2713} Awesome! \"{}\" is in the word! Nice job!", guess)
                            .green()
                    )
                }
                MakeGuessSuccess::Incorrect => {
                    println!(
                        "{}",
                        format!("\u{2717} Sorry! \"{}\" is not in the word!", guess).red()
                    )
                }
            },
            Err(error) => match error {
                MakeGuessError::Empty => {
                    println!("{}", "\u{2717} Your guess was empty!".red())
                }
                MakeGuessError::TooLong => {
                    println!(
                        "{}",
                        "\u{2717} You entered more than one character! That's cheating!".red()
                    )
                }
                MakeGuessError::Invalid => {
                    println!(
                        "{}",
                        format!(
                            "\u{2717} You entered an invalid character! I don't know what to do with \"{}\".",
                            guess
                        ).red()
                    )
                }
                MakeGuessError::AlreadyGuessed => {
                    println!("{}", format!("You've already guessed \"{}\"!", guess).red())
                }
                MakeGuessError::GameComplete => {
                    panic!("Cannot make a guess as the game is complete.")
                }
            },
        }

        println!();
    }

    fn handle_complete_game(game: &Game, complete_game_status: &CompleteGameStatus) {
        match complete_game_status {
            CompleteGameStatus::Won => {
                let lives_remaining = game.lives_remaining();

                println!(
                    "{}",
                    format!(
                        "Well done! You guessed the word with {} {} remaining!",
                        lives_remaining,
                        if lives_remaining != 1 {
                            "guesses"
                        } else {
                            "guess"
                        }
                    )
                    .on_bright_green()
                    .bright_white()
                );
                println!();

                Self::output_current_word_state(game);
                println!();
            }
            CompleteGameStatus::Lost => {
                println!(
                    "{}",
                    "Oh no! You ran out of lives! I'll tell you what is was though:"
                        .on_bright_red()
                        .bright_white()
                );
                println!();

                Self::output_current_word_state(game);
                println!();

                Self::output_unblanked_word(game);
                println!();
            }
        }
    }

    fn prompt_for_new_game() -> bool {
        loop {
            println!(
                "Would you like to play again? {}",
                "(y/n)".dimmed().italic()
            );
            println!();

            let input = Self::read_input().to_lowercase();
            println!();

            if input == "y" {
                break true;
            } else if input == "n" {
                break false;
            }
        }
    }

    fn read_input() -> String {
        let mut result = Self::read_line();

        while result.is_err() {
            println!();

            result = Self::read_line();
        }

        result.unwrap()
    }

    fn read_line() -> Result<String, Error> {
        let mut buffer: String = String::new();
        stdin().read_line(&mut buffer)?;
        Ok(buffer.trim_end_matches(&['\r', '\n'][..]).to_string())
    }
}
