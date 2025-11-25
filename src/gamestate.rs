/*
    Game logic module.

    Provides core functions and types for running a Mastermind game round,
    checking guesses, and handling end-of-round logic.

    Public API:
    - check_for_matches: compares a guess line to the target and returns
      the corresponding flags and feedback.
    - human_guess: prompts the player for input and updates the gamestate.
    - print_complexity_analysis: prints estimated computational complexity for the bot.
    - handle_end_of_round: processes autosave, updates gamestate, prints results,
      and prepares the next round.
    - reset_bot_for_new_round: helper to reset bot state if a bot is in use.
    - bot_guess / handle_bot_input: handles the bot making a guess.

    Public types:
    - LoopAction: indicates whether the main game loop should continue or break.
    - RoundResult: stores information about the outcome of a round.

    Internal helpers / private items:
    - calculate_round_result: computes results at the end of a round.
    - TargetProvider: identifies who will pick the target next (Human or Bot).
    - format_number: formats large numbers with commas.
    - who_picks_target: decides which player or bot sets the next target line.

    Notes:
    - All functions work directly with Gamestate and may mutate its contents.
    - Feedback calculation distinguishes exact-position matches (black) and
      color-only matches (white).
*/

use crate::types::{Color, GameMode, Line};

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum RoundStatus {
    Ongoing,
    Win,
    Loss,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Gamestate {
    // Game Specific
    pub game_mode: GameMode,
    pub round_length: u8,
    pub is_empty_allowed: bool,
    pub pegs_in_a_line: usize,

    // Round specific
    pub target_line: Line,
    pub guessed_lines: Vec<Line>,
    pub flag_pegs: Vec<Line>,
    pub p1_score: u8,
    pub p2_score: u8,
    pub current_round: u8,
    pub p1s_turn: bool, // In 2P: True = P1 is Code Breaker, else P2 is Code Breaker.
    pub round_status: RoundStatus,
    pub round_over: bool,
    pub is_bot_guessing_this_round: bool,
}

impl Gamestate {
    pub fn new(
        game_mode: GameMode,
        number_of_guesses: u8,
        pegs_in_a_line: usize,
        target_line: Line,
        is_empty_allowed: bool,
    ) -> Self {
        Gamestate {
            game_mode: game_mode,
            round_length: number_of_guesses,
            is_empty_allowed: is_empty_allowed,
            pegs_in_a_line: pegs_in_a_line,
            target_line: target_line,
            guessed_lines: Vec::new(),
            flag_pegs: Vec::new(),
            p1_score: 0,
            p2_score: 0,
            current_round: 1,
            p1s_turn: true,
            round_status: RoundStatus::Ongoing,
            round_over: false,
            is_bot_guessing_this_round: false,
        }
    }

    pub fn check_for_win(&self) -> bool {
        self.guessed_lines.contains(&self.target_line)
    }

    pub fn check_for_loss(&self) -> bool {
        self.guessed_lines.len() >= self.round_length as usize
    }

    pub fn reset_round_statuses(&mut self) {
        self.round_status = RoundStatus::Ongoing;
        self.round_over = false;
    }

    pub fn prepare_next_round(&mut self) {
        self.guessed_lines.clear();
        self.flag_pegs.clear();
        self.current_round += 1;

        // Swap turns
        self.p1s_turn = !self.p1s_turn;
    }

    pub fn is_it_bots_turn_to_guess(&self) -> bool {
        match self.game_mode {
            GameMode::TwoPlayer => false,
            GameMode::PlayerVsBot => !self.p1s_turn,
            GameMode::Practice => false,
            GameMode::SpectateBot => true,
        }
    }

    pub fn update_round_status(&mut self) {
        self.round_status = self.get_round_status();
    }

    pub fn get_round_status(&self) -> RoundStatus {
        if self.check_for_win() {
            RoundStatus::Win
        } else if self.check_for_loss() {
            RoundStatus::Loss
        } else {
            RoundStatus::Ongoing
        }
    }

    pub fn randomize_target_line(&self) -> Line {
        let mut rng = rand::rng();
        let mut line = Line::empty(self.pegs_in_a_line);

        for i in 0..self.pegs_in_a_line {
            let color = if self.is_empty_allowed {
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
}
