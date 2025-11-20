use crate::gamestate::GameMode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_mode: GameMode,
    pub number_of_guesses: u8,
    pub pegs_in_a_line: u8,
    pub is_empty_pegs_allowed: bool,
    pub is_bot_guesser: bool,
}

impl GameConfig {
    pub fn load_from_file(filename: &str) -> Option<Self> {
        if !Path::new(filename).exists() {
            println!("Config file not found, falling back to defaults...");
            return None;
        }

        let content = fs::read_to_string(filename).ok()?;

        // Default values in case a line is missing
        let mut game_mode = GameMode::SinglePlayer;
        let mut guesses = 10;
        let mut pegs = 4;
        let mut empty = false;
        let mut bot = false;

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
                    "game_mode" => {
                        game_mode = parse_game_mode(value);
                    }
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
                    "is_bot_guesser" => {
                        bot = parse_bool(value);
                    }
                    _ => {
                        println!("Unknown key '{}' in config, ignoring...", key);
                    }
                }
            }
        }

        println!("Loaded configuration from {}!", filename);
        Some(GameConfig {
            game_mode: game_mode, // Replace with actual value when supported
            number_of_guesses: guesses,
            pegs_in_a_line: pegs,
            is_empty_pegs_allowed: empty,
            is_bot_guesser: bot,
        })
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(
        value.to_lowercase().as_str(),
        "true" | "1" | "y" | "yes" | "on"
    )
}

fn parse_game_mode(value: &str) -> GameMode {
    let mode = value.trim().to_lowercase(); // Trim and lowercase for safe comparison
    match mode.as_str() {
        "1p" | "solo" | "single" | "singleplayer" | "single_player" => GameMode::SinglePlayer,
        "pvp" | "2p" | "two" | "two_player" | "twoplayer" | "2-player" => GameMode::TwoPlayer,
        "pvb" | "bot" | "vsbot" | "player_vs_bot" | "player-vs-bot" => GameMode::PlayerVsBot,
        other => {
            println!(
                "Warning: Unknown game_mode '{}', defaulting to SinglePlayer.",
                other
            );
            GameMode::SinglePlayer
        }
    }
}
