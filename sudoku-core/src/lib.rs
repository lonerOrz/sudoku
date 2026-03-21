// sudoku-core: 数独核心算法库

/// 单元格状态
#[derive(Clone, Copy, PartialEq)]
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

/// 检查在 (row, col) 位置放置 val 是否有效（无同行同列同宫冲突）
pub fn is_valid(grid: &Grid, row: usize, col: usize, val: u8) -> bool {
    for (r, c) in peers(row, col) {
        if let Some(v) = grid[r][c].value() {
            if v == val {
                return false;
            }
        }
    }
    true
}

/// 检查数独是否已完成且正确
pub fn is_solved(grid: &Grid) -> bool {
    // 检查是否有空格
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                return false;
            }
        }
    }

    // 检查每个格子是否符合规则
    for r in 0..9 {
        for c in 0..9 {
            if let Some(val) = grid[r][c].value() {
                if !is_valid(grid, r, c, val) {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peers_count() {
        // 同行 8 个 + 同列 8 个 + 同宫 8 个 = 24 个
        let peers_count = peers(4, 4).count();
        assert_eq!(peers_count, 24);
    }

    #[test]
    fn test_peers_not_include_self() {
        // peers 不应该包含自己
        let self_included = peers(4, 4).any(|(r, c)| r == 4 && c == 4);
        assert!(!self_included);
    }

    #[test]
    fn test_is_valid_same_row() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);

        assert!(!is_valid(&grid, 0, 3, 5));
        assert!(is_valid(&grid, 0, 3, 3));
    }

    #[test]
    fn test_is_valid_same_col() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);

        assert!(!is_valid(&grid, 5, 0, 5));
        assert!(is_valid(&grid, 5, 0, 3));
    }

    #[test]
    fn test_is_valid_same_box() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);

        assert!(!is_valid(&grid, 2, 2, 5));
        assert!(is_valid(&grid, 4, 4, 5));
    }

    #[test]
    fn test_is_solved_empty() {
        let grid: Grid = [[Cell::Empty; 9]; 9];

        assert!(!is_solved(&grid));
    }

    #[test]
    fn test_is_solved_complete() {
        let grid: Grid = [
            [
                Cell::Given(5),
                Cell::Given(3),
                Cell::Given(4),
                Cell::Given(6),
                Cell::Given(7),
                Cell::Given(8),
                Cell::Given(9),
                Cell::Given(1),
                Cell::Given(2),
            ],
            [
                Cell::Given(6),
                Cell::Given(7),
                Cell::Given(2),
                Cell::Given(1),
                Cell::Given(9),
                Cell::Given(5),
                Cell::Given(3),
                Cell::Given(4),
                Cell::Given(8),
            ],
            [
                Cell::Given(1),
                Cell::Given(9),
                Cell::Given(8),
                Cell::Given(3),
                Cell::Given(4),
                Cell::Given(2),
                Cell::Given(5),
                Cell::Given(6),
                Cell::Given(7),
            ],
            [
                Cell::Given(8),
                Cell::Given(5),
                Cell::Given(9),
                Cell::Given(7),
                Cell::Given(6),
                Cell::Given(1),
                Cell::Given(4),
                Cell::Given(2),
                Cell::Given(3),
            ],
            [
                Cell::Given(4),
                Cell::Given(2),
                Cell::Given(6),
                Cell::Given(8),
                Cell::Given(5),
                Cell::Given(3),
                Cell::Given(7),
                Cell::Given(9),
                Cell::Given(1),
            ],
            [
                Cell::Given(7),
                Cell::Given(1),
                Cell::Given(3),
                Cell::Given(9),
                Cell::Given(2),
                Cell::Given(4),
                Cell::Given(8),
                Cell::Given(5),
                Cell::Given(6),
            ],
            [
                Cell::Given(9),
                Cell::Given(6),
                Cell::Given(1),
                Cell::Given(5),
                Cell::Given(3),
                Cell::Given(7),
                Cell::Given(2),
                Cell::Given(8),
                Cell::Given(4),
            ],
            [
                Cell::Given(2),
                Cell::Given(8),
                Cell::Given(7),
                Cell::Given(4),
                Cell::Given(1),
                Cell::Given(9),
                Cell::Given(6),
                Cell::Given(3),
                Cell::Given(5),
            ],
            [
                Cell::Given(3),
                Cell::Given(4),
                Cell::Given(5),
                Cell::Given(2),
                Cell::Given(8),
                Cell::Given(6),
                Cell::Given(1),
                Cell::Given(7),
                Cell::Given(9),
            ],
        ];

        assert!(is_solved(&grid));
    }

    #[test]
    fn test_is_solved_incomplete() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::UserInput(5);

        assert!(!is_solved(&grid));
    }

    #[test]
    fn test_cell_value() {
        assert_eq!(Cell::Given(5).value(), Some(5));
        assert_eq!(Cell::UserInput(3).value(), Some(3));
        assert_eq!(Cell::Empty.value(), None);
    }
}
