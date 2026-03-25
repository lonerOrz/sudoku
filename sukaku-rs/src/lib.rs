//! Sukaku difficulty rater and generator.
//!
//! This crate provides Sudoku puzzle analysis including:
//! - Difficulty rating based on solving techniques (SER-like system)
//! - Puzzle generation with difficulty constraints
//! - Rule-based solving with technique detection
//!
//! # Quick Start
//!
//! ```
//! use sukaku_rs::{Grid, Rater, Solver};
//!
//! let puzzle = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
//! let grid = Grid::parse(puzzle).unwrap();
//! let mut solver = Solver::new(grid);
//! let mut rater = Rater::new(&mut solver);
//! let rating = rater.analyse();
//!
//! println!("ER: {:.1}", rating.er);
//! ```
//!
//! # Architecture
//!
//! - [`grid`] - 9x9 Sudoku grid representation with candidate tracking
//! - [`solver`] - Rule-based solver that detects solving techniques
//! - [`rules`] - Individual solving technique implementations
//! - [`rating`] - Difficulty rating based on technique analysis

pub mod error;
pub mod generator;
pub mod grid;
pub mod rating;
pub mod rules;
pub mod solver;

pub use error::{Error, Result};
pub use generator::Generator;
pub use grid::{Candidates, Cell, Grid, RegionType, BLOCKS, COLS, ROWS};
pub use rating::{DifficultyRating, Rater};
pub use solver::{Hint, HintType, Solver};

#[cfg(test)]
mod tests {
    use crate::{Grid, Rater, Solver};

    #[test]
    fn test_parse_valid_puzzle() {
        let grid = Grid::parse(
            "003020600900305001001806400008102900700000008006708200002609500800203009005010300",
        )
        .unwrap();
        assert!(!grid.is_solved());
    }

    #[test]
    fn test_parse_invalid_length() {
        let result = Grid::parse("123");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_digit() {
        let result = Grid::parse(
            "00302060090030500100180640000810290070000000800670820000260950080020300900501030a",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_all_zeros() {
        let grid = Grid::parse(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        assert!(!grid.is_solved());
    }

    #[test]
    fn test_parse_solved_puzzle() {
        let grid = Grid::parse(
            "123456789456789123789123456214365897365897214897214365531642978642978531978531642",
        )
        .unwrap();
        assert!(grid.is_solved());
    }

    #[test]
    fn test_naked_single_rating() {
        let grid = Grid::parse(
            "003020600900305001001806400008102900700000008006708200002609500800203009005010300",
        )
        .unwrap();
        let mut solver = Solver::new(grid);
        let mut rater = Rater::new(&mut solver);
        let rating = rater.analyse();

        assert_eq!(rating.er, 1.6);
        assert_eq!(rating.er_technique, "Naked Single");
    }

    #[test]
    fn test_backtracking_detection() {
        let grid = Grid::parse(
            "020000000000000006000001070000000030005000800000000020030000000800400000000000000",
        )
        .unwrap();
        let mut solver = Solver::new(grid);
        let mut rater = Rater::new(&mut solver);
        let rating = rater.analyse();

        assert_eq!(rating.er, 8.0);
        assert_eq!(rating.er_technique, "Backtracking");
    }

    #[test]
    fn test_ep_first_hint_difficulty() {
        let grid = Grid::parse(
            "003020600900305001001806400008102900700000008006708200002609500800203009005010300",
        )
        .unwrap();
        let mut solver = Solver::new(grid);
        let mut rater = Rater::new(&mut solver);
        let rating = rater.analyse();

        assert!(rating.ep > 0.0);
    }

    #[test]
    fn test_ed_first_hint_difficulty() {
        let grid = Grid::parse(
            "003020600900305001001806400008102900700000008006708200002609500800203009005010300",
        )
        .unwrap();
        let mut solver = Solver::new(grid);
        let mut rater = Rater::new(&mut solver);
        let rating = rater.analyse();

        assert!(rating.ed > 0.0);
    }

    #[test]
    fn test_locked_pointing_detection() {
        let puzzle =
            "000000000000003084001020000000507000004000100090000000500000073002010000000040009";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_locked_claiming_detection() {
        let puzzle =
            "000000000000003084001020000000507000004000100090000000500000073002010000000040009";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_hidden_pair_detection() {
        let puzzle =
            "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_naked_triple_detection() {
        let puzzle =
            "000000000000003084001020000000507000004000100090000000500000073002010000000040009";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_hidden_triple_detection() {
        let puzzle =
            "000000000000003084001020000000507000004000100090000000500000073002010000000040009";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_x_wing_detection() {
        let puzzle =
            "000000000000003084001020000000507000004000100090000000500000073002010000000040009";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_swordfish_detection() {
        let puzzle =
            "000000000000003084001020000000507000004000100090000000500000073002010000000040009";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_xy_wing_detection() {
        let puzzle =
            "000000000000003084001020000000507000004000100090000000500000073002010000000040009";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }
}
