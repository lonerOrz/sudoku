// hints.rs: 数独提示技巧 - bitmask优化版本

#![allow(clippy::needless_range_loop)]

use crate::board::Grid;

const ALL_VALUES: u16 = 0x3FE;

#[derive(Clone)]
struct BitmaskGrid {
    rows: [u16; 9],
    cols: [u16; 9],
    boxes: [u16; 9],
}

impl BitmaskGrid {
    fn from_grid(grid: &Grid) -> Self {
        let mut masks = Self {
            rows: [0; 9],
            cols: [0; 9],
            boxes: [0; 9],
        };
        for (r, row) in grid.iter().enumerate().take(9) {
            for (c, cell) in row.iter().enumerate().take(9) {
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
    fn count_bits(mask: u16) -> u8 {
        mask.count_ones() as u8
    }
}

pub fn possible_values(grid: &Grid, row: usize, col: usize) -> Vec<u8> {
    if grid[row][col].value().is_some() {
        return vec![];
    }
    let masks = BitmaskGrid::from_grid(grid);
    let mask = masks.candidates(row, col);
    let mut result = Vec::with_capacity(9);
    let mut m = mask;
    while m != 0 {
        let lsb = m & m.wrapping_neg();
        m &= m - 1;
        let val = lsb.trailing_zeros() as u8;
        result.push(val);
    }
    result
}

pub fn find_naked_single(grid: &Grid) -> Option<((usize, usize), u8)> {
    let masks = BitmaskGrid::from_grid(grid);
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                let mask = masks.candidates(r, c);
                if BitmaskGrid::count_bits(mask) == 1 {
                    let val = mask.trailing_zeros() as u8;
                    return Some(((r, c), val));
                }
            }
        }
    }
    None
}

pub fn find_hidden_single(grid: &Grid) -> Option<((usize, usize), u8)> {
    let masks = BitmaskGrid::from_grid(grid);

    for r in 0..9 {
        for val in 1..=9 {
            let bit = 1u16 << val;
            if masks.rows[r] & bit == 0 {
                let mut count = 0u8;
                let mut pos = 0usize;
                for c in 0..9 {
                    if grid[r][c].value().is_none()
                        && masks.cols[c] & bit == 0
                        && masks.boxes[(r / 3) * 3 + c / 3] & bit == 0
                    {
                        count += 1;
                        pos = c;
                        if count > 1 {
                            break;
                        }
                    }
                }
                if count == 1 {
                    return Some(((r, pos), val));
                }
            }
        }
    }

    for c in 0..9 {
        for val in 1..=9 {
            let bit = 1u16 << val;
            if masks.cols[c] & bit == 0 {
                let mut count = 0u8;
                let mut pos = 0usize;
                for r in 0..9 {
                    if grid[r][c].value().is_none()
                        && masks.rows[r] & bit == 0
                        && masks.boxes[(r / 3) * 3 + c / 3] & bit == 0
                    {
                        count += 1;
                        pos = r;
                        if count > 1 {
                            break;
                        }
                    }
                }
                if count == 1 {
                    return Some(((pos, c), val));
                }
            }
        }
    }

    for box_r in (0..9).step_by(3) {
        for box_c in (0..9).step_by(3) {
            for val in 1..=9 {
                let bit = 1u16 << val;
                let b = (box_r / 3) * 3 + box_c / 3;
                if masks.boxes[b] & bit == 0 {
                    let mut count = 0u8;
                    let mut pos = (0usize, 0usize);
                    for dr in 0..3 {
                        for dc in 0..3 {
                            let r = box_r + dr;
                            let c = box_c + dc;
                            if grid[r][c].value().is_none()
                                && masks.rows[r] & bit == 0
                                && masks.cols[c] & bit == 0
                            {
                                count += 1;
                                pos = (r, c);
                                if count > 1 {
                                    break;
                                }
                            }
                        }
                        if count > 1 {
                            break;
                        }
                    }
                    if count == 1 {
                        return Some((pos, val));
                    }
                }
            }
        }
    }

    None
}
