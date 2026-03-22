// state.rs: 应用状态

use sudoku_core::{Difficulty, Grid};

#[derive(Clone)]
pub struct HistoryEntry {
    pub puzzle: Grid,
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
