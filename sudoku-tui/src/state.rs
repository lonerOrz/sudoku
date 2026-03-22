// state.rs: 应用状态

use sudoku_core::{Difficulty, Grid, Solution};

pub type PencilMarks = [[Vec<u8>; 9]; 9];

#[derive(Clone)]
pub struct HistoryEntry {
    pub puzzle: Grid,
    pub pencil_marks: PencilMarks,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub mistakes: u8,
}

#[allow(clippy::large_enum_variant)]
pub enum AppState {
    Menu {
        difficulty: Difficulty,
    },
    Playing {
        puzzle: Grid,
        solution: Solution,
        pencil_marks: PencilMarks,
        pencil_mode: bool,
        cursor_row: usize,
        cursor_col: usize,
        errors: [bool; 81],
        difficulty: Difficulty,
        mistakes: u8,
        start_time: std::time::Instant,
        elapsed_secs: u64,
        paused: bool,
        history: Vec<HistoryEntry>,
    },
    Won {
        difficulty: Difficulty,
        elapsed_secs: u64,
    },
    Failed {
        difficulty: Difficulty,
        elapsed_secs: u64,
    },
}
