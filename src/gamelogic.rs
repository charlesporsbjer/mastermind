use crate::gamestate::Gamestate;
use crate::io::{continue_playing, get_validated_line_input};
use crate::types::{Color, Feedback, Line};
use crate::{Bot, GameConfig, GameMode};

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
pub fn create_new_solo_session(mode: GameMode, config: &GameConfig) -> Gamestate {
    let no_of_pegs = config.pegs_in_a_line as usize;

    // Target Line for the new session
    let target_line = match mode {
        GameMode::SinglePlayer => randomize_target_line(no_of_pegs, config.is_empty_pegs_allowed),
        GameMode::PlayerVsBot => Line::empty(no_of_pegs),
        _ => panic!("create_new_solo_session called in inappropriate mode"),
    };

    let mut new_gamestate = Gamestate::new(
        mode,
        config.number_of_guesses,
        no_of_pegs,
        target_line,
        config.is_empty_pegs_allowed,
    );

    // If PvB, get new code from the human before loop starts
    if mode == GameMode::PlayerVsBot {
        new_gamestate.target_line = get_human_target_line(&new_gamestate);
    }

    new_gamestate
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

pub fn handle_two_player_end_of_round(
    gameconfig: &GameConfig,
    gamestate: &mut Gamestate,
    bot: &Option<Bot>,
    is_empty_pegs_allowed: bool,
) -> bool {
    let (maker, breaker) = get_player_strings(gamestate);

    let guesses_used = gamestate.guessed_lines.len() as u8;
    let is_win = check_for_win(gamestate);

    let bonus = if !is_win { 1 } else { 0 };
    let score_delta = guesses_used + bonus;

    // Update Maker's score
    if gamestate.p1s_turn {
        gamestate.p2_score += score_delta;
    } else {
        gamestate.p1_score += score_delta;
    }

    let win_or_loss_str = if is_win { "solved" } else { "didn't solve" };
    let bonus_str = if bonus != 0 {
        format!(" + {} bonus", bonus)
    } else {
        "".to_string()
    };
    let p2_string = if gameconfig.game_mode == GameMode::PlayerVsBot {
        "BOT"
    } else {
        "P2"
    };

    println!("\n--- ROUND {} ENDED ---", gamestate.current_round);
    println!(
        "{} {} the code in {} guesses.",
        breaker, win_or_loss_str, guesses_used
    );
    println!(
        "{} (Code Maker) gains {}{} points.",
        maker, guesses_used, bonus_str
    );
    println!(
        "--- SCOREBOARD: P1: {} | {}: {}",
        gamestate.p1_score, p2_string, gamestate.p2_score
    );

    // Ask if players want to continue
    let continue_playing = continue_playing(gameconfig, gamestate, bot);

    if continue_playing {
        // Prepare for next round (clears board, swaps p1s_turn)
        gamestate.prepare_next_round(Line::empty(gamestate.pegs_in_a_line));

        // After prepare_next_round, roles have swapped:
        // If P1 is now the Breaker (p1s_turn=true), P2 is the Maker.
        let next_breaker_is_bot =
            gamestate.game_mode == GameMode::PlayerVsBot && !gamestate.p1s_turn;

        let target_line = if next_breaker_is_bot {
            // PvB where P2 (Bot) is about to guess so player sets target.
            get_human_target_line(gamestate)
        } else {
            // PvB where P1 (Human) is about to guess so bot sets target.
            randomize_target_line(gamestate.pegs_in_a_line, is_empty_pegs_allowed)
        };

        gamestate.target_line = target_line;
    }
    continue_playing
}
