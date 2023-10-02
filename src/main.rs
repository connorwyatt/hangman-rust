mod game;

use crate::game::Game;
use colored::Colorize;
use std::io::{stdin, Error};

fn main() -> Result<(), Error> {
    print_intro();

    loop {
        Game::new()?.play();

        println!("{}", "Press enter to play again!".bold());
        stdin().read_line(&mut String::new())?;
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
