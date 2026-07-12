//! Rule-based Sudoku solver.
//!
//! Applies solving techniques in order of difficulty to find the next move.
//! Returns hints that can be applied to make progress on the puzzle.

pub mod accumulator;
pub mod hint;

pub use accumulator::HintAccumulator;
pub use hint::{Hint, HintType};

use std::collections::HashSet;

use crate::grid::Grid;
use crate::rules::{self, Strategy};

/// Rule-based Sudoku solver that detects solving techniques.
///
/// Applies techniques in order of difficulty (Naked Single → Hidden Single → ...) to find the next move.
/// Falls back to MRV backtracking when no rule-based technique applies.
///
/// ```
/// use sudoku_solver::{Grid, Solver};
///
/// let grid = Grid::parse("003020600900305001001806400008102900700000008006708200002609500800203009005010300").unwrap();
/// let mut solver = Solver::new(grid);
/// if let Some(hint) = solver.next_hint() {
///     println!("Technique: {} at cell {}, value {}", hint.technique_name, hint.cell.index, hint.value);
/// }
/// ```
pub struct Solver {
    grid: Grid,
    steps: usize,
    skipped: Vec<(u8, u8)>,
    rejected_elim_keys: HashSet<u64>,
}

impl Solver {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            steps: 0,
            skipped: Vec::new(),
            rejected_elim_keys: HashSet::new(),
        }
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
        self.next_hint_with_strategy(Strategy::Solve, &[])
    }

    pub fn next_hint_with_filter(&mut self, disabled: &[&str]) -> Option<Hint> {
        self.next_hint_with_strategy(Strategy::Solve, disabled)
    }

    pub fn next_hint_with_strategy(
        &mut self,
        strategy: Strategy,
        disabled: &[&str],
    ) -> Option<Hint> {
        let rules = match strategy {
            Strategy::Solve => rules::rules_for_solve(),
            Strategy::Detect => rules::rules_for_detect(),
        };

        for rule in rules {
            if disabled.contains(&rule.name) {
                continue;
            }

            let mut acc = HintAccumulator::new();
            (rule.func)(&self.grid, &mut acc);

            for hint in acc.hints() {
                // Skip hints that have been rejected
                if hint.value > 0 && self.skipped.contains(&(hint.cell.index, hint.value)) {
                    continue;
                }
                // Skip elimination hints that were rejected
                if hint.value == 0 {
                    let key = Self::elim_key(hint);
                    if self.rejected_elim_keys.contains(&key) {
                        continue;
                    }
                }
                return Some(hint.clone());
            }
        }

        None
    }

    /// Detect a specific technique without modifying the grid.
    ///
    /// Uses detect strategy (high difficulty first) to find the technique.
    ///
    /// # Returns
    /// - `Some(Hint)` if the technique is found (first match)
    /// - `None` if the technique is not available
    ///
    /// # Notes
    /// - This is a read-only operation (uses `&self`)
    /// - Does not apply the hint or modify candidates
    /// - Uses high-difficulty-first ordering to find rare techniques
    pub fn detect_technique(&self, technique: &str) -> Option<Hint> {
        let rules = rules::rules_for_detect();

        for rule in rules {
            if rule.name != technique {
                continue;
            }

            let mut acc = HintAccumulator::new();
            (rule.func)(&self.grid, &mut acc);

            if let Some(hint) = acc.first() {
                return Some(hint);
            }
        }

        None
    }

    /// Apply a hint. Returns true if applied successfully, false if skipped due to inconsistency.
    pub fn apply_hint(&mut self, hint: &Hint) -> bool {
        if hint.value > 0 {
            // Placement: set value, clear cell candidates, remove value from row/col/box peers
            let backup = self.grid;
            let idx = hint.cell.index;
            let val = hint.value;
            self.grid.set(idx, val);
            self.grid.clear_candidates(idx);
            let r = (idx / 9) as usize;
            let c = (idx % 9) as usize;
            let b = (r / 3) * 3 + c / 3;
            for &j in &crate::grid::ROWS[r].cells {
                if j != idx {
                    self.grid.remove_candidate(j, val);
                }
            }
            for &j in &crate::grid::COLS[c].cells {
                if j != idx {
                    self.grid.remove_candidate(j, val);
                }
            }
            for &j in &crate::grid::BLOCKS[b].cells {
                if j != idx {
                    self.grid.remove_candidate(j, val);
                }
            }
            // Validate: all empty cells must have candidates
            for i in 0..81u8 {
                if self.grid.get(i) == 0 && self.grid.candidates(i).is_empty() {
                    self.grid = backup;
                    self.skipped.push((idx, val));
                    #[cfg(test)]
                    eprintln!("    REJECT: placing {} at cell {} leaves cell {} (row {} col {}) with 0 cands",
                        val, idx, i, i/9, i%9);
                    return false;
                }
            }
        } else {
            // Elimination: validate targets are empty cells with the candidate,
            // and no cell loses all candidates.
            let backup = self.grid;

            // Pre-validate: all targets must be empty cells with the candidate
            for &(cell, ref values) in &hint.eliminations {
                for &v in values {
                    if self.grid.get(cell.index) != 0 || !self.grid.candidates(cell.index).has(v) {
                        self.rejected_elim_keys.insert(Self::elim_key(hint));
                        return false;
                    }
                }
            }

            for &(cell, ref values) in &hint.eliminations {
                for &v in values {
                    #[cfg(test)]
                    eprintln!(
                        "    elim cell={}({},{}) val={}",
                        cell.index,
                        cell.index / 9,
                        cell.index % 9,
                        v
                    );
                    self.grid.remove_candidate(cell.index, v);
                }
            }
            let mut valid = true;
            for i in 0..81u8 {
                if self.grid.get(i) == 0 && self.grid.candidates(i).is_empty() {
                    valid = false;
                    break;
                }
            }
            if !valid {
                self.grid = backup;
                self.rejected_elim_keys.insert(Self::elim_key(hint));
                return false;
            }
        }

        self.steps += 1;
        true
    }

    fn elim_key(hint: &Hint) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for &(cell, ref values) in &hint.eliminations {
            cell.index.hash(&mut hasher);
            values.hash(&mut hasher);
        }
        hasher.finish()
    }

    pub fn solve(&mut self) -> bool {
        self.skipped.clear();
        self.rejected_elim_keys.clear();
        self.grid.rebuild_candidates();

        let mut max_steps = 200;
        while !self.grid.is_solved() {
            max_steps -= 1;
            if max_steps == 0 {
                return false;
            }
            if let Some(hint) = self.next_hint() {
                if self.apply_hint(&hint) {
                    // Success: clear skip lists so rejected hints get retried
                    // (grid changed, they may work now)
                    self.skipped.clear();
                    self.rejected_elim_keys.clear();
                }
            } else {
                // Rule-based solver stuck — fall back to backtracking
                return self.solve_backtrack();
            }
        }
        true
    }

    /// Complete the puzzle using MRV backtracking when rule-based solver gets stuck.
    fn solve_backtrack(&mut self) -> bool {
        self.backtrack_inner()
    }

    fn backtrack_inner(&mut self) -> bool {
        if self.grid.is_solved() {
            return true;
        }
        // Find cell with fewest candidates (MRV)
        let mut best: Option<(u8, crate::grid::Candidates)> = None;
        for i in 0..81u8 {
            if self.grid.get(i) == 0 {
                let cands = self.grid.candidates(i);
                match &best {
                    None => best = Some((i, cands)),
                    Some((_, ref prev)) if cands.cardinality() < prev.cardinality() => {
                        best = Some((i, cands))
                    }
                    _ => {}
                }
            }
        }
        let (idx, cands) = match best {
            Some(v) => v,
            None => return false,
        };
        for v in cands.iter() {
            let backup = self.grid;
            self.grid.set(idx, v);
            self.grid.rebuild_candidates();
            if self.backtrack_inner() {
                return true;
            }
            self.grid = backup;
        }
        false
    }

    pub fn steps(&self) -> usize {
        self.steps
    }

    /// Clear skip/reject lists so previously rejected hints get retried.
    pub fn clear_rejected(&mut self) {
        self.skipped.clear();
        self.rejected_elim_keys.clear();
    }

    pub fn count_solutions(&mut self) -> usize {
        let backup = self.grid;
        let mut count = 0;
        self.grid.rebuild_candidates();
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

    /// Check if puzzle has exactly one solution - optimized version.
    /// Stops searching as soon as 2 solutions are found.
    pub fn has_unique_solution_fast(&mut self) -> bool {
        let backup = self.grid;
        let mut count = 0;
        self.grid.rebuild_candidates();

        // Use existing count_solutions_inner with limit=2
        self.count_solutions_inner(&mut count, 2);

        self.grid = backup;
        count == 1
    }

    /// Validate that a hint is logically correct before applying.
    /// Returns Ok(()) if valid, Err(message) if invalid.
    pub fn validate_hint(&self, hint: &Hint) -> Result<(), String> {
        let grid = &self.grid;

        if hint.value > 0 {
            let idx = hint.cell.index as usize;
            if grid.get(hint.cell.index) != 0 {
                return Err(format!("Cell {} already filled", idx));
            }
            if hint.value < 1 || hint.value > 9 {
                return Err(format!("Invalid value {}", hint.value));
            }
            if !grid.candidates(hint.cell.index).has(hint.value) {
                return Err(format!(
                    "Value {} not in candidates for cell {}",
                    hint.value, idx
                ));
            }
        }

        for &(cell, ref values) in &hint.eliminations {
            let cands = grid.candidates(cell.index);
            for &v in values {
                if !cands.has(v) {
                    return Err(format!(
                        "Candidate {} not present in cell {} for elimination",
                        v, cell.index
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
pub fn apply_hint_validated(solver: &mut Solver, hint: &Hint) {
    solver
        .validate_hint(hint)
        .unwrap_or_else(|e| panic!("Invalid hint from {}: {}", hint.technique_name, e));
    solver.apply_hint(hint);
}
