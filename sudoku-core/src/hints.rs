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
                    return Some(Clue {
                        target_row: r,
                        target_col: pos,
                        value: val as u8,
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
                    return Some(Clue {
                        target_row: pos,
                        target_col: c,
                        value: val as u8,
                        technique: "Hidden Single",
                    });
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
                        return Some(Clue {
                            target_row: pos.0,
                            target_col: pos.1,
                            value: val as u8,
                            technique: "Hidden Single",
                        });
                    }
                }
            }
        }
    }

    None
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
