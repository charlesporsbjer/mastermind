use crate::bot::Bot;
use crate::gameconfig::GameConfig;
use crate::gamestate::Gamestate;
use crate::savegame::{SAVE_DIR, SaveData};

use std::fs;
use std::io;
use std::io::Write;

pub fn handle_load() -> (GameConfig, Gamestate, Option<Bot>) {
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
    let save_data = load_game(&save_name).unwrap();
    (save_data.game_config, save_data.gamestate, save_data.bot)
}

pub fn load_game(filename: &str) -> io::Result<SaveData> {
    let path = format!("{}/{}.json", SAVE_DIR, filename.replace(".json", ""));

    let content = fs::read_to_string(path)?;
    let data: SaveData = serde_json::from_str(&content)?;

    Ok(data)
}

pub fn list_save_files() -> Vec<String> {
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
