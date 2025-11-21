use crate::bot::Bot;
use crate::gamestate::Gamestate;

use ::std::fs;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::Path;

pub const SAVE_DIR: &str = "savegames";

// Wraps all data to reconstruct game.
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub gamestate: Gamestate,
    pub bot: Option<Bot>,
}

// Ensure save dir exists
pub fn init_save_sys() {
    if !Path::new(SAVE_DIR).exists() {
        if let Err(e) = fs::create_dir(SAVE_DIR) {
            eprintln!("Failed to create save directory: {}", e);
        }
    }
}

pub fn handle_save(gamestate: &Gamestate, bot: &Option<Bot>) {
    loop {
        print!("Enter save name (or type 'cancel' to go back): ");
        io::stdout().flush().unwrap();

        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        // Allow user to abort
        if name.eq_ignore_ascii_case("cancel") {
            println!("Save cancelled.");
            break;
        }

        // Don't allow empty filenames
        if name.is_empty() {
            continue;
        }

        // Attempt Save
        match save_game(name, gamestate, bot) {
            Ok(_) => {
                println!("Game saved successfully to {}/{}.sjon", SAVE_DIR, name);
                break;
            }
            Err(e) => {
                println!("Error saving game: {}. Please try again.", e);
            }
        }
    }
}

pub fn save_game(filename: &str, gamestate: &Gamestate, bot: &Option<Bot>) -> io::Result<()> {
    init_save_sys();

    let save_data = SaveData {
        gamestate: gamestate.clone(), // Clone needed to own data for serialzation.
        bot: bot.clone(),             // Might be slow if bot is huge, with HashSet.
    };

    // Serialize to pretty printed JSON string.
    let json_string = serde_json::to_string_pretty(&save_data)?;

    // Esnure filename has .json extension.
    let path = format!("{}/{}.json", SAVE_DIR, filename.replace(".json", ""));

    fs::write(path, json_string)?;

    Ok(())
}
