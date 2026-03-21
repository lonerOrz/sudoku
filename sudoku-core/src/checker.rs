// checker.rs: 数独验证规则

use crate::board::{peers, Grid};

/// 检查在 (row, col) 位置放置 val 是否有效
pub fn is_valid(grid: &Grid, row: usize, col: usize, val: u8) -> bool {
    for (r, c) in peers(row, col) {
        if let Some(v) = grid[r][c].value() {
            if v == val {
                return false;
            }
        }
    }
    true
}

/// 检查数独是否已完成且正确
pub fn is_solved(grid: &Grid) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                return false;
            }
        }
    }
    for r in 0..9 {
        for c in 0..9 {
            if let Some(val) = grid[r][c].value() {
                if !is_valid(grid, r, c, val) {
                    return false;
                }
            }
        }
    }
    true
}

/// 计算 (row, col) 位置可能的数字
pub fn possible_values(grid: &Grid, row: usize, col: usize) -> Vec<u8> {
    if grid[row][col].value().is_some() {
        return vec![];
    }

    let mut candidates = Vec::new();
    for val in 1..=9 {
        if is_valid(grid, row, col, val) {
            candidates.push(val);
        }
    }
    candidates
}

/// 找出所有有冲突的格子
pub fn find_errors(grid: &Grid) -> Vec<(usize, usize)> {
    let mut errors = Vec::new();

    for r in 0..9 {
        for c in 0..9 {
            if let Some(val) = grid[r][c].value() {
                let mut is_error = false;
                for (pr, pc) in peers(r, c) {
                    if let Some(other) = grid[pr][pc].value() {
                        if other == val {
                            is_error = true;
                            break;
                        }
                    }
                }
                if is_error {
                    errors.push((r, c));
                }
            }
        }
    }

    errors
}
