// solver.rs: 数独解题算法 - bitmask + MRV优化

use crate::board::{Cell, Grid};

const ALL_VALUES: u16 = 0x3FE;

#[derive(Clone)]
struct BitmaskGrid {
    rows: [u16; 9],
    cols: [u16; 9],
    boxes: [u16; 9],
}

impl BitmaskGrid {
    fn new() -> Self {
        Self {
            rows: [0; 9],
            cols: [0; 9],
            boxes: [0; 9],
        }
    }

    fn from_grid(grid: &Grid) -> Self {
        let mut masks = Self::new();
        for (r, row) in grid.iter().take(9).enumerate() {
            for (c, cell) in row.iter().take(9).enumerate() {
                if let Some(v) = cell.value() {
                    let bit = 1u16 << v;
                    masks.rows[r] |= bit;
                    masks.cols[c] |= bit;
                    masks.boxes[(r / 3) * 3 + c / 3] |= bit;
                }
            }
        }
        masks
    }

    #[inline]
    fn candidates(&self, r: usize, c: usize) -> u16 {
        let b = (r / 3) * 3 + c / 3;
        ALL_VALUES & !(self.rows[r] | self.cols[c] | self.boxes[b])
    }

    #[inline]
    fn place(&mut self, r: usize, c: usize, v: u8) {
        let bit = 1u16 << v;
        self.rows[r] |= bit;
        self.cols[c] |= bit;
        self.boxes[(r / 3) * 3 + c / 3] |= bit;
    }

    #[inline]
    fn remove(&mut self, r: usize, c: usize, v: u8) {
        let bit = 1u16 << v;
        self.rows[r] &= !bit;
        self.cols[c] &= !bit;
        self.boxes[(r / 3) * 3 + c / 3] &= !bit;
    }
}

#[inline]
fn get_value(grid: &Grid, idx: usize) -> Option<u8> {
    grid[idx / 9][idx % 9].value()
}

fn solve_inner(grid: &mut Grid, masks: &mut BitmaskGrid) -> bool {
    let mut best_idx = 81;
    let mut best_mask = 0u16;
    let mut best_count = 10;

    let mut idx = 0;
    while idx < 81 {
        if get_value(grid, idx).is_none() {
            let mask = masks.candidates(idx / 9, idx % 9);
            let count = mask.count_ones() as usize;
            if count < best_count {
                best_count = count;
                best_idx = idx;
                best_mask = mask;
                if count == 1 {
                    break;
                }
            }
        }
        idx += 1;
    }

    if best_idx >= 81 {
        return true;
    }

    let (r, c) = (best_idx / 9, best_idx % 9);

    let mut mask = best_mask;
    while mask != 0 {
        let lsb = mask & mask.wrapping_neg();
        mask &= mask - 1;
        let val = lsb.trailing_zeros() as u8;

        grid[r][c] = Cell::Given(val);
        masks.place(r, c, val);

        if solve_inner(grid, masks) {
            return true;
        }

        grid[r][c] = Cell::Empty;
        masks.remove(r, c, val);
    }

    false
}

pub fn solve(grid: &mut Grid) -> bool {
    let mut masks = BitmaskGrid::from_grid(grid);
    solve_inner(grid, &mut masks)
}

fn count_inner(grid: &mut Grid, masks: &mut BitmaskGrid, count: &mut usize, max: usize) {
    if *count >= max {
        return;
    }

    let mut best_idx = 81;
    let mut best_mask = 0u16;
    let mut best_count = 10;

    let mut idx = 0;
    while idx < 81 {
        if get_value(grid, idx).is_none() {
            let mask = masks.candidates(idx / 9, idx % 9);
            let count_cands = mask.count_ones() as usize;
            if count_cands < best_count {
                best_count = count_cands;
                best_idx = idx;
                best_mask = mask;
                if count_cands == 1 {
                    break;
                }
            }
        }
        idx += 1;
    }

    if best_idx >= 81 {
        *count += 1;
        return;
    }

    let (r, c) = (best_idx / 9, best_idx % 9);

    let mut mask = best_mask;
    while mask != 0 {
        let lsb = mask & mask.wrapping_neg();
        mask &= mask - 1;
        let val = lsb.trailing_zeros() as u8;

        grid[r][c] = Cell::Given(val);
        masks.place(r, c, val);

        count_inner(grid, masks, count, max);

        grid[r][c] = Cell::Empty;
        masks.remove(r, c, val);
    }
}

pub fn count_solutions(grid: &mut Grid) -> usize {
    let mut masks = BitmaskGrid::from_grid(grid);
    let mut count = 0;
    count_inner(grid, &mut masks, &mut count, 2);
    count
}
