// generator.rs: 数独谜题生成

use crate::board::{Cell, Grid, Solution};
use crate::difficulty::Difficulty;
use crate::solver::{count_solutions, solve};
use rand::{Rng, seq::SliceRandom, thread_rng};

fn grid_to_solution(grid: &Grid) -> Solution {
    core::array::from_fn(|r| core::array::from_fn(|c| grid[r][c].value().unwrap()))
}

fn apply_transformations(grid: &mut Grid) {
    let mut rng = thread_rng();

    let mut digits: Vec<u8> = (1..=9).collect();
    digits.shuffle(&mut rng);
    for row in grid.iter_mut().take(9) {
        for cell in row.iter_mut().take(9) {
            if let Cell::Given(v) = *cell {
                *cell = Cell::Given(digits[(v - 1) as usize]);
            }
        }
    }

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

    let mut bands: Vec<usize> = (0..3).collect();
    bands.shuffle(&mut rng);
    let mut temp = [[Cell::Empty; 9]; 9];
    temp.copy_from_slice(grid);
    for r in 0..9 {
        grid[r] = temp[bands[r / 3] * 3 + r % 3];
    }

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

pub fn generate(difficulty: Difficulty) -> (Grid, Solution) {
    let mut grid: Grid = [[Cell::Empty; 9]; 9];
    solve(&mut grid);
    apply_transformations(&mut grid);
    let solution = grid;

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
