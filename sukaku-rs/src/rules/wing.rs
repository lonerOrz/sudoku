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

fn common_peers(cell1: u8, cell2: u8) -> Vec<u8> {
    let mut peers: Vec<u8> = Vec::new();

    let row1 = cell1 / 9;
    let col1 = cell1 % 9;
    let block1 = (cell1 / 27) * 3 + (col1 / 3);

    let row2 = cell2 / 9;
    let col2 = cell2 % 9;
    let block2 = (cell2 / 27) * 3 + (col2 / 3);

    for i in 0..81 {
        if i == cell1 || i == cell2 {
            continue;
        }

        let row_i = i / 9;
        let col_i = i % 9;
        let block_i = (i / 27) * 3 + (col_i / 3);

        let visible_to_1 = row_i == row1 || col_i == col1 || block_i == block1;
        let visible_to_2 = row_i == row2 || col_i == col2 || block_i == block2;

        if visible_to_1 && visible_to_2 {
            peers.push(i);
        }
    }

    peers
}

/// Find XY-Wing patterns: three cells forming a Y-shape where a digit can be eliminated.
pub fn xy_wing(grid: &Grid, acc: &mut HintAccumulator) {
    for pivot_idx in 0..81 {
        if grid.get(pivot_idx) != 0 {
            continue;
        }

        let pivot_cands = grid.candidates(pivot_idx);
        if pivot_cands.cardinality() != 2 {
            continue;
        }

        let pivot_values: Vec<u8> = pivot_cands.iter().collect();
        let x = pivot_values[0];
        let y = pivot_values[1];

        let pivot_row = pivot_idx / 9;
        let pivot_col = pivot_idx % 9;
        let _pivot_block = (pivot_idx / 27) * 3 + (pivot_col / 3);

        for cell1 in 0..81 {
            if cell1 == pivot_idx || grid.get(cell1) != 0 {
                continue;
            }

            let cands1 = grid.candidates(cell1);
            if cands1.cardinality() != 2 {
                continue;
            }

            let values1: Vec<u8> = cands1.iter().collect();

            let has_x = values1.contains(&x);
            let has_z1 = values1.iter().find(|&&v| v != x && v != y).copied();

            if !has_x || has_z1.is_none() {
                continue;
            }

            let z = has_z1.unwrap();
            let wing1_idx = cell1;

            let visible_to_pivot = is_visible(pivot_idx, cell1);
            if !visible_to_pivot {
                continue;
            }

            for cell2 in 0..81 {
                if cell2 == pivot_idx || cell2 == cell1 || grid.get(cell2) != 0 {
                    continue;
                }

                let cands2 = grid.candidates(cell2);
                if cands2.cardinality() != 2 {
                    continue;
                }

                let values2: Vec<u8> = cands2.iter().collect();

                let has_y = values2.contains(&y);
                let has_z2 = values2.iter().find(|&&v| v != x && v != y).copied();

                if !has_y || has_z2.is_none() || has_z2.unwrap() != z {
                    continue;
                }

                let visible_to_pivot2 = is_visible(pivot_idx, cell2);
                if !visible_to_pivot2 {
                    continue;
                }

                let wing2_idx = cell2;

                let targets = common_peers(wing1_idx, wing2_idx);
                let mut eliminations = Vec::new();

                for &target in &targets {
                    if grid.get(target) != 0 {
                        continue;
                    }
                    if grid.candidates(target).has(z) {
                        eliminations.push((Cell::from(target), vec![z]));
                    }
                }

                if !eliminations.is_empty() {
                    let desc = format!(
                        "XY-Wing pivot ({},{}) wings ({},{}) ({}) and ({},{}) ({}) -> eliminate {}",
                        pivot_row + 1,
                        pivot_col + 1,
                        cell1 / 9 + 1,
                        cell1 % 9 + 1,
                        x,
                        cell2 / 9 + 1,
                        cell2 % 9 + 1,
                        y,
                        z
                    );
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::XYWing,
                        difficulty: 4.2,
                        technique_name: "XY-Wing".to_string(),
                        description: desc,
                        cell: Cell::from(pivot_idx),
                        value: 0,
                        eliminations,
                    });
                }
            }
        }
    }
}

/// Find XYZ-Wing patterns: three cells forming a Y-shape with a three-candidate pivot.
pub fn xyz_wing(grid: &Grid, acc: &mut HintAccumulator) {
    for pivot_idx in 0..81 {
        if grid.get(pivot_idx) != 0 {
            continue;
        }

        let pivot_cands = grid.candidates(pivot_idx);
        if pivot_cands.cardinality() != 3 {
            continue;
        }

        let pivot_values: Vec<u8> = pivot_cands.iter().collect();
        let x = pivot_values[0];
        let y = pivot_values[1];
        let z = pivot_values[2];

        for cell1 in 0..81 {
            if cell1 == pivot_idx || grid.get(cell1) != 0 {
                continue;
            }

            let cands1 = grid.candidates(cell1);
            if cands1.cardinality() != 2 {
                continue;
            }

            let values1: Vec<u8> = cands1.iter().collect();

            let has_x = values1.contains(&x);
            let has_z1 = values1
                .iter()
                .find(|&&v| v != x && v != y && v != z)
                .copied();

            if !has_x || has_z1.is_none() {
                continue;
            }

            let wing1_idx = cell1;

            let visible_to_pivot = is_visible(pivot_idx, cell1);
            if !visible_to_pivot {
                continue;
            }

            for cell2 in 0..81 {
                if cell2 == pivot_idx || cell2 == cell1 || grid.get(cell2) != 0 {
                    continue;
                }

                let cands2 = grid.candidates(cell2);
                if cands2.cardinality() != 2 {
                    continue;
                }

                let values2: Vec<u8> = cands2.iter().collect();

                let has_y = values2.contains(&y);
                let has_z2 = values2
                    .iter()
                    .find(|&&v| v != x && v != y && v != z)
                    .copied();

                if !has_y || has_z2.is_none() || has_z2.unwrap() != z {
                    continue;
                }

                let visible_to_pivot2 = is_visible(pivot_idx, cell2);
                if !visible_to_pivot2 {
                    continue;
                }

                let wing2_idx = cell2;

                let pivot_row = (pivot_idx / 9) as usize;
                let pivot_col = (pivot_idx % 9) as usize;
                let pivot_block = ((pivot_idx / 27) * 3 + (pivot_idx % 9) / 3) as usize;

                let pivot_peers = ROWS[pivot_row]
                    .cells
                    .iter()
                    .chain(COLS[pivot_col].cells.iter())
                    .chain(BLOCKS[pivot_block].cells.iter())
                    .filter(|&&idx| idx != pivot_idx)
                    .copied()
                    .collect::<Vec<_>>();

                let mut targets: Vec<u8> = Vec::new();

                let wing1_row = wing1_idx / 9;
                let wing1_col = wing1_idx % 9;
                let wing1_block = (wing1_idx / 27) * 3 + (wing1_col / 3);

                let wing2_row = wing2_idx / 9;
                let wing2_col = wing2_idx % 9;
                let wing2_block = (wing2_idx / 27) * 3 + (wing2_col / 3);

                for &peer in &pivot_peers {
                    if peer == wing1_idx || peer == wing2_idx {
                        continue;
                    }

                    let peer_row = peer / 9;
                    let peer_col = peer % 9;
                    let peer_block = (peer / 27) * 3 + (peer_col / 3);

                    let visible_to_wing1 =
                        peer_row == wing1_row || peer_col == wing1_col || peer_block == wing1_block;
                    let visible_to_wing2 =
                        peer_row == wing2_row || peer_col == wing2_col || peer_block == wing2_block;

                    if visible_to_wing1 && visible_to_wing2 {
                        targets.push(peer);
                    }
                }

                let mut eliminations = Vec::new();
                for &target in &targets {
                    if grid.get(target) != 0 {
                        continue;
                    }
                    if grid.candidates(target).has(z) {
                        eliminations.push((Cell::from(target), vec![z]));
                    }
                }

                if !eliminations.is_empty() {
                    let desc = format!(
                        "XYZ-Wing pivot ({},{}) {{{},{},{}}} -> eliminate {}",
                        pivot_idx / 9 + 1,
                        pivot_idx % 9 + 1,
                        x,
                        y,
                        z,
                        z
                    );
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::XYZWing,
                        difficulty: 4.4,
                        technique_name: "XYZ-Wing".to_string(),
                        description: desc,
                        cell: Cell::from(pivot_idx),
                        value: 0,
                        eliminations,
                    });
                }
            }
        }
    }
}

/// Find WXYZ-Wing patterns: four cells forming a wing structure with a four-candidate pivot.
pub fn wxyz_wing(grid: &Grid, acc: &mut HintAccumulator) {
    for pivot_idx in 0..81 {
        if grid.get(pivot_idx) != 0 {
            continue;
        }

        let pivot_cands = grid.candidates(pivot_idx);
        if pivot_cands.cardinality() != 4 {
            continue;
        }

        let pivot_values: Vec<u8> = pivot_cands.iter().collect();
        let w = pivot_values[0];
        let x = pivot_values[1];
        let y = pivot_values[2];
        let z = pivot_values[3];

        for cell1 in 0..81 {
            if cell1 == pivot_idx || grid.get(cell1) != 0 {
                continue;
            }

            let cands1 = grid.candidates(cell1);
            if cands1.cardinality() != 2 {
                continue;
            }

            let values1: Vec<u8> = cands1.iter().collect();

            let has_w = values1.contains(&w);
            let has_x = values1.contains(&x);
            if !has_w || !has_x {
                continue;
            }

            let visible_to_pivot1 = is_visible(pivot_idx, cell1);
            if !visible_to_pivot1 {
                continue;
            }

            for cell2 in 0..81 {
                if cell2 == pivot_idx || cell2 == cell1 || grid.get(cell2) != 0 {
                    continue;
                }

                let cands2 = grid.candidates(cell2);
                if cands2.cardinality() != 2 {
                    continue;
                }

                let values2: Vec<u8> = cands2.iter().collect();

                let has_w2 = values2.contains(&w);
                let has_y = values2.contains(&y);
                if !has_w2 || !has_y {
                    continue;
                }

                let visible_to_pivot2 = is_visible(pivot_idx, cell2);
                if !visible_to_pivot2 {
                    continue;
                }

                for cell3 in 0..81 {
                    if cell3 == pivot_idx
                        || cell3 == cell1
                        || cell3 == cell2
                        || grid.get(cell3) != 0
                    {
                        continue;
                    }

                    let cands3 = grid.candidates(cell3);
                    if cands3.cardinality() != 2 {
                        continue;
                    }

                    let values3: Vec<u8> = cands3.iter().collect();

                    let has_w3 = values3.contains(&w);
                    let has_z = values3.contains(&z);
                    if !has_w3 || !has_z {
                        continue;
                    }

                    let visible_to_pivot3 = is_visible(pivot_idx, cell3);
                    if !visible_to_pivot3 {
                        continue;
                    }

                    let targets = common_peers(cell1, cell2);
                    let targets2 = common_peers(cell1, cell3);
                    let targets3 = common_peers(cell2, cell3);

                    let mut common_targets: Vec<u8> = Vec::new();
                    for &t in &targets {
                        if targets2.contains(&t) && targets3.contains(&t) {
                            common_targets.push(t);
                        }
                    }

                    let mut eliminations = Vec::new();
                    for &target in &common_targets {
                        if grid.get(target) != 0 {
                            continue;
                        }
                        if grid.candidates(target).has(w) {
                            eliminations.push((Cell::from(target), vec![w]));
                        }
                    }

                    if !eliminations.is_empty() {
                        let desc = format!(
                            "WXYZ-Wing pivot ({},{}) {{{},{},{},{}}} -> eliminate {}",
                            pivot_idx / 9 + 1,
                            pivot_idx % 9 + 1,
                            w,
                            x,
                            y,
                            z,
                            w
                        );
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::WXYZWing,
                            difficulty: 5.5,
                            technique_name: "WXYZ-Wing".to_string(),
                            description: desc,
                            cell: Cell::from(pivot_idx),
                            value: 0,
                            eliminations,
                        });
                        return;
                    }
                }
            }
        }
    }
}
