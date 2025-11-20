use crate::gameconfig::GameConfig;
use crate::gamelogic::check_for_matches;
use crate::savegame::handle_save;
use crate::types::{Color, Line};
use crate::{Bot, Gamestate};

use std::io::{self, Write};

pub fn continue_playing(gameconfig: &GameConfig, gamestate: &Gamestate, bot: &Option<Bot>) -> bool {
    loop {
        println!("\nDo you want to continue?");
        println!("\nOptions: (y) Play Next Round | (n) Quit | (s) Save Game");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "y" | "yes" => {
                return true;
            }
            "n" | "no" | "quit" => {
                return false;
            }
            "s" | "save" => {
                handle_save(gameconfig, gamestate, bot); // Don't return, loop restarts.
            }
            _ => {
                println!("Invalid selection. Please type 'y', 'n' or 's'.");
            }
        }
    }
}

// This is the core logic, extracted and reusable.
pub fn get_validated_line_input(pegs_count: usize, is_empty_allowed: bool) -> Line {
    loop {
        // Prompting is done by the caller (await_input or get_human_target_line)

        // Ensure stdout is flushed for immediate print
        io::stdout().flush().unwrap();

        let mut guess_input = String::new();
        io::stdin().read_line(&mut guess_input).unwrap();
        let guess = guess_input.trim();

        let colors: Vec<&str> = guess.split_whitespace().collect();
        let mut line = Line::empty(pegs_count);

        if colors.len() != pegs_count {
            println!("You must enter exactly {} colors.", pegs_count);
            continue;
        }

        let mut valid = true;
        for (i, c) in colors.iter().enumerate() {
            let color = parse_guess(c);
            // This is the specific logic to prevent setting an Invalid color unless "empty" was typed
            if color == Color::Empty && (c.to_lowercase() != "empty" || !is_empty_allowed) {
                println!("Invalid color: '{}'", c);
                valid = false;
                break;
            }
            line.pegs[i].color = color;
        }

        if valid {
            return line; // Return the fully validated Line
        }
    }
}

pub fn parse_guess(guess: &str) -> Color {
    match guess.to_lowercase().as_str() {
        "empty" => Color::Empty, // Allow empty peg
        "white" => Color::White,
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        _ => Color::Empty, // Invalid input
    }
}

pub fn await_input(gamestate: &mut Gamestate) {
    print!(
        "Enter {} colors (or 'empty') separated by spaces: ",
        gamestate.pegs_in_a_line
    );

    // Get the validated guess line
    let line = get_validated_line_input(gamestate.pegs_in_a_line, gamestate.is_empty_allowed);

    // Update Gamestate (This logic remains the same)
    gamestate.guessed_lines.push(line);
    let (flags, _) = check_for_matches(
        &gamestate.target_line,
        gamestate.guessed_lines.last().unwrap(),
    );
    gamestate.flag_pegs.push(flags);
    gamestate.guess_made = true;
}
