// solver.rs: 数独解题算法

use crate::board::{Cell, Grid};
use crate::checker::is_valid;

#[allow(clippy::needless_range_loop)]
fn find_empty(grid: &Grid) -> Option<(usize, usize)> {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                return Some((r, c));
            }
        }
    }
    None
}

fn count_solutions_inner(grid: &mut Grid, count: &mut usize, max_count: usize) {
    if *count >= max_count {
        return;
    }

    if let Some((r, c)) = find_empty(grid) {
        for val in 1..=9 {
            if is_valid(grid, r, c, val) {
                grid[r][c] = Cell::Given(val);
                count_solutions_inner(grid, count, max_count);
                grid[r][c] = Cell::Empty;
            }
        }
    } else {
        *count += 1;
    }
}

/// 解数独（回溯算法）
pub fn solve(grid: &mut Grid) -> bool {
    if let Some((r, c)) = find_empty(grid) {
        for val in 1..=9 {
            if is_valid(grid, r, c, val) {
                grid[r][c] = Cell::UserInput(val);

                if solve(grid) {
                    return true;
                }

                grid[r][c] = Cell::Empty;
            }
        }
        return false;
    }
    true
}

/// 计算解的数量
pub fn count_solutions(grid: &mut Grid) -> usize {
    let mut count = 0;
    count_solutions_inner(grid, &mut count, 2);
    count
}
