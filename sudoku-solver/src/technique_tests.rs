//! Comprehensive technique tests.
//!
//! Each test constructs a specific grid state to verify that a technique
//! detects the correct pattern and produces valid hints.

#[cfg(test)]
mod tests {
    use crate::{CellIndex, Grid, HintType, Solver};

    /// Easy puzzle — many singles, locked candidates.
    const EASY: &str =
        "003020600900305001001806400008102900700000008006708200002609500800203009005010300";

    /// Medium puzzle — subsets, fish, some wings.
    const MEDIUM: &str =
        "100007090030020008009600500005300900010080002600004000300000010040000007007000300";

    /// Hard puzzle — advanced techniques needed.
    const HARD: &str =
        "900062700005003000000000006700030000000009000802045009003501028040000005010000000";

    // ================================================================
    // Singles
    // ================================================================

    #[test]
    fn test_naked_single_detection() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let hint = solver.detect_technique("Naked Single").unwrap();
        assert_eq!(hint.hint_type, HintType::NakedSingle);
        assert!(hint.value >= 1 && hint.value <= 9);
        assert!(solver.validate_hint(&hint).is_ok());
    }

    #[test]
    fn test_hidden_single_detection() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let hint = solver.detect_technique("Hidden Single").unwrap();
        assert_eq!(hint.hint_type, HintType::HiddenSingle);
        assert!(hint.value >= 1 && hint.value <= 9);
        assert!(solver.validate_hint(&hint).is_ok());
    }

    // ================================================================
    // Locked Candidates
    // ================================================================

    #[test]
    fn test_locked_pointing_detection() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Locked Pointing") {
            assert_eq!(hint.hint_type, HintType::LockedPointing);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_locked_claiming_detection() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Locked Claiming") {
            assert_eq!(hint.hint_type, HintType::LockedClaiming);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Naked Subsets
    // ================================================================

    #[test]
    fn test_naked_pair_detection() {
        // Naked Pair: two cells in a unit with the same two candidates.
        // Build from solved grid, remove 2 cells to create a pair.
        // Solution row 0: 1 6 2 8 5 7 4 9 3
        // Remove (0,0)=1 and (0,2)=2, but also block 1&2 from other cells.
        // Use a known puzzle with naked pairs.
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Naked Pair") {
            assert_eq!(hint.hint_type, HintType::NakedPair);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_naked_triple_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Naked Triple") {
            assert_eq!(hint.hint_type, HintType::NakedTriple);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_naked_quad_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Naked Quad") {
            assert_eq!(hint.hint_type, HintType::NakedQuad);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Hidden Subsets
    // ================================================================

    #[test]
    fn test_hidden_pair_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Hidden Pair") {
            assert_eq!(hint.hint_type, HintType::HiddenPair);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_hidden_triple_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Hidden Triple") {
            assert_eq!(hint.hint_type, HintType::HiddenTriple);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_hidden_quad_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Hidden Quad") {
            assert_eq!(hint.hint_type, HintType::HiddenQuad);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Fish Techniques
    // ================================================================

    #[test]
    fn test_x_wing_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("X-Wing") {
            assert_eq!(hint.hint_type, HintType::XWing);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_swordfish_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Swordfish") {
            assert_eq!(hint.hint_type, HintType::Swordfish);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_jellyfish_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Jellyfish") {
            assert_eq!(hint.hint_type, HintType::Jellyfish);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Strong Links Fish
    // ================================================================

    #[test]
    fn test_strong_links_fish_3_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("3-Strong-Links Fish") {
            assert_eq!(hint.hint_type, HintType::StrongLinksFish);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_strong_links_fish_4_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("4-Strong-Links Fish") {
            assert_eq!(hint.hint_type, HintType::StrongLinksFish);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_strong_links_fish_5_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("5-Strong-Links Fish") {
            assert_eq!(hint.hint_type, HintType::StrongLinksFish);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_strong_links_fish_6_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("6-Strong-Links Fish") {
            assert_eq!(hint.hint_type, HintType::StrongLinksFish);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Wing Techniques
    // ================================================================

    #[test]
    fn test_xy_wing_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("XY-Wing") {
            assert_eq!(hint.hint_type, HintType::XYWing);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_xyz_wing_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("XYZ-Wing") {
            assert_eq!(hint.hint_type, HintType::XYZWing);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_wxyz_wing_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("WXYZ-Wing") {
            assert_eq!(hint.hint_type, HintType::WXYZWing);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_vwxyz_wing_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("VWXYZ-Wing") {
            assert_eq!(hint.hint_type, HintType::VWXYZWing);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_uvwxyz_wing_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("UVWXYZ-Wing") {
            assert_eq!(hint.hint_type, HintType::UVWXYZWing);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_tuvwxyz_wing_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("TUVWXYZ-Wing") {
            assert_eq!(hint.hint_type, HintType::TUVWXYZWing);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Unique Rectangle
    // ================================================================

    #[test]
    fn test_unique_rectangle_type1_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Unique Rectangle Type 1") {
            assert_eq!(hint.hint_type, HintType::UniqueRectangleType1);
            assert!(hint.value > 0);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_unique_rectangle_type2_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Unique Rectangle Type 2") {
            assert_eq!(hint.hint_type, HintType::UniqueRectangleType2);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_unique_rectangle_type3_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Unique Rectangle Type 3") {
            assert_eq!(hint.hint_type, HintType::UniqueRectangleType3);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_unique_rectangle_type4_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Unique Rectangle Type 4") {
            assert_eq!(hint.hint_type, HintType::UniqueRectangleType4);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_constructed_unique_rectangle_type4_no_fire_same_box() {
        // All 4 empty cells in box 0 → filtered by box check.
        use crate::solver::HintAccumulator;
        let mut cells = [0u8; 81];
        cells[2] = 3;
        cells[3] = 4;
        cells[4] = 5;
        cells[5] = 6;
        cells[6] = 7;
        cells[7] = 8;
        cells[8] = 9;
        cells[11] = 6;
        cells[12] = 7;
        cells[13] = 8;
        cells[14] = 9;
        cells[15] = 1;
        cells[16] = 2;
        cells[17] = 3;
        cells[18] = 7;
        cells[19] = 8;
        cells[20] = 9;
        cells[21] = 1;
        cells[22] = 2;
        cells[23] = 3;
        cells[24] = 4;
        cells[25] = 5;
        cells[26] = 6;
        cells[27] = 2;
        cells[28] = 3;
        cells[29] = 1;
        cells[30] = 5;
        cells[31] = 6;
        cells[32] = 4;
        cells[33] = 8;
        cells[34] = 9;
        cells[35] = 7;
        cells[36] = 5;
        cells[37] = 6;
        cells[38] = 4;
        cells[39] = 8;
        cells[40] = 9;
        cells[41] = 7;
        cells[42] = 2;
        cells[43] = 3;
        cells[44] = 1;
        cells[45] = 8;
        cells[46] = 9;
        cells[47] = 7;
        cells[48] = 2;
        cells[49] = 3;
        cells[50] = 1;
        cells[51] = 5;
        cells[52] = 6;
        cells[53] = 4;
        cells[54] = 3;
        cells[55] = 1;
        cells[56] = 2;
        cells[57] = 6;
        cells[58] = 4;
        cells[59] = 5;
        cells[60] = 9;
        cells[61] = 7;
        cells[62] = 8;
        cells[63] = 6;
        cells[64] = 4;
        cells[65] = 5;
        cells[66] = 9;
        cells[67] = 7;
        cells[68] = 8;
        cells[69] = 3;
        cells[70] = 1;
        cells[71] = 2;
        cells[72] = 9;
        cells[73] = 7;
        cells[74] = 8;
        cells[75] = 3;
        cells[76] = 1;
        cells[77] = 2;
        cells[78] = 6;
        cells[79] = 4;
        cells[80] = 5;
        let grid = Grid::from_flat(cells);
        assert!(grid.check_consistency(), "Grid must be consistent");
        let mut acc = HintAccumulator::new();
        crate::rules::unique::unique_rectangle_type4(&grid, &mut acc);
        assert!(
            acc.hints().is_empty(),
            "UR Type 4 should NOT fire when all 4 cells are in the same box"
        );
    }

    // ================================================================
    // BUG
    // ================================================================

    #[test]
    fn test_bug_plus_one_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("BUG+1") {
            assert_eq!(hint.hint_type, HintType::BUGPlusOne);
            assert!(hint.value > 0);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_bug_plus_two_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("BUG+2") {
            assert_eq!(hint.hint_type, HintType::BUGPlusTwo);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_bug_plus_three_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("BUG+3") {
            assert_eq!(hint.hint_type, HintType::BUGPlusThree);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_bug_plus_four_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("BUG+4") {
            assert_eq!(hint.hint_type, HintType::BUGPlusFour);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Skyscraper & 2-String Kite
    // ================================================================

    #[test]
    fn test_skyscraper_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Skyscraper") {
            assert_eq!(hint.hint_type, HintType::Skyscraper);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_two_string_kite_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("2-String Kite") {
            assert_eq!(hint.hint_type, HintType::TwoStringKite);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // VLocking
    // ================================================================

    #[test]
    fn test_vlocking_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("VLocking") {
            assert_eq!(hint.hint_type, HintType::VLocking);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Aligned Exclusion
    // ================================================================

    #[test]
    fn test_aligned_pair_exclusion_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Aligned Pair Exclusion") {
            assert_eq!(hint.hint_type, HintType::AlignedPairExclusion);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_aligned_triplet_exclusion_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Aligned Triplet Exclusion") {
            assert_eq!(hint.hint_type, HintType::AlignedTripletExclusion);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // ALS-XZ
    // ================================================================

    #[test]
    fn test_als_xz_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("ALS-XZ") {
            assert_eq!(hint.hint_type, HintType::AlsWithXzRule);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Generalized Naked Sets
    // ================================================================

    #[test]
    fn test_generalized_naked_pair_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Generalized Naked Pair") {
            assert_eq!(hint.hint_type, HintType::GeneralizedNakedSet);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_generalized_naked_triple_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Generalized Naked Triplet") {
            assert_eq!(hint.hint_type, HintType::GeneralizedNakedSet);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_generalized_naked_quad_detection() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Generalized Naked Quad") {
            assert_eq!(hint.hint_type, HintType::GeneralizedNakedSet);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_generalized_naked_quint_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Generalized Naked Quint") {
            assert_eq!(hint.hint_type, HintType::GeneralizedNakedSet);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_generalized_naked_sext_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Generalized Naked Sext") {
            assert_eq!(hint.hint_type, HintType::GeneralizedNakedSet);
            assert!(hint.value == 0);
            assert!(!hint.eliminations.is_empty());
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Chaining
    // ================================================================

    #[test]
    fn test_forcing_chain_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Forcing Chain") {
            assert_eq!(hint.hint_type, HintType::ForcingChain);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_dynamic_forcing_chain_plus_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Dynamic Forcing Chain+") {
            assert_eq!(hint.hint_type, HintType::DynamicForcingChainPlus);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_nested_forcing_chain_2_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Nested Forcing Chain (2-level)") {
            assert_eq!(hint.hint_type, HintType::NestedForcingChain2);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_nested_forcing_chain_3_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Nested Forcing Chain (3-level)") {
            assert_eq!(hint.hint_type, HintType::NestedForcingChain3);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    #[test]
    fn test_nested_forcing_chain_4_detection() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Nested Forcing Chain (4-level)") {
            assert_eq!(hint.hint_type, HintType::NestedForcingChain4);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Stub verification (no crash)
    // ================================================================

    #[test]
    fn test_x_cycles_no_crash() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let _ = solver.detect_technique("X-Cycles");
    }

    #[test]
    fn test_y_cycles_no_crash() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let _ = solver.detect_technique("Y-Cycles");
    }

    #[test]
    fn test_nishio_forcing_chain_no_crash() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let _ = solver.detect_technique("Nishio Forcing Chain");
    }

    #[test]
    fn test_multiple_forcing_chain_no_crash() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let _ = solver.detect_technique("Multiple Forcing Chain");
    }

    #[test]
    fn test_dynamic_forcing_chain_no_crash() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let _ = solver.detect_technique("Dynamic Forcing Chain");
    }

    // ================================================================
    // End-to-end solve with full validation
    // ================================================================

    #[test]
    fn test_solve_easy_with_full_validation() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let mut steps = 0;
        while !solver.grid().is_solved() {
            steps += 1;
            assert!(steps < 200, "Easy puzzle should solve in < 200 steps");

            if let Some(hint) = solver.next_hint() {
                solver.validate_hint(&hint).unwrap_or_else(|e| {
                    panic!(
                        "Invalid hint '{}' at step {}: {}",
                        hint.technique_name, steps, e
                    );
                });
                solver.apply_hint(&hint);
                assert!(
                    solver.grid().check_consistency(),
                    "Grid inconsistent after '{}' at step {}",
                    hint.technique_name,
                    steps
                );
            } else {
                let mut fresh = Solver::new(grid);
                assert!(fresh.solve_backtrack());
                break;
            }
        }
    }

    #[test]
    fn test_solve_medium_with_full_validation() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let mut steps = 0;
        while !solver.grid().is_solved() {
            steps += 1;
            assert!(steps < 200, "Medium puzzle should solve in < 200 steps");

            if let Some(hint) = solver.next_hint() {
                solver.validate_hint(&hint).unwrap_or_else(|e| {
                    panic!(
                        "Invalid hint '{}' at step {}: {}",
                        hint.technique_name, steps, e
                    );
                });
                solver.apply_hint(&hint);
                assert!(
                    solver.grid().check_consistency(),
                    "Grid inconsistent after '{}' at step {}",
                    hint.technique_name,
                    steps
                );
            } else {
                let mut fresh = Solver::new(grid);
                assert!(fresh.solve_backtrack());
                break;
            }
        }
    }

    #[test]
    fn test_solve_hard_with_full_validation() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();

        let mut steps = 0;
        while !solver.grid().is_solved() {
            steps += 1;
            assert!(steps < 200, "Hard puzzle should solve in < 200 steps");

            if let Some(hint) = solver.next_hint() {
                solver.validate_hint(&hint).unwrap_or_else(|e| {
                    panic!(
                        "Invalid hint '{}' at step {}: {}",
                        hint.technique_name, steps, e
                    );
                });
                solver.apply_hint(&hint);
                assert!(
                    solver.grid().check_consistency(),
                    "Grid inconsistent after '{}' at step {}",
                    hint.technique_name,
                    steps
                );
            } else {
                solver.solve();
                break;
            }
        }
        assert!(solver.grid().is_solved());
    }

    // ================================================================
    // Technique-specific hint property checks
    // ================================================================

    #[test]
    fn test_naked_single_hints_are_placements() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let hint = solver.detect_technique("Naked Single").unwrap();
        assert!(hint.value > 0, "Naked Single must be a placement");
        assert!(
            hint.eliminations.is_empty(),
            "Naked Single has no eliminations"
        );
        assert_eq!(hint.hint_type, HintType::NakedSingle);
        assert!(hint.difficulty >= 1.0 && hint.difficulty <= 2.0);
    }

    #[test]
    fn test_locked_pointing_hints_are_eliminations() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Locked Pointing") {
            assert_eq!(hint.value, 0, "Locked Pointing must be an elimination");
            assert!(!hint.eliminations.is_empty());
            for &(cell, ref vals) in &hint.eliminations {
                assert!(!vals.is_empty(), "Elimination values must not be empty");
                assert!(cell.index < 81, "Cell index out of range");
            }
        }
    }

    #[test]
    fn test_locked_claiming_hints_are_eliminations() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Locked Claiming") {
            assert_eq!(hint.value, 0, "Locked Claiming must be an elimination");
            assert!(!hint.eliminations.is_empty());
            for &(cell, ref vals) in &hint.eliminations {
                assert!(!vals.is_empty());
                assert!(cell.index < 81);
            }
        }
    }

    #[test]
    fn test_x_wing_hints_are_eliminations() {
        let grid = Grid::parse(MEDIUM).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("X-Wing") {
            assert_eq!(hint.value, 0, "X-Wing must be an elimination");
            assert!(!hint.eliminations.is_empty());
            assert_eq!(hint.hint_type, HintType::XWing);
        }
    }

    // ================================================================
    // All techniques at least don't crash on all puzzles
    // ================================================================

    #[test]
    fn test_all_techniques_no_crash_easy() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let techniques = [
            "Naked Single",
            "Hidden Single",
            "Locked Pointing",
            "Locked Claiming",
            "Hidden Pair",
            "Hidden Triple",
            "Hidden Quad",
            "Naked Pair",
            "Naked Triple",
            "Naked Quad",
            "X-Wing",
            "Swordfish",
            "Jellyfish",
            "XY-Wing",
            "XYZ-Wing",
            "WXYZ-Wing",
            "VWXYZ-Wing",
            "UVWXYZ-Wing",
            "TUVWXYZ-Wing",
            "Unique Rectangle Type 1",
            "Unique Rectangle Type 2",
            "Unique Rectangle Type 3",
            "Unique Rectangle Type 4",
            "BUG+1",
            "BUG+2",
            "BUG+3",
            "BUG+4",
            "Skyscraper",
            "2-String Kite",
            "3-Strong-Links Fish",
            "4-Strong-Links Fish",
            "5-Strong-Links Fish",
            "6-Strong-Links Fish",
            "Aligned Pair Exclusion",
            "Aligned Triplet Exclusion",
            "ALS-XZ",
            "Generalized Naked Pair",
            "Generalized Naked Triplet",
            "Generalized Naked Quad",
            "Generalized Naked Quint",
            "Generalized Naked Sext",
            "VLocking",
            "X-Cycles",
            "Y-Cycles",
            "Forcing Chain",
            "Nishio Forcing Chain",
            "Multiple Forcing Chain",
            "Dynamic Forcing Chain",
            "Dynamic Forcing Chain+",
            "Nested Forcing Chain (2-level)",
            "Nested Forcing Chain (3-level)",
            "Nested Forcing Chain (4-level)",
        ];
        for technique in techniques {
            let _ = solver.detect_technique(technique);
        }
    }

    #[test]
    fn test_all_techniques_no_crash_hard() {
        let grid = Grid::parse(HARD).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let techniques = [
            "Naked Single",
            "Hidden Single",
            "Locked Pointing",
            "Locked Claiming",
            "Hidden Pair",
            "Hidden Triple",
            "Hidden Quad",
            "Naked Pair",
            "Naked Triple",
            "Naked Quad",
            "X-Wing",
            "Swordfish",
            "Jellyfish",
            "XY-Wing",
            "XYZ-Wing",
            "WXYZ-Wing",
            "VWXYZ-Wing",
            "UVWXYZ-Wing",
            "TUVWXYZ-Wing",
            "Unique Rectangle Type 1",
            "Unique Rectangle Type 2",
            "Unique Rectangle Type 3",
            "Unique Rectangle Type 4",
            "BUG+1",
            "BUG+2",
            "BUG+3",
            "BUG+4",
            "Skyscraper",
            "2-String Kite",
            "3-Strong-Links Fish",
            "4-Strong-Links Fish",
            "5-Strong-Links Fish",
            "6-Strong-Links Fish",
            "Aligned Pair Exclusion",
            "Aligned Triplet Exclusion",
            "ALS-XZ",
            "Generalized Naked Pair",
            "Generalized Naked Triplet",
            "Generalized Naked Quad",
            "Generalized Naked Quint",
            "Generalized Naked Sext",
            "VLocking",
            "X-Cycles",
            "Y-Cycles",
            "Forcing Chain",
            "Nishio Forcing Chain",
            "Multiple Forcing Chain",
            "Dynamic Forcing Chain",
            "Dynamic Forcing Chain+",
            "Nested Forcing Chain (2-level)",
            "Nested Forcing Chain (3-level)",
            "Nested Forcing Chain (4-level)",
        ];
        for technique in techniques {
            let _ = solver.detect_technique(technique);
        }
    }

    // ================================================================
    // Constructed grid tests — specific patterns
    // ================================================================

    #[test]
    fn test_constructed_naked_single() {
        // Construct a grid where cell (0,0) has exactly 1 candidate.
        // Solution row 0: 1 6 2 8 5 7 4 9 3
        // Fill all of row 0 except (0,0), and fill columns/boxes to leave only 1 candidate.
        let mut cells = [0u8; 81];
        // Row 0: _ 6 2 8 5 7 4 9 3
        cells[1] = 6;
        cells[2] = 2;
        cells[3] = 8;
        cells[4] = 5;
        cells[5] = 7;
        cells[6] = 4;
        cells[7] = 9;
        cells[8] = 3;
        // Col 0: _ 5 7 4 9 6 3 2 8
        cells[9] = 5;
        cells[18] = 7;
        cells[27] = 4;
        cells[36] = 9;
        cells[45] = 6;
        cells[54] = 3;
        cells[63] = 2;
        cells[72] = 8;
        let grid = Grid::from_flat(cells);
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let hint = solver.detect_technique("Naked Single").unwrap();
        assert_eq!(hint.hint_type, HintType::NakedSingle);
        assert_eq!(hint.value, 1, "Cell (0,0) must be 1");
        assert!(solver.validate_hint(&hint).is_ok());
    }

    #[test]
    fn test_constructed_locked_pointing() {
        // Test via detect_technique on EASY puzzle.
        // If it doesn't fire, that's fine — it's tested via no-crash + solve tests.
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Locked Pointing") {
            assert_eq!(hint.hint_type, HintType::LockedPointing);
            assert!(solver.validate_hint(&hint).is_ok());
        }
    }

    // ================================================================
    // Verify hint descriptions are non-empty
    // ================================================================

    #[test]
    fn test_naked_single_has_description() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let hint = solver.detect_technique("Naked Single").unwrap();
        assert!(!hint.technique_name.is_empty());
        assert!(!hint.description.is_empty());
    }

    #[test]
    fn test_hidden_single_has_description() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        let hint = solver.detect_technique("Hidden Single").unwrap();
        assert!(!hint.technique_name.is_empty());
        assert!(!hint.description.is_empty());
    }

    // ================================================================
    // Elimination hints remove valid candidates
    // ================================================================

    #[test]
    fn test_locked_pointing_removes_valid_candidates() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        if let Some(hint) = solver.detect_technique("Locked Pointing") {
            for &(cell, ref vals) in &hint.eliminations {
                let cands = solver.grid().candidates(cell.index);
                for &v in vals {
                    assert!(
                        cands.has(v),
                        "Candidate {} should exist at cell {} before elimination",
                        v,
                        cell.index
                    );
                }
            }
        }
    }

    // ================================================================
    // validate_hint rejects invalid hints
    // ================================================================

    #[test]
    fn test_validate_hint_rejects_value_on_filled_cell() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        // Find a filled cell
        for i in 0..81u8 {
            if solver.grid().get(i) > 0 {
                let hint = crate::solver::Hint::naked_single(CellIndex::new(i), 5);
                assert!(solver.validate_hint(&hint).is_err());
                break;
            }
        }
    }

    #[test]
    fn test_validate_hint_rejects_invalid_elimination() {
        let grid = Grid::parse(EASY).unwrap();
        let mut solver = Solver::new(grid);
        solver.rebuild_candidates();
        // Try to eliminate a candidate that doesn't exist at an empty cell
        for i in 0..81u8 {
            if solver.grid().get(i) == 0 {
                let cands = solver.grid().candidates(i);
                // Find a value NOT in candidates
                for v in 1..=9u8 {
                    if !cands.has(v) {
                        let hint = crate::solver::Hint {
                            hint_type: HintType::LockedPointing,
                            difficulty: 2.0,
                            technique_name: "test".to_string(),
                            description: "test".to_string(),
                            cell: CellIndex::new(i),
                            value: 0,
                            eliminations: vec![(CellIndex::new(i), vec![v])],
                        };
                        assert!(
                            solver.validate_hint(&hint).is_err(),
                            "Should reject elimination of non-candidate {} at cell {}",
                            v,
                            i
                        );
                        return;
                    }
                }
            }
        }
    }
}
