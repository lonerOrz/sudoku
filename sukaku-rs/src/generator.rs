use crate::error::Error;
use crate::error::Result;
use crate::grid::Grid;

pub struct Generator {
    #[allow(dead_code)]
    seed: Option<u64>,
}

impl Generator {
    pub fn new() -> Self {
        Self { seed: None }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self { seed: Some(seed) }
    }

    pub fn generate(&mut self, _min_diff: f64, _max_diff: f64) -> Result<Grid> {
        let mut grid = Grid::new();
        if !self.solve_random(&mut grid) {
            return Err(Error::GenerationFailed);
        }
        Ok(grid)
    }

    fn solve_random(&mut self, grid: &mut Grid) -> bool {
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

        let candidates: Vec<u8> = min_cands.1.iter().collect();

        for v in candidates {
            grid.set(min_cands.0, v);
            grid.rebuild_candidates();

            if grid.is_valid_move(min_cands.0, v) && self.solve_random(grid) {
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
