// solver.rs: 数独解题算法

use crate::board::{Cell, Grid, is_valid};

fn find_empty(grid: &Grid) -> Option<usize> {
    (0..81).find(|&idx| grid[idx / 9][idx % 9].value().is_none())
}

fn candidates(grid: &Grid, idx: usize) -> Vec<u8> {
    let (r, c) = (idx / 9, idx % 9);
    if grid[r][c].value().is_some() {
        return vec![];
    }
    (1..=9).filter(|&val| is_valid(grid, idx, val)).collect()
}

fn count_solutions_inner(grid: &mut Grid, count: &mut usize, max_count: usize) {
    if *count >= max_count {
        return;
    }

    if let Some(idx) = find_empty(grid) {
        for val in candidates(grid, idx) {
            grid[idx / 9][idx % 9] = Cell::Given(val);
            count_solutions_inner(grid, count, max_count);
            grid[idx / 9][idx % 9] = Cell::Empty;
        }
    } else {
        *count += 1;
    }
}

pub fn solve(grid: &mut Grid) -> bool {
    if let Some(idx) = find_empty(grid) {
        for val in candidates(grid, idx) {
            grid[idx / 9][idx % 9] = Cell::Given(val);

            if solve(grid) {
                return true;
            }

            grid[idx / 9][idx % 9] = Cell::Empty;
        }
        return false;
    }
    true
}

pub fn count_solutions(grid: &mut Grid) -> usize {
    let mut count = 0;
    count_solutions_inner(grid, &mut count, 2);
    count
}
