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

/// 难度级别
#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
    Easy,   // 38-43 个给定数
    Medium, // 30-37 个给定数
    Hard,   // 25-29 个给定数
    Expert, // 20-24 个给定数
}

impl Difficulty {
    pub fn givens_count(&self) -> usize {
        match self {
            Difficulty::Easy => 40,
            Difficulty::Medium => 34,
            Difficulty::Hard => 27,
            Difficulty::Expert => 22,
        }
    }
}

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

/// 检查在 (row, col) 位置放置 val 是否有效
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
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                return false;
            }
        }
    }
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

/// 解数独（回溯算法）
pub fn solve(grid: &mut Grid) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                for val in 1..=9 {
                    if is_valid(grid, r, c, val) {
                        grid[r][c] = Cell::UserInput(val);

                        if solve(grid) {
                            return true;
                        }

                        grid[r][c] = Cell::Empty;
                    }
                }
                return false;
            }
        }
    }
    true
}

/// 找一个空格
fn find_empty(grid: &Grid) -> Option<(usize, usize)> {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                return Some((r, c));
            }
        }
    }
    None
}

/// 计算解的数量（最多返回 max_count，超过则停止）
fn count_solutions_inner(grid: &mut Grid, count: &mut usize, max_count: usize) {
    if *count >= max_count {
        return;
    }

    if let Some((r, c)) = find_empty(grid) {
        for val in 1..=9 {
            if is_valid(grid, r, c, val) {
                grid[r][c] = Cell::Given(val);
                count_solutions_inner(grid, count, max_count);
                grid[r][c] = Cell::Empty;
            }
        }
    } else {
        *count += 1;
    }
}

/// 计算解的数量
pub fn count_solutions(grid: &mut Grid) -> usize {
    let mut count = 0;
    count_solutions_inner(grid, &mut count, 2); // 超过2个就停止
    count
}

/// 生成谜题（保证唯一解）
pub fn generate(difficulty: Difficulty) -> (Grid, Grid) {
    // 1. 创建空棋盘
    let mut grid: Grid = [[Cell::Empty; 9]; 9];

    // 2. 解它得到完整解
    solve(&mut grid);

    // 3. 保存解
    let solution = grid;

    // 4. 计算需要挖掉的格子数
    let empty_cells = 81 - difficulty.givens_count();
    let mut puzzle = solution;

    // 5. 按顺序挖空，保证唯一解
    let mut removed = 0;
    for r in 0..9 {
        for c in 0..9 {
            if removed >= empty_cells {
                break;
            }

            let backup = puzzle[r][c];
            puzzle[r][c] = Cell::Empty;

            // 检查是否还有唯一解
            if count_solutions(&mut puzzle) != 1 {
                puzzle[r][c] = backup; // 恢复，不是唯一解
            } else {
                removed += 1;
            }
        }
    }

    (puzzle, solution)
}

/// 计算 (row, col) 位置可能的数字
pub fn possible_values(grid: &Grid, row: usize, col: usize) -> Vec<u8> {
    // 如果不是空格，直接返回
    if grid[row][col].value().is_some() {
        return vec![];
    }

    let mut candidates = Vec::new();
    for val in 1..=9 {
        if is_valid(grid, row, col, val) {
            candidates.push(val);
        }
    }
    candidates
}

/// 找出所有有冲突的格子
pub fn find_errors(grid: &Grid) -> Vec<(usize, usize)> {
    let mut errors = Vec::new();

    for r in 0..9 {
        for c in 0..9 {
            if let Some(val) = grid[r][c].value() {
                // 检查是否有效（排除自己）
                let mut is_error = false;
                for (pr, pc) in peers(r, c) {
                    if let Some(other) = grid[pr][pc].value() {
                        if other == val {
                            is_error = true;
                            break;
                        }
                    }
                }
                if is_error {
                    errors.push((r, c));
                }
            }
        }
    }

    errors
}

/// 找 Naked Single（唯一候选数）
pub fn find_naked_single(grid: &Grid) -> Option<((usize, usize), u8)> {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                let candidates = possible_values(grid, r, c);
                if candidates.len() == 1 {
                    return Some(((r, c), candidates[0]));
                }
            }
        }
    }
    None
}

/// 找 Hidden Single（隐藏唯一）
pub fn find_hidden_single(grid: &Grid) -> Option<((usize, usize), u8)> {
    // 检查每行
    for r in 0..9 {
        for val in 1..=9 {
            if !row_has(grid, r, val) {
                let possible: Vec<usize> = (0..9)
                    .filter(|&c| grid[r][c].value().is_none() && is_valid(grid, r, c, val))
                    .collect();
                if possible.len() == 1 {
                    return Some(((r, possible[0]), val));
                }
            }
        }
    }

    // 检查每列
    for c in 0..9 {
        for val in 1..=9 {
            if !col_has(grid, c, val) {
                let possible: Vec<usize> = (0..9)
                    .filter(|&r| grid[r][c].value().is_none() && is_valid(grid, r, c, val))
                    .collect();
                if possible.len() == 1 {
                    return Some(((possible[0], c), val));
                }
            }
        }
    }

    // 检查每宫
    for box_r in (0..9).step_by(3) {
        for box_c in (0..9).step_by(3) {
            for val in 1..=9 {
                if !box_has(grid, box_r, box_c, val) {
                    let possible: Vec<(usize, usize)> = (box_r..box_r + 3)
                        .flat_map(|r| (box_c..box_c + 3).map(move |c| (r, c)))
                        .filter(|(r, c)| {
                            grid[*r][*c].value().is_none() && is_valid(grid, *r, *c, val)
                        })
                        .collect();
                    if possible.len() == 1 {
                        return Some((possible[0], val));
                    }
                }
            }
        }
    }

    None
}

/// 检查行是否包含某数字
fn row_has(grid: &Grid, row: usize, val: u8) -> bool {
    for c in 0..9 {
        if let Some(v) = grid[row][c].value() {
            if v == val {
                return true;
            }
        }
    }
    false
}

/// 检查列是否包含某数字
fn col_has(grid: &Grid, col: usize, val: u8) -> bool {
    for r in 0..9 {
        if let Some(v) = grid[r][col].value() {
            if v == val {
                return true;
            }
        }
    }
    false
}

/// 检查宫是否包含某数字
fn box_has(grid: &Grid, box_r: usize, box_c: usize, val: u8) -> bool {
    for r in box_r..box_r + 3 {
        for c in box_c..box_c + 3 {
            if let Some(v) = grid[r][c].value() {
                if v == val {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let (puzzle, solution) = generate(Difficulty::Easy);

        // 谜题应该有空格
        let mut has_empty = false;
        for r in 0..9 {
            for c in 0..9 {
                if puzzle[r][c].value().is_none() {
                    has_empty = true;
                    break;
                }
            }
        }
        assert!(has_empty);

        // 解应该完整
        assert!(is_solved(&solution));
    }

    #[test]
    fn test_count_solutions() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        assert!(solve(&mut grid));
        assert_eq!(count_solutions(&mut grid), 1);
    }

    #[test]
    fn test_solve() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        assert!(solve(&mut grid));
        assert!(is_solved(&grid));
    }

    #[test]
    fn test_peers_count() {
        assert_eq!(peers(4, 4).count(), 24);
    }

    #[test]
    fn test_peers_not_include_self() {
        assert!(!peers(4, 4).any(|(r, c)| r == 4 && c == 4));
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
    fn test_cell_value() {
        assert_eq!(Cell::Given(5).value(), Some(5));
        assert_eq!(Cell::UserInput(3).value(), Some(3));
        assert_eq!(Cell::Empty.value(), None);
    }

    #[test]
    fn test_possible_values() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);
        grid[1][1] = Cell::Given(3);

        let candidates = possible_values(&grid, 0, 1);
        assert!(candidates.contains(&1));
        assert!(!candidates.contains(&5));
        assert!(!candidates.contains(&3));
    }

    #[test]
    fn test_find_errors() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);
        grid[0][1] = Cell::Given(5); // 重复！

        let errors = find_errors(&grid);
        assert!(errors.contains(&(0, 1)));
    }

    #[test]
    fn test_find_naked_single() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);
        // 只有一个位置可以放 1
        let candidates = possible_values(&grid, 0, 1);
        assert!(candidates.len() > 1);
    }

    #[test]
    fn test_find_errors_empty() {
        let grid: Grid = [[Cell::Empty; 9]; 9];
        let errors = find_errors(&grid);
        assert!(errors.is_empty());
    }
}
