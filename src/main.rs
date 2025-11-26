mod bot;
mod draw;
mod gameconfig;
mod gamelogic;
mod gamestate;
mod loadgame;
mod manualconfig;
mod parse;
mod prints;
mod savegame;
mod startup;
mod twoplayer;
mod types;
mod usersetup;

use crate::{
    bot::bot_guess,
    draw::draw_board,
    gamelogic::{LoopAction, handle_end_of_round, human_guess},
    gamestate::RoundStatus,
    prints::print_complexity_analysis,
    startup::handle_startup,
    types::GameMode,
    usersetup::user_setup,
};

use std::env;

// TODO:
// OPTIONAL: Convert bot logic from HashSet to Index-To-Line Conversion / Base-N Counting.
// OPTIONAL: fix the math in time_estimation.
// OPTIONAL: expand number of colors to allow more colors.
// OPTIONAL: use real graphics lib.

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    println!("Mastermind is running!");

    let startup_action = user_setup();
    let (mut gamestate, mut bot) = handle_startup(startup_action);
    match gamestate.game_mode {
        GameMode::Practice | GameMode::SpectateBot | GameMode::PlayerVsBot => {
            gamestate.target_line = gamestate.randomize_target_line();
        }
        _ => {}
    }

    // MAIN GAME LOOP
    'game_session: loop {
        gamestate.is_bot_guessing_this_round = gamestate.is_it_bots_turn_to_guess();

        // --- INPUTS ---
        if !gamestate.round_over {
            if gamestate.is_bot_guessing_this_round {
                bot_guess(&mut gamestate, &mut bot);
            } else if !gamestate.round_over {
                // Human is guessing (P1 in PvB, Practice, or TwoPlayer)
                human_guess(&mut gamestate);
            }
        }

        // --- UPDATE ROUND STATES ---
        gamestate.update_round_status();
        gamestate.round_over = (gamestate.round_status == RoundStatus::Win)
            || (gamestate.round_status == RoundStatus::Loss);

        // --- DRAW BOARD ---
        draw_board(&gamestate);
        if gamestate.game_mode == GameMode::SpectateBot {
            print_complexity_analysis(gamestate.pegs_in_a_line, gamestate.is_empty_allowed);
        }

        // --- HANDLE ROUND END ---
        if gamestate.round_over {
            match handle_end_of_round(&mut gamestate, &mut bot) {
                LoopAction::Continue => continue 'game_session,
                LoopAction::Break => break 'game_session,
            }
        }
    }
}
