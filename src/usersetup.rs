/*
    Startup module.

    Handles the user's initial setup, determining whether to start a new game
    or load an existing one, and whether to use a config file or manual settings.

    Public API:
    - StartupAction: enum representing the user's choice:
        - NewGame(GameConfig): start a new game with the specified configuration.
        - LoadGame: load a previously saved game.
    - user_setup: interactively asks the user for choices and returns a StartupAction.

    Notes:
    - If the user opts to load a game, no further configuration is requested.
    - If the user chooses a new game, the module checks for a config file first,
      falling back to manual configuration if none is found or invalid.
    - All input is validated with loops until a valid response is provided.
*/

use crate::{gameconfig::GameConfig, manualconfig::get_manual_config};
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
