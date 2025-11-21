use crate::{
    gamelogic::{check_for_loss, check_for_win},
    types::Line,
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum GameMode {
    SinglePlayer,
    TwoPlayer,
    PlayerVsBot,
    SpectateBot,
}
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
    pub is_bot_only_guesser: bool,

    // Round specific
    pub guess_made: bool,
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
        is_bot_only_guesser: bool,
    ) -> Self {
        Gamestate {
            game_mode: game_mode,
            round_length: number_of_guesses,
            is_empty_allowed: is_empty_allowed,
            pegs_in_a_line: pegs_in_a_line,
            guess_made: false,
            target_line: target_line,
            guessed_lines: Vec::new(),
            flag_pegs: Vec::new(),
            p1_score: 0,
            p2_score: 0,
            current_round: 1,
            p1s_turn: true,
            round_status: RoundStatus::Ongoing,
            round_over: false,
            is_bot_only_guesser: is_bot_only_guesser,
            is_bot_guessing_this_round: false,
        }
    }

    pub fn prepare_next_round(&mut self, new_target: Line) {
        self.guessed_lines.clear();
        self.flag_pegs.clear();
        self.target_line = new_target;
        self.guess_made = false;
        self.current_round += 1;

        // Swap turns
        self.p1s_turn = !self.p1s_turn;
    }

    pub fn is_bot_guessing_this_round(&self) -> bool {
        match self.game_mode {
            GameMode::TwoPlayer => false,
            GameMode::PlayerVsBot => !self.p1s_turn,
            GameMode::SinglePlayer => self.is_bot_only_guesser,
            GameMode::SpectateBot => true,
        }
    }

    pub fn update_round_status(&mut self) {
        self.round_status = self.get_round_status();
    }

    pub fn get_round_status(&self) -> RoundStatus {
        if check_for_win(self) {
            RoundStatus::Win
        } else if check_for_loss(self) {
            RoundStatus::Loss
        } else {
            RoundStatus::Ongoing
        }
    }
}
