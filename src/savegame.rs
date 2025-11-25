/*
    Save game module.

    Handles saving and autosaving of game state to the filesystem.

    Public API:
    - autosave: saves the current Gamestate to a temporary "autosave.json" file.
    - handle_save_from_autosave: prompts the user to provide a filename and
      copies the autosave to a permanent save file.

    Internal helpers / private items:
    - init_save_sys: ensures the save directory exists, creates it if missing.
    - rename_autosave: copies/renames the autosave file to a user-specified filename.

    Notes:
    - All saves are stored in the "savegames" directory.
    - Filenames are normalized to avoid ".json" duplication.
    - handle_save_from_autosave allows the user to cancel or retry on errors.
    - Uses serde_json for serializing Gamestate to JSON.
*/

use crate::gamestate::Gamestate;

use ::std::fs;
use std::io::{self, Write};
use std::path::Path;

pub const SAVE_DIR: &str = "savegames";

// Ensure save dir exists
fn init_save_sys() {
    if !Path::new(SAVE_DIR).exists() {
        if let Err(e) = fs::create_dir(SAVE_DIR) {
            eprintln!("Failed to create save directory: {}", e);
        }
    }
}

pub fn autosave(gamestate: &mut Gamestate) -> io::Result<()> {
    init_save_sys();
    let save_data = gamestate.clone();

    let json_string = serde_json::to_string_pretty(&save_data)?;
    let path = format!("{}/autosave.json", SAVE_DIR);

    fs::write(path, json_string)
}

fn rename_autosave(new_name: &str) -> io::Result<()> {
    init_save_sys();

    let src = format!("{}/autosave.json", SAVE_DIR);
    let dest = format!("{}/{}.json", SAVE_DIR, new_name.replace(".json", ""));

    fs::copy(&src, dest)?;

    Ok(())
}

pub fn handle_save_from_autosave() {
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
        match rename_autosave(name) {
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
