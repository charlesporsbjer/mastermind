/* This file contains the game logic functions */

use crate::{
    bot::{Bot, reset_bot_for_new_round},
    gamestate::Gamestate,
    parse::{continue_playing, get_validated_line_input},
    prints::print_win_or_loss,
    savegame::autosave,
    twoplayer::{get_human_target_line, two_player_end_of_round_score_and_prints},
    types::{Color, Feedback, GameMode, Line},
};

pub fn check_for_matches(target: &Line, guess: &Line) -> (Line, Feedback) {
    let width = target.pegs.len();
    let mut flags = Line::empty(width);
    let mut target_used = vec![false; width];
    let mut flag_index: usize = 0;
    let mut feedback = Feedback {
        correct_position: 0,
        correct_color: 0,
    };

    for i in 0..width {
        if guess.pegs[i].color == target.pegs[i].color {
            // Hard match
            flags.pegs[flag_index].color = Color::Black;
            flag_index += 1;
            target_used[i] = true;
            feedback.correct_position += 1;
        }
    }
    for i in 0..width {
        if guess.pegs[i].color != target.pegs[i].color {
            for j in 0..width {
                if !target_used[j] && guess.pegs[i].color == target.pegs[j].color {
                    flags.pegs[flag_index].color = Color::White;
                    flag_index += 1;
                    target_used[j] = true;
                    feedback.correct_color += 1;
                    break;
                }
            }
        }
    }
    (flags, feedback)
}

pub fn human_guess(gamestate: &mut Gamestate) {
    print!(
        "Enter {} colors (or 'empty') separated by spaces: ",
        gamestate.pegs_in_a_line
    );

    // Get validated guess line
    let line = get_validated_line_input(&gamestate);

    // Update Gamestate
    gamestate.guessed_lines.push(line);
    let (flags, _) = check_for_matches(
        &gamestate.target_line,
        gamestate.guessed_lines.last().unwrap(),
    );
    gamestate.flag_pegs.push(flags);
}

pub enum LoopAction {
    Continue,
    Break,
}

enum TargetProvider {
    Human,
    Bot,
}

pub struct RoundResult {
    pub guesses_used: u8,
    pub is_win: bool,
    pub bonus: u8,
    pub score_delta: u8,
}

fn calculate_round_result(gamestate: &Gamestate) -> RoundResult {
    let guesses_used = gamestate.guessed_lines.len() as u8;
    let is_win = gamestate.check_for_win();
    let bonus = if !is_win { 1 } else { 0 };

    RoundResult {
        guesses_used,
        is_win,
        bonus,
        score_delta: guesses_used + bonus,
    }
}

pub fn handle_end_of_round(gamestate: &mut Gamestate, bot: &mut Option<Bot>) -> LoopAction {
    autosave(gamestate).ok();

    let round_result = calculate_round_result(gamestate);

    gamestate.reset_round_statuses();

    print_win_or_loss(&gamestate, &round_result);

    match gamestate.game_mode {
        GameMode::TwoPlayer | GameMode::PlayerVsBot => {
            two_player_end_of_round_score_and_prints(gamestate, &round_result);
        }
        _ => {}
    }

    if !continue_playing() {
        return LoopAction::Break;
    }

    gamestate.prepare_next_round();

    match who_picks_target(gamestate) {
        TargetProvider::Human => {
            gamestate.target_line = get_human_target_line(gamestate);
        }
        TargetProvider::Bot => {
            gamestate.target_line = gamestate.randomize_target_line();
        }
    }

    reset_bot_for_new_round(bot, gamestate);

    LoopAction::Continue
}

fn who_picks_target(gamestate: &Gamestate) -> TargetProvider {
    match gamestate.game_mode {
        GameMode::TwoPlayer => TargetProvider::Human,
        GameMode::PlayerVsBot => {
            if gamestate.p1s_turn {
                TargetProvider::Bot
            } else {
                TargetProvider::Human
            }
        }
        GameMode::Practice => TargetProvider::Bot,
        GameMode::SpectateBot => TargetProvider::Bot,
    }
}
