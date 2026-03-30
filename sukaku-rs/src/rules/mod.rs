//! Sudoku solving techniques/rules registry.
//!
//! This module provides a unified registry of all solving techniques
//! with metadata for strategy-based execution.

use crate::grid::Grid;
use crate::solver::HintAccumulator;

/// Strategy for rule execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    /// Solve mode: prioritize lowest difficulty first (greedy)
    Solve,
    /// Detect mode: prioritize highest difficulty for technique detection
    Detect,
}

/// Descriptor for a solving technique.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Human-readable technique name.
    pub name: &'static str,
    /// The rule function.
    pub func: fn(&Grid, &mut HintAccumulator),
    /// Difficulty rating (SER-like scale).
    pub difficulty: f32,
}

impl Rule {
    /// Create a new rule descriptor.
    pub const fn new(
        name: &'static str,
        func: fn(&Grid, &mut HintAccumulator),
        difficulty: f32,
    ) -> Self {
        Self {
            name,
            func,
            difficulty,
        }
    }
}

/// Get all rules ordered by difficulty (ascending for Solve mode).
pub fn all_rules() -> Vec<Rule> {
    vec![
        // Singles (difficulty 1.0-1.2)
        Rule::new("Naked Single", naked_single, 1.0),
        Rule::new("Hidden Single", hidden_single, 1.2),
        // Locked candidates (difficulty 2.0-2.1)
        Rule::new("Locked Pointing", locked_pointing, 2.0),
        Rule::new("Locked Claiming", locked_claiming, 2.1),
        // Hidden subsets (difficulty 2.5-3.5)
        Rule::new("Hidden Pair", hidden_pair, 2.5),
        Rule::new("Hidden Triple", hidden_triple, 3.0),
        Rule::new("Hidden Quad", hidden_quad, 3.5),
        // Naked subsets (difficulty 2.6-3.6)
        Rule::new("Naked Pair", naked_pair, 2.6),
        Rule::new("Naked Triple", naked_triple, 3.1),
        Rule::new("Naked Quad", naked_quad, 3.6),
        // Fish techniques (difficulty 4.0-6.0)
        Rule::new("X-Wing", x_wing, 4.0),
        Rule::new("Swordfish", swordfish, 5.0),
        Rule::new("Jellyfish", jellyfish, 6.0),
        // Wing techniques (difficulty 4.5-6.2)
        Rule::new("XY-Wing", xy_wing, 4.5),
        Rule::new("XYZ-Wing", xyz_wing, 5.0),
        Rule::new("WXYZ-Wing", wxyz_wing, 5.5),
        Rule::new("VWXYZ-Wing", vwxyz_wing, 6.2),
        Rule::new("UVWXYZ-Wing", uvwxyz_wing, 6.6),
        Rule::new("TUVWXYZ-Wing", tuvwxyz_wing, 7.5),
        Rule::new("ALS-XZ", als_xz_rule, 7.0),
        // Unique Rectangle (difficulty 5.0-5.3)
        Rule::new("Unique Rectangle Type 1", unique_rectangle_type1, 5.0),
        Rule::new("Unique Rectangle Type 2", unique_rectangle_type2, 5.1),
        Rule::new("Unique Rectangle Type 3", unique_rectangle_type3, 5.2),
        Rule::new("Unique Rectangle Type 4", unique_rectangle_type4, 5.3),
        // Advanced (difficulty 6.0-6.5)
        Rule::new("BUG+1", bug_plus_one, 6.0),
        Rule::new("BUG+2", bug_plus_two, 5.8),
        Rule::new("BUG+3", bug_plus_three, 6.0),
        Rule::new("BUG+4", bug_plus_four, 6.2),
        Rule::new("Skyscraper", skyscraper, 4.2),
        Rule::new("2-String Kite", two_string_kite, 4.3),
        Rule::new("3-Strong-Links Fish", strong_links_fish_3, 5.4),
        Rule::new("4-Strong-Links Fish", strong_links_fish_4, 5.8),
        Rule::new("5-Strong-Links Fish", strong_links_fish_5, 6.0),
        Rule::new("6-Strong-Links Fish", strong_links_fish_6, 6.2),
        Rule::new("Aligned Pair Exclusion", aligned_pair_exclusion, 6.2),
        Rule::new("Aligned Triplet Exclusion", aligned_triplet_exclusion, 7.5),
        Rule::new("Generalized Naked Pair", generalized_naked_pair, 3.0),
        Rule::new("Generalized Naked Triplet", generalized_naked_triple, 3.6),
        Rule::new("Generalized Naked Quad", generalized_naked_quad, 5.0),
        Rule::new("Generalized Naked Quint", generalized_naked_quint, 5.4),
        Rule::new("Generalized Naked Sext", generalized_naked_sext, 5.8),
        Rule::new("VLocking", vlocking, 4.5),
        Rule::new("X-Cycles", x_cycles_simple, 6.5),
        Rule::new("Y-Cycles", y_cycles_simple, 6.5),
        Rule::new("Forcing Chain", forcing_chain, 7.0),
        Rule::new("Nishio Forcing Chain", nishio_forcing_chain, 7.5),
        Rule::new("Multiple Forcing Chain", multiple_forcing_chain, 8.0),
        Rule::new("Dynamic Forcing Chain", dynamic_forcing_chain, 8.5),
        Rule::new("Dynamic Forcing Chain+", dynamic_forcing_chain_plus, 9.0),
        Rule::new(
            "Nested Forcing Chain (2-level)",
            nested_forcing_chain_2,
            9.5,
        ),
        Rule::new(
            "Nested Forcing Chain (3-level)",
            nested_forcing_chain_3,
            10.0,
        ),
        Rule::new(
            "Nested Forcing Chain (4-level)",
            nested_forcing_chain_4,
            10.5,
        ),
        Rule::new("X-Diagonal", x_diagonal_var, 5.5),
    ]
}

/// Get rules ordered for Solve mode (lowest difficulty first).
pub fn rules_for_solve() -> Vec<Rule> {
    let mut rules = all_rules();
    rules.sort_by(|a, b| a.difficulty.partial_cmp(&b.difficulty).unwrap());
    rules
}

/// Get rules ordered for Detect mode (highest difficulty first).
pub fn rules_for_detect() -> Vec<Rule> {
    let mut rules = all_rules();
    rules.sort_by(|a, b| b.difficulty.partial_cmp(&a.difficulty).unwrap());
    rules
}

/// Find a specific rule by name.
pub fn find_rule(name: &str) -> Option<Rule> {
    all_rules().into_iter().find(|r| r.name == name)
}

pub mod chaining;
pub mod direct;
pub mod exclusion;
pub mod fish;
pub mod indirect;
pub mod locked;
pub mod strong_link;
pub mod subset;
pub mod subset_gen;
pub mod unique;
pub mod variant;
pub mod vlocking;
pub mod wing;

pub use chaining::{
    dynamic_forcing_chain, dynamic_forcing_chain_plus, forcing_chain, multiple_forcing_chain,
    nested_forcing_chain_2, nested_forcing_chain_3, nested_forcing_chain_4, nishio_forcing_chain,
    x_cycles_simple, y_cycles_simple,
};
pub use direct::{hidden_single, naked_single};
pub use exclusion::{aligned_pair_exclusion, aligned_triplet_exclusion};
pub use fish::{jellyfish, swordfish, x_wing};
pub use indirect::{hidden_pair, naked_pair};
pub use locked::{locked_claiming, locked_pointing};
pub use strong_link::{
    skyscraper, strong_links_fish_3, strong_links_fish_4, strong_links_fish_5, strong_links_fish_6,
    two_string_kite,
};
pub use subset::{hidden_quad, hidden_triple, naked_quad, naked_triple};
pub use subset_gen::{
    generalized_naked_pair, generalized_naked_quad, generalized_naked_quint,
    generalized_naked_sext, generalized_naked_triple,
};
pub use unique::{
    bug_plus_four, bug_plus_one, bug_plus_three, bug_plus_two, unique_rectangle_type1,
    unique_rectangle_type2, unique_rectangle_type3, unique_rectangle_type4,
};
pub use variant::x_diagonal_var;
pub use vlocking::vlocking;
pub use wing::{als_xz_rule, tuvwxyz_wing, uvwxyz_wing, vwxyz_wing, wxyz_wing, xy_wing, xyz_wing};
