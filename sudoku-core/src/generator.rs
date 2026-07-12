// generator.rs: 数独谜题生成
//
// 使用 sudoku-solver 的 SukakuExplainer 算法生成谜题，
// 然后转换为 sudoku-core 的数据格式。

use crate::board::{Cell, Grid, Solution};
use crate::difficulty::Difficulty;
use rand::{Rng, seq::SliceRandom, thread_rng};
use sudoku_solver::{Generator as SukakuGenerator, Symmetry};

/// 将 sudoku-core::Grid 转换为 Solution
fn grid_to_solution(grid: &Grid) -> Solution {
    core::array::from_fn(|r| core::array::from_fn(|c| grid[r][c].value().unwrap()))
}

/// 应用变换以增加谜题多样性
///
/// 变换包括：
/// - 数字置换 (1-9 随机映射)
/// - 行 band 内交换
/// - 列 stack 内交换
/// - band 之间交换
/// - stack 之间交换
fn apply_transformations(grid: &mut Grid) {
    let mut rng = thread_rng();

    // 数字置换
    let mut digits: Vec<u8> = (1..=9).collect();
    digits.shuffle(&mut rng);
    for row in grid.iter_mut().take(9) {
        for cell in row.iter_mut().take(9) {
            if let Cell::Given(v) = *cell {
                *cell = Cell::Given(digits[(v - 1) as usize]);
            }
        }
    }

    // 行 band 内交换
    for band in 0..3 {
        let start = band * 3;
        let mut row_indices: Vec<usize> = (start..start + 3).collect();
        row_indices.shuffle(&mut rng);
        let mut temp = [[Cell::Empty; 9]; 3];
        temp.copy_from_slice(&grid[start..start + 3]);
        for i in 0..3 {
            let orig = start + i;
            let shuffled_idx = row_indices.iter().position(|&r| r == orig).unwrap();
            grid[start + i] = temp[shuffled_idx];
        }
    }

    // 列 stack 内交换
    for stack in 0..3 {
        let start = stack * 3;
        let mut col_indices: Vec<usize> = (0..3).collect();
        col_indices.shuffle(&mut rng);
        let mut temp = [[Cell::Empty; 9]; 9];
        for r in 0..9 {
            for i in 0..3 {
                temp[r][i] = grid[r][start + i];
            }
        }
        for r in 0..9 {
            for i in 0..3 {
                grid[r][start + i] = temp[r][col_indices[i]];
            }
        }
    }

    // band 之间交换
    let mut bands: Vec<usize> = (0..3).collect();
    bands.shuffle(&mut rng);
    let mut temp = [[Cell::Empty; 9]; 9];
    temp.copy_from_slice(&**grid);
    for r in 0..9 {
        grid[r] = temp[bands[r / 3] * 3 + r % 3];
    }

    // stack 之间交换
    let mut stacks: Vec<usize> = (0..3).collect();
    stacks.shuffle(&mut rng);
    for row in grid.iter_mut().take(9) {
        let temp_row = *row;
        for (c, cell) in row.iter_mut().enumerate().take(9) {
            let stack = c / 3;
            let pos = c % 3;
            *cell = temp_row[stacks[stack] * 3 + pos];
        }
    }
}

/// 计算谜题中已填数字的数量
fn count_givens(grid: &Grid) -> usize {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|cell| matches!(cell, Cell::Given(_)))
        .count()
}

/// 生成指定难度的数独谜题
///
/// # 算法
/// 1. 使用 sudoku-solver 的 SukakuExplainer 算法生成谜题
///    - 6 轮最大化移除
///    - 根据目标提示数调整移除策略
///    - 保证唯一解
///    - 验证实际 ER 难度匹配目标范围
/// 2. 应用变换增加多样性
///    - 数字置换
///    - 行/列/块交换
///
/// # 参数
/// - `difficulty`: 目标难度 (Easy/Medium/Hard/Expert)
///
/// # 返回
/// - `(puzzle, solution)`: 谜题和完整解
pub fn generate(difficulty: Difficulty) -> (Grid, Solution) {
    let (min_givens, max_givens) = difficulty.givens_range();
    let (min_er, max_er) = difficulty.er_range();

    // 创建 sudoku-solver 生成器，设置 ER 范围
    let mut generator = SukakuGenerator::new();
    generator.require_unique = true;
    generator.symmetry = Symmetry::None;
    generator.min_difficulty = min_er;
    generator.max_difficulty = max_er;
    generator.verify_difficulty = true;

    // Reduce attempts for faster fallback
    let max_attempts = match difficulty {
        Difficulty::Easy => 30,
        Difficulty::Medium => 20,
        Difficulty::Hard => 15,
        Difficulty::Expert => 10,
    };

    for _ in 0..max_attempts {
        let solver_grid = match generator.generate() {
            Ok(g) => g,
            Err(_) => continue,
        };

        let flat: [u8; 81] = core::array::from_fn(|i| solver_grid.get(i as u8));
        let mut grid = Grid::from_flat(flat);

        // 检查提示数是否在目标范围内
        let givens = count_givens(&grid);
        if givens >= min_givens && givens <= max_givens {
            apply_transformations(&mut grid);

            let mut solved_grid = grid;
            crate::solver::solve(&mut solved_grid);
            let solution = grid_to_solution(&solved_grid);

            return (grid, solution);
        }
    }

    generate_fallback(difficulty)
}

/// 回退生成算法（当 sudoku-solver 生成失败时使用）
fn generate_fallback(difficulty: Difficulty) -> (Grid, Solution) {
    use crate::solver::count_solutions;

    let mut grid = Grid::new();
    crate::solver::solve(&mut grid);

    // 应用变换
    apply_transformations(&mut grid);
    let solution = grid;

    // 移除数字
    let (min_givens, max_givens) = difficulty.givens_range();
    let target_givens = (min_givens + max_givens) / 2;
    let empty_cells = 81 - target_givens;
    let mut puzzle = solution;

    let mut candidates: Vec<usize> = (0..81).collect();
    let mut removed = 0;

    while removed < empty_cells && !candidates.is_empty() {
        let pos = thread_rng().gen_range(0..candidates.len());
        let idx = candidates[pos];
        let r = idx / 9;
        let c = idx % 9;
        let backup = puzzle[r][c];
        puzzle[r][c] = Cell::Empty;

        if count_solutions(&mut puzzle) != 1 {
            puzzle[r][c] = backup;
            candidates.swap_remove(pos);
        } else {
            removed += 1;
        }
    }

    (puzzle, grid_to_solution(&solution))
}
