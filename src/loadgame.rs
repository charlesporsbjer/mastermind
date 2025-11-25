/*
    Loading module.

    Handles reading saved game data from disk and reconstructing the Gamestate.

    Public API:
    - handle_load: prompts the user to select a save file from the save directory,
      loads it, and returns a fully populated Gamestate.

    Internal helpers / private items:
    - load_game: reads a JSON file and deserializes it into a Gamestate.
    - list_save_files: scans the save directory for valid JSON save files.

    Notes:
    - Save files are expected to be stored as JSON in the SAVE_DIR directory.
    - Filenames are matched without the ".json" extension.
    - User input is validated against existing save files; the prompt loops until
      a valid save is chosen.
*/

use crate::{gamestate::Gamestate, savegame::SAVE_DIR};

use std::fs;
use std::io;
use std::io::Write;

pub fn handle_load() -> Gamestate {
    let mut save_name = String::new();
    println!("Type the name of your save to load it.");
    println!("Saves: ");

    for save in list_save_files() {
        println!("{} ", save);
    }

    io::stdout().flush().unwrap();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let save_input = input.trim().to_string();

        for save in list_save_files() {
            if save == save_input {
                save_name = save_input.clone();
                break;
            }
        }
        if !save_name.is_empty() {
            break;
        }
        println!("No save with name: {}. Try again.", save_input);
    }
    let saved_gs = load_game(&save_name).unwrap();

    saved_gs
}

fn load_game(filename: &str) -> io::Result<Gamestate> {
    let path = format!("{}/{}.json", SAVE_DIR, filename.replace(".json", ""));

    let content = fs::read_to_string(path)?;
    let data: Gamestate = serde_json::from_str(&content)?;

    Ok(data)
}

fn list_save_files() -> Vec<String> {
    let mut saves = Vec::new();

    if let Ok(entries) = fs::read_dir(SAVE_DIR) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        saves.push(stem.to_string());
                    }
                }
            }
        }
    }
    saves
}
