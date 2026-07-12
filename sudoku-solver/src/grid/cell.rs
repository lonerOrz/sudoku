#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CellIndex {
    pub index: u8,
}

impl CellIndex {
    pub const COUNT: usize = 81;

    #[inline]
    pub fn new(index: u8) -> Self {
        debug_assert!(index < 81);
        Self { index }
    }

    #[inline]
    pub fn x(self) -> u8 {
        self.index % 9
    }

    #[inline]
    pub fn y(self) -> u8 {
        self.index / 9
    }

    #[inline]
    pub fn box_index(self) -> u8 {
        (self.y() / 3) * 3 + (self.x() / 3)
    }
}

impl From<u8> for CellIndex {
    fn from(index: u8) -> Self {
        CellIndex::new(index)
    }
}

impl From<usize> for CellIndex {
    fn from(index: usize) -> Self {
        CellIndex::new(index as u8)
    }
}
