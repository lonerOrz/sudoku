//! Difficulty rating based on solving technique analysis.
//!
//! ER (Experience Rating) measures puzzle difficulty by the hardest technique required.
//! EP (Entry Point) and ED (Entry Difficulty) track the initial solving step.

#[derive(Debug, Clone)]
pub struct DifficultyRating {
    pub er: f64,
    pub er_technique: String,
    pub ep: f64,
    pub ed: f64,
}

impl DifficultyRating {
    pub fn new() -> Self {
        Self {
            er: 0.0,
            er_technique: String::new(),
            ep: 0.0,
            ed: 0.0,
        }
    }
}

impl Default for DifficultyRating {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Rater<'a> {
    solver: &'a mut crate::solver::Solver,
}

impl<'a> Rater<'a> {
    pub fn new(solver: &'a mut crate::solver::Solver) -> Self {
        Self { solver }
    }

    pub fn analyse(&mut self) -> DifficultyRating {
        let mut rating = DifficultyRating::new();
        let backup = self.solver.grid();

        self.solver.rebuild_candidates();

        let mut pearl_found = false;
        let mut steps = 0;
        const MAX_STEPS: usize = 1000;

        while !self.solver.grid().is_solved() {
            steps += 1;

            if steps > MAX_STEPS {
                rating.er = 8.0;
                rating.er_technique = "Backtracking".to_string();
                break;
            }

            if let Some(hint) = self.solver.next_hint() {
                if hint.difficulty > rating.er {
                    rating.er = hint.difficulty;
                    rating.er_technique = hint.technique_name.clone();
                }

                if rating.ed == 0.0 {
                    rating.ed = hint.difficulty;
                }

                if !pearl_found {
                    rating.ep = hint.difficulty;
                    pearl_found = true;
                }

                self.solver.apply_hint(&hint);
            } else {
                rating.er = 8.0;
                rating.er_technique = "Backtracking".to_string();
                break;
            }
        }

        *self.solver.grid_mut() = backup;

        if rating.er == 0.0 && steps == 0 {
            rating.er = 0.1;
            rating.er_technique = "Trivial".to_string();
        }

        rating
    }
}
