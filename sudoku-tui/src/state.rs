// state.rs: 应用状态

use sudoku_core::{Difficulty, Grid};

#[allow(clippy::large_enum_variant, dead_code)]
pub enum AppState {
    Menu { difficulty: Difficulty },
    Playing { puzzle: Grid, solution: Grid },
}
