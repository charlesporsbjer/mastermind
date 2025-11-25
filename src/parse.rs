/*
    Parsing and terminal interaction module.

    Handles user input parsing, screen clearing, and continuing game prompts.

    Public API:
    - clear_screen: clears the terminal screen using crossterm.
    - continue_playing: asks the user if they want to continue playing, quit, or save,
      returns true/false for continuation.
    - get_validated_line_input: reads a line of user input, validates color guesses
      according to gamestate, and returns a populated Line struct.

    Internal helpers / private items:
    - hide_line: conditionally clears the screen if input should be hidden.
    - parse_guess: converts a string input into a Color enum, supports
      abbreviations and "empty" if allowed.

    Notes:
    - Validates the exact number of pegs for guesses.
    - Supports flexible input for colors (full names or single-letter abbreviations).
    - Hides input for TwoPlayer mode to prevent cheating.
    - Loop continues until valid input is provided.
*/

use crate::{
    gamestate::Gamestate,
    savegame::handle_save_from_autosave,
    types::{Color, GameMode, Line},
};

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write, stdout};

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

pub fn continue_playing() -> bool {
    loop {
        println!("\nDo you want to continue?");
        println!("\nOptions: (y) Play Next Round | (n) Quit | (s) Save Game");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "y" | "yes" | "p" | "play" => {
                return true;
            }
            "n" | "no" | "q" | "quit" => {
                return false;
            }
            "s" | "save" => {
                handle_save_from_autosave(); // Don't return, loop restarts.
            }
            _ => {
                println!("Invalid selection. Please type 'y', 'n' or 's'.");
            }
        }
    }
}

fn hide_line(need_to_hide_line: bool) {
    if need_to_hide_line {
        clear_screen();
    }
}

pub fn get_validated_line_input(gamestate: &Gamestate) -> Line {
    let need_to_hide_line = if gamestate.game_mode == GameMode::TwoPlayer {
        true
    } else {
        false
    };
    loop {
        io::stdout().flush().unwrap();

        let mut guess_input = String::new();
        io::stdin().read_line(&mut guess_input).unwrap();
        let guess = guess_input.trim();

        let colors: Vec<&str> = guess.split_whitespace().collect();
        let mut line = Line::empty(gamestate.pegs_in_a_line);

        if colors.len() != gamestate.pegs_in_a_line {
            hide_line(need_to_hide_line);
            println!(
                "You must enter exactly {} colors.",
                gamestate.pegs_in_a_line
            );
            continue;
        }

        let mut valid = true;
        for (i, c) in colors.iter().enumerate() {
            let color = parse_guess(c);
            // Prevent setting an Invalid color unless "empty" was typed
            if color == Color::Empty && (c.to_lowercase() != "empty" || !gamestate.is_empty_allowed)
            {
                hide_line(need_to_hide_line);
                println!("Invalid color: '{}'", c);
                valid = false;
                break;
            }
            line.pegs[i].color = color;
        }

        if valid {
            hide_line(need_to_hide_line);
            return line;
        }
    }
}

fn parse_guess(guess: &str) -> Color {
    match guess.to_lowercase().as_str() {
        "e" | "empty" => Color::Empty, // Allow empty peg
        "w" | "white" => Color::White,
        "b" | "black" => Color::Black,
        "r" | "red" => Color::Red,
        "g" | "green" => Color::Green,
        "u" | "blue" => Color::Blue,
        "y" | "yellow" => Color::Yellow,
        _ => Color::Empty, // Invalid input
    }
}
