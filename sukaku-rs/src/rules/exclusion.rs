use crate::grid::{Cell, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

fn is_visible(cell1: u8, cell2: u8) -> bool {
    if cell1 == cell2 {
        return false;
    }
    let row1 = cell1 / 9;
    let row2 = cell2 / 9;
    if row1 == row2 {
        return true;
    }
    let col1 = cell1 % 9;
    let col2 = cell2 % 9;
    if col1 == col2 {
        return true;
    }
    let block1 = (cell1 / 27) * 3 + (cell1 % 9) / 3;
    let block2 = (cell2 / 27) * 3 + (cell2 % 9) / 3;
    block1 == block2
}

/// Find Aligned Pair Exclusion patterns.
/// Difficulty: SE 6.2
pub fn aligned_pair_exclusion(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        find_ape_in_rows(grid, acc, digit);
        find_ape_in_cols(grid, acc, digit);
        find_ape_in_blocks(grid, acc, digit);
    }
}

fn find_ape_in_rows(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for (row_idx, row) in ROWS.iter().enumerate() {
        let cells_with_digit: Vec<u8> = row
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
            .collect();

        if cells_with_digit.len() < 2 {
            continue;
        }

        for i in 0..cells_with_digit.len() {
            for j in (i + 1)..cells_with_digit.len() {
                let cell1 = cells_with_digit[i];
                let cell2 = cells_with_digit[j];

                let c1 = cell1 % 9;
                let c2 = cell2 % 9;

                let common_peers = get_common_peers(cell1, cell2);
                for &peer in &common_peers {
                    if grid.get(peer) != 0 {
                        continue;
                    }
                    let peer_cands = grid.candidates(peer);
                    if peer_cands.has(digit) {
                        let elim_cands: Vec<u8> = peer_cands
                            .iter()
                            .filter(|_| !is_visible(cell1, peer) || !is_visible(cell2, peer))
                            .collect();

                        if !elim_cands.is_empty() {
                            let desc = format!(
                                "Aligned Pair Exclusion: digit {} in row {} cells ({},{}) and ({},{}) -> eliminate from ({},{})",
                                digit,
                                row_idx + 1,
                                cell1 / 9 + 1,
                                c1 + 1,
                                cell2 / 9 + 1,
                                c2 + 1,
                                peer / 9 + 1,
                                peer % 9 + 1
                            );

                            acc.add(Hint {
                                hint_type: crate::solver::HintType::AlignedPairExclusion,
                                difficulty: 6.2,
                                technique_name: "Aligned Pair Exclusion".to_string(),
                                description: desc,
                                cell: Cell::from(cell1),
                                value: 0,
                                eliminations: vec![(Cell::from(peer), elim_cands)],
                            });
                        }
                    }
                }
            }
        }
    }
}

fn find_ape_in_cols(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for (col_idx, col) in COLS.iter().enumerate() {
        let cells_with_digit: Vec<u8> = col
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
            .collect();

        if cells_with_digit.len() < 2 {
            continue;
        }

        for i in 0..cells_with_digit.len() {
            for j in (i + 1)..cells_with_digit.len() {
                let cell1 = cells_with_digit[i];
                let cell2 = cells_with_digit[j];

                let r1 = cell1 / 9;
                let r2 = cell2 / 9;

                let common_peers = get_common_peers(cell1, cell2);
                for &peer in &common_peers {
                    if grid.get(peer) != 0 {
                        continue;
                    }
                    let peer_cands = grid.candidates(peer);
                    if peer_cands.has(digit) {
                        let elim_cands: Vec<u8> = peer_cands
                            .iter()
                            .filter(|_| !is_visible(cell1, peer) || !is_visible(cell2, peer))
                            .collect();

                        if !elim_cands.is_empty() {
                            let desc = format!(
                                "Aligned Pair Exclusion: digit {} in column {} cells ({},{}) and ({},{})",
                                digit,
                                col_idx + 1,
                                r1 + 1,
                                cell1 % 9 + 1,
                                r2 + 1,
                                cell2 % 9 + 1
                            );

                            acc.add(Hint {
                                hint_type: crate::solver::HintType::AlignedPairExclusion,
                                difficulty: 6.2,
                                technique_name: "Aligned Pair Exclusion".to_string(),
                                description: desc,
                                cell: Cell::from(cell1),
                                value: 0,
                                eliminations: vec![(Cell::from(peer), elim_cands)],
                            });
                        }
                    }
                }
            }
        }
    }
}

fn find_ape_in_blocks(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for (block_idx, block) in BLOCKS.iter().enumerate() {
        let cells_with_digit: Vec<u8> = block
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
            .collect();

        if cells_with_digit.len() < 2 {
            continue;
        }

        for i in 0..cells_with_digit.len() {
            for j in (i + 1)..cells_with_digit.len() {
                let cell1 = cells_with_digit[i];
                let cell2 = cells_with_digit[j];

                let common_peers = get_common_peers(cell1, cell2);
                for &peer in &common_peers {
                    if grid.get(peer) != 0 {
                        continue;
                    }
                    let peer_cands = grid.candidates(peer);
                    if peer_cands.has(digit) {
                        let elim_cands: Vec<u8> = peer_cands
                            .iter()
                            .filter(|_| !is_visible(cell1, peer) || !is_visible(cell2, peer))
                            .collect();

                        if !elim_cands.is_empty() {
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::AlignedPairExclusion,
                                difficulty: 6.2,
                                technique_name: "Aligned Pair Exclusion".to_string(),
                                description: format!(
                                    "Aligned Pair Exclusion in block {}",
                                    block_idx + 1
                                ),
                                cell: Cell::from(cell1),
                                value: 0,
                                eliminations: vec![(Cell::from(peer), elim_cands)],
                            });
                        }
                    }
                }
            }
        }
    }
}

fn get_common_peers(cell1: u8, cell2: u8) -> Vec<u8> {
    let mut peers = Vec::new();
    for i in 0..81 {
        if i != cell1 && i != cell2 && is_visible(cell1, i) && is_visible(cell2, i) {
            peers.push(i);
        }
    }
    peers
}

/// Find Aligned Triplet Exclusion patterns.
/// Difficulty: SE 7.5
pub fn aligned_triplet_exclusion(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        find_ate_in_rows(grid, acc, digit);
        find_ate_in_cols(grid, acc, digit);
    }
}

fn find_ate_in_rows(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for (row_idx, row) in ROWS.iter().enumerate() {
        let cells_with_digit: Vec<u8> = row
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
            .collect();

        if cells_with_digit.len() < 3 {
            continue;
        }

        for i in 0..cells_with_digit.len() {
            for j in (i + 1)..cells_with_digit.len() {
                for k in (j + 1)..cells_with_digit.len() {
                    let cell1 = cells_with_digit[i];
                    let cell2 = cells_with_digit[j];
                    let cell3 = cells_with_digit[k];

                    let common_peers = get_triplet_common_peers(cell1, cell2, cell3);
                    for &peer in &common_peers {
                        if grid.get(peer) != 0 {
                            continue;
                        }
                        let peer_cands = grid.candidates(peer);
                        if peer_cands.has(digit) {
                            let elim_cands: Vec<u8> = peer_cands
                                .iter()
                                .filter(|_| {
                                    !is_visible(cell1, peer)
                                        || !is_visible(cell2, peer)
                                        || !is_visible(cell3, peer)
                                })
                                .collect();

                            if !elim_cands.is_empty() {
                                acc.add(Hint {
                                    hint_type: crate::solver::HintType::AlignedTripletExclusion,
                                    difficulty: 7.5,
                                    technique_name: "Aligned Triplet Exclusion".to_string(),
                                    description: format!(
                                        "Aligned Triplet Exclusion: digit {} in row {}",
                                        digit,
                                        row_idx + 1
                                    ),
                                    cell: Cell::from(cell1),
                                    value: 0,
                                    eliminations: vec![(Cell::from(peer), elim_cands)],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

fn find_ate_in_cols(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for (col_idx, col) in COLS.iter().enumerate() {
        let cells_with_digit: Vec<u8> = col
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
            .collect();

        if cells_with_digit.len() < 3 {
            continue;
        }

        for i in 0..cells_with_digit.len() {
            for j in (i + 1)..cells_with_digit.len() {
                for k in (j + 1)..cells_with_digit.len() {
                    let cell1 = cells_with_digit[i];
                    let cell2 = cells_with_digit[j];
                    let cell3 = cells_with_digit[k];

                    let common_peers = get_triplet_common_peers(cell1, cell2, cell3);
                    for &peer in &common_peers {
                        if grid.get(peer) != 0 {
                            continue;
                        }
                        let peer_cands = grid.candidates(peer);
                        if peer_cands.has(digit) {
                            let elim_cands: Vec<u8> = peer_cands
                                .iter()
                                .filter(|_| {
                                    !is_visible(cell1, peer)
                                        || !is_visible(cell2, peer)
                                        || !is_visible(cell3, peer)
                                })
                                .collect();

                            if !elim_cands.is_empty() {
                                acc.add(Hint {
                                    hint_type: crate::solver::HintType::AlignedTripletExclusion,
                                    difficulty: 7.5,
                                    technique_name: "Aligned Triplet Exclusion".to_string(),
                                    description: format!(
                                        "Aligned Triplet Exclusion: digit {} in column {}",
                                        digit,
                                        col_idx + 1
                                    ),
                                    cell: Cell::from(cell1),
                                    value: 0,
                                    eliminations: vec![(Cell::from(peer), elim_cands)],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_triplet_common_peers(cell1: u8, cell2: u8, cell3: u8) -> Vec<u8> {
    let mut peers = Vec::new();
    for i in 0..81 {
        if i != cell1
            && i != cell2
            && i != cell3
            && is_visible(cell1, i)
            && is_visible(cell2, i)
            && is_visible(cell3, i)
        {
            peers.push(i);
        }
    }
    peers
}
