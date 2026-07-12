// hints.rs: 数独提示技巧

#![allow(clippy::needless_range_loop)]

use crate::board::{Grid, Solution, build_candidates};

#[derive(Clone, Debug)]
pub struct Clue {
    pub target_row: usize,
    pub target_col: usize,
    pub value: u8,
    pub technique: &'static str,
}

pub fn find_clue(grid: &Grid, solution: &Solution) -> Option<Clue> {
    if let Some(clue) = find_naked_single(grid) {
        return Some(clue);
    }
    if let Some(clue) = find_hidden_single(grid) {
        return Some(clue);
    }
    find_direct_reveal(grid, solution)
}

pub fn find_naked_single(grid: &Grid) -> Option<Clue> {
    let cands = build_candidates(grid);
    for r in 0..9 {
        for c in 0..9 {
            let idx = r * 9 + c;
            if grid[r][c].value().is_none() && cands[idx].is_single() {
                return Some(Clue {
                    target_row: r,
                    target_col: c,
                    value: cands[idx].first().unwrap(),
                    technique: "Naked Single",
                });
            }
        }
    }
    None
}

pub fn find_hidden_single(grid: &Grid) -> Option<Clue> {
    let cands = build_candidates(grid);

    // Check rows
    for r in 0..9 {
        for val in 1..=9 {
            let mut count = 0u8;
            let mut pos = 0usize;
            for c in 0..9 {
                let idx = r * 9 + c;
                if grid[r][c].value().is_none() && cands[idx].has(val) {
                    count += 1;
                    pos = c;
                    if count > 1 {
                        break;
                    }
                }
            }
            if count == 1 {
                return Some(Clue {
                    target_row: r,
                    target_col: pos,
                    value: val,
                    technique: "Hidden Single",
                });
            }
        }
    }

    // Check columns
    for c in 0..9 {
        for val in 1..=9 {
            let mut count = 0u8;
            let mut pos = 0usize;
            for r in 0..9 {
                let idx = r * 9 + c;
                if grid[r][c].value().is_none() && cands[idx].has(val) {
                    count += 1;
                    pos = r;
                    if count > 1 {
                        break;
                    }
                }
            }
            if count == 1 {
                return Some(Clue {
                    target_row: pos,
                    target_col: c,
                    value: val,
                    technique: "Hidden Single",
                });
            }
        }
    }

    // Check boxes
    for box_r in (0..9).step_by(3) {
        for box_c in (0..9).step_by(3) {
            for val in 1..=9 {
                let mut count = 0u8;
                let mut pos = (0usize, 0usize);
                for dr in 0..3 {
                    for dc in 0..3 {
                        let r = box_r + dr;
                        let c = box_c + dc;
                        let idx = r * 9 + c;
                        if grid[r][c].value().is_none() && cands[idx].has(val) {
                            count += 1;
                            pos = (r, c);
                            if count > 1 {
                                break;
                            }
                        }
                    }
                    if count > 1 {
                        break;
                    }
                }
                if count == 1 {
                    return Some(Clue {
                        target_row: pos.0,
                        target_col: pos.1,
                        value: val,
                        technique: "Hidden Single",
                    });
                }
            }
        }
    }

    None
}

pub fn find_direct_reveal(grid: &Grid, solution: &Solution) -> Option<Clue> {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                return Some(Clue {
                    target_row: r,
                    target_col: c,
                    value: solution[r][c],
                    technique: "Direct",
                });
            }
        }
    }
    None
}

pub fn find_solver_hint(grid: &Grid) -> Option<sudoku_solver::Hint> {
    let solver_grid = sudoku_solver::Grid::from_flat(grid.flat());
    let mut solver = sudoku_solver::Solver::new(solver_grid);
    solver.next_hint()
}
