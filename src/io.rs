use crate::gameconfig::GameConfig;
use crate::gamelogic::check_for_matches;
use crate::savegame::handle_save;
use crate::types::{Color, Line};
use crate::{Bot, Gamestate};

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write, stdout};

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

fn print_target_line(target: &Line) {
    for peg in target.pegs.iter() {
        print!("{:?} ", peg.color);
    }
    print!("\n");
}

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

pub fn print_win_or_loss(is_win: bool, rounds_used: usize, gamestate: Gamestate) {
    if is_win {
        println!(
            "You won with {} out of {} guesses! Target was solved.",
            rounds_used, gamestate.round_length
        );
    } else {
        print!(
            "You lost! Target not found in {} guesses.\nThe target was: ",
            gamestate.round_length
        );
        // Just wanna test this print for now.
        print_target_line(&gamestate.target_line);
    }
}

pub fn get_validated_line_input(pegs_count: usize, is_empty_allowed: bool) -> Line {
    loop {
        io::stdout().flush().unwrap();

        let mut guess_input = String::new();
        io::stdin().read_line(&mut guess_input).unwrap();
        let guess = guess_input.trim();

        let colors: Vec<&str> = guess.split_whitespace().collect();
        let mut line = Line::empty(pegs_count);

        if colors.len() != pegs_count {
            clear_screen();
            println!("You must enter exactly {} colors.", pegs_count);
            continue;
        }

        let mut valid = true;
        for (i, c) in colors.iter().enumerate() {
            let color = parse_guess(c);
            // Prevent setting an Invalid color unless "empty" was typed
            if color == Color::Empty && (c.to_lowercase() != "empty" || !is_empty_allowed) {
                clear_screen();
                println!("Invalid color: '{}'", c);
                valid = false;
                break;
            }
            line.pegs[i].color = color;
        }

        if valid {
            clear_screen();
            return line;
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

    // Get validated guess line
    let line = get_validated_line_input(gamestate.pegs_in_a_line, gamestate.is_empty_allowed);

    // Update Gamestate
    gamestate.guessed_lines.push(line);
    let (flags, _) = check_for_matches(
        &gamestate.target_line,
        gamestate.guessed_lines.last().unwrap(),
    );
    gamestate.flag_pegs.push(flags);
    gamestate.guess_made = true;
}
