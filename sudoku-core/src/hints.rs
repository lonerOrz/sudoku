// hints.rs: 数独提示技巧

#![allow(clippy::needless_range_loop)]

use crate::board::{BitmaskGrid, Grid};

pub fn find_naked_single(grid: &Grid) -> Option<((usize, usize), u8)> {
    let masks = BitmaskGrid::from_grid(grid);
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].value().is_none() {
                let mask = masks.candidates(r, c);
                if mask.count_ones() == 1 {
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
