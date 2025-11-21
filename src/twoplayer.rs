use crate::bot::Bot;
use crate::gamelogic::{check_for_win, randomize_target_line};
use crate::gamestate::{GameMode, Gamestate};
use crate::io::{continue_playing, get_validated_line_input};
use crate::types::Line;

pub struct RoundResult {
    guesses_used: u8,
    is_win: bool,
    bonus: u8,
    score_delta: u8,
}

fn calculate_round_result(gamestate: &Gamestate) -> RoundResult {
    let guesses_used = gamestate.guessed_lines.len() as u8;
    let is_win = check_for_win(gamestate);
    let bonus = if !is_win { 1 } else { 0 };

    RoundResult {
        guesses_used,
        is_win,
        bonus,
        score_delta: guesses_used + bonus,
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

fn apply_score(gamestate: &mut Gamestate, score_delta: u8) {
    // Update Maker's score
    if gamestate.p1s_turn {
        gamestate.p2_score += score_delta;
    } else {
        gamestate.p1_score += score_delta;
    }
}

fn print_round_summary(gamestate: &Gamestate, result: &RoundResult) {
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

// fn handle_two_player_continue()

pub fn handle_two_player_end_of_round(gamestate: &mut Gamestate, bot: &Option<Bot>) -> bool {
    let result = calculate_round_result(gamestate);

    apply_score(gamestate, result.score_delta);

    print_round_summary(gamestate, &result);

    // Ask if players want to continue
    let continue_playing = continue_playing(gamestate, bot);

    if continue_playing {
        // Prepare for next round (clears board, swaps p1s_turn)
        gamestate.prepare_next_round(Line::empty(gamestate.pegs_in_a_line));

        // After prepare_next_round, roles have swapped:
        // If P1 is now the Breaker (p1s_turn=true), P2 is the Maker.
        let next_breaker_is_bot =
            gamestate.game_mode == GameMode::PlayerVsBot && !gamestate.p1s_turn;

        let target_line = if gamestate.game_mode == GameMode::TwoPlayer {
            get_human_target_line(gamestate)
        } else if next_breaker_is_bot {
            // PvB where P2 (Bot) is about to guess so player sets target.
            get_human_target_line(gamestate)
        } else {
            // PvB where P1 (Human) is about to guess so bot sets target.
            randomize_target_line(gamestate.pegs_in_a_line, gamestate.is_empty_allowed)
        };

        gamestate.target_line = target_line;
    }
    continue_playing
}

pub fn get_human_target_line(gamestate: &Gamestate) -> Line {
    let (maker, breaker) = match gamestate.game_mode {
        GameMode::TwoPlayer => {
            // P1 is breaker when p1s_turn is true, P2 is breaker when p1s_turn is false
            if gamestate.p1s_turn {
                ("PLAYER 2", "PLAYER 1")
            } else {
                ("PLAYER 1", "PLAYER 2")
            }
        }
        GameMode::PlayerVsBot => {
            // P1 is Maker (Human) only when P2 (Bot) is the Breaker.
            // This function is only called when the human (P1) is the Maker.
            ("PLAYER 1", "BOT")
        }
        _ => panic!("get_human_target_line called in inappropriate mode"),
    };

    println!("\n\n\n========================================");
    println!("{} (Code Maker): Please enter the secret code.", maker);
    println!("{}: LOOK AWAY!", breaker);
    println!("========================================\n");
    print!("ENTER {} COLORS: ", gamestate.pegs_in_a_line);

    // Former function clears screen
    let line = get_validated_line_input(gamestate.pegs_in_a_line, gamestate.is_empty_allowed);

    println!(
        "Code set! Scroll up strictly forbidden. Passing to Code Breaker ({})",
        breaker
    );
    line
}
