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

pub mod direct;
pub mod fish;
pub mod indirect;
pub mod locked;
pub mod strong_link;
pub mod subset;
pub mod unique;
pub mod wing;

pub use direct::{hidden_single, naked_single};
pub use fish::{jellyfish, swordfish, x_wing};
pub use indirect::{hidden_pair, naked_pair};
pub use locked::{locked_claiming, locked_pointing};
pub use strong_link::{skyscraper, strong_links_fish_3, strong_links_fish_4, two_string_kite};
pub use subset::{hidden_quad, hidden_triple, naked_quad, naked_triple};
pub use unique::{
    bug_plus_four, bug_plus_one, bug_plus_three, bug_plus_two, unique_rectangle_type1,
    unique_rectangle_type2, unique_rectangle_type3, unique_rectangle_type4,
};
pub use wing::{vwxyz_wing, wxyz_wing, xy_wing, xyz_wing};
