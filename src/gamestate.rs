use crate::types::Line;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum GameMode {
    SinglePlayer,
    TwoPlayer,
    PlayerVsBot,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Gamestate {
    pub game_mode: GameMode,
    pub round_length: u8,
    pub is_empty_allowed: bool,
    pub pegs_in_a_line: usize,
    pub guess_made: bool,
    pub target_line: Line,
    pub guessed_lines: Vec<Line>,
    pub flag_pegs: Vec<Line>,
    pub p1_score: u8,
    pub p2_score: u8,
    pub current_round: u8,
    pub p1s_turn: bool, // In 2P: True = P1 is Code Breaker, else P2 is Code Breaker.
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
            guess_made: false,
            target_line: target_line,
            guessed_lines: Vec::new(),
            flag_pegs: Vec::new(),
            p1_score: 0,
            p2_score: 0,
            current_round: 1,
            p1s_turn: true,
        }
    }

    pub fn prepare_next_round(&mut self, new_target: Line) {
        // What is really meant by round?
        self.guessed_lines.clear(); // Maybe incorrect?
        self.flag_pegs.clear();
        self.target_line = new_target;
        self.guess_made = false;
        self.current_round += 1;

        // Swap turns
        self.p1s_turn = !self.p1s_turn;
    }
}
