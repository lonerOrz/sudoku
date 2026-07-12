use crate::grid::{CellIndex, Grid};
use crate::solver::{Hint, HintAccumulator};

pub fn x_diagonal_var(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        let mut cells_with_digit = Vec::new();
        for cell in 0..81u8 {
            if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                cells_with_digit.push(cell);
            }
        }

        for i in 0..cells_with_digit.len() {
            for j in (i + 1)..cells_with_digit.len() {
                let cell1 = cells_with_digit[i];
                let cell2 = cells_with_digit[j];
                let row1 = cell1 / 9;
                let col1 = cell1 % 9;
                let row2 = cell2 / 9;
                let col2 = cell2 % 9;

                if (row1 == col1 && row2 == col2) || (row1 + col1 == 8 && row2 + col2 == 8) {
                    for &target in &[cell1, cell2] {
                        if target != cell1 && target != cell2 {
                            continue;
                        }
                        let mut elim = Vec::new();
                        for d in 1..=9u8 {
                            if d != digit && grid.candidates(target).has(d) {
                                elim.push((CellIndex::from(target), vec![d]));
                            }
                        }
                        if !elim.is_empty() {
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::XDiagonal,
                                difficulty: 5.5,
                                technique_name: "X-Diagonal".to_string(),
                                description: format!("X-Diagonal: digit {} in diagonal", digit),
                                cell: CellIndex::from(cell1),
                                value: 0,
                                eliminations: elim,
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn disjoint_groups_var(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        for pos_in_box in 0..9u8 {
            let box_row = pos_in_box / 3;
            let box_col = pos_in_box % 3;
            let mut cells_at_position = Vec::new();
            for box_idx in 0..9u8 {
                let box_r = box_idx / 3;
                let box_c = box_idx % 3;
                let cell_row = box_r * 3 + box_row;
                let cell_col = box_c * 3 + box_col;
                let cell = cell_row * 9 + cell_col;
                cells_at_position.push(cell);
            }
            let mut cells_with_digit = Vec::new();
            for &cell in &cells_at_position {
                if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                    cells_with_digit.push(cell);
                }
            }
            for &cell in &cells_with_digit {
                if grid.candidates(cell).cardinality() > 1 {
                    let elim = vec![(CellIndex::from(cell), vec![digit])];
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::DisjointGroups,
                        difficulty: 5.5,
                        technique_name: "Disjoint Groups".to_string(),
                        description: format!("Disjoint Groups: digit {} in box position", digit),
                        cell: CellIndex::from(cell),
                        value: 0,
                        eliminations: elim,
                    });
                }
            }
        }
    }
}

pub fn windows_var(grid: &Grid, acc: &mut HintAccumulator) {
    let windows = [(1, 1), (1, 4), (4, 1), (4, 4)];
    for digit in 1..=9u8 {
        for &(wr, wc) in &windows {
            let mut cells_in_window = Vec::new();
            for dr in 0..3 {
                for dc in 0..3 {
                    let r = wr + dr;
                    let c = wc + dc;
                    let cell = r * 9 + c;
                    cells_in_window.push(cell);
                }
            }
            let cells_with_digit: Vec<u8> = cells_in_window
                .iter()
                .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
                .copied()
                .collect();
            if cells_with_digit.len() == 1 {
                let cell = cells_with_digit[0];
                let mut elim = Vec::new();
                for d in 1..=9u8 {
                    if d != digit && grid.candidates(cell).has(d) {
                        elim.push((CellIndex::from(cell), vec![d]));
                    }
                }
                if !elim.is_empty() {
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::Windows,
                        difficulty: 5.5,
                        technique_name: "Windows".to_string(),
                        description: format!("Windows: digit {} in window", digit),
                        cell: CellIndex::from(cell),
                        value: digit,
                        eliminations: elim,
                    });
                }
            }
        }
    }
}

pub fn center_dot_var(grid: &Grid, acc: &mut HintAccumulator) {
    let center_dots = [10, 13, 16, 37, 40, 43, 64, 67, 70];
    for digit in 1..=9u8 {
        let mut cells_with_digit = Vec::new();
        for &cell in &center_dots {
            if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                cells_with_digit.push(cell);
            }
        }
        for &cell in &cells_with_digit {
            if grid.candidates(cell).cardinality() > 1 {
                let elim = vec![(CellIndex::from(cell), vec![digit])];
                acc.add(Hint {
                    hint_type: crate::solver::HintType::CenterDot,
                    difficulty: 5.5,
                    technique_name: "Center Dot".to_string(),
                    description: format!("Center Dot: digit {} in center cells", digit),
                    cell: CellIndex::from(cell),
                    value: 0,
                    eliminations: elim,
                });
            }
        }
    }
}

pub fn asterisk_var(grid: &Grid, acc: &mut HintAccumulator) {
    let asterisk = [4, 10, 16, 36, 40, 44, 64, 70, 76];
    for digit in 1..=9u8 {
        let mut cells_with_digit = Vec::new();
        for &cell in &asterisk {
            if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                cells_with_digit.push(cell);
            }
        }
        for &cell in &cells_with_digit {
            if grid.candidates(cell).cardinality() > 1 {
                let elim = vec![(CellIndex::from(cell), vec![digit])];
                acc.add(Hint {
                    hint_type: crate::solver::HintType::Asterisk,
                    difficulty: 5.5,
                    technique_name: "Asterisk".to_string(),
                    description: format!("Asterisk: digit {} in asterisk cells", digit),
                    cell: CellIndex::from(cell),
                    value: 0,
                    eliminations: elim,
                });
            }
        }
    }
}

pub fn girandola_var(grid: &Grid, acc: &mut HintAccumulator) {
    let girandola = [0, 8, 20, 40, 60, 72, 80, 24, 56];
    for digit in 1..=9u8 {
        let mut cells_with_digit = Vec::new();
        for &cell in &girandola {
            if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                cells_with_digit.push(cell);
            }
        }
        for &cell in &cells_with_digit {
            if grid.candidates(cell).cardinality() > 1 {
                let elim = vec![(CellIndex::from(cell), vec![digit])];
                acc.add(Hint {
                    hint_type: crate::solver::HintType::Girandola,
                    difficulty: 5.5,
                    technique_name: "Girandola".to_string(),
                    description: format!("Girandola: digit {} in girandola cells", digit),
                    cell: CellIndex::from(cell),
                    value: 0,
                    eliminations: elim,
                });
            }
        }
    }
}

pub fn non_consecutive_var(grid: &Grid, acc: &mut HintAccumulator) {
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let cands = grid.candidates(cell);
        let row = cell / 9;
        let col = cell % 9;
        let neighbors = [
            (row.wrapping_sub(1), col),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
        ];
        for &(nr, nc) in &neighbors {
            if nr >= 9 || nc >= 9 {
                continue;
            }
            let neighbor = nr * 9 + nc;
            if grid.get(neighbor) == 0 {
                let neighbor_cands = grid.candidates(neighbor);
                for d in 1..=9u8 {
                    if cands.has(d) {
                        let mut elim = Vec::new();
                        for nd in 1..=9u8 {
                            if nd.abs_diff(d) == 1 && neighbor_cands.has(nd) {
                                elim.push((CellIndex::from(neighbor), vec![nd]));
                            }
                        }
                        if !elim.is_empty() {
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::NonConsecutive,
                                difficulty: 5.5,
                                technique_name: "Non-Consecutive".to_string(),
                                description: format!(
                                    "Non-Consecutive: digit {} eliminates consecutive from neighbor",
                                    d
                                ),
                                cell: CellIndex::from(cell),
                                value: 0,
                                eliminations: elim,
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn anti_knight_var(grid: &Grid, acc: &mut HintAccumulator) {
    let knight_moves = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let row = (cell / 9) as i32;
        let col = (cell % 9) as i32;
        for &(dr, dc) in &knight_moves {
            let nr = row + dr;
            let nc = col + dc;
            if (0..9).contains(&nr) && (0..9).contains(&nc) {
                let neighbor = (nr * 9 + nc) as u8;
                if grid.get(neighbor) == 0 {
                    let cands = grid.candidates(cell);
                    let neighbor_cands = grid.candidates(neighbor);
                    for d in 1..=9u8 {
                        if cands.has(d) && neighbor_cands.has(d) {
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::AntiKnight,
                                difficulty: 5.5,
                                technique_name: "Anti-Knight".to_string(),
                                description: format!(
                                    "Anti-Knight: digit {} in knight-adjacent cells",
                                    d
                                ),
                                cell: CellIndex::from(cell),
                                value: 0,
                                eliminations: vec![(CellIndex::from(neighbor), vec![d])],
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn anti_king_var(grid: &Grid, acc: &mut HintAccumulator) {
    let king_moves = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let row = (cell / 9) as i32;
        let col = (cell % 9) as i32;
        for &(dr, dc) in &king_moves {
            let nr = row + dr;
            let nc = col + dc;
            if (0..9).contains(&nr) && (0..9).contains(&nc) {
                let neighbor = (nr * 9 + nc) as u8;
                if grid.get(neighbor) == 0 {
                    let cands = grid.candidates(cell);
                    let neighbor_cands = grid.candidates(neighbor);
                    for d in 1..=9u8 {
                        if cands.has(d) && neighbor_cands.has(d) {
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::AntiKing,
                                difficulty: 5.5,
                                technique_name: "Anti-King".to_string(),
                                description: format!(
                                    "Anti-King: digit {} in king-adjacent cells",
                                    d
                                ),
                                cell: CellIndex::from(cell),
                                value: 0,
                                eliminations: vec![(CellIndex::from(neighbor), vec![d])],
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn toroidal_var(grid: &Grid, acc: &mut HintAccumulator) {
    let adjacencies = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let row = (cell / 9) as i32;
        let col = (cell % 9) as i32;
        for &(dr, dc) in &adjacencies {
            let nr = ((row + dr + 9) % 9) as u8;
            let nc = ((col + dc + 9) % 9) as u8;
            let neighbor = nr * 9 + nc;
            if neighbor == cell {
                continue;
            }
            if grid.get(neighbor) == 0 {
                let cands = grid.candidates(cell);
                let neighbor_cands = grid.candidates(neighbor);
                for d in 1..=9u8 {
                    if cands.has(d) && neighbor_cands.has(d) {
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::Toroidal,
                            difficulty: 6.0,
                            technique_name: "Toroidal".to_string(),
                            description: format!(
                                "Toroidal: digit {} in adjacent cells (wrapping)",
                                d
                            ),
                            cell: CellIndex::from(cell),
                            value: 0,
                            eliminations: vec![(CellIndex::from(neighbor), vec![d])],
                        });
                    }
                }
            }
        }
    }
}

pub fn ferz_nc_var(grid: &Grid, acc: &mut HintAccumulator) {
    let ferz_moves = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let row = (cell / 9) as i32;
        let col = (cell % 9) as i32;
        for &(dr, dc) in &ferz_moves {
            let nr = row + dr;
            let nc = col + dc;
            if (0..9).contains(&nr) && (0..9).contains(&nc) {
                let neighbor = (nr * 9 + nc) as u8;
                if grid.get(neighbor) == 0 {
                    let cands = grid.candidates(cell);
                    let neighbor_cands = grid.candidates(neighbor);
                    for d in 1..=9u8 {
                        if cands.has(d) {
                            let mut elim = Vec::new();
                            for nd in 1..=9u8 {
                                if nd.abs_diff(d) == 1 && neighbor_cands.has(nd) {
                                    elim.push((CellIndex::from(neighbor), vec![nd]));
                                }
                            }
                            if !elim.is_empty() {
                                acc.add(Hint {
                                    hint_type: crate::solver::HintType::FerzNC,
                                    difficulty: 6.0,
                                    technique_name: "Ferz NC".to_string(),
                                    description: format!(
                                        "Ferz NC: digit {} eliminates consecutive from diagonal neighbor",
                                        d
                                    ),
                                    cell: CellIndex::from(cell),
                                    value: 0,
                                    eliminations: elim,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
