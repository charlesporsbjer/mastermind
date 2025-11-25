/*
    Two-player game module.

    Handles logic specific to two-player or player-vs-bot rounds, including
    score calculation and human code entry.

    Public API:
    - two_player_end_of_round_score_and_prints: updates the Maker's score based
      on the round result and prints a summary of the round.
    - get_human_target_line: prompts the human player to input a secret code
      while ensuring the other player (or bot) cannot see it.

    Internal helpers / private items:
    - apply_2p_score: updates the Maker's score based on the score delta.

    Notes:
    - Assumes correct game mode when calling get_human_target_line.
    - Prints clear instructions and warnings to maintain secrecy of the target code.
    - Integrates with gamestate and RoundResult for score tracking and round summaries.
*/

use crate::{
    gamelogic::RoundResult,
    gamestate::Gamestate,
    parse::get_validated_line_input,
    prints::print_round_summary,
    types::{GameMode, Line},
};

fn apply_2p_score(gamestate: &mut Gamestate, score_delta: u8) {
    // Update Maker's score
    if gamestate.p1s_turn {
        gamestate.p2_score += score_delta;
    } else {
        gamestate.p1_score += score_delta;
    }
}

pub fn two_player_end_of_round_score_and_prints(
    gamestate: &mut Gamestate,
    round_result: &RoundResult,
) {
    apply_2p_score(gamestate, round_result.score_delta);

    print_round_summary(gamestate, &round_result);
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
    let line = get_validated_line_input(&gamestate);

    println!(
        "Code set! Scroll up strictly forbidden. Passing to Code Breaker ({})",
        breaker
    );
    line
}
