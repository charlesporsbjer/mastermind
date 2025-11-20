mod bot;
mod draw;
mod gameconfig;
mod gamelogic;
mod gamestate;
mod io;
mod loadgame;
mod manualconfig;
mod savegame;
mod types;
mod usersetup;

use crate::bot::Bot;
use crate::draw::draw_board;
use crate::gameconfig::GameConfig;
use crate::gamelogic::{
    check_for_loss, check_for_win, create_new_solo_session, get_human_target_line,
    handle_bot_input, handle_two_player_end_of_round, print_complexity_analysis,
    randomize_target_line,
};
use crate::gamestate::{GameMode, Gamestate};
use crate::io::{await_input, continue_playing, print_win_or_loss};
use crate::loadgame::handle_load;
use crate::types::Line;
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

    let (config, mut gamestate, mut bot) = match user_setup() {
        StartupAction::LoadGame => handle_load(),
        StartupAction::NewGame(cfg) => {
            let no_of_pegs = cfg.pegs_in_a_line as usize;

            // Setup Bot
            let needs_bot = cfg.is_bot_guesser || cfg.game_mode == GameMode::PlayerVsBot;
            let bot = if needs_bot {
                Some(Bot::new(cfg.is_empty_pegs_allowed, no_of_pegs))
            } else {
                None
            };

            // Setup Target Line
            let target_line = match cfg.game_mode {
                GameMode::TwoPlayer => Line::empty(no_of_pegs),
                GameMode::SinglePlayer | GameMode::PlayerVsBot => {
                    randomize_target_line(no_of_pegs, cfg.is_empty_pegs_allowed)
                }
            };

            // Setup Gamestate
            let mut gs = Gamestate::new(
                cfg.game_mode,
                cfg.number_of_guesses,
                no_of_pegs,
                target_line,
                cfg.is_empty_pegs_allowed,
            );

            // Handle initial Human Input for PvP
            if cfg.game_mode == GameMode::TwoPlayer {
                gs.target_line = get_human_target_line(&mut gs);
            }

            if cfg.is_bot_guesser && cfg.game_mode == GameMode::SinglePlayer {
                print_complexity_analysis(no_of_pegs, cfg.is_empty_pegs_allowed);
            }

            (cfg, gs, bot)
        }
    };

    // Initialize game variables using config object
    let game_mode = config.game_mode;
    let pegs_in_a_line = config.pegs_in_a_line;
    let is_empty_pegs_allowed = config.is_empty_pegs_allowed;
    let is_bot_guesser = config.is_bot_guesser; // Determines guesser in solo mode

    let no_of_pegs = pegs_in_a_line as usize;

    // MAIN GAME LOOP
    'game_session: loop {
        let current_guesser_is_bot = match game_mode {
            GameMode::TwoPlayer => false,                 // PvP is always human
            GameMode::PlayerVsBot => !gamestate.p1s_turn, // Bot plays as P2 (guesses when it's P2's turn)
            GameMode::SinglePlayer => is_bot_guesser,     // Guided by flag (fixed role)
        };

        if current_guesser_is_bot {
            if let Some(bot_ref) = bot.as_mut() {
                handle_bot_input(bot_ref, &mut gamestate);
            } else {
                await_input(&mut gamestate); // Fallback if bot wasn't initiated. (Should never happen) REMOVE?
            }
        } else {
            // Human is guessing (P1 in PvB, or Solo mode without bot, or TwoPlayer)
            await_input(&mut gamestate);
        }

        // Check round end conditions.
        let rounds_used = gamestate.guessed_lines.len();
        let is_win = check_for_win(&gamestate);
        let is_loss = check_for_loss(&gamestate);
        let round_over = is_win || is_loss;

        if round_over {
            draw_board(&gamestate);

            print_win_or_loss(is_win, rounds_used, gamestate.clone());

            if game_mode == GameMode::TwoPlayer || game_mode == GameMode::PlayerVsBot {
                // Both PvP and PvB use the same end-of-round logic (scoring, swapping, input)

                if handle_two_player_end_of_round(
                    &config,
                    &mut gamestate,
                    &bot,
                    is_empty_pegs_allowed,
                ) {
                    // Reset Bot's solution set if continuing
                    if let Some(bot_ref) = bot.as_mut() {
                        bot_ref.reset_possible_solutions(is_empty_pegs_allowed, no_of_pegs);
                    }
                    continue; // Continue to the next round in the session
                } else {
                    break 'game_session; // Exit the entire application
                }
            }

            let continue_playing = continue_playing(&config, &gamestate, &bot);

            if continue_playing {
                // Reset the gamestate for a new session
                gamestate = create_new_solo_session(game_mode, &config);
                // Reset Bot's solution set if continuing
                if let Some(bot_ref) = bot.as_mut() {
                    bot_ref.reset_possible_solutions(is_empty_pegs_allowed, no_of_pegs);
                }
                continue 'game_session;
            } else {
                break 'game_session;
            }
        }
        draw_board(&gamestate);
    }
}
