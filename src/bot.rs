use crate::gamelogic::check_for_matches;
use crate::types::{Color, Feedback, Line};

use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize)]
pub struct Bot {
    pub possible_solutions: HashSet<Line>,
    pub available_colors: Vec<Color>,
    pub no_of_pegs: usize,
    pub guessed_lines: Vec<Line>,
    pub current_feedback: Feedback,
    pub current_guess: Line,
    pub is_first_guess: bool,
}

impl Bot {
    pub fn new(is_empty_allowed: bool, no_of_pegs: usize) -> Self {
        Bot {
            possible_solutions: Self::populate_set_with_solutions(is_empty_allowed, no_of_pegs),
            available_colors: Color::all_colors(),
            no_of_pegs: no_of_pegs,
            guessed_lines: Vec::new(),
            current_feedback: Feedback::empty(),
            current_guess: Line::empty(no_of_pegs),
            is_first_guess: true,
        }
    }

    fn populate_set_with_solutions(is_empty_allowed: bool, width: usize) -> HashSet<Line> {
        let mut set_of_all_combinations = HashSet::new();

        let color_limit = if is_empty_allowed { 7 } else { 6 };

        let valid_colors: Vec<Color> = (0..color_limit).map(|n| num_to_color(n)).collect();

        // Buffer to hold the current line being built.
        let mut current_buffer = Vec::with_capacity(width);

        // Start recursion.
        Self::generate_recursive(
            width,
            &valid_colors,
            &mut current_buffer,
            &mut set_of_all_combinations,
        );

        set_of_all_combinations
    }

    fn generate_recursive(
        target_width: usize,
        valid_colors: &Vec<Color>,
        current_buffer: &mut Vec<Color>,
        results: &mut HashSet<Line>,
    ) {
        // BASE CASE: If buffer is full we have a complete line.
        if current_buffer.len() == target_width {
            results.insert(Line::new(current_buffer.clone()));
            return;
        }
        // RECURSIVE STEP: Try every color for the current slot.
        for color in valid_colors {
            current_buffer.push(*color); // Choose

            Self::generate_recursive(target_width, valid_colors, current_buffer, results);

            current_buffer.pop();
        }
    }

    pub fn make_first_guess(&mut self) -> Line {
        let mut rng = rng();
        let n = self.available_colors.len();

        let index1 = rng.random_range(0..n);
        let mut index2 = rng.random_range(0..n);

        while index2 == index1 {
            index2 = rng.random_range(0..n);
        }

        let color1 = self.available_colors[index1];
        let color2 = self.available_colors[index2];

        let middle_index = self.no_of_pegs / 2;
        let mut colors = Vec::new();
        for i in 0..self.no_of_pegs {
            if i < middle_index {
                colors.push(color1);
            } else {
                colors.push(color2);
            }
        }
        Line::new(colors)
    }

    pub fn prune_non_viable_solutions(&mut self) {
        if !self.is_first_guess {
            // Only retain lines that give the same feedback as self.current_feedback
            // retain calls the closure for all elements
            // If closure returns false, remove, else keep solution.
            self.possible_solutions.retain(|line| {
                // |line| = for each line in possible_solutions...
                // Compare feedback to self.current_feedback.
                let (_, simulated_feedback) = check_for_matches(line, &self.current_guess);
                // If match, retain else remove.
                simulated_feedback == self.current_feedback
            });
        }
    }

    pub fn reset_possible_solutions(&mut self, is_empty_allowed: bool, no_of_pegs: usize) {
        self.possible_solutions = Self::populate_set_with_solutions(is_empty_allowed, no_of_pegs);
        self.guessed_lines.clear();
        self.current_feedback = Feedback::empty();
        self.is_first_guess = true;
    }

    pub fn make_educated_guess(&mut self) -> Line {
        // Just make a starting guess if is_first_guess
        if self.is_first_guess {
            self.is_first_guess = false;
            let guess = self.make_first_guess();
            self.guessed_lines.push(guess.clone());
            return guess;
        }

        let mut best_guess = None;
        let mut best_score = usize::MAX;

        for guess in &self.possible_solutions {
            if self.guessed_lines.contains(guess) {
                continue; // Skip already guessed lines
            }
            // Map to count how often each feedback occurs when comparing this guess with all
            // potential solutions.
            let mut feedback_counts = std::collections::HashMap::new();

            // Increment the usize variable in solutions that would produce this feedback
            // - entry(feedback) looks up the key 'feedback' in the Map
            // - or_insert(0) inserts 0 if the key doesn't exist yet
            // -*... += 1 derefences the mutable reference returned by or_insert
            for solution in &self.possible_solutions {
                let (_, feedback) = check_for_matches(solution, guess);
                *feedback_counts.entry(feedback).or_insert(0) += 1;
            }

            // Find the feedback value that would leave the most remaining solutions (worst-case)
            // - values() returns references to the usize counts
            // - cloned() converts them to owned usize values
            // - max() finds the largest count
            // unwrap_or(0) handles the case where the map is empty
            let worst_case = feedback_counts.values().cloned().max().unwrap_or(0);

            // Minimize the worst case after each iteration
            if worst_case < best_score {
                best_score = worst_case;
                best_guess = Some(guess.clone());
            }
        }
        let final_guess = best_guess
            .or_else(|| self.possible_solutions.iter().next().cloned())
            .expect("possible_solutions set is empty");

        self.guessed_lines.push(final_guess.clone());
        final_guess
    }
}

fn num_to_color(n: u8) -> Color {
    match n {
        0 => Color::White,
        1 => Color::Black,
        2 => Color::Red,
        3 => Color::Green,
        4 => Color::Blue,
        5 => Color::Yellow,
        _ => Color::Empty,
    }
}
