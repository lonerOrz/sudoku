#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionType {
    Row,
    Column,
    Block,
}

#[derive(Clone, Copy, Debug)]
pub struct Region {
    pub region_type: RegionType,
    pub index: u8,
    pub cells: [u8; 9],
}

impl Region {
    pub const fn new(region_type: RegionType, index: u8, cells: [u8; 9]) -> Self {
        Self {
            region_type,
            index,
            cells,
        }
    }
}

pub static ROWS: [Region; 9] = [
    Region::new(RegionType::Row, 0, [0, 1, 2, 3, 4, 5, 6, 7, 8]),
    Region::new(RegionType::Row, 1, [9, 10, 11, 12, 13, 14, 15, 16, 17]),
    Region::new(RegionType::Row, 2, [18, 19, 20, 21, 22, 23, 24, 25, 26]),
    Region::new(RegionType::Row, 3, [27, 28, 29, 30, 31, 32, 33, 34, 35]),
    Region::new(RegionType::Row, 4, [36, 37, 38, 39, 40, 41, 42, 43, 44]),
    Region::new(RegionType::Row, 5, [45, 46, 47, 48, 49, 50, 51, 52, 53]),
    Region::new(RegionType::Row, 6, [54, 55, 56, 57, 58, 59, 60, 61, 62]),
    Region::new(RegionType::Row, 7, [63, 64, 65, 66, 67, 68, 69, 70, 71]),
    Region::new(RegionType::Row, 8, [72, 73, 74, 75, 76, 77, 78, 79, 80]),
];

pub static COLS: [Region; 9] = [
    Region::new(RegionType::Column, 0, [0, 9, 18, 27, 36, 45, 54, 63, 72]),
    Region::new(RegionType::Column, 1, [1, 10, 19, 28, 37, 46, 55, 64, 73]),
    Region::new(RegionType::Column, 2, [2, 11, 20, 29, 38, 47, 56, 65, 74]),
    Region::new(RegionType::Column, 3, [3, 12, 21, 30, 39, 48, 57, 66, 75]),
    Region::new(RegionType::Column, 4, [4, 13, 22, 31, 40, 49, 58, 67, 76]),
    Region::new(RegionType::Column, 5, [5, 14, 23, 32, 41, 50, 59, 68, 77]),
    Region::new(RegionType::Column, 6, [6, 15, 24, 33, 42, 51, 60, 69, 78]),
    Region::new(RegionType::Column, 7, [7, 16, 25, 34, 43, 52, 61, 70, 79]),
    Region::new(RegionType::Column, 8, [8, 17, 26, 35, 44, 53, 62, 71, 80]),
];

pub static BLOCKS: [Region; 9] = [
    Region::new(RegionType::Block, 0, [0, 1, 2, 9, 10, 11, 18, 19, 20]),
    Region::new(RegionType::Block, 1, [3, 4, 5, 12, 13, 14, 21, 22, 23]),
    Region::new(RegionType::Block, 2, [6, 7, 8, 15, 16, 17, 24, 25, 26]),
    Region::new(RegionType::Block, 3, [27, 28, 29, 36, 37, 38, 45, 46, 47]),
    Region::new(RegionType::Block, 4, [30, 31, 32, 39, 40, 41, 48, 49, 50]),
    Region::new(RegionType::Block, 5, [33, 34, 35, 42, 43, 44, 51, 52, 53]),
    Region::new(RegionType::Block, 6, [54, 55, 56, 63, 64, 65, 72, 73, 74]),
    Region::new(RegionType::Block, 7, [57, 58, 59, 66, 67, 68, 75, 76, 77]),
    Region::new(RegionType::Block, 8, [60, 61, 62, 69, 70, 71, 78, 79, 80]),
];
