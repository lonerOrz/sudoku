// checker.rs: 数独验证规则

#![allow(clippy::needless_range_loop)]

use crate::board::{Grid, build_candidates};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct ConflictType(u8);

impl ConflictType {
    pub const NONE: Self = Self(0);
    pub const ROW: Self = Self(1);
    pub const COL: Self = Self(2);
    pub const BOX: Self = Self(4);

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl std::ops::BitOr for ConflictType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for ConflictType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

pub type Conflicts = [[ConflictType; 9]; 9];

pub fn compute_conflicts(grid: &Grid) -> Conflicts {
    let mut conflicts: Conflicts = [[ConflictType::NONE; 9]; 9];

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

pub fn is_solved(grid: &Grid) -> bool {
    let cands = build_candidates(grid);
    for i in 0..81 {
        if grid[i / 9][i % 9].value().is_none() || !cands[i].is_empty() {
            return false;
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
    let cands = build_candidates(grid);
    let idx = row * 9 + col;
    cands[idx].iter().collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_type_flags() {
        let mut ct = ConflictType::NONE;
        assert!(!ct.contains(ConflictType::ROW));
        assert!(!ct.contains(ConflictType::COL));
        assert!(!ct.contains(ConflictType::BOX));

        ct.insert(ConflictType::ROW);
        assert!(ct.contains(ConflictType::ROW));
        assert!(!ct.contains(ConflictType::COL));

        ct.insert(ConflictType::COL);
        ct.insert(ConflictType::BOX);
        assert!(ct.contains(ConflictType::ROW));
        assert!(ct.contains(ConflictType::COL));
        assert!(ct.contains(ConflictType::BOX));
    }

    #[test]
    fn test_compute_conflicts_row() {
        let mut grid: Grid = Grid::new();
        grid[0][0] = crate::Cell::Given(5);
        grid[0][3] = crate::Cell::UserInput(5);

        let conflicts = compute_conflicts(&grid);
        assert!(conflicts[0][0].contains(ConflictType::ROW));
        assert!(conflicts[0][3].contains(ConflictType::ROW));
    }

    #[test]
    fn test_compute_conflicts_no_conflict() {
        let mut grid: Grid = Grid::new();
        grid[0][0] = crate::Cell::Given(5);
        grid[0][3] = crate::Cell::Given(3);

        let conflicts = compute_conflicts(&grid);
        assert!(conflicts[0][0].is_empty());
        assert!(conflicts[0][3].is_empty());
    }
}
