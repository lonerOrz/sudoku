// generator.rs: 数独谜题生成

use crate::board::{Cell, Grid};
use crate::solver::{count_solutions, solve};

/// 难度级别
#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl Difficulty {
    pub fn givens_count(&self) -> usize {
        match self {
            Difficulty::Easy => 40,
            Difficulty::Medium => 34,
            Difficulty::Hard => 27,
            Difficulty::Expert => 22,
        }
    }
}

/// 生成谜题（保证唯一解）
pub fn generate(difficulty: Difficulty) -> (Grid, Grid) {
    let mut grid: Grid = [[Cell::Empty; 9]; 9];

    solve(&mut grid);
    let solution = grid;

    let empty_cells = 81 - difficulty.givens_count();
    let mut puzzle = solution;

    let mut removed = 0;
    for r in 0..9 {
        for c in 0..9 {
            if removed >= empty_cells {
                break;
            }

            let backup = puzzle[r][c];
            puzzle[r][c] = Cell::Empty;

            if count_solutions(&mut puzzle) != 1 {
                puzzle[r][c] = backup;
            } else {
                removed += 1;
            }
        }
    }

    (puzzle, solution)
}
