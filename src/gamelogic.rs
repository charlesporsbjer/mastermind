use crate::gamestate::Gamestate;
use crate::types::{Color, Feedback, Line};
use crate::{Bot, GameMode};

use rand::Rng;

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

pub fn print_complexity_analysis(pegs: usize, allow_empty: bool) {
    let colors: u128 = if allow_empty { 7 } else { 6 };
    let width = pegs as u32;

    // Calculate Search Space (N)
    let total_combinations = colors.pow(width);

    // Calculate Minimax Complexity (N^2)
    // Since we compare every possible solution against every other solution
    let minimax_checks = total_combinations
        .checked_mul(total_combinations)
        .unwrap_or(u128::MAX);

    // Estimate CPU to 20 million "match checks" per second
    let ops_per_sec = 20_000_000_u128;
    let seconds_est = minimax_checks / ops_per_sec;

    println!("\n---   !! COMPUTATIONAL COMPLEXITY WARNING !!    ---");
    println!("Formula: Colors^Pegs = Search Space (N)");
    println!("Minimax Complexity: N * N (The bot compares everything against everything)");
    println!("----------------------------------------------------");
    println!("{:<20} : {}", "Pegs", pegs);
    println!("{:<20} : {}", "Colors", colors);
    println!(
        "{:<20} : {}",
        "Total Candidates",
        format_number(total_combinations)
    );
    println!(
        "{:<20} : {}",
        "Minimax Checks",
        format_number(minimax_checks)
    );
    println!("----------------------------------------------------");

    if seconds_est > 60 {
        let minutes = seconds_est / 60;
        println!("ESTIMATED TIME PER TURN: ~{} minutes", minutes);
        println!("(!) This configuration is likely too slow for the HashSet/Minimax approach.");
    } else if seconds_est > 5 {
        println!("ESTIMATED TIME PER TURN: ~{} seconds", seconds_est);
        println!("(!) You will notice a delay.");
    } else {
        println!("ESTIMATED TIME PER TURN: < 1 second (Instant)");
    }
    println!("----------------------------------------------------\n");
}

// Makes big numbers readable
fn format_number(n: u128) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }
    result.chars().rev().collect()
}

pub fn randomize_target_line(pegs_in_a_line: usize, is_empty_pegs_allowed: bool) -> Line {
    let mut rng = rand::rng();
    let mut line = Line::empty(pegs_in_a_line);

    for i in 0..pegs_in_a_line {
        let color = if is_empty_pegs_allowed {
            match rng.random_range(0..7) {
                0 => Color::White,
                1 => Color::Black,
                2 => Color::Red,
                3 => Color::Green,
                4 => Color::Blue,
                5 => Color::Yellow,
                _ => Color::Empty,
            }
        } else {
            match rng.random_range(0..6) {
                0 => Color::White,
                1 => Color::Black,
                2 => Color::Red,
                3 => Color::Green,
                4 => Color::Blue,
                _ => Color::Yellow,
            }
        };
        line.pegs[i].color = color;
    }
    line
}

pub fn check_for_win(gamestate: &Gamestate) -> bool {
    gamestate.guessed_lines.contains(&gamestate.target_line)
}

pub fn check_for_loss(gamestate: &Gamestate) -> bool {
    gamestate.guessed_lines.len() >= gamestate.round_length as usize
}

pub fn handle_bot_input(bot_ref: &mut Bot, gamestate: &mut Gamestate) {
    let new_guess = bot_ref.make_educated_guess();
    let (flags, feedback) = check_for_matches(&gamestate.target_line, &new_guess);
    bot_ref.current_guess = new_guess.clone();
    bot_ref.current_feedback = feedback;
    gamestate.guessed_lines.push(new_guess.clone());
    gamestate.flag_pegs.push(flags);
    bot_ref.prune_non_viable_solutions();
}

// Create clean Gamestate for restarting Solo/PvB
pub fn create_new_solo_session(gamestate: &Gamestate) -> Gamestate {
    // Target Line for the new session
    let target_line = match gamestate.game_mode {
        GameMode::SinglePlayer => {
            randomize_target_line(gamestate.pegs_in_a_line, gamestate.is_empty_allowed)
        }
        GameMode::PlayerVsBot => Line::empty(gamestate.pegs_in_a_line),
        _ => panic!("create_new_solo_session called in inappropriate mode"),
    };

    let new_gamestate = Gamestate::new(
        gamestate.game_mode,
        gamestate.round_length,
        gamestate.pegs_in_a_line,
        target_line,
        gamestate.is_empty_allowed,
        gamestate.is_bot_only_guesser,
    );

    new_gamestate
}
