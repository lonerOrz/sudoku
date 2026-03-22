// state.rs: 应用状态

use sudoku_core::{Difficulty, Grid};

#[allow(clippy::large_enum_variant, dead_code)]
pub enum AppState {
    Menu {
        difficulty: Difficulty,
    },
    Playing {
        puzzle: Grid,
        solution: Grid,
        cursor_row: usize,
        cursor_col: usize,
        errors: Vec<(usize, usize)>,
        difficulty: Difficulty,
        mistakes: u8,
    },
    Won {
        difficulty: Difficulty,
    },
    Failed {
        difficulty: Difficulty,
    },
}
