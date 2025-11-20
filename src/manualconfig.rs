use crate::{gameconfig::GameConfig, gamestate::GameMode};

use std::io::{self, Write};

pub fn get_manual_config() -> GameConfig {
    io::stdout().flush().unwrap();

    println!("What game mode do you wish to play? (Solo / 2P / PvB (PlayerVSBot))");
    let game_mode: GameMode = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "solo" => break GameMode::SinglePlayer,
            "2p" => break GameMode::TwoPlayer,
            "pvb" => break GameMode::PlayerVsBot,
            _ => println!("Please enter 'solo', '2p' or 'pvb':"),
        }
    };

    println!("How many guesses do you want? (1-255)");
    let number_of_guesses: u8 = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u8>() {
            Ok(0) => {
                println!("0 is not allowed. Please enter a number from 1 to 255:");
            }
            Ok(n) => break n,
            Err(_) => println!("Please enter a number from 1 to 255:"),
        }
    };

    println!("How many pegs in a line do you wish to play with? (Base rules is 4)");
    let pegs_in_a_line: u8 = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u8>() {
            Ok(0) => {
                println!("0 is not allowed. Please enter a number from 1 to 255:");
            }
            Ok(n) => break n,
            Err(_) => println!("Please enter a number from 1 to 255:"),
        }
    };

    println!("Do you wish to include empty pegs? (y/n)");
    let is_empty_pegs_allowed: bool = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" => break true,
            "yes" => break true,
            "n" => break false,
            "no" => break false,
            _ => println!("Please enter 'y' or 'n':"),
        }
    };

    println!("Do you want a bot to make the guesses? (y/n)");
    let is_bot_guesser: bool = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" => break true,
            "yes" => break true,
            "n" => break false,
            "no" => break false,
            _ => println!("Please enter 'y' or 'n':"),
        }
    };

    GameConfig {
        game_mode,
        number_of_guesses,
        pegs_in_a_line,
        is_empty_pegs_allowed,
        is_bot_guesser,
    }
}
