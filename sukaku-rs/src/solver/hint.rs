//! Hint types for solving techniques.
//!
//! Represents the output of rule-based solving - a technique with its difficulty.

use crate::grid::Cell;

#[derive(Debug, Clone, PartialEq)]
pub enum HintType {
    NakedSingle,
    HiddenSingle,
    NakedPair,
    HiddenPair,
    NakedTriple,
    HiddenTriple,
    NakedQuad,
    HiddenQuad,
    XWing,
    Swordfish,
    Jellyfish,
    XYWing,
    XYZWing,
    WXYZWing,
    VWXYZWing,
    UVWXYZWing,
    TUVWXYZWing,
    UniqueRectangleType1,
    UniqueRectangleType2,
    UniqueRectangleType3,
    UniqueRectangleType4,
    BUGPlusOne,
    BUGPlusTwo,
    BUGPlusThree,
    BUGPlusFour,
    LockedPointing,
    LockedClaiming,
    Skyscraper,
    TwoStringKite,
    StrongLinksFish,
    AlignedPairExclusion,
    AlignedTripletExclusion,
    GeneralizedNakedSet,
    VLocking,
    XCyclesSimple,
    YCyclesSimple,
    ForcingChain,
}

/// A hint represents a solving technique that can be applied to make progress.
///
/// - For single-filling techniques (Naked Single, Hidden Single): `value` contains the digit to fill
/// - For elimination techniques (Naked Pair, Hidden Pair, etc.): `eliminations` contains (cell, candidates) pairs
#[derive(Debug, Clone)]
pub struct Hint {
    pub hint_type: HintType,
    pub difficulty: f64,
    pub technique_name: String,
    pub description: String,
    pub cell: Cell,
    /// The digit to fill in the cell (0 for elimination techniques)
    pub value: u8,
    /// Eliminations: (cell, candidates to remove from that cell)
    pub eliminations: Vec<(Cell, Vec<u8>)>,
}

impl Hint {
    pub fn naked_single(cell: Cell, value: u8) -> Self {
        Self {
            hint_type: HintType::NakedSingle,
            difficulty: 1.6,
            technique_name: "Naked Single".to_string(),
            description: format!("Cell {:?} = {}", cell, value),
            cell,
            value,
            eliminations: vec![],
        }
    }

    pub fn hidden_single(cell: Cell, value: u8, difficulty: f64) -> Self {
        Self {
            hint_type: HintType::HiddenSingle,
            difficulty,
            technique_name: "Hidden Single".to_string(),
            description: format!("{} in row/col/box", value),
            cell,
            value,
            eliminations: vec![],
        }
    }
}
