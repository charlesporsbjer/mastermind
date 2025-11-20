use crate::gamestate::Gamestate;
use crate::types::{Color, Line};

use colored::{ColoredString, Colorize};

pub fn draw_board(gamestate: &Gamestate) {
    let symbol = "●";
    let n = gamestate.pegs_in_a_line;

    // 1. Calculate Board Widths
    // Rule: Minimum 10 width for text fitting, otherwise grow by 2 chars per peg
    let col_width = std::cmp::max(n * 2 + 2, 10);
    let total_width = (col_width * 2) + 1;

    // 2. Define the Legend
    // The Colored version is for printing
    let legend_colored = format!(
        " Colors {} {} {} {} {} {} ",
        symbol.white(),
        symbol.black(),
        symbol.red(),
        symbol.blue(),
        symbol.green(),
        symbol.yellow()
    );
    // The Plain version is ONLY for measuring length (must match spaces of colored version)
    let legend_plain = " Colors ● ● ● ● ● ● ";
    let visual_len = legend_plain.chars().count();

    // 3. The "Wildcard" Calculation
    // We subtract the legend length from the board width.
    // This naturally adds 2 spaces per peg over 4, and subtracts for under 4.
    let total_empty_space = total_width.saturating_sub(visual_len);

    let pad_left_len = total_empty_space / 2;
    let pad_right_len = total_empty_space - pad_left_len; // Handles odd numbers

    // Create the string of spaces
    let wildcard_padding_left = " ".repeat(pad_left_len);
    let wildcard_padding_right = " ".repeat(pad_right_len);

    // --- DRAWING ---

    let roof = format!("╔{}╗", "═".repeat(total_width));
    let separator = format!("╠{}╬{}╣", "═".repeat(col_width), "═".repeat(col_width));
    let top_separator = format!("╠{}╦{}╣", "═".repeat(col_width), "═".repeat(col_width));
    let floor = format!("╚{}╩{}╝", "═".repeat(col_width), "═".repeat(col_width));

    println!("{}", roof.on_bright_black());

    // 4. Inject the Wildcard Padding
    println!(
        "{}",
        format!(
            "║{}{}{}║",
            wildcard_padding_left, legend_colored, wildcard_padding_right
        )
        .on_bright_black()
    );

    println!("{}", top_separator.on_bright_black());

    println!(
        "{}",
        format!("║{:^w$}║{:^w$}║", "Guesses", "Hits", w = col_width).on_bright_black()
    );
    println!("{}", separator.on_bright_black());

    // FIX STARTS HERE
    let rounds = gamestate.guessed_lines.len();
    let pegs_count = gamestate.pegs_in_a_line;

    // Calculate the visual length of a row: (N symbols) + (N-1 spaces between them)
    // e.g. 4 pegs = "● ● ● ●" = 7 chars
    let content_visual_len = (pegs_count * 2).saturating_sub(1);

    // Calculate how much padding is needed for one column
    let row_padding = col_width.saturating_sub(content_visual_len);
    let pad_l_len = row_padding / 2;
    let pad_r_len = row_padding - pad_l_len;

    let pad_l = " ".repeat(pad_l_len);
    let pad_r = " ".repeat(pad_r_len);

    for i in 0..rounds {
        let guess_row = format_line(&gamestate.guessed_lines[i]);
        let flag_row = format_line(&gamestate.flag_pegs[i]);

        // We manually sandwich the content between the calculated padding.
        // We also removed the extra spaces you had in the string "║ {:^w$}...".
        println!(
            "{}",
            format!(
                "║{}{}{}║{}{}{}║",
                pad_l,
                guess_row,
                pad_r, // Column 1
                pad_l,
                flag_row,
                pad_r // Column 2
            )
            .on_bright_black()
        );
    }
    println!("{}", floor.on_bright_black());
}

// Helper: This remains mostly the same, just ensures consistent spacing
fn format_line(line: &Line) -> String {
    line.pegs
        .iter()
        .map(|peg| colored_symbol(peg.color).to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

// Helper: Remains the same
fn colored_symbol(color: Color) -> ColoredString {
    let symbol = "●";
    match color {
        Color::Empty => " ".normal(), // Might want a small dot "." for empty placeholder
        Color::White => symbol.white(),
        Color::Black => symbol.black(),
        Color::Red => symbol.red(),
        Color::Green => symbol.green(),
        Color::Blue => symbol.blue(),
        Color::Yellow => symbol.yellow(),
    }
}
