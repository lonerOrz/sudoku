// sudoku-core: 数独核心算法库

/// 九宫格数独类型: [[value; 9]; 9], 0 表示空格
pub type Grid = [[u8; 9]; 9];

/// 检查在 (row, col) 位置放置 val 是否有效（无同行同列同宫冲突）
pub fn is_valid(grid: &Grid, row: usize, col: usize, val: u8) -> bool {
    if val == 0 {
        return true;
    }

    // 检查同行
    for c in 0..9 {
        if c != col && grid[row][c] == val {
            return false;
        }
    }

    // 检查同列
    for r in 0..9 {
        if r != row && grid[r][col] == val {
            return false;
        }
    }

    // 检查同宫
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for r in box_row..box_row + 3 {
        for c in box_col..box_col + 3 {
            if !(r == row && c == col) && grid[r][c] == val {
                return false;
            }
        }
    }

    true
}

/// 检查数独是否已完成且正确
pub fn is_solved(grid: &Grid) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_same_row() {
        let mut grid: Grid = [[0; 9]; 9];
        grid[0][0] = 5;

        assert!(!is_valid(&grid, 0, 3, 5)); // 同行冲突
        assert!(is_valid(&grid, 0, 3, 3)); // 不同行，有效
    }

    #[test]
    fn test_is_valid_same_col() {
        let mut grid: Grid = [[0; 9]; 9];
        grid[0][0] = 5;

        assert!(!is_valid(&grid, 5, 0, 5)); // 同列冲突
        assert!(is_valid(&grid, 5, 0, 3)); // 不同列，有效
    }

    #[test]
    fn test_is_valid_same_box() {
        let mut grid: Grid = [[0; 9]; 9];
        grid[0][0] = 5;

        assert!(!is_valid(&grid, 2, 2, 5)); // 同宫冲突
        assert!(is_valid(&grid, 4, 4, 5)); // 不同宫，有效
    }

    #[test]
    fn test_is_valid_zero() {
        let grid: Grid = [[0; 9]; 9];

        assert!(is_valid(&grid, 0, 0, 0)); // 放 0 永远有效
    }
}
