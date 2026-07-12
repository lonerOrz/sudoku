//! Candidate bitmask for pencilmark-style solving.

/// Bitmask of possible values (1-9) for a cell. Bit `n` represents digit `n`.
///
/// `Candidates::EMPTY` = no candidates (cell is filled).
/// `Candidates::FULL` = digits 1-9 all possible (empty cell, no eliminations).
#[derive(Clone, Copy, Default)]
pub struct Candidates(u16);

impl Candidates {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(0x3FE);

    #[inline]
    pub fn empty() -> Self {
        Self::EMPTY
    }

    #[inline]
    pub fn full() -> Self {
        Self::FULL
    }

    #[inline]
    pub fn single(value: u8) -> Self {
        debug_assert!((1..=9).contains(&value));
        Self(1 << value)
    }

    #[inline]
    pub fn has(self, value: u8) -> bool {
        debug_assert!((1..=9).contains(&value));
        (self.0 & (1 << value)) != 0
    }

    #[inline]
    pub fn set(&mut self, value: u8) {
        debug_assert!((1..=9).contains(&value));
        self.0 |= 1 << value;
    }

    #[inline]
    pub fn remove(&mut self, value: u8) {
        debug_assert!((1..=9).contains(&value));
        self.0 &= !(1 << value);
    }

    #[inline]
    pub fn cardinality(self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_single(self) -> bool {
        self.0.count_ones() == 1
    }

    #[inline]
    pub fn first(self) -> Option<u8> {
        (1..=9).find(|&v| self.has(v))
    }

    #[inline]
    pub fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn iter(self) -> impl Iterator<Item = u8> {
        (1..=9).filter(move |&v| self.has(v))
    }
}

impl PartialEq for Candidates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Candidates {}

impl std::fmt::Debug for Candidates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for v in 1..=9 {
            if self.has(v) {
                write!(f, "{}", v)?;
            }
        }
        write!(f, "}}")
    }
}
