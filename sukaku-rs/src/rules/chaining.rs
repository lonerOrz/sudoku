use crate::grid::{Cell, Grid, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

const GRID_SIZE: u8 = 9;
const MIN_PIVOTS: usize = 2;
const DIFFICULTY: f64 = 6.5;

pub fn x_cycles_simple(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9 {
        find_cycle_horizontal(grid, acc, digit);
        find_cycle_vertical(grid, acc, digit);
    }
}

fn find_cycle_horizontal(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for row1 in 0..9 {
        for row2 in (row1 + 1)..9 {
            let (c1, c2) = find_matching_columns(grid, row1, row2, digit);
            if c1.len() == MIN_PIVOTS && c2.len() == MIN_PIVOTS {
                apply_cycle_elimination(grid, acc, digit, row1, row2, c1[0], c2[0]);
            }
        }
    }
}

fn find_cycle_vertical(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for col1 in 0..9 {
        for col2 in (col1 + 1)..9 {
            let (r1, r2) = find_matching_rows(grid, col1, col2, digit);
            if r1.len() == MIN_PIVOTS && r2.len() == MIN_PIVOTS {
                apply_cycle_elimination(grid, acc, digit, col1, col2, r1[0], r2[0]);
            }
        }
    }
}

fn find_matching_columns(grid: &Grid, r1: u8, r2: u8, digit: u8) -> (Vec<u8>, Vec<u8>) {
    let row1_cells: Vec<u8> = ROWS[r1 as usize]
        .cells
        .iter()
        .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
        .map(|&c| c % GRID_SIZE)
        .collect();
    let row2_cells: Vec<u8> = ROWS[r2 as usize]
        .cells
        .iter()
        .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
        .map(|&c| c % GRID_SIZE)
        .collect();

    let common: Vec<u8> = row1_cells
        .iter()
        .filter(|c| row2_cells.contains(c))
        .copied()
        .collect();

    (common.clone(), common)
}

fn find_matching_rows(grid: &Grid, c1: u8, c2: u8, digit: u8) -> (Vec<u8>, Vec<u8>) {
    let col1_cells: Vec<u8> = COLS[c1 as usize]
        .cells
        .iter()
        .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
        .map(|&c| c / GRID_SIZE)
        .collect();
    let col2_cells: Vec<u8> = COLS[c2 as usize]
        .cells
        .iter()
        .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
        .map(|&c| c / GRID_SIZE)
        .collect();

    let common: Vec<u8> = col1_cells
        .iter()
        .filter(|c| col2_cells.contains(c))
        .copied()
        .collect();

    (common.clone(), common)
}

fn apply_cycle_elimination(
    grid: &Grid,
    acc: &mut HintAccumulator,
    digit: u8,
    idx1: u8,
    idx2: u8,
    pos1: u8,
    pos2: u8,
) {
    let c1 = idx1 * GRID_SIZE + pos1;
    let c2 = idx1 * GRID_SIZE + pos2;
    let c3 = idx2 * GRID_SIZE + pos1;
    let c4 = idx2 * GRID_SIZE + pos2;

    if grid.get(c1) != 0 || grid.get(c2) != 0 || grid.get(c3) != 0 || grid.get(c4) != 0 {
        return;
    }

    let mut eliminations = Vec::new();

    for r in 0..9 {
        if r == idx1 || r == idx2 {
            continue;
        }
        for &c in &[pos1, pos2] {
            let cell = r * GRID_SIZE + c;
            if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                eliminations.push((Cell::from(cell), vec![digit]));
            }
        }
    }

    if !eliminations.is_empty() {
        acc.add(Hint {
            hint_type: crate::solver::HintType::XCyclesSimple,
            difficulty: DIFFICULTY,
            technique_name: "X-Cycles".to_string(),
            description: format!(
                "X-Cycle: digit {} in rows {},{} cols {},{}",
                digit,
                idx1 + 1,
                idx2 + 1,
                pos1 + 1,
                pos2 + 1
            ),
            cell: Cell::from(c1),
            value: 0,
            eliminations,
        });
    }
}
