// checker.rs: 数独验证规则

#![allow(clippy::needless_range_loop)]

use crate::board::{ALL_VALUES, BitmaskGrid, Grid};

pub fn is_solved(grid: &Grid) -> bool {
    let masks = BitmaskGrid::from_grid(grid);
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                return false;
            }
        }
    }
    masks.rows.iter().all(|&m| m == ALL_VALUES)
        && masks.cols.iter().all(|&m| m == ALL_VALUES)
        && masks.boxes.iter().all(|&m| m == ALL_VALUES)
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
    let masks = BitmaskGrid::from_grid(grid);
    let mask = masks.candidates(row, col);
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

    for r in 0..9 {
        let mut seen_row = [false; 9];
        let mut positions_row = Vec::new();
        for c in 0..9 {
            if let Some(val) = grid[r][c].value() {
                let v = val as usize - 1;
                if seen_row[v] {
                    for &(or, oc) in &positions_row {
                        if !errors.contains(&(or, oc)) {
                            errors.push((or, oc));
                        }
                    }
                    if !errors.contains(&(r, c)) {
                        errors.push((r, c));
                    }
                } else {
                    seen_row[v] = true;
                    positions_row.push((r, c));
                }
            }
        }
    }

    for c in 0..9 {
        let mut seen_row = [false; 9];
        let mut positions_row = Vec::new();
        for r in 0..9 {
            if let Some(val) = grid[r][c].value() {
                let v = val as usize - 1;
                if seen_row[v] {
                    for &(or, oc) in &positions_row {
                        if !errors.contains(&(or, oc)) {
                            errors.push((or, oc));
                        }
                    }
                    if !errors.contains(&(r, c)) {
                        errors.push((r, c));
                    }
                } else {
                    seen_row[v] = true;
                    positions_row.push((r, c));
                }
            }
        }
    }

    for box_r in (0..9).step_by(3) {
        for box_c in (0..9).step_by(3) {
            let mut seen_row = [false; 9];
            let mut positions_row = Vec::new();
            for dr in 0..3 {
                for dc in 0..3 {
                    let r = box_r + dr;
                    let c = box_c + dc;
                    if let Some(val) = grid[r][c].value() {
                        let v = val as usize - 1;
                        if seen_row[v] {
                            for &(or, oc) in &positions_row {
                                if !errors.contains(&(or, oc)) {
                                    errors.push((or, oc));
                                }
                            }
                            if !errors.contains(&(r, c)) {
                                errors.push((r, c));
                            }
                        } else {
                            seen_row[v] = true;
                            positions_row.push((r, c));
                        }
                    }
                }
            }
        }
    }

    errors
}
