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
pub use generator::{Generator, Symmetry};
pub use grid::{Candidates, Cell, Grid, RegionType, BLOCKS, COLS, ROWS};
pub use rating::{DifficultyRating, Rater};
pub use solver::{Hint, HintType, Solver};

#[cfg(test)]
mod tests {
    use crate::{Generator, Grid, Rater, Solver, Symmetry};

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
    fn test_naked_quad_detection() {
        let puzzle =
            "000000060000030047032500000600007005207010908081004000000002000000000001005870000";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        solver.next_hint();
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
            "900062700005003000000000006700030000000009000802045009003501028040000005010000000";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_swordfish_detection() {
        let puzzle =
            "160540070008001030030800000700050069600902057000000000000030040000000016000164500";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_jellyfish_detection() {
        let puzzle =
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        solver.next_hint();
    }

    #[test]
    fn test_hidden_quad_detection() {
        let puzzle =
            "800570290390000000000200000001000508000496000000800000209000001008000070560000082";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        solver.next_hint();
    }

    #[test]
    fn test_xy_wing_detection() {
        let puzzle =
            "010000508000403000056700000000020080400000302200376001908000254000007000000000003";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_wxyz_wing_detection() {
        let puzzle =
            "010000508000403000056700000000020080400000302200376001908000254000007000000000003";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        solver.next_hint();
    }

    #[test]
    fn test_unique_rectangle_type1() {
        let puzzle =
            "000008960100700000067500300210007800004890003700004005021900004000000000000000026";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_unique_rectangle_type2() {
        let puzzle =
            "060500201100000000023900006640000000000027090052000080000000060001005900500070100";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_unique_rectangle_type3() {
        let puzzle =
            "000503470500800000000090002850000600024607590006000037200060000000008005043902000";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        solver.next_hint();
    }

    #[test]
    fn test_unique_rectangle_type4() {
        let puzzle =
            "000206803002000050060700009003090005050000020100040900500008070030000400807009000";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        solver.next_hint();
    }

    #[test]
    fn test_bug_plus_one() {
        let puzzle =
            "000000000000000107300600000900800000020000810060024000008007590007100320400596000";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let hint = solver.next_hint();
        assert!(hint.is_some(), "Should find a hint");
    }

    #[test]
    fn test_unique_solution() {
        let puzzle =
            "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let count = solver.count_solutions();
        assert_eq!(count, 1, "Should have exactly 1 solution, got {}", count);
    }

    #[test]
    fn test_multiple_solutions() {
        let puzzle =
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        assert!(
            !solver.has_unique_solution(),
            "Should have multiple solutions"
        );
    }

    #[test]
    fn test_generator_struct() {
        let gen = Generator::new();
        assert_eq!(gen.min_difficulty, 0.0);
        assert_eq!(gen.max_difficulty, 10.0);
    }

    #[test]
    fn test_generator_with_seed() {
        let gen = Generator::with_seed(42);
        assert!(gen.seed.is_some());
    }

    #[test]
    fn test_generator_with_difficulty() {
        let gen = Generator::with_difficulty(3.0, 5.0);
        assert_eq!(gen.min_difficulty, 3.0);
        assert_eq!(gen.max_difficulty, 5.0);
    }

    #[test]
    fn test_generator_no_unique_check() {
        let mut gen = Generator::new();
        gen.require_unique = false;
        gen.min_difficulty = 0.0;
        gen.max_difficulty = 10.0;

        // This should work without unique solution check
        let result = gen.generate();
        assert!(
            result.is_ok() || result.is_err(),
            "Should either succeed or fail gracefully"
        );
    }

    #[test]
    fn test_symmetry_enum() {
        let sym = Symmetry::Rotational180;
        assert_eq!(sym, Symmetry::Rotational180);
    }

    #[test]
    fn test_symmetry_get_positions() {
        let sym = Symmetry::Rotational180;
        let positions = sym.get_symmetric_positions(0);
        assert!(positions.contains(&0));
        assert!(positions.contains(&80)); // 180 degree rotation
    }

    #[test]
    fn test_generator_with_symmetry() {
        let gen = Generator::with_symmetry(Symmetry::Rotational180);
        assert_eq!(gen.symmetry, Symmetry::Rotational180);
    }
}
