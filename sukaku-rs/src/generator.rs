use rand::seq::SliceRandom;
use rand::Rng;

use crate::error::Error;
use crate::error::Result;
use crate::grid::Grid;
use crate::rating::Rater;
use crate::solver::Solver;

pub struct Generator {
    #[allow(dead_code)]
    pub seed: Option<u64>,
    pub min_difficulty: f64,
    pub max_difficulty: f64,
    pub require_unique: bool,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            seed: None,
            min_difficulty: 0.0,
            max_difficulty: 10.0,
            require_unique: true,
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed: Some(seed),
            min_difficulty: 0.0,
            max_difficulty: 10.0,
            require_unique: true,
        }
    }

    pub fn with_difficulty(min: f64, max: f64) -> Self {
        Self {
            seed: None,
            min_difficulty: min,
            max_difficulty: max,
            require_unique: true,
        }
    }

    pub fn generate(&mut self) -> Result<Grid> {
        let mut rng = rand::thread_rng();
        let max_attempts = 1000;

        for _ in 0..max_attempts {
            let mut filled_grid = Grid::new();
            if !self.solve_random(&mut filled_grid, &mut rng) {
                continue;
            }

            let mut puzzle = filled_grid;
            self.remove_digits(&mut puzzle, &mut rng);

            let mut solver = Solver::new(puzzle);
            solver.rebuild_candidates();

            if self.require_unique && !solver.has_unique_solution() {
                continue;
            }

            let mut rater = Rater::new(&mut solver);
            let rating = rater.analyse();

            if rating.er >= self.min_difficulty && rating.er <= self.max_difficulty {
                return Ok(puzzle);
            }
        }

        Err(Error::GenerationFailed)
    }

    fn solve_random(&mut self, grid: &mut Grid, rng: &mut impl Rng) -> bool {
        if grid.is_solved() {
            return true;
        }

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

        let mut candidates: Vec<u8> = min_cands.1.iter().collect();
        candidates.shuffle(rng);

        for v in candidates {
            grid.set(min_cands.0, v);
            grid.rebuild_candidates();

            if grid.is_valid_move(min_cands.0, v) && self.solve_random(grid, rng) {
                return true;
            }

            grid.set(min_cands.0, 0);
            grid.rebuild_candidates();
        }

        false
    }

    fn remove_digits(&self, grid: &mut Grid, rng: &mut impl Rng) {
        let mut positions: Vec<u8> = (0..81).collect();
        positions.shuffle(rng);

        let mut removed = 0;
        let target_remove = rng.gen_range(40..=50);

        for &pos in &positions {
            if removed >= target_remove {
                break;
            }

            let backup = grid.get(pos);
            if backup == 0 {
                continue;
            }

            grid.set(pos, 0);
            grid.rebuild_candidates();

            let mut solver = Solver::new(*grid);
            solver.rebuild_candidates();

            if solver.has_unique_solution() {
                removed += 1;
            } else {
                grid.set(pos, backup);
                grid.rebuild_candidates();
            }
        }
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
