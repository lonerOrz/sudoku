// checker.rs: 数独验证规则

use crate::board::is_valid as board_is_valid;
use crate::board::{Grid, PEERS};

pub fn is_solved(grid: &Grid) -> bool {
    (0..81).all(|idx| match grid[idx / 9][idx % 9].value() {
        Some(val) => board_is_valid(grid, idx, val),
        None => false,
    })
}

pub fn has_empty(grid: &Grid) -> bool {
    (0..81).any(|idx| grid[idx / 9][idx % 9].value().is_none())
}

pub fn possible_values(grid: &Grid, row: usize, col: usize) -> Vec<u8> {
    let idx = row * 9 + col;
    if grid[row][col].value().is_some() {
        return vec![];
    }
    (1..=9)
        .filter(|&val| board_is_valid(grid, idx, val))
        .collect()
}

pub fn find_errors(grid: &Grid) -> Vec<(usize, usize)> {
    let mut errors = Vec::new();
    for (idx, peers) in PEERS.iter().enumerate() {
        let r = idx / 9;
        let c = idx % 9;
        if let Some(val) = grid[r][c].value() {
            for &peer in peers {
                if let Some(other) = grid[(peer / 9) as usize][(peer % 9) as usize].value()
                    && other == val
                {
                    errors.push((r, c));
                    break;
                }
            }
        }
    }
    errors
}
