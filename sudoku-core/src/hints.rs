// hints.rs: 数独提示技巧

#![allow(clippy::needless_range_loop)]

use crate::board::{BitmaskGrid, Grid, Solution};

#[derive(Clone, Debug)]
pub struct Clue {
    pub target_row: usize,
    pub target_col: usize,
    pub value: u8,
    pub technique: &'static str,
}

pub fn find_clue(grid: &Grid, solution: &Solution) -> Option<Clue> {
    if let Some(clue) = find_naked_single(grid) {
        return Some(clue);
    }
    if let Some(clue) = find_hidden_single(grid) {
        return Some(clue);
    }
    find_direct_reveal(grid, solution)
}

pub fn find_naked_single(grid: &Grid) -> Option<Clue> {
    let masks = BitmaskGrid::from_grid(grid);
    for r in 0..9 {
        for c in 0..9 {
            let mask = masks.candidates(r, c);
            if mask.count_ones() == 1 {
                let val = mask.trailing_zeros() as u8;
                return Some(Clue {
                    target_row: r,
                    target_col: c,
                    value: val,
                    technique: "Naked Single",
                });
            }
        }
    }
    None
}

pub fn find_hidden_single(grid: &Grid) -> Option<Clue> {
    let masks = BitmaskGrid::from_grid(grid);

    let mut row_empty = [0u16; 9];
    let mut col_empty = [0u16; 9];
    let mut box_empty = [0u16; 9];
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                let bit = 1u16 << c;
                row_empty[r] |= bit;
                col_empty[c] |= bit;
                let b = (r / 3) * 3 + c / 3;
                box_empty[b] |= bit;
            }
        }
    }

    for r in 0..9 {
        for val in 1..=9 {
            let bit = 1u16 << val;
            if masks.rows[r] & bit == 0 {
                let pos = row_empty[r]
                    & !col_bitmask_for_val(&masks, val)
                    & !box_bitmask_for_val_in_row(&masks, r, val);
                if pos.count_ones() == 1 {
                    let c = pos.trailing_zeros() as usize;
                    return Some(Clue {
                        target_row: r,
                        target_col: c,
                        value: val,
                        technique: "Hidden Single",
                    });
                }
            }
        }
    }

    for c in 0..9 {
        for val in 1..=9 {
            let bit = 1u16 << val;
            if masks.cols[c] & bit == 0 {
                let pos = col_empty[c]
                    & !row_bitmask_for_val(&masks, val)
                    & !box_bitmask_for_val_in_col(&masks, c, val);
                if pos.count_ones() == 1 {
                    let r = pos.trailing_zeros() as usize;
                    return Some(Clue {
                        target_row: r,
                        target_col: c,
                        value: val,
                        technique: "Hidden Single",
                    });
                }
            }
        }
    }

    for box_r in (0..9).step_by(3) {
        for box_c in (0..9).step_by(3) {
            let b = (box_r / 3) * 3 + box_c / 3;
            let mut box_pos = 0u16;
            for dr in 0..3 {
                for dc in 0..3 {
                    let r = box_r + dr;
                    let c = box_c + dc;
                    if grid[r][c].value().is_none() {
                        box_pos |= 1u16 << (dr * 3 + dc);
                    }
                }
            }

            for val in 1..=9 {
                let bit = 1u16 << val;
                if masks.boxes[b] & bit == 0 {
                    let pos = box_pos
                        & !box_row_mask_for_val(&masks, box_r, val)
                        & !box_col_mask_for_val(&masks, box_c, val);
                    if pos.count_ones() == 1 {
                        let idx = pos.trailing_zeros() as usize;
                        let dr = idx / 3;
                        let dc = idx % 3;
                        return Some(Clue {
                            target_row: box_r + dr,
                            target_col: box_c + dc,
                            value: val,
                            technique: "Hidden Single",
                        });
                    }
                }
            }
        }
    }

    None
}

#[inline]
fn col_bitmask_for_val(masks: &BitmaskGrid, val: u8) -> u16 {
    let bit = 1u16 << val;
    let mut result = 0u16;
    for c in 0..9 {
        if masks.cols[c] & bit != 0 {
            result |= 1u16 << c;
        }
    }
    result
}

#[inline]
fn box_bitmask_for_val_in_row(masks: &BitmaskGrid, r: usize, val: u8) -> u16 {
    let bit = 1u16 << val;
    let box_row = r / 3;
    let mut result = 0u16;
    for dc in 0..3 {
        let c = box_row * 3 + dc;
        if masks.boxes[(r / 3) * 3 + c / 3] & bit != 0 {
            result |= 1u16 << c;
        }
    }
    result
}

#[inline]
fn row_bitmask_for_val(masks: &BitmaskGrid, val: u8) -> u16 {
    let bit = 1u16 << val;
    let mut result = 0u16;
    for r in 0..9 {
        if masks.rows[r] & bit != 0 {
            result |= 1u16 << r;
        }
    }
    result
}

#[inline]
fn box_bitmask_for_val_in_col(masks: &BitmaskGrid, c: usize, val: u8) -> u16 {
    let bit = 1u16 << val;
    let box_col = c / 3;
    let mut result = 0u16;
    for dr in 0..3 {
        let r = box_col * 3 + dr;
        if masks.boxes[(r / 3) * 3 + c / 3] & bit != 0 {
            result |= 1u16 << r;
        }
    }
    result
}

#[inline]
fn box_row_mask_for_val(masks: &BitmaskGrid, box_r: usize, val: u8) -> u16 {
    let bit = 1u16 << val;
    let box_start = box_r / 3 * 3;
    let mut result = 0u16;
    for dc in 0..3 {
        let c = box_start + dc;
        if masks.cols[c] & bit != 0 {
            for dr in 0..3 {
                result |= 1u16 << (dr * 3 + dc);
            }
        }
    }
    result
}

#[inline]
fn box_col_mask_for_val(masks: &BitmaskGrid, box_c: usize, val: u8) -> u16 {
    let bit = 1u16 << val;
    let box_start = box_c / 3 * 3;
    let mut result = 0u16;
    for dr in 0..3 {
        let r = box_start + dr;
        if masks.rows[r] & bit != 0 {
            for dc in 0..3 {
                result |= 1u16 << (dr * 3 + dc);
            }
        }
    }
    result
}

pub fn find_direct_reveal(grid: &Grid, solution: &Solution) -> Option<Clue> {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                return Some(Clue {
                    target_row: r,
                    target_col: c,
                    value: solution[r][c],
                    technique: "Direct",
                });
            }
        }
    }
    None
}
