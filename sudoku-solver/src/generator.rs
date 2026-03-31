//! Sudoku puzzle generator with difficulty control and symmetry support.
//!
//! This module implements a generator similar to SukakuExplainer's approach:
//! 1. Generate a random complete grid
//! 2. Remove digits based on target difficulty (not always maximal)
//! 3. Verify the resulting puzzle meets difficulty constraints (if specified)
//!
//! Difficulty-to-clue mapping:
//! - ER 1.0-2.0: 30-40 clues (easy, more clues)
//! - ER 2.0-3.0: 25-30 clues (medium, moderate removal)
//! - ER 3.0-5.0: 22-26 clues (hard, aggressive removal)
//! - ER 5.0+:    17-22 clues (expert, maximal removal)

use rand::seq::SliceRandom;
use rand::Rng;

use crate::error::Error;
use crate::error::Result;
use crate::grid::Grid;
use crate::rating::Rater;
use crate::solver::Solver;

/// Symmetry types for puzzle generation.
///
/// Symmetric puzzles have matching patterns of filled/empty cells.
/// This affects the aesthetic quality and perceived difficulty of puzzles.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Symmetry {
    /// No symmetry - cells are removed randomly
    #[default]
    None,
    /// 180-degree rotational symmetry (center cell maps to opposite)
    Rotational180,
    /// 90-degree rotational symmetry (4-fold symmetry)
    Rotational90,
    /// Horizontal mirror symmetry (left-right)
    Horizontal,
    /// Vertical mirror symmetry (top-bottom)
    Vertical,
    /// Main diagonal symmetry (top-left to bottom-right)
    DiagonalMain,
    /// Anti-diagonal symmetry (top-right to bottom-left)
    DiagonalAnti,
    /// Full D4 symmetry group (all 8 symmetries)
    Full,
}

impl Symmetry {
    /// Returns all symmetric positions for a given cell position.
    ///
    /// For example, with Rotational180 symmetry, position (0,0) maps to (8,8).
    /// Positions are automatically deduplicated (e.g., center cell in Rotational90).
    pub fn get_symmetric_positions(&self, pos: u8) -> Vec<u8> {
        let row = pos / 9;
        let col = pos % 9;

        let mut positions: Vec<u8> = match self {
            Symmetry::None => vec![pos],
            Symmetry::Rotational180 => vec![pos, (8 - row) * 9 + (8 - col)],
            Symmetry::Horizontal => vec![pos, row * 9 + (8 - col)],
            Symmetry::Vertical => vec![pos, (8 - row) * 9 + col],
            Symmetry::DiagonalMain => vec![pos, col * 9 + row],
            Symmetry::DiagonalAnti => vec![pos, (8 - col) * 9 + (8 - row)],
            Symmetry::Rotational90 => vec![
                pos,
                col * 9 + (8 - row),
                (8 - row) * 9 + (8 - col),
                (8 - col) * 9 + row,
            ],
            Symmetry::Full => vec![
                pos,
                (8 - row) * 9 + (8 - col),
                row * 9 + (8 - col),
                (8 - row) * 9 + col,
                col * 9 + row,
                (8 - col) * 9 + (8 - row),
                (8 - col) * 9 + row,
                col * 9 + (8 - row),
            ],
        };

        positions.sort();
        positions.dedup();

        positions
    }
}

/// Sudoku puzzle generator with difficulty and symmetry control.
///
/// # Example
/// ```
/// use sudoku_solver::{Generator, Symmetry};
///
/// let mut gen = Generator::new();
/// gen.symmetry = Symmetry::Rotational180;
///
/// match gen.generate() {
///     Ok(puzzle) => println!("Generated puzzle"),
///     Err(e) => println!("Generation failed: {}", e),
/// }
/// ```
pub struct Generator {
    /// Optional seed for reproducible generation
    pub seed: Option<u64>,
    /// Minimum difficulty (ER rating)
    pub min_difficulty: f64,
    /// Maximum difficulty (ER rating)
    pub max_difficulty: f64,
    /// Whether to require unique solution
    pub require_unique: bool,
    /// Symmetry type for the generated puzzle
    pub symmetry: Symmetry,
    /// Techniques to exclude (e.g., ["X-Wing", "Swordfish"])
    pub exclude_techniques: Vec<String>,
    /// Techniques to include (only these will be used, empty = all)
    pub include_techniques: Vec<String>,
}

impl Generator {
    /// Creates a new generator with default settings.
    pub fn new() -> Self {
        Self {
            seed: None,
            min_difficulty: 0.0,
            max_difficulty: 10.0,
            require_unique: true,
            symmetry: Symmetry::None,
            exclude_techniques: Vec::new(),
            include_techniques: Vec::new(),
        }
    }

    /// Creates a generator with a specific seed for reproducible results.
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed: Some(seed),
            min_difficulty: 0.0,
            max_difficulty: 10.0,
            require_unique: true,
            symmetry: Symmetry::None,
            exclude_techniques: Vec::new(),
            include_techniques: Vec::new(),
        }
    }

    /// Creates a generator with a specific difficulty range.
    pub fn with_difficulty(min: f64, max: f64) -> Self {
        Self {
            seed: None,
            min_difficulty: min,
            max_difficulty: max,
            require_unique: true,
            symmetry: Symmetry::None,
            exclude_techniques: Vec::new(),
            include_techniques: Vec::new(),
        }
    }

    pub fn with_symmetry(symmetry: Symmetry) -> Self {
        Self {
            seed: None,
            min_difficulty: 0.0,
            max_difficulty: 10.0,
            require_unique: true,
            symmetry,
            exclude_techniques: Vec::new(),
            include_techniques: Vec::new(),
        }
    }

    pub fn generate(&mut self) -> Result<Grid> {
        let mut rng = rand::thread_rng();
        let max_attempts = 100;
        let verify_difficulty = self.min_difficulty > 0.0 || self.max_difficulty < 10.0;

        // Map difficulty range to target clue count
        let (min_clues, max_clues) = self.difficulty_to_clue_range();

        for _ in 0..max_attempts {
            // Step 1: Generate a random complete grid
            let mut filled_grid = Grid::new();
            if !self.solve_random(&mut filled_grid, &mut rng) {
                continue;
            }

            let solution = filled_grid;
            let mut puzzle = solution;

            // Step 2: Remove digits to target clue count
            self.remove_digits_to_target(&mut puzzle, &solution, &mut rng, min_clues, max_clues);

            // Step 3: Verify unique solution
            if self.require_unique {
                let mut solver = Solver::new(puzzle);
                solver.rebuild_candidates();
                if !solver.has_unique_solution() {
                    continue;
                }
            }

            // Step 4: Verify difficulty (if constraints specified)
            if verify_difficulty {
                let mut solver = Solver::new(puzzle);
                solver.rebuild_candidates();
                let mut rater = Rater::new(&mut solver);
                let rating = rater.analyse();

                if rating.er >= self.min_difficulty && rating.er <= self.max_difficulty {
                    return Ok(puzzle);
                }
                continue;
            }

            return Ok(puzzle);
        }

        Err(Error::GenerationFailed)
    }

    /// Maps target difficulty range to clue count range.
    ///
    /// Higher difficulty → fewer clues (more digits removed).
    fn difficulty_to_clue_range(&self) -> (usize, usize) {
        // Use average of min/max to determine target
        let avg_diff = (self.min_difficulty + self.max_difficulty) / 2.0;

        if avg_diff <= 2.0 {
            // Easy: 30-40 clues
            (30, 40)
        } else if avg_diff <= 3.0 {
            // Medium: 25-30 clues
            (25, 30)
        } else if avg_diff <= 5.0 {
            // Hard: 22-26 clues
            (22, 26)
        } else {
            // Expert: 17-22 clues
            (17, 22)
        }
    }

    /// Removes digits to reach a target clue count range.
    ///
    /// Unlike maximal removal, this stops once the target is reached.
    /// This helps achieve medium difficulty puzzles.
    fn remove_digits_to_target(
        &self,
        grid: &mut Grid,
        solution: &Grid,
        rng: &mut impl Rng,
        min_clues: usize,
        max_clues: usize,
    ) {
        let symmetry = self.symmetry;
        let mut attempts = 0;

        while attempts < 6 {
            let mut positions: Vec<u8> = (0..81).collect();
            positions.shuffle(rng);

            let mut any_removed = false;
            let current_clues = (0..81).filter(|&i| grid.get(i) != 0).count();

            // Stop if we've reached target clue count
            if current_clues <= max_clues {
                return;
            }

            for &pos in &positions {
                let current_clues = (0..81).filter(|&i| grid.get(i) != 0).count();

                // Stop if we've reached minimum clue count
                if current_clues <= min_clues {
                    return;
                }

                let symmetric_positions = symmetry.get_symmetric_positions(pos);

                // Skip if any symmetric position is already empty
                let all_filled = symmetric_positions.iter().all(|&p| grid.get(p) != 0);
                if !all_filled {
                    continue;
                }

                // Try removing symmetric positions
                for &p in &symmetric_positions {
                    grid.set(p, 0);
                }
                grid.rebuild_candidates();

                if self.require_unique {
                    let mut solver = Solver::new(*grid);
                    solver.rebuild_candidates();

                    if solver.has_unique_solution() {
                        any_removed = true;
                    } else {
                        // Restore if uniqueness lost
                        for &p in &symmetric_positions {
                            grid.set(p, solution.get(p));
                        }
                        grid.rebuild_candidates();
                    }
                } else {
                    any_removed = true;
                }
            }

            if any_removed {
                attempts = 0;
            } else {
                attempts += 1;
            }
        }
    }

    /// Fills an empty grid with a random valid Sudoku solution.
    ///
    /// Uses backtracking with MRV heuristic and random ordering.
    fn solve_random(&self, grid: &mut Grid, rng: &mut impl Rng) -> bool {
        if grid.is_solved() {
            return true;
        }

        // Find cell with minimum candidates (MRV heuristic)
        let mut min_cands = (0u8, crate::grid::Candidates::full());
        for i in 0..81 {
            if grid.get(i) == 0 {
                let cands = grid.candidates(i);
                if cands.cardinality() < min_cands.1.cardinality() {
                    min_cands = (i, cands);
                }
            }
        }

        if min_cands.1.is_empty() {
            return false;
        }

        // Try candidates in random order
        let mut candidates: Vec<u8> = min_cands.1.iter().collect();
        candidates.shuffle(rng);

        for v in candidates {
            if !grid.is_valid_move(min_cands.0, v) {
                continue;
            }

            grid.set(min_cands.0, v);
            grid.rebuild_candidates();

            if self.solve_random(grid, rng) {
                return true;
            }

            grid.set(min_cands.0, 0);
            grid.rebuild_candidates();
        }

        false
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
