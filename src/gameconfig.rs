/*
    Configuration module.

    Provides the GameConfig struct and logic for loading game settings from
    a configuration file.

    Public API:
    - GameConfig: holds all user-configurable parameters.
    - GameConfig::load_from_file: reads a config file, applies defaults,
      parses key/value pairs, and returns a populated GameConfig.

    Internal helpers (private):
    - parse_bool: converts common string forms ("true", "yes", "1", "on") to bool.

    Notes:
    - Lines may contain comments; everything after '#' is ignored.
    - Missing or invalid values fall back to safe defaults.
    - Unknown keys are ignored but reported.
*/

use crate::{manualconfig::parse_game_mode, types::GameMode};

use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct GameConfig {
    pub game_mode: GameMode,
    pub number_of_guesses: u8,
    pub pegs_in_a_line: u8,
    pub is_empty_pegs_allowed: bool,
}

impl GameConfig {
    pub fn load_from_file(filename: &str) -> Option<Self> {
        if !Path::new(filename).exists() {
            println!("Config file not found, falling back to defaults...");
            return None;
        }

        let content = fs::read_to_string(filename).ok()?;

        // Default values in case a line is missing
        let mut game_mode = GameMode::Practice;
        let mut guesses = 10;
        let mut pegs = 4;
        let mut empty = false;

        for line in content.lines() {
            // Remove comments and whitespace
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "game_mode" => game_mode = parse_game_mode(value).unwrap(),
                    "number_of_guesses" => {
                        if let Ok(v) = value.parse::<u8>() {
                            if v > 0 {
                                guesses = v;
                            }
                        }
                    }
                    "pegs_in_a_line" => {
                        if let Ok(v) = value.parse::<u8>() {
                            if v > 0 {
                                pegs = v;
                            }
                        }
                    }
                    "include_empty_pegs" => {
                        empty = parse_bool(value);
                    }
                    _ => {
                        println!("Unknown key '{}' in config, ignoring...", key);
                    }
                }
            }
        }

        println!("Loaded configuration from {}!", filename);
        Some(GameConfig {
            game_mode: game_mode,
            number_of_guesses: guesses,
            pegs_in_a_line: pegs,
            is_empty_pegs_allowed: empty,
        })
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(
        value.to_lowercase().as_str(),
        "true" | "1" | "y" | "yes" | "on"
    )
}
