use ratatui::style::Color;
use sudoku_core::Difficulty;

pub fn cycle(d: Difficulty, forward: bool) -> Difficulty {
    match (d, forward) {
        (Difficulty::Easy, false) => Difficulty::Expert,
        (Difficulty::Easy, true) => Difficulty::Medium,
        (Difficulty::Medium, false) => Difficulty::Easy,
        (Difficulty::Medium, true) => Difficulty::Hard,
        (Difficulty::Hard, false) => Difficulty::Medium,
        (Difficulty::Hard, true) => Difficulty::Expert,
        (Difficulty::Expert, false) => Difficulty::Hard,
        (Difficulty::Expert, true) => Difficulty::Easy,
    }
}

pub fn label(d: Difficulty) -> &'static str {
    match d {
        Difficulty::Easy => "Easy",
        Difficulty::Medium => "Medium",
        Difficulty::Hard => "Hard",
        Difficulty::Expert => "Expert",
    }
}

pub fn color(d: Difficulty) -> Color {
    match d {
        Difficulty::Easy => Color::Green,
        Difficulty::Medium => Color::Yellow,
        Difficulty::Hard => Color::Red,
        Difficulty::Expert => Color::Magenta,
    }
}
