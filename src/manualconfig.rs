/*
    Manual configuration module.

    Handles interactive setup of game parameters via user input.

    Public API:
    - get_manual_config: prompts the user for game mode, number of guesses,
      number of pegs, and whether empty pegs are allowed, returning a fully
      populated GameConfig.
    - parse_game_mode: parses a string input into a GameMode enum if valid.

    Internal helpers / private items:
    - ask_game_mode: repeatedly prompts the user until a valid game mode is selected.

    Notes:
    - Input is validated to ensure reasonable values (e.g., 1-255 for guesses/pegs).
    - Accepts multiple formats for game mode input (abbreviations, full names, numbers).
    - Empty peg inclusion is confirmed via yes/no style prompts.
*/

use crate::{gameconfig::GameConfig, types::GameMode};

use std::io::{self, Write};

pub fn get_manual_config() -> GameConfig {
    io::stdout().flush().unwrap();

    let game_mode = ask_game_mode();

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

    GameConfig {
        game_mode,
        number_of_guesses,
        pegs_in_a_line,
        is_empty_pegs_allowed,
    }
}

pub fn parse_game_mode(value: &str) -> Option<GameMode> {
    let mode = value.trim().to_lowercase();

    match mode.as_str() {
        "p" | "practice" => Some(GameMode::Practice),

        "2" | "pvp" | "2p" | "two" | "two_player" | "twoplayer" | "2-player" => {
            Some(GameMode::TwoPlayer)
        }

        "b" | "pvb" | "vsbot" | "player_vs_bot" | "player-vs-bot" => Some(GameMode::PlayerVsBot),
        "s" | "spectate" | "spectate_bot" => Some(GameMode::SpectateBot),
        _ => None,
    }
}

fn ask_game_mode() -> GameMode {
    loop {
        print!(
            "What game mode do you wish to play? (P)ractice / (2)-Player / Player Vs (B)ot) / (S)pectate Bot: "
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(mode) = parse_game_mode(&input) {
            return mode;
        }

        println!("Please enter '(P)ractice', '(2)-Player' or 'Player Vs (B)ot' (S)pectate Bot: ");
    }
}
