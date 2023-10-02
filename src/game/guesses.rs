use std::io::{stdin, Error};

pub(crate) fn prompt_for_guess() -> char {
    let mut result = ask_for_guess();

    while let Err(error) = &result {
        match error {
            GuessError::Empty => {
                println!("Your guess was empty!")
            }
            GuessError::TooLong => {
                println!("You entered more than one character! That's cheating!")
            }
            GuessError::Invalid(invalid_character) => {
                println!(
                    "You entered an invalid character! I don't know what to do with \"{}\".",
                    invalid_character
                )
            }
            GuessError::Unknown => {
                println!("Your guess was invalid in a way I don't even understand!")
            }
        };

        println!();

        result = ask_for_guess();
    }

    result.unwrap().to_ascii_uppercase()
}

#[derive(Debug)]
enum GuessError {
    Unknown,
    Empty,
    TooLong,
    Invalid(char),
}

impl GuessError {
    fn from_error(_: Error) -> GuessError {
        GuessError::Unknown
    }
}

fn ask_for_guess() -> Result<char, GuessError> {
    let line = read_line().map_err(GuessError::from_error)?;
    let line = line.trim_end_matches(&['\r', '\n'][..]);

    if line.len() > 1 {
        return Err(GuessError::TooLong);
    }

    let Some(character) = line.chars().next() else {
        return Err(GuessError::Empty);
    };

    if !character.is_ascii_alphabetic() {
        return Err(GuessError::Invalid(character));
    }

    Ok(character)
}

fn read_line() -> Result<String, Error> {
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
