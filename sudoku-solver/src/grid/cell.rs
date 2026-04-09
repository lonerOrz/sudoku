#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Cell {
    pub index: u8,
}

impl Cell {
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

impl From<u8> for Cell {
    fn from(index: u8) -> Self {
        Cell::new(index)
    }
}

impl From<usize> for Cell {
    fn from(index: usize) -> Self {
        Cell::new(index as u8)
    }
}
