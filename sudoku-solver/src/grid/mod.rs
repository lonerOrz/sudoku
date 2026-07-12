//! 9x9 Sudoku grid with candidate tracking.
//!
//! Grid is the core data structure representing a standard 9x9 Sudoku puzzle.
//! Supports pencilmark-style candidate tracking for solving technique detection.

pub mod candidates;
pub mod cell;
pub mod region;

pub use candidates::Candidates;
pub use cell::CellIndex;
pub use region::{Region, RegionType, BLOCKS, COLS, ROWS};

use crate::error::Error;
use crate::error::Result;
use std::str::FromStr;

/// A 9x9 Sudoku grid with candidate tracking.
///
/// Stores 81 cell values as `u8` (0 = empty, 1-9 = placed digit) and
/// per-cell candidate bitmasks for pencilmark-style solving.
///
/// Implements `FromStr` for parsing from 81-character strings where
/// digits 1-9 are clues, `0` or `.` are empty cells.
///
/// ```
/// use sudoku_solver::Grid;
///
/// let grid = Grid::parse("003020600900305001001806400008102900700000008006708200002609500800203009005010300").unwrap();
/// assert_eq!(grid.clue_count(), 32);
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Grid {
    cells: [u8; 81],
    candidates: [Candidates; 81],
    /// Clue count: number of filled cells (non-zero)
    clue_count: usize,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [0; 81],
            candidates: [Candidates::full(); 81],
            clue_count: 0,
        }
    }

    pub fn parse(s: &str) -> Result<Self> {
        Self::from_str(s)
    }

    pub fn from_flat(cells: [u8; 81]) -> Self {
        let mut grid = Self::new();
        let mut clue_count = 0;
        for (i, &v) in cells.iter().enumerate() {
            grid.cells[i] = v;
            if v > 0 {
                clue_count += 1;
                grid.candidates[i] = Candidates::empty();
            }
        }
        grid.clue_count = clue_count;
        grid.rebuild_candidates();
        grid
    }

    #[inline]
    pub fn get(&self, idx: u8) -> u8 {
        self.cells[idx as usize]
    }

    pub fn set(&mut self, idx: u8, value: u8) {
        debug_assert!(value <= 9);
        let old_value = self.cells[idx as usize];
        if old_value > 0 && value == 0 {
            self.clue_count -= 1;
        } else if old_value == 0 && value > 0 {
            self.clue_count += 1;
        }
        self.cells[idx as usize] = value;
        if value > 0 {
            self.candidates[idx as usize] = Candidates::empty();
        }
    }

    #[inline]
    pub fn candidates(&self, idx: u8) -> Candidates {
        self.candidates[idx as usize]
    }

    #[inline]
    pub fn clue_count(&self) -> usize {
        self.clue_count
    }

    pub fn remove_candidate(&mut self, idx: u8, value: u8) {
        if self.cells[idx as usize] == 0 {
            self.candidates[idx as usize].remove(value);
        }
    }

    /// Clear all candidates for a cell (used when a value is placed).
    pub fn clear_candidates(&mut self, idx: u8) {
        self.candidates[idx as usize] = Candidates::empty();
    }

    pub fn rebuild_candidates(&mut self) {
        for i in 0..81 {
            if self.cells[i] > 0 {
                self.candidates[i] = Candidates::empty();
                continue;
            }

            let mut cands = Candidates::full();
            let r = i / 9;
            let c = i % 9;
            let b = (r / 3) * 3 + c / 3;

            for &j in &ROWS[r].cells {
                let v = self.cells[j as usize];
                if v > 0 {
                    cands.remove(v);
                }
            }
            for &j in &COLS[c].cells {
                let v = self.cells[j as usize];
                if v > 0 {
                    cands.remove(v);
                }
            }
            for &j in &BLOCKS[b].cells {
                let v = self.cells[j as usize];
                if v > 0 {
                    cands.remove(v);
                }
            }

            self.candidates[i] = cands;
        }
    }

    pub fn is_solved(&self) -> bool {
        self.cells.iter().all(|&v| v > 0)
    }

    pub fn is_valid_move(&self, idx: u8, val: u8) -> bool {
        let r = idx / 9;
        let c = idx % 9;
        let b = (r / 3) * 3 + c / 3;

        for &i in &ROWS[r as usize].cells {
            if self.cells[i as usize] == val {
                return false;
            }
        }
        for &i in &COLS[c as usize].cells {
            if self.cells[i as usize] == val {
                return false;
            }
        }
        for &i in &BLOCKS[b as usize].cells {
            if self.cells[i as usize] == val {
                return false;
            }
        }
        true
    }

    /// Check grid invariants. Returns true if consistent.
    pub fn check_consistency(&self) -> bool {
        for i in 0..81 {
            let val = self.cells[i];
            if val == 0 {
                if self.candidates[i].is_empty() {
                    return false;
                }
            } else {
                if !self.candidates[i].is_empty() {
                    return false;
                }
                if val > 9 {
                    return false;
                }
            }
        }
        // Check rows and columns
        for unit in 0..9 {
            let mut seen = [false; 10];
            for c in 0..9 {
                let v = self.cells[unit * 9 + c] as usize;
                if v > 0 && seen[v] {
                    return false;
                }
                if v > 0 {
                    seen[v] = true;
                }
            }
            let mut seen = [false; 10];
            for r in 0..9 {
                let v = self.cells[r * 9 + unit] as usize;
                if v > 0 && seen[v] {
                    return false;
                }
                if v > 0 {
                    seen[v] = true;
                }
            }
        }
        // Check boxes
        for b in 0..9 {
            let mut seen = [false; 10];
            let br = (b / 3) * 3;
            let bc = (b % 3) * 3;
            for r in br..br + 3 {
                for c in bc..bc + 3 {
                    let v = self.cells[r * 9 + c] as usize;
                    if v > 0 && seen[v] {
                        return false;
                    }
                    if v > 0 {
                        seen[v] = true;
                    }
                }
            }
        }
        true
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let digits: Vec<u8> = s
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .map(|c| match c {
                '0' | '.' => 0,
                _ => c.to_digit(10).unwrap_or(0) as u8,
            })
            .collect();

        if digits.len() != 81 {
            return Err(Error::InvalidLength(digits.len()));
        }

        let mut clue_count = 0;
        let mut grid = Self::new();
        for (i, &digit) in digits.iter().enumerate() {
            if digit > 9 {
                return Err(Error::InvalidDigit(i, digit));
            }
            grid.cells[i] = digit;
            if digit > 0 {
                clue_count += 1;
                grid.candidates[i] = Candidates::empty();
            }
        }
        grid.clue_count = clue_count;
        grid.rebuild_candidates();
        Ok(grid)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &v) in self.cells.iter().enumerate() {
            write!(f, "{}", if v == 0 { '.' } else { (v + b'0') as char })?;
            if (i + 1) % 9 == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid:")?;
        for r in 0..9 {
            for c in 0..9 {
                let v = self.cells[r * 9 + c];
                if v == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", v)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
