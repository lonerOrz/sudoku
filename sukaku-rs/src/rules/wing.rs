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
            let has_z_in_wing1 = values1.contains(&z);

            if !has_x || !has_z_in_wing1 || values1.len() != 2 {
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
                let has_z_in_wing2 = values2.contains(&z);

                if !has_y || !has_z_in_wing2 || values2.len() != 2 {
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

/// Find VWXYZ-Wing patterns: five cells forming a wing structure with a five-candidate pivot.
pub fn vwxyz_wing(grid: &Grid, acc: &mut HintAccumulator) {
    for pivot_idx in 0..81 {
        if grid.get(pivot_idx) != 0 {
            continue;
        }

        let pivot_cands = grid.candidates(pivot_idx);
        if pivot_cands.cardinality() != 5 {
            continue;
        }

        let pivot_values: Vec<u8> = pivot_cands.iter().collect();
        let v = pivot_values[0];
        let w = pivot_values[1];
        let x = pivot_values[2];
        let y = pivot_values[3];
        let z = pivot_values[4];

        for cell1 in 0..81 {
            if cell1 == pivot_idx || grid.get(cell1) != 0 {
                continue;
            }

            let cands1 = grid.candidates(cell1);
            if cands1.cardinality() != 2 {
                continue;
            }

            let values1: Vec<u8> = cands1.iter().collect();
            let has_v = values1.contains(&v);
            let has_w = values1.contains(&w);
            if !has_v || !has_w {
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
                let has_v2 = values2.contains(&v);
                let has_x = values2.contains(&x);
                if !has_v2 || !has_x {
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
                    let has_v3 = values3.contains(&v);
                    let has_y = values3.contains(&y);
                    if !has_v3 || !has_y {
                        continue;
                    }

                    let visible_to_pivot3 = is_visible(pivot_idx, cell3);
                    if !visible_to_pivot3 {
                        continue;
                    }

                    for cell4 in 0..81 {
                        if cell4 == pivot_idx
                            || cell4 == cell1
                            || cell4 == cell2
                            || cell4 == cell3
                            || grid.get(cell4) != 0
                        {
                            continue;
                        }

                        let cands4 = grid.candidates(cell4);
                        if cands4.cardinality() != 2 {
                            continue;
                        }

                        let values4: Vec<u8> = cands4.iter().collect();
                        let has_v4 = values4.contains(&v);
                        let has_z = values4.contains(&z);
                        if !has_v4 || !has_z {
                            continue;
                        }

                        let visible_to_pivot4 = is_visible(pivot_idx, cell4);
                        if !visible_to_pivot4 {
                            continue;
                        }

                        let targets1 = common_peers(cell1, cell2);
                        let targets2 = common_peers(cell1, cell3);
                        let targets3 = common_peers(cell1, cell4);
                        let targets4 = common_peers(cell2, cell3);
                        let targets5 = common_peers(cell2, cell4);
                        let targets6 = common_peers(cell3, cell4);

                        let mut common_targets: Vec<u8> = Vec::new();
                        for &t in &targets1 {
                            if targets2.contains(&t)
                                && targets3.contains(&t)
                                && targets4.contains(&t)
                                && targets5.contains(&t)
                                && targets6.contains(&t)
                            {
                                common_targets.push(t);
                            }
                        }

                        let mut eliminations = Vec::new();
                        for &target in &common_targets {
                            if grid.get(target) != 0 {
                                continue;
                            }
                            if grid.candidates(target).has(v) {
                                eliminations.push((Cell::from(target), vec![v]));
                            }
                        }

                        if !eliminations.is_empty() {
                            let desc = format!(
                                "VWXYZ-Wing pivot ({},{}) {{{},{},{},{},{}}} -> eliminate {}",
                                pivot_idx / 9 + 1,
                                pivot_idx % 9 + 1,
                                v,
                                w,
                                x,
                                y,
                                z,
                                v
                            );
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::VWXYZWing,
                                difficulty: 6.2,
                                technique_name: "VWXYZ-Wing".to_string(),
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
}

/// Find UVWXYZ-Wing patterns: six cells forming a wing structure with a six-candidate pivot.
pub fn uvwxyz_wing(grid: &Grid, acc: &mut HintAccumulator) {
    for pivot_idx in 0..81 {
        if grid.get(pivot_idx) != 0 {
            continue;
        }

        let pivot_cands = grid.candidates(pivot_idx);
        if pivot_cands.cardinality() != 6 {
            continue;
        }

        let pivot_values: Vec<u8> = pivot_cands.iter().collect();
        let u = pivot_values[0];

        let wing_configs = [
            (
                pivot_values[0],
                pivot_values[1],
                pivot_values[2],
                pivot_values[3],
                pivot_values[4],
            ),
            (
                pivot_values[0],
                pivot_values[1],
                pivot_values[2],
                pivot_values[3],
                pivot_values[5],
            ),
            (
                pivot_values[0],
                pivot_values[1],
                pivot_values[2],
                pivot_values[4],
                pivot_values[5],
            ),
            (
                pivot_values[0],
                pivot_values[1],
                pivot_values[3],
                pivot_values[4],
                pivot_values[5],
            ),
            (
                pivot_values[0],
                pivot_values[2],
                pivot_values[3],
                pivot_values[4],
                pivot_values[5],
            ),
            (
                pivot_values[1],
                pivot_values[2],
                pivot_values[3],
                pivot_values[4],
                pivot_values[5],
            ),
        ];

        for wing_vals in wing_configs.iter() {
            find_uvwxyz_wing_pattern(grid, acc, pivot_idx, u, *wing_vals);
        }
    }
}

fn find_uvwxyz_wing_pattern(
    grid: &Grid,
    acc: &mut HintAccumulator,
    pivot_idx: u8,
    elim_digit: u8,
    wing_vals: (u8, u8, u8, u8, u8),
) {
    let (w1, w2, w3, w4, w5) = wing_vals;
    let wing_set = [w1, w2, w3, w4, w5];

    let mut wing_cells: Vec<u8> = Vec::new();

    for i in 0..81 {
        if i == pivot_idx || grid.get(i) != 0 {
            continue;
        }
        if !is_visible(pivot_idx, i) {
            continue;
        }

        let cands = grid.candidates(i);
        if cands.cardinality() != 2 {
            continue;
        }

        let vals: Vec<u8> = cands.iter().collect();
        let has_elim = vals.contains(&elim_digit);

        if let Some(other_val) = vals.iter().find(|&&v| v != elim_digit) {
            if has_elim && wing_set.contains(other_val) {
                wing_cells.push(i);
            }
        }

        if wing_cells.len() >= 5 {
            break;
        }
    }

    if wing_cells.len() != 5 {
        return;
    }

    let targets1 = common_peers(wing_cells[0], wing_cells[1]);
    let targets2 = common_peers(wing_cells[0], wing_cells[2]);
    let targets3 = common_peers(wing_cells[0], wing_cells[3]);
    let targets4 = common_peers(wing_cells[0], wing_cells[4]);
    let targets5 = common_peers(wing_cells[1], wing_cells[2]);
    let targets6 = common_peers(wing_cells[1], wing_cells[3]);
    let targets7 = common_peers(wing_cells[1], wing_cells[4]);
    let targets8 = common_peers(wing_cells[2], wing_cells[3]);
    let targets9 = common_peers(wing_cells[2], wing_cells[4]);
    let targets10 = common_peers(wing_cells[3], wing_cells[4]);

    let mut common: Vec<u8> = Vec::new();
    for &t in &targets1 {
        if targets2.contains(&t)
            && targets3.contains(&t)
            && targets4.contains(&t)
            && targets5.contains(&t)
            && targets6.contains(&t)
            && targets7.contains(&t)
            && targets8.contains(&t)
            && targets9.contains(&t)
            && targets10.contains(&t)
        {
            common.push(t);
        }
    }

    let mut eliminations = Vec::new();
    for &target in &common {
        if grid.get(target) == 0 && grid.candidates(target).has(elim_digit) {
            eliminations.push((Cell::from(target), vec![elim_digit]));
        }
    }

    if !eliminations.is_empty() {
        let desc = format!(
            "UVWXYZ-Wing pivot ({},{}) -> eliminate {}",
            pivot_idx / 9 + 1,
            pivot_idx % 9 + 1,
            elim_digit
        );
        acc.add(Hint {
            hint_type: crate::solver::HintType::UVWXYZWing,
            difficulty: 6.6,
            technique_name: "UVWXYZ-Wing".to_string(),
            description: desc,
            cell: Cell::from(pivot_idx),
            value: 0,
            eliminations,
        });
    }
}
pub fn tuvwxyz_wing(grid: &Grid, acc: &mut HintAccumulator) {
    for pivot_idx in 0..81 {
        if grid.get(pivot_idx) != 0 {
            continue;
        }

        let pivot_cands = grid.candidates(pivot_idx);
        if pivot_cands.cardinality() != 7 {
            continue;
        }

        let pivot_values: Vec<u8> = pivot_cands.iter().collect();
        let elim_digit = pivot_values[0];

        let wing_configs = [
            (
                pivot_values[0],
                pivot_values[1],
                pivot_values[2],
                pivot_values[3],
                pivot_values[4],
                pivot_values[5],
            ),
            (
                pivot_values[0],
                pivot_values[1],
                pivot_values[2],
                pivot_values[3],
                pivot_values[4],
                pivot_values[6],
            ),
            (
                pivot_values[0],
                pivot_values[1],
                pivot_values[2],
                pivot_values[3],
                pivot_values[5],
                pivot_values[6],
            ),
            (
                pivot_values[0],
                pivot_values[1],
                pivot_values[2],
                pivot_values[4],
                pivot_values[5],
                pivot_values[6],
            ),
            (
                pivot_values[0],
                pivot_values[1],
                pivot_values[3],
                pivot_values[4],
                pivot_values[5],
                pivot_values[6],
            ),
            (
                pivot_values[0],
                pivot_values[2],
                pivot_values[3],
                pivot_values[4],
                pivot_values[5],
                pivot_values[6],
            ),
            (
                pivot_values[1],
                pivot_values[2],
                pivot_values[3],
                pivot_values[4],
                pivot_values[5],
                pivot_values[6],
            ),
        ];

        for wing_vals in wing_configs.iter() {
            find_tuvwxyz_wing_pattern(grid, acc, pivot_idx, elim_digit, *wing_vals);
        }
    }
}

fn find_tuvwxyz_wing_pattern(
    grid: &Grid,
    acc: &mut HintAccumulator,
    pivot_idx: u8,
    elim_digit: u8,
    wing_vals: (u8, u8, u8, u8, u8, u8),
) {
    let (w1, w2, w3, w4, w5, w6) = wing_vals;
    let wing_set = [w1, w2, w3, w4, w5, w6];

    let mut wing_cells: Vec<u8> = Vec::new();

    for i in 0..81 {
        if i == pivot_idx || grid.get(i) != 0 {
            continue;
        }
        if !is_visible(pivot_idx, i) {
            continue;
        }

        let cands = grid.candidates(i);
        if cands.cardinality() != 2 {
            continue;
        }

        let vals: Vec<u8> = cands.iter().collect();
        let has_elim = vals.contains(&elim_digit);

        if let Some(other_val) = vals.iter().find(|&&v| v != elim_digit) {
            if has_elim && wing_set.contains(other_val) {
                wing_cells.push(i);
            }
        }

        if wing_cells.len() >= 6 {
            break;
        }
    }

    if wing_cells.len() != 6 {
        return;
    }

    let mut all_targets: Vec<Vec<u8>> = Vec::new();
    for i in 0..6 {
        for j in (i + 1)..6 {
            all_targets.push(common_peers(wing_cells[i], wing_cells[j]));
        }
    }

    let mut common: Vec<u8> = Vec::new();
    for &t in &all_targets[0] {
        let mut is_common = true;
        for target_list in &all_targets {
            if !target_list.contains(&t) {
                is_common = false;
                break;
            }
        }
        if is_common {
            common.push(t);
        }
    }

    let mut eliminations = Vec::new();
    for &target in &common {
        if grid.get(target) == 0 && grid.candidates(target).has(elim_digit) {
            eliminations.push((Cell::from(target), vec![elim_digit]));
        }
    }

    if !eliminations.is_empty() {
        let desc = format!(
            "TUVWXYZ-Wing pivot ({},{}) -> eliminate {}",
            pivot_idx / 9 + 1,
            pivot_idx % 9 + 1,
            elim_digit
        );
        acc.add(Hint {
            hint_type: crate::solver::HintType::TUVWXYZWing,
            difficulty: 7.0,
            technique_name: "TUVWXYZ-Wing".to_string(),
            description: desc,
            cell: Cell::from(pivot_idx),
            value: 0,
            eliminations,
        });
    }
}
