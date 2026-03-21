// board.rs: 数独棋盘数据结构

/// 单元格状态
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Given(u8),
    UserInput(u8),
    Empty,
}

impl Cell {
    pub fn value(&self) -> Option<u8> {
        match self {
            Cell::Given(v) | Cell::UserInput(v) => Some(*v),
            Cell::Empty => None,
        }
    }
}

/// 九宫格数独类型
pub type Grid = [[Cell; 9]; 9];

/// 返回 (row, col) 的所有"同行同列同宫"格子（不含自身）
pub fn peers(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;

    let row_peers = (0..9).map(move |c| (row, c));
    let col_peers = (0..9).map(move |r| (r, col));
    let box_peers =
        (box_row..box_row + 3).flat_map(move |r| (box_col..box_col + 3).map(move |c| (r, c)));

    row_peers
        .chain(col_peers)
        .chain(box_peers)
        .filter(move |&(r, c)| r != row || c != col)
}
