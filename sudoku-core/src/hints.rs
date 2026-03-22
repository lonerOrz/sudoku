// hints.rs: 数独提示技巧

use crate::board::{Grid, is_valid};
use crate::checker::possible_values;

fn row_has(grid: &Grid, row: usize, val: u8) -> bool {
    (0..9).any(|c| grid[row][c].value() == Some(val))
}

fn col_has(grid: &Grid, col: usize, val: u8) -> bool {
    (0..9).any(|r| grid[r][col].value() == Some(val))
}

fn box_has(grid: &Grid, box_r: usize, box_c: usize, val: u8) -> bool {
    (box_r..box_r + 3).any(|r| (box_c..box_c + 3).any(|c| grid[r][c].value() == Some(val)))
}

pub fn find_naked_single(grid: &Grid) -> Option<((usize, usize), u8)> {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                let candidates = possible_values(grid, r, c);
                if candidates.len() == 1 {
                    return Some(((r, c), candidates[0]));
                }
            }
        }
    }
    None
}

pub fn find_hidden_single(grid: &Grid) -> Option<((usize, usize), u8)> {
    for r in 0..9 {
        for val in 1..=9 {
            if !row_has(grid, r, val) {
                let possible: Vec<usize> = (0..9)
                    .filter(|&c| grid[r][c].value().is_none() && is_valid(grid, r * 9 + c, val))
                    .collect();
                if possible.len() == 1 {
                    return Some(((r, possible[0]), val));
                }
            }
        }
    }

    for c in 0..9 {
        for val in 1..=9 {
            if !col_has(grid, c, val) {
                let possible: Vec<usize> = (0..9)
                    .filter(|&r| grid[r][c].value().is_none() && is_valid(grid, r * 9 + c, val))
                    .collect();
                if possible.len() == 1 {
                    return Some(((possible[0], c), val));
                }
            }
        }
    }

    for box_r in (0..9).step_by(3) {
        for box_c in (0..9).step_by(3) {
            for val in 1..=9 {
                if !box_has(grid, box_r, box_c, val) {
                    let possible: Vec<(usize, usize)> = (box_r..box_r + 3)
                        .flat_map(|r| (box_c..box_c + 3).map(move |c| (r, c)))
                        .filter(|(r, c)| {
                            grid[*r][*c].value().is_none() && is_valid(grid, *r * 9 + *c, val)
                        })
                        .collect();
                    if possible.len() == 1 {
                        return Some((possible[0], val));
                    }
                }
            }
        }
    }

    None
}
