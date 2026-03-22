// generator.rs: 数独谜题生成

use crate::board::{Cell, Grid};
use crate::difficulty::Difficulty;
use crate::solver::{count_solutions, solve};
use rand::{thread_rng, Rng};

pub fn generate(difficulty: Difficulty) -> (Grid, Grid) {
    let mut grid: Grid = [[Cell::Empty; 9]; 9];
    solve(&mut grid);
    let solution = grid;

    let (min_givens, max_givens) = difficulty.givens_range();
    let target_givens = (min_givens + max_givens) / 2;
    let empty_cells = 81 - target_givens;
    let mut puzzle = solution;

    let mut candidates: Vec<usize> = (0..81).collect();

    let mut removed = 0;
    while removed < empty_cells && !candidates.is_empty() {
        let pos = thread_rng().gen_range(0..candidates.len());
        let idx = candidates[pos];

        let r = idx / 9;
        let c = idx % 9;
        let backup = puzzle[r][c];
        puzzle[r][c] = Cell::Empty;

        if count_solutions(&mut puzzle) != 1 {
            puzzle[r][c] = backup;
            candidates.swap_remove(pos);
        } else {
            removed += 1;
        }
    }

    (puzzle, solution)
}
