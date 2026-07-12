// hints.rs: 数独提示技巧

use crate::board::Grid;

pub fn find_solver_hint(grid: &Grid) -> Option<sudoku_solver::Hint> {
    let solver_grid = sudoku_solver::Grid::from_flat(grid.flat());
    let mut solver = sudoku_solver::Solver::new(solver_grid);
    solver.next_hint()
}
