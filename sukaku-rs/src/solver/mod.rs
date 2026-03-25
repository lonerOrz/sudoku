//! Rule-based Sudoku solver.
//!
//! Applies solving techniques in order of difficulty to find the next move.
//! Returns hints that can be applied to make progress on the puzzle.

pub mod accumulator;
pub mod hint;

pub use accumulator::HintAccumulator;
pub use hint::{Hint, HintType};

use crate::grid::Grid;
use crate::rules;

pub struct Solver {
    grid: Grid,
    steps: usize,
}

impl Solver {
    pub fn new(grid: Grid) -> Self {
        Self { grid, steps: 0 }
    }

    pub fn grid(&self) -> Grid {
        self.grid
    }

    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    pub fn rebuild_candidates(&mut self) {
        self.grid.rebuild_candidates();
    }

    pub fn next_hint(&mut self) -> Option<Hint> {
        let mut acc = HintAccumulator::new();

        rules::naked_single(&self.grid, &mut acc);
        if let Some(hint) = acc.first() {
            return Some(hint);
        }

        rules::hidden_single(&self.grid, &mut acc);
        if let Some(hint) = acc.first() {
            return Some(hint);
        }

        rules::naked_pair(&self.grid, &mut acc);
        if let Some(hint) = acc.first() {
            return Some(hint);
        }

        rules::hidden_pair(&self.grid, &mut acc);
        if let Some(hint) = acc.first() {
            return Some(hint);
        }

        rules::naked_triple(&self.grid, &mut acc);
        if let Some(hint) = acc.first() {
            return Some(hint);
        }

        rules::hidden_triple(&self.grid, &mut acc);
        if let Some(hint) = acc.first() {
            return Some(hint);
        }

        rules::locked_pointing(&self.grid, &mut acc);
        if let Some(hint) = acc.first() {
            return Some(hint);
        }

        rules::locked_claiming(&self.grid, &mut acc);
        acc.first()
    }

    pub fn apply_hint(&mut self, hint: &Hint) {
        if hint.value > 0 {
            self.grid.set(hint.cell.index, hint.value);
        }

        for &(cell, ref values) in &hint.eliminations {
            for &v in values {
                self.grid.remove_candidate(cell.index, v);
            }
        }

        self.grid.rebuild_candidates();
        self.steps += 1;
    }

    pub fn solve(&mut self) -> bool {
        self.grid.rebuild_candidates();

        while !self.grid.is_solved() {
            if let Some(hint) = self.next_hint() {
                self.apply_hint(&hint);
            } else {
                return false;
            }
        }
        true
    }

    pub fn steps(&self) -> usize {
        self.steps
    }

    pub fn count_solutions(&mut self) -> usize {
        let backup = self.grid;
        let mut count = 0;
        self.count_solutions_inner(&mut count, 2);
        self.grid = backup;
        count
    }

    fn count_solutions_inner(&mut self, count: &mut usize, limit: usize) {
        if *count >= limit {
            return;
        }

        if self.grid.is_solved() {
            *count += 1;
            return;
        }

        let mut min_cands = (0u8, crate::grid::Candidates::full());
        for i in 0..81 {
            if self.grid.get(i) == 0 {
                let cands = self.grid.candidates(i);
                if cands.cardinality() < min_cands.1.cardinality() {
                    min_cands = (i, cands);
                }
            }
        }

        if min_cands.1.is_empty() {
            return;
        }

        for v in min_cands.1.iter() {
            self.grid.set(min_cands.0, v);
            self.grid.rebuild_candidates();
            self.count_solutions_inner(count, limit);
            self.grid.set(min_cands.0, 0);
            self.grid.rebuild_candidates();
        }
    }

    pub fn has_unique_solution(&mut self) -> bool {
        self.count_solutions() == 1
    }
}
