/* This file handles the user's initial setup, where we find out if the user wants to start a new
 * game or load a game as well as if he wants to use the config file or make a manual setup, it
 * then sends us where we need to go accordingly */

use crate::gameconfig::GameConfig;
use crate::manualconfig::get_manual_config;
use std::io::{self, Write};

pub enum StartupAction {
    NewGame(GameConfig),
    LoadGame,
}

pub fn user_setup() -> StartupAction {
    println!("(N)ew Game or (L)oad Game?");
    io::stdout().flush().unwrap();

    let load_game: bool = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "n" | "new" => break false,
            "l" | "load" => break true,
            _ => println!("Please enter 'n' or 'l':"),
        }
    };

    // If Load Game, return immediately. Don't ask for config.
    if load_game {
        return StartupAction::LoadGame;
    }

    println!("Do you want to use the config file for game settings? (y/n)");
    io::stdout().flush().unwrap();

    let use_config_file = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => break true,
            "n" | "no" => break false,
            _ => println!("Please enter 'y' or 'n':"),
        }
    };

    if use_config_file {
        // Try to load. If successful, return NewGame with that config.
        // If fail, fall through to manual.
        if let Some(cfg) = GameConfig::load_from_file("config.txt") {
            return StartupAction::NewGame(cfg);
        }
        println!("No config file found or invalid configuration. Entering manual setup...");
    }
    StartupAction::NewGame(get_manual_config())
}
