// checker.rs: 数独验证规则

#![allow(clippy::needless_range_loop)]

use crate::board::Grid;

const ALL_VALUES: u16 = 0x3FE;

pub fn is_solved(grid: &Grid) -> bool {
    let mut masks = ([0u16; 9], [0u16; 9], [0u16; 9]);
    for r in 0..9 {
        for c in 0..9 {
            if let Some(v) = grid[r][c].value() {
                let bit = 1u16 << v;
                masks.0[r] |= bit;
                masks.1[c] |= bit;
                masks.2[(r / 3) * 3 + c / 3] |= bit;
            } else {
                return false;
            }
        }
    }
    true
}

pub fn has_empty(grid: &Grid) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                return true;
            }
        }
    }
    false
}

pub fn possible_values(grid: &Grid, row: usize, col: usize) -> Vec<u8> {
    if grid[row][col].value().is_some() {
        return vec![];
    }

    let mut rows = [0u16; 9];
    let mut cols = [0u16; 9];
    let mut boxes = [0u16; 9];

    for r in 0..9 {
        for c in 0..9 {
            if let Some(v) = grid[r][c].value() {
                let bit = 1u16 << v;
                rows[r] |= bit;
                cols[c] |= bit;
                boxes[(r / 3) * 3 + c / 3] |= bit;
            }
        }
    }

    let mask = ALL_VALUES & !(rows[row] | cols[col] | boxes[(row / 3) * 3 + col / 3]);
    let mut result = Vec::with_capacity(9);
    let mut m = mask;
    while m != 0 {
        let lsb = m & m.wrapping_neg();
        m &= m - 1;
        let val = lsb.trailing_zeros() as u8;
        result.push(val);
    }
    result
}

pub fn find_errors(grid: &Grid) -> Vec<(usize, usize)> {
    let mut errors = Vec::new();
    let mut seen = [[false; 9]; 9];

    for r in 0..9 {
        for c in 0..9 {
            if let Some(val) = grid[r][c].value() {
                if seen[r][val as usize - 1] {
                    errors.push((r, c));
                } else {
                    seen[r][val as usize - 1] = true;
                }
            }
        }
    }

    for c in 0..9 {
        seen = [[false; 9]; 9];
        for r in 0..9 {
            if let Some(val) = grid[r][c].value() {
                if seen[c][val as usize - 1] {
                    if !errors.contains(&(r, c)) {
                        errors.push((r, c));
                    }
                } else {
                    seen[c][val as usize - 1] = true;
                }
            }
        }
    }

    for box_r in (0..9).step_by(3) {
        for box_c in (0..9).step_by(3) {
            seen = [[false; 9]; 9];
            for dr in 0..3 {
                for dc in 0..3 {
                    let r = box_r + dr;
                    let c = box_c + dc;
                    if let Some(val) = grid[r][c].value() {
                        if seen[0][val as usize - 1] {
                            if !errors.contains(&(r, c)) {
                                errors.push((r, c));
                            }
                        } else {
                            seen[0][val as usize - 1] = true;
                        }
                    }
                }
            }
        }
    }

    errors
}
