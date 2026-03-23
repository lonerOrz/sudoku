// checker.rs: 数独验证规则

#![allow(clippy::needless_range_loop)]

use crate::board::{ALL_VALUES, BitmaskGrid, Grid};
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Default)]
    pub struct ConflictType: u8 {
        const ROW = 0b001;
        const COL = 0b010;
        const BOX = 0b100;
    }
}

pub type Conflicts = [[ConflictType; 9]; 9];

pub fn compute_conflicts(grid: &Grid) -> Conflicts {
    let mut conflicts: Conflicts = [[ConflictType::empty(); 9]; 9];

    for r in 0..9 {
        let mut seen = 0u16;
        for c in 0..9 {
            if let Some(val) = grid[r][c].value() {
                let bit = 1u16 << val;
                if seen & bit != 0 {
                    for c2 in 0..9 {
                        if grid[r][c2].value() == Some(val) {
                            conflicts[r][c2].insert(ConflictType::ROW);
                        }
                    }
                }
                seen |= bit;
            }
        }
    }

    for c in 0..9 {
        let mut seen = 0u16;
        for r in 0..9 {
            if let Some(val) = grid[r][c].value() {
                let bit = 1u16 << val;
                if seen & bit != 0 {
                    for r2 in 0..9 {
                        if grid[r2][c].value() == Some(val) {
                            conflicts[r2][c].insert(ConflictType::COL);
                        }
                    }
                }
                seen |= bit;
            }
        }
    }

    for box_r in (0..9).step_by(3) {
        for box_c in (0..9).step_by(3) {
            let mut seen = 0u16;
            for dr in 0..3 {
                for dc in 0..3 {
                    let r = box_r + dr;
                    let c = box_c + dc;
                    if let Some(val) = grid[r][c].value() {
                        let bit = 1u16 << val;
                        if seen & bit != 0 {
                            for dr2 in 0..3 {
                                for dc2 in 0..3 {
                                    let r2 = box_r + dr2;
                                    let c2 = box_c + dc2;
                                    if grid[r2][c2].value() == Some(val) {
                                        conflicts[r2][c2].insert(ConflictType::BOX);
                                    }
                                }
                            }
                        }
                        seen |= bit;
                    }
                }
            }
        }
    }

    conflicts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_type_flags() {
        let mut ct = ConflictType::empty();
        assert!(!ct.intersects(ConflictType::ROW));
        assert!(!ct.intersects(ConflictType::COL));
        assert!(!ct.intersects(ConflictType::BOX));

        ct.insert(ConflictType::ROW);
        assert!(ct.intersects(ConflictType::ROW));
        assert!(!ct.intersects(ConflictType::COL));

        ct.insert(ConflictType::COL);
        ct.insert(ConflictType::BOX);
        assert!(ct.intersects(ConflictType::ROW));
        assert!(ct.intersects(ConflictType::COL));
        assert!(ct.intersects(ConflictType::BOX));
    }

    #[test]
    fn test_compute_conflicts_row() {
        let mut grid: Grid = [[crate::Cell::Empty; 9]; 9];
        grid[0][0] = crate::Cell::Given(5);
        grid[0][3] = crate::Cell::UserInput(5);

        let conflicts = compute_conflicts(&grid);
        assert!(conflicts[0][0].intersects(ConflictType::ROW));
        assert!(conflicts[0][3].intersects(ConflictType::ROW));
    }

    #[test]
    fn test_compute_conflicts_no_conflict() {
        let mut grid: Grid = [[crate::Cell::Empty; 9]; 9];
        grid[0][0] = crate::Cell::Given(5);
        grid[0][3] = crate::Cell::Given(3);

        let conflicts = compute_conflicts(&grid);
        assert!(conflicts[0][0].is_empty());
        assert!(conflicts[0][3].is_empty());
    }
}

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

pub fn find_conflicts_at(grid: &Grid, row: usize, col: usize, val: u8) -> Vec<(usize, usize)> {
    let mut errors = Vec::new();
    let mut added = 0u128;

    for c in 0..9 {
        if c != col && grid[row][c].value() == Some(val) {
            let idx = row * 9 + c;
            let bit = 1u128 << idx;
            if added & bit == 0 {
                errors.push((row, c));
                added |= bit;
            }
        }
    }

    for r in 0..9 {
        if r != row && grid[r][col].value() == Some(val) {
            let idx = r * 9 + col;
            let bit = 1u128 << idx;
            if added & bit == 0 {
                errors.push((r, col));
                added |= bit;
            }
        }
    }

    let box_r = (row / 3) * 3;
    let box_c = (col / 3) * 3;
    for dr in 0..3 {
        for dc in 0..3 {
            let r = box_r + dr;
            let c = box_c + dc;
            if r != row && c != col && grid[r][c].value() == Some(val) {
                let idx = r * 9 + c;
                let bit = 1u128 << idx;
                if added & bit == 0 {
                    errors.push((r, c));
                    added |= bit;
                }
            }
        }
    }

    if !errors.is_empty() {
        errors.insert(0, (row, col));
    }

    errors
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
    let mut added = 0u128;

    for r in 0..9 {
        let mut seen = 0u16;
        for c in 0..9 {
            if let Some(val) = grid[r][c].value() {
                let bit = 1u16 << val;
                if seen & bit != 0 {
                    for c2 in 0..9 {
                        if grid[r][c2].value() == Some(val) {
                            let idx = r * 9 + c2;
                            if added & (1u128 << idx) == 0 {
                                errors.push((r, c2));
                                added |= 1u128 << idx;
                            }
                        }
                    }
                }
                seen |= bit;
            }
        }
    }

    for c in 0..9 {
        let mut seen = 0u16;
        for r in 0..9 {
            if let Some(val) = grid[r][c].value() {
                let bit = 1u16 << val;
                if seen & bit != 0 {
                    for r2 in 0..9 {
                        if grid[r2][c].value() == Some(val) {
                            let idx = r2 * 9 + c;
                            if added & (1u128 << idx) == 0 {
                                errors.push((r2, c));
                                added |= 1u128 << idx;
                            }
                        }
                    }
                }
                seen |= bit;
            }
        }
    }

    for box_r in (0..9).step_by(3) {
        for box_c in (0..9).step_by(3) {
            let mut seen = 0u16;
            for dr in 0..3 {
                for dc in 0..3 {
                    let r = box_r + dr;
                    let c = box_c + dc;
                    if let Some(val) = grid[r][c].value() {
                        let bit = 1u16 << val;
                        if seen & bit != 0 {
                            for dr2 in 0..3 {
                                for dc2 in 0..3 {
                                    let r2 = box_r + dr2;
                                    let c2 = box_c + dc2;
                                    if grid[r2][c2].value() == Some(val) {
                                        let idx = r2 * 9 + c2;
                                        if added & (1u128 << idx) == 0 {
                                            errors.push((r2, c2));
                                            added |= 1u128 << idx;
                                        }
                                    }
                                }
                            }
                        }
                        seen |= bit;
                    }
                }
            }
        }
    }

    errors
}
