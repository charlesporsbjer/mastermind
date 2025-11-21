/* This file contains all smaller custom datatypes and their implementations */
use serde::{Deserialize, Serialize};

// Begin Color
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Color {
    Empty,
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    pub fn all_colors() -> Vec<Color> {
        let colors = vec![
            Color::Empty,
            Color::White,
            Color::Black,
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
        ];
        colors
    }
} // End Color

// Begin Peg
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Peg {
    pub color: Color,
}

impl Peg {
    pub fn new() -> Self {
        Peg {
            color: Color::Empty,
        }
    }
} // End Peg

// Begin Line
#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Line {
    pub pegs: Vec<Peg>,
}

impl Line {
    pub fn empty(width: usize) -> Self {
        let mut pegs = Vec::new();
        for _ in 0..width {
            pegs.push(Peg::new());
        }
        Line { pegs }
    }
}

impl Line {
    pub fn new(colors: Vec<Color>) -> Self {
        let pegs = colors.into_iter().map(|c| Peg { color: c }).collect();

        Line { pegs }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for peg in &self.pegs {
            match peg.color {
                Color::Empty => write!(f, "_ ")?,
                c => write!(f, "{:?} ", c)?,
            }
        }
        Ok(())
    }
} // End Line

// Begin Feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Feedback {
    pub correct_position: u8,
    pub correct_color: u8,
}

impl Feedback {
    pub fn empty() -> Self {
        Feedback {
            correct_position: 0,
            correct_color: 0,
        }
    }
} // End Feedback
