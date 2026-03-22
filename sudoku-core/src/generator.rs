use crate::board::{Cell, Grid};
use crate::difficulty::Difficulty;
use crate::solver::{count_solutions, solve};

pub fn generate(difficulty: Difficulty) -> (Grid, Grid) {
    let mut grid: Grid = [[Cell::Empty; 9]; 9];
    solve(&mut grid);
    let solution = grid;

    let (min_givens, max_givens) = difficulty.givens_range();
    let target_givens = (min_givens + max_givens) / 2;
    let empty_cells = 81 - target_givens;
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
