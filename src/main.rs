mod game;

use crate::game::Game;
use std::io::{stdin, Error};

fn main() -> Result<(), Error> {
    print_intro();

    loop {
        Game::new().play();

        println!("Press enter to play again!");
        stdin().read_line(&mut String::new())?;
    }
}

fn print_intro() {
    println!("Welcome to");
    println!();

    println!("    ===============================================");
    println!("     _   _                                         ");
    println!("    | | | | __ _ _ __   __ _ _ __ ___   __ _ _ __  ");
    println!("    | |_| |/ _` | '_ \\ / _` | '_ ` _ \\ / _` | '_ \\ ");
    println!("    |  _  | (_| | | | | (_| | | | | | | (_| | | | |");
    println!("    |_| |_|\\__,_|_| |_|\\__, |_| |_| |_|\\__,_|_| |_|");
    println!("                       |___/                       ");
    println!("    ===============================================");

    println!();
}
