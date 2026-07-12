//! Difficulty rating based on solving technique analysis.
//!
//! ER (Experience Rating) measures puzzle difficulty by the hardest technique required.
//! EP (Entry Point) and ED (Entry Difficulty) track the initial solving step.

/// Difficulty rating for a Sudoku puzzle.
///
/// - `er` (Experience Rating): hardest technique required (SER-like scale 1.0-11.0)
/// - `er_technique`: name of the hardest technique
/// - `ep` (Entry Point): difficulty of the first technique
/// - `ed` (Entry Difficulty): numeric difficulty of the first technique
#[derive(Debug, Clone, PartialEq)]
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

    pub fn with_revised_scale(&mut self) -> &mut Self {
        if self.er >= 1.0 && self.er <= 10.0 {
            if self.er <= 2.0 {
                self.er = 1.0;
            } else if self.er <= 3.0 {
                self.er = 2.0;
            } else if self.er <= 4.0 {
                self.er = 3.0;
            } else if self.er <= 5.0 {
                self.er = 4.0;
            } else if self.er <= 6.0 {
                self.er = 5.0;
            } else if self.er <= 7.0 {
                self.er = 6.0;
            } else if self.er <= 8.0 {
                self.er = 7.0;
            } else if self.er <= 9.0 {
                self.er = 8.0;
            } else {
                self.er = 9.0;
            }
        }
        self
    }

    pub fn er_technique_short(&self) -> &str {
        match self.er_technique.as_str() {
            "Naked Single" => "NS",
            "Hidden Single" => "HS",
            "Naked Pair" => "NP",
            "Hidden Pair" => "HP",
            "Naked Triple" => "NT",
            "Hidden Triple" => "HT",
            "Naked Quad" => "NQ",
            "Hidden Quad" => "HQ",
            "X-Wing" => "XW",
            "Swordfish" => "SF",
            "Jellyfish" => "JF",
            "XY-Wing" => "XY",
            "XYZ-Wing" => "XYZ",
            "WXYZ-Wing" => "WXYZ",
            "VWXYZ-Wing" => "VWXYZ",
            "UVWXYZ-Wing" => "UVWXYZ",
            "TUVWXYZ-Wing" => "TUVWXYZ",
            "Unique Rectangle Type 1" => "UR1",
            "Unique Rectangle Type 2" => "UR2",
            "Unique Rectangle Type 3" => "UR3",
            "Unique Rectangle Type 4" => "UR4",
            "BUG+1" => "BUG1",
            "BUG+2" => "BUG2",
            "BUG+3" => "BUG3",
            "BUG+4" => "BUG4",
            "X-Cycles" => "XC",
            "Y-Cycles" => "YC",
            "Forcing Chain" => "FC",
            "Nishio" => "NI",
            "Nishio Forcing Chain" => "NFC",
            "Multiple Forcing Chain" => "MFC",
            "Dynamic Forcing Chain" => "DFC",
            "Dynamic Forcing Chain+" => "DFC+",
            "Skyscraper" => "SS",
            "2-String Kite" => "2SK",
            "3-Strong-Links Fish" => "3SL",
            "4-Strong-Links Fish" => "4SL",
            "5-Strong-Links Fish" => "5SL",
            "6-Strong-Links Fish" => "6SL",
            "Aligned Pair Exclusion" => "APE",
            "Aligned Triplet Exclusion" => "ATE",
            "Generalized Naked Pair" => "GNP",
            "Generalized Naked Triple" => "GNT",
            "Generalized Naked Quad" => "GNQ",
            "Generalized Naked Quint" => "GNQ5",
            "Generalized Naked Sext" => "GNS",
            "VLocking" => "VL",
            "Backtracking" => "BT",
            _ => &self.er_technique,
        }
    }

    pub fn override_technique_difficulty(&mut self, technique: &str, difficulty: f64) -> &mut Self {
        if self.er_technique == technique {
            self.er = difficulty;
        }
        self
    }

    pub fn technique_name_localized(&self, lang: &str) -> String {
        match lang {
            "zh" => match self.er_technique.as_str() {
                "Naked Single" => "裸单".to_string(),
                "Hidden Single" => "隐单".to_string(),
                "Naked Pair" => "裸对".to_string(),
                "Hidden Pair" => "隐对".to_string(),
                "Naked Triple" => "裸三".to_string(),
                "Hidden Triple" => "隐三".to_string(),
                "Naked Quad" => "裸四".to_string(),
                "Hidden Quad" => "隐四".to_string(),
                "X-Wing" => "X翼".to_string(),
                "Swordfish" => "剑鱼".to_string(),
                "Jellyfish" => "水母".to_string(),
                "XY-Wing" => "XY翼".to_string(),
                "Backtracking" => "回溯".to_string(),
                _ => self.er_technique.clone(),
            },
            "ja" => match self.er_technique.as_str() {
                "Naked Single" => "ネイキッドシングル".to_string(),
                "Hidden Single" => "ヒドゥンシングル".to_string(),
                "X-Wing" => "Xウィング".to_string(),
                "Backtracking" => "バックトラッキング".to_string(),
                _ => self.er_technique.clone(),
            },
            _ => self.er_technique.clone(),
        }
    }
}

impl Default for DifficultyRating {
    fn default() -> Self {
        Self::new()
    }
}

/// Rates Sudoku puzzle difficulty by running the solver and analyzing techniques used.
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
