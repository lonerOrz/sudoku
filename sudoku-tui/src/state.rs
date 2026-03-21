// state.rs: 应用状态

use sudoku_core::Difficulty;

pub enum AppState {
    Menu { difficulty: Difficulty },
}
