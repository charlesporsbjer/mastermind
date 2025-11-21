mod bot;
mod draw;
mod gameconfig;
mod gamelogic;
mod gamestate;
mod io;
mod loadgame;
mod manualconfig;
mod savegame;
mod startup;
mod twoplayer;
mod types;
mod usersetup;

use crate::bot::Bot;
use crate::draw::draw_board;
use crate::gameconfig::GameConfig;
use crate::gamelogic::{check_for_loss, check_for_win, create_new_solo_session, handle_bot_input};
use crate::gamestate::{GameMode, Gamestate};
use crate::io::{await_input, continue_playing, print_win_or_loss};
use crate::startup::handle_startup;
use crate::twoplayer::handle_two_player_end_of_round;
use crate::usersetup::{StartupAction, user_setup};

use std::env;

// TODO: Make bot guess better, DONE
// make initial bot guess a bit more random, DONE
// make number of pegs variable, DONE
// make print more pleasant, DONE
// make print react to amount of pegs, DONE
// allow for use of config file, DONE
// allow p1 to compete against the AI, DONE
// change pvp and pvb prints to be more clear about whos turn it is, DONE
// make sure player cannot enter Color::empty if it is not in the game, DONE
// allow for user to save and load game. DONE
// obscure target line after user enters it somehow DONE
// Refine solution print when user is out of guesses. DONE
// Solve bug where player is said to have won when out of guesses. DONE
// PRIORITY: Solve bug where information is added at load game.
// NORMAL: Convert bot logic from HashSet to Index-To-Line Conversion / Base-N Counting.
// OPTIONAL: fix the math in time_estimation
// OPTIONAL: expand number of colors to allow more colors, POSTPONED
// OPTIONAL: use real graphics lib.

// REMINDER FOR OTHER PROJECT: Huffman, ANS, Arithmetic Encoding, jpeg excell
fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    println!("Mastermind is running!");

    let startup_action = user_setup();
    let (mut gamestate, mut bot) = handle_startup(startup_action);

    // MAIN GAME LOOP
    'game_session: loop {
        let is_current_guesser_bot = match gamestate.game_mode {
            GameMode::TwoPlayer => false,                 // PvP is always human
            GameMode::PlayerVsBot => !gamestate.p1s_turn, // Bot plays as P2 (guesses when it's P2's turn)
            GameMode::SinglePlayer => gamestate.is_bot_only_guesser, // Guided by flag (fixed role)
            GameMode::SpectateBot => true,
        };

        if is_current_guesser_bot {
            if let Some(bot_ref) = bot.as_mut() {
                handle_bot_input(bot_ref, &mut gamestate);
            } else {
                await_input(&mut gamestate); // Fallback if bot wasn't initiated. (Should never happen) REMOVE?
            }
        } else if !gamestate.round_over {
            // Human is guessing (P1 in PvB, or Solo mode without bot, or TwoPlayer)
            await_input(&mut gamestate);
        }

        // Check round end conditions.
        let rounds_used = gamestate.guessed_lines.len();
        let is_win = check_for_win(&gamestate);
        let is_loss = check_for_loss(&gamestate);
        gamestate.round_over = is_win || is_loss;

        if gamestate.round_over {
            draw_board(&gamestate);

            print_win_or_loss(is_win, rounds_used, gamestate.clone());

            if gamestate.game_mode == GameMode::TwoPlayer
                || gamestate.game_mode == GameMode::PlayerVsBot
            {
                // Both PvP and PvB use the same end-of-round logic (scoring, swapping, input)

                if handle_two_player_end_of_round(&mut gamestate, &bot) {
                    // Reset Bot's solution set if continuing
                    if let Some(bot_ref) = bot.as_mut() {
                        bot_ref.reset_possible_solutions(&gamestate);
                    }
                    continue; // Continue to the next round in the session
                } else {
                    break 'game_session; // Exit the entire application
                }
            }

            let continue_playing = continue_playing(&gamestate, &bot);

            if continue_playing {
                // Reset the gamestate for a new session
                gamestate = create_new_solo_session(&gamestate);
                // Reset Bot's solution set if continuing
                if let Some(bot_ref) = bot.as_mut() {
                    bot_ref.reset_possible_solutions(&gamestate);
                }
                continue 'game_session;
            } else {
                break 'game_session;
            }
        }
    }
}
