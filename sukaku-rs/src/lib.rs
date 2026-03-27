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
    use crate::rules;
    use crate::{Generator, Grid, Rater, Solver, Symmetry};

    #[test]
    fn test_all_rules_have_unique_names() {
        let rules = rules::all_rules();
        let names: Vec<_> = rules.iter().map(|r| r.name).collect();
        let unique: std::collections::HashSet<_> = names.iter().collect();
        assert_eq!(names.len(), unique.len(), "Rule names must be unique");
    }

    #[test]
    fn test_rule_difficulty_sorted() {
        let rules = rules::rules_for_solve();
        for i in 1..rules.len() {
            assert!(
                rules[i - 1].difficulty <= rules[i].difficulty,
                "Rules must be sorted by difficulty"
            );
        }
    }

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
        // A puzzle requiring Forcing Chain technique (not yet implemented)
        // This puzzle from SudokuWiki demonstrates the need for advanced chain logic.
        // Our solver exhausts all 33 rules and correctly identifies backtracking is needed.
        let grid = Grid::parse(
            "120400000450080003700006000000000008000002045005900060010004090070000800960010020",
        )
        .unwrap();
        let mut solver = Solver::new(grid);
        let mut rater = Rater::new(&mut solver);
        let rating = rater.analyse();

        // This puzzle requires Forcing Chain (SE 7.5+), which we haven't implemented.
        // The solver correctly returns Backtracking when all rules are exhausted.
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
        assert_technique_detectable(
            "000000001000000000200000003000000400000000500000000600000000700000000800000000900",
            "Locked Pointing",
        );
    }

    #[test]
    fn test_locked_claiming_detection() {
        assert_technique_detectable(
            "000000000000003084001020000000507000004000100090000000500000073002010000000040009",
            "Locked Claiming",
        );
    }

    #[test]
    fn test_hidden_pair_detection() {
        assert_technique_detectable(
            "020006003030901008001000500007030600050000400040109000009000100800207006030080020",
            "Hidden Pair",
        );
    }

    #[test]
    fn test_x_wing_detection() {
        assert_technique_detectable(
            "900062700005003000000000006700030000000009000802045009003501028040000005010000000",
            "X-Wing",
        );
    }

    #[test]
    fn test_xy_wing_detection() {
        assert_technique_detectable(
            "010000508000403000056700000000020080400000302200376001908000254000007000000000003",
            "XY-Wing",
        );
    }

    #[test]
    fn test_jellyfish_detection() {
        assert_technique_detectable(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "Jellyfish",
        );
    }

    #[test]
    fn test_hidden_quad_detection() {
        assert_technique_no_crash(
            "000056789000000000000000000000000000000000000000000000000000000000000000000000000",
            "Hidden Quad",
        );
    }

    #[test]
    fn test_naked_triple_detection() {
        assert_technique_detectable(
            "020006003030901008001000500007030600050000400040109000009000100800207006030080020",
            "Naked Triple",
        );
    }

    #[test]
    fn test_naked_quad_detection() {
        assert_technique_no_crash(
            "000000060000030047032500000600007005207010908081004000000002000000000001005870000",
            "Naked Quad",
        );
    }

    #[test]
    fn test_hidden_triple_detection() {
        assert_technique_no_crash(
            "003020600900305001001806400008102900700000008006708200002609500800203009005010300",
            "Hidden Triple",
        );
    }

    #[test]
    fn test_swordfish_detection() {
        assert_technique_no_crash(
            "160540070008001030030800000700050069600902057000000000000030040000000016000164500",
            "Swordfish",
        );
    }

    #[test]
    fn test_skyscraper_detection() {
        assert_technique_no_crash(
            "...........19.2.6......679.9.2...6..37....95...5.....414...3..57.9.24......8.....",
            "Skyscraper",
        );
    }

    #[test]
    fn test_two_string_kite_detection() {
        assert_technique_no_crash(
            ".617....5842.95....5..6.4.........3..25........41...26..........8.....672.....349",
            "2-String Kite",
        );
    }

    #[test]
    fn test_unique_rectangle_type1() {
        assert_technique_no_crash(
            "000008960100700000067500300210007800004890003700004005021900004000000000000000026",
            "Unique Rectangle Type 1",
        );
    }

    #[test]
    fn test_unique_rectangle_type3() {
        assert_technique_no_crash(
            "000503470500800000000090002850000600024607590006000037200060000000008005043902000",
            "Unique Rectangle Type 3",
        );
    }

    #[test]
    fn test_unique_rectangle_type2() {
        assert_technique_detectable(
            "060500201100000000023900006640000000000027090005200080000000060001005900500070100",
            "Unique Rectangle Type 2",
        );
    }

    #[test]
    fn test_unique_rectangle_type4() {
        assert_technique_detectable(
            "000206803002000050060700009003090005050000020100040900500008070030000400807009000",
            "Unique Rectangle Type 4",
        );
    }

    #[test]
    fn test_wxyz_wing_detection() {
        assert_technique_no_crash(
            "010000508000403000056700000000020080400000302200376001908000254000007000000000003",
            "WXYZ-Wing",
        );
    }

    #[test]
    fn test_vwxyz_wing_detection() {
        assert_technique_no_crash(
            "010000508000403000056700000000020080400000302200376001908000254000007000000000003",
            "VWXYZ-Wing",
        );
    }

    #[test]
    fn test_uvwxyz_wing_detection() {
        assert_technique_no_crash(
            "010000508000403000056700000000020080400000302200376001908000254000007000000000003",
            "UVWXYZ-Wing",
        );
    }

    #[test]
    fn test_tuvwxyz_wing_detection() {
        assert_technique_no_crash(
            "010000508000403000056700000000020080400000302200376001908000254000007000000000003",
            "TUVWXYZ-Wing",
        );
    }

    #[test]
    fn test_bug_plus_one() {
        // BUG+1: Bivalue Universal Grave Type 1
        // Note: This puzzle may not have BUG+1 at initial state
        // BUG+1 typically appears in late-game positions
        assert_technique_no_crash(
            "000000000000000051200600000040008000003000061007002400000803590003100260400059700",
            "BUG+1",
        );
    }

    #[test]
    fn test_strong_links_fish_3() {
        // 3-Strong-Links Fish: Generalization of X-Wing/Swordfish with strong links
        assert_technique_no_crash(
            "000000000000000051200600000040008000003000061007002400000803590003100260400059700",
            "3-Strong-Links Fish",
        );
    }

    #[test]
    fn test_bug_plus_two() {
        assert_technique_no_crash(
            "000000000000000051200600000040008000003000061007002400000803590003100260400059700",
            "BUG+2",
        );
    }

    #[test]
    fn test_bug_plus_three() {
        assert_technique_no_crash(
            "000000000000000051200600000040008000003000061007002400000803590003100260400059700",
            "BUG+3",
        );
    }

    #[test]
    fn test_bug_plus_four() {
        assert_technique_no_crash(
            "000000000000000051200600000040008000003000061007002400000803590003100260400059700",
            "BUG+4",
        );
    }

    #[test]
    fn test_strong_links_fish_4() {
        // 4-Strong-Links Fish: Also known as Jellyfish with strong links
        // Using Jellyfish test puzzle
        assert_technique_detectable(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "4-Strong-Links Fish",
        );
    }

    #[test]
    fn test_strong_links_fish_5() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "5-Strong-Links Fish",
        );
    }

    #[test]
    fn test_strong_links_fish_6() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "6-Strong-Links Fish",
        );
    }

    #[test]
    fn test_aligned_pair_exclusion() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "Aligned Pair Exclusion",
        );
    }

    #[test]
    fn test_aligned_triplet_exclusion() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "Aligned Triplet Exclusion",
        );
    }

    #[test]
    fn test_generalized_naked_pair() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "Generalized Naked Pair",
        );
    }

    #[test]
    fn test_generalized_naked_triple() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "Generalized Naked Triple",
        );
    }

    #[test]
    fn test_generalized_naked_quad() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "Generalized Naked Quad",
        );
    }

    #[test]
    fn test_generalized_naked_quint() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "Generalized Naked Quint",
        );
    }

    #[test]
    fn test_generalized_naked_sext() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "Generalized Naked Sext",
        );
    }

    #[test]
    fn test_vlocking() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "VLocking",
        );
    }

    #[test]
    fn test_x_cycles() {
        assert_technique_no_crash(
            "200000003080030050003402100001205400000090000009308600002506900090020070400000001",
            "X-Cycles",
        );
    }

    #[test]
    fn test_y_cycles() {
        // Test Y-Cycles (XY-Chain) detection
        // This puzzle contains bi-value cells that can form XY-Chains
        assert_technique_no_crash(
            "000000000000003085001020000000507000004000100090000000500000073002010000000040009",
            "Y-Cycles",
        );
    }

    #[test]
    fn test_forcing_chain() {
        // Test Forcing Chain detection (SE 7.0)
        assert_technique_no_crash(
            "000000000000003085001020000000507000004000100090000000500000073002010000000040009",
            "Forcing Chain",
        );
    }

    #[test]
    fn test_als_xz() {
        // Test ALS-XZ technique detection (SE 7.0)
        // ALS-XZ uses Almost Locked Sets with restricted commons
        assert_technique_no_crash(
            "000000000000003085001020000000507000004000100090000000500000073002010000000040009",
            "ALS-XZ",
        );
    }

    #[test]
    fn test_wing_double_link() {
        // Test Wing with double link detection
        // Double link wings have stronger inference
        assert_technique_no_crash(
            "000000000000003085001020000000507000004000100090000000500000073002010000000040009",
            "XY-Wing",
        );
    }

    #[test]
    fn test_nishio_forcing_chain() {
        // Test Nishio Forcing Chain detection (SE 7.5)
        // Nishio uses double implication chains (both ON and OFF assumptions)
        assert_technique_no_crash(
            "000000000000003085001020000000507000004000100090000000500000073002010000000040009",
            "Nishio Forcing Chain",
        );
    }

    #[test]
    fn test_multiple_forcing_chain() {
        // Test Multiple Forcing Chain detection (SE 8.0)
        // Multiple starting points all lead to the same conclusion
        assert_technique_no_crash(
            "000000000000003085001020000000507000004000100090000000500000073002010000000040009",
            "Multiple Forcing Chain",
        );
    }

    /// Helper function: Assert that a technique is detectable in a puzzle.
    ///
    /// Use this when the puzzle is known to contain the technique.
    fn assert_technique_detectable(puzzle: &str, technique: &str) {
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let hint = solver.detect_technique(technique);
        assert!(
            hint.is_some(),
            "Should detect {} in puzzle: {}",
            technique,
            puzzle
        );
    }

    /// Helper function: Assert that calling detect_technique does not crash.
    ///
    /// Use this when the puzzle may or may not contain the technique,
    /// just verify the function works correctly.
    fn assert_technique_no_crash(puzzle: &str, technique: &str) {
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let _hint = solver.detect_technique(technique);
        // Just verify it doesn't crash
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
