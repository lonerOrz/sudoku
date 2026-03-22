use ratatui::style::Color;
use sudoku_core::Difficulty;

pub fn cycle(d: Difficulty, forward: bool) -> Difficulty {
    if forward { d.next() } else { d.prev() }
}

pub fn color(d: Difficulty) -> Color {
    match d {
        Difficulty::Easy => Color::Green,
        Difficulty::Medium => Color::Yellow,
        Difficulty::Hard => Color::Red,
        Difficulty::Expert => Color::Magenta,
    }
}
