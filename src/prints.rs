/*
    Printing and display module.

    Handles terminal output related to game results, target lines, and round summaries.

    Public API:
    - print_win_or_loss: prints whether the target was solved and displays the target line.
    - print_round_summary: prints a summary of the round including winner/loser, scores,
      and bonus points if any.

    Internal helpers / private items:
    - print_target_line: prints a Line struct’s peg colors in a human-readable format.
    - get_player_strings: determines the display strings for Code Maker and Code Breaker
      based on game mode and current turn.

    Notes:
    - Supports Player vs Player and Player vs Bot modes.
    - Properly annotates BOT as Player 2 when applicable.
    - Includes bonuses in score reporting when present.
    - Round summaries include clear separation markers for readability.
*/

use crate::{
    gamelogic::RoundResult,
    gamestate::Gamestate,
    types::{GameMode, Line},
};

fn print_target_line(target: &Line) {
    for peg in target.pegs.iter() {
        print!("{:?} ", peg.color);
    }
    print!("\n");
}

pub fn print_win_or_loss(gamestate: &Gamestate, round_result: &RoundResult) {
    if round_result.is_win == true {
        print!(
            "Code solved with {} out of {} guesses!\nThe target was: ",
            round_result.guesses_used, gamestate.round_length
        );
        print_target_line(&gamestate.target_line);
    } else {
        print!(
            "Target not found in {} guesses.\nThe target was: ",
            gamestate.round_length
        );
        // Just wanna test this print for now. Nevermind it's perfection.
        print_target_line(&gamestate.target_line);
    }
}

fn get_player_strings(gamestate: &Gamestate) -> (&'static str, &'static str) {
    let p2_bot_string = "BOT";
    let is_p2_a_bot = gamestate.game_mode == GameMode::PlayerVsBot;

    // Logic to determine Maker vs Breaker
    // If p1s_turn is TRUE, it means P1 is currently guessing (Breaker)
    if gamestate.p1s_turn {
        let breaker = "Player 1";
        let maker = if is_p2_a_bot {
            p2_bot_string
        } else {
            "Player 2"
        };
        (maker, breaker)
    } else {
        // P2 is guessing (Breaker)
        let breaker = if is_p2_a_bot {
            p2_bot_string
        } else {
            "Player 2"
        };
        let maker = "Player 1";
        (maker, breaker)
    }
}

pub fn print_round_summary(gamestate: &Gamestate, result: &RoundResult) {
    let (maker, breaker) = get_player_strings(gamestate);

    let win_or_loss_str = if result.is_win {
        "solved"
    } else {
        "didn't solve"
    };
    let bonus_str = if result.bonus != 0 {
        format!(" + {} bonus", result.bonus)
    } else {
        "".to_string()
    };
    let p2_string = if gamestate.game_mode == GameMode::PlayerVsBot {
        "BOT"
    } else {
        "P2"
    };

    println!("\n--- ROUND {} ENDED ---", gamestate.current_round);
    println!(
        "{} {} the code in {} guesses.",
        breaker, win_or_loss_str, result.guesses_used
    );
    println!(
        "{} (Code Maker) gains {}{} points.",
        maker, result.guesses_used, bonus_str
    );
    println!(
        "--- SCOREBOARD: P1: {} | {}: {}",
        gamestate.p1_score, p2_string, gamestate.p2_score
    );
}
