use crate::grid::{CellIndex, Grid, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

/// Find X-Wing patterns: when a digit appears in exactly two rows/columns forming a rectangle.
#[allow(clippy::needless_range_loop)]
pub fn x_wing(grid: &Grid, acc: &mut HintAccumulator) {
    for row1_idx in 0..9 {
        for row2_idx in (row1_idx + 1)..9 {
            let row1 = &ROWS[row1_idx];
            let row2 = &ROWS[row2_idx];

            for digit in 1..=9u8 {
                let row1_cols: Vec<u8> = row1
                    .cells
                    .iter()
                    .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                    .map(|&idx| idx % 9)
                    .collect();

                let row2_cols: Vec<u8> = row2
                    .cells
                    .iter()
                    .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                    .map(|&idx| idx % 9)
                    .collect();

                if row1_cols.len() == 2 && row2_cols.len() == 2 {
                    let common: Vec<u8> = row1_cols
                        .iter()
                        .filter(|&&c| row2_cols.contains(&c))
                        .copied()
                        .collect();

                    if common.len() == 2 {
                        let col1 = common[0];
                        let col2 = common[1];

                        let mut eliminations = Vec::new();

                        for r in 0..9 {
                            if r == row1_idx || r == row2_idx {
                                continue;
                            }
                            let row = &ROWS[r];
                            for &c in &common {
                                let cell_idx = row.cells[c as usize];
                                if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                    eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                                }
                            }
                        }

                        if !eliminations.is_empty() {
                            let desc = format!(
                                "X-Wing {} in rows {}, {} and columns {}, {}",
                                digit, row1_idx, row2_idx, col1, col2
                            );
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::XWing,
                                difficulty: 3.2,
                                technique_name: "X-Wing".to_string(),
                                description: desc,
                                cell: CellIndex::from(row1.cells[col1 as usize]),
                                value: 0,
                                eliminations,
                            });
                        }
                    }
                }
            }
        }
    }

    for col1_idx in 0..9 {
        for col2_idx in (col1_idx + 1)..9 {
            let col1 = &COLS[col1_idx];
            let col2 = &COLS[col2_idx];

            for digit in 1..=9u8 {
                let col1_rows: Vec<u8> = col1
                    .cells
                    .iter()
                    .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                    .map(|&idx| idx / 9)
                    .collect();

                let col2_rows: Vec<u8> = col2
                    .cells
                    .iter()
                    .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                    .map(|&idx| idx / 9)
                    .collect();

                if col1_rows.len() == 2 && col2_rows.len() == 2 {
                    let common: Vec<u8> = col1_rows
                        .iter()
                        .filter(|&&r| col2_rows.contains(&r))
                        .copied()
                        .collect();

                    if common.len() == 2 {
                        let row1 = common[0];
                        let row2 = common[1];

                        let mut eliminations = Vec::new();

                        for c in 0..9 {
                            if c == col1_idx || c == col2_idx {
                                continue;
                            }
                            let row = &ROWS[row1 as usize];
                            let cell_idx = row.cells[c];
                            if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                            }
                        }

                        for c in 0..9 {
                            if c == col1_idx || c == col2_idx {
                                continue;
                            }
                            let row = &ROWS[row2 as usize];
                            let cell_idx = row.cells[c];
                            if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                            }
                        }

                        if !eliminations.is_empty() {
                            let desc = format!(
                                "X-Wing {} in columns {}, {} and rows {}, {}",
                                digit, col1_idx, col2_idx, row1, row2
                            );
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::XWing,
                                difficulty: 3.2,
                                technique_name: "X-Wing".to_string(),
                                description: desc,
                                cell: CellIndex::from(COLS[col1_idx].cells[row1 as usize]),
                                value: 0,
                                eliminations,
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Find Swordfish patterns: when a digit appears in exactly three rows/columns.
#[allow(clippy::needless_range_loop)]
pub fn swordfish(grid: &Grid, acc: &mut HintAccumulator) {
    for row1_idx in 0..9 {
        for row2_idx in (row1_idx + 1)..9 {
            for row3_idx in (row2_idx + 1)..9 {
                let row1 = &ROWS[row1_idx];
                let row2 = &ROWS[row2_idx];
                let row3 = &ROWS[row3_idx];

                for digit in 1..=9u8 {
                    let row1_cols: Vec<u8> = row1
                        .cells
                        .iter()
                        .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                        .map(|&idx| idx % 9)
                        .collect();

                    if row1_cols.is_empty() || row1_cols.len() > 3 {
                        continue;
                    }

                    let row2_cols: Vec<u8> = row2
                        .cells
                        .iter()
                        .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                        .map(|&idx| idx % 9)
                        .collect();

                    if row2_cols.is_empty() || row2_cols.len() > 3 {
                        continue;
                    }

                    let row3_cols: Vec<u8> = row3
                        .cells
                        .iter()
                        .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                        .map(|&idx| idx % 9)
                        .collect();

                    if row3_cols.is_empty() || row3_cols.len() > 3 {
                        continue;
                    }

                    let mut all_cols: Vec<u8> = row1_cols.clone();
                    for &c in &row2_cols {
                        if !all_cols.contains(&c) {
                            all_cols.push(c);
                        }
                    }
                    for &c in &row3_cols {
                        if !all_cols.contains(&c) {
                            all_cols.push(c);
                        }
                    }

                    if all_cols.len() == 3 {
                        let col1 = all_cols[0];
                        let col2 = all_cols[1];
                        let col3 = all_cols[2];

                        let mut eliminations = Vec::new();
                        for r in 0..9 {
                            if r == row1_idx || r == row2_idx || r == row3_idx {
                                continue;
                            }
                            let row = &ROWS[r];
                            for &c in &all_cols {
                                let cell_idx = row.cells[c as usize];
                                if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                    eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                                }
                            }
                        }

                        if !eliminations.is_empty() {
                            let desc = format!(
                                "Swordfish {} in rows {}, {}, {} and columns {}, {}, {}",
                                digit, row1_idx, row2_idx, row3_idx, col1, col2, col3
                            );
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::Swordfish,
                                difficulty: 4.0,
                                technique_name: "Swordfish".to_string(),
                                description: desc,
                                cell: CellIndex::from(row1.cells[col1 as usize]),
                                value: 0,
                                eliminations,
                            });
                        }
                    }
                }
            }
        }
    }

    for col1_idx in 0..9 {
        for col2_idx in (col1_idx + 1)..9 {
            for col3_idx in (col2_idx + 1)..9 {
                let col1 = &COLS[col1_idx];
                let col2 = &COLS[col2_idx];
                let col3 = &COLS[col3_idx];

                for digit in 1..=9u8 {
                    let col1_rows: Vec<u8> = col1
                        .cells
                        .iter()
                        .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                        .map(|&idx| idx / 9)
                        .collect();

                    if col1_rows.is_empty() || col1_rows.len() > 3 {
                        continue;
                    }

                    let col2_rows: Vec<u8> = col2
                        .cells
                        .iter()
                        .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                        .map(|&idx| idx / 9)
                        .collect();

                    if col2_rows.is_empty() || col2_rows.len() > 3 {
                        continue;
                    }

                    let col3_rows: Vec<u8> = col3
                        .cells
                        .iter()
                        .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(digit))
                        .map(|&idx| idx / 9)
                        .collect();

                    if col3_rows.is_empty() || col3_rows.len() > 3 {
                        continue;
                    }

                    let mut all_rows: Vec<u8> = col1_rows.clone();
                    for &r in &col2_rows {
                        if !all_rows.contains(&r) {
                            all_rows.push(r);
                        }
                    }
                    for &r in &col3_rows {
                        if !all_rows.contains(&r) {
                            all_rows.push(r);
                        }
                    }

                    if all_rows.len() == 3 {
                        let row1 = all_rows[0];
                        let row2 = all_rows[1];
                        let row3 = all_rows[2];

                        let mut eliminations = Vec::new();
                        for c in 0..9 {
                            if c == col1_idx || c == col2_idx || c == col3_idx {
                                continue;
                            }
                            for &r in &all_rows {
                                let row = &ROWS[r as usize];
                                let cell_idx = row.cells[c];
                                if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                    eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                                }
                            }
                        }

                        if !eliminations.is_empty() {
                            let desc = format!(
                                "Swordfish {} in columns {}, {}, {} and rows {}, {}, {}",
                                digit, col1_idx, col2_idx, col3_idx, row1, row2, row3
                            );
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::Swordfish,
                                difficulty: 4.0,
                                technique_name: "Swordfish".to_string(),
                                description: desc,
                                cell: CellIndex::from(COLS[col1_idx].cells[row1 as usize]),
                                value: 0,
                                eliminations,
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Find Jellyfish patterns: when a digit appears in exactly four rows/columns.
#[allow(clippy::needless_range_loop)]
pub fn jellyfish(grid: &Grid, acc: &mut HintAccumulator) {
    // Row-based Jellyfish: 4 rows with digit in exactly 4 columns
    for row0_idx in 0..9 {
        for row1_idx in (row0_idx + 1)..9 {
            for row2_idx in (row1_idx + 1)..9 {
                for row3_idx in (row2_idx + 1)..9 {
                    let rows = [
                        &ROWS[row0_idx],
                        &ROWS[row1_idx],
                        &ROWS[row2_idx],
                        &ROWS[row3_idx],
                    ];

                    for digit in 1..=9u8 {
                        let row_cols: Vec<Vec<u8>> = rows
                            .iter()
                            .map(|row| {
                                row.cells
                                    .iter()
                                    .filter(|&&idx| {
                                        grid.get(idx) == 0 && grid.candidates(idx).has(digit)
                                    })
                                    .map(|&idx| idx % 9)
                                    .collect()
                            })
                            .collect();

                        if row_cols
                            .iter()
                            .any(|cols| cols.is_empty() || cols.len() > 4)
                        {
                            continue;
                        }

                        let mut all_cols: Vec<u8> = Vec::new();
                        for cols in &row_cols {
                            for &c in cols {
                                if !all_cols.contains(&c) {
                                    all_cols.push(c);
                                }
                            }
                        }

                        if all_cols.len() == 4 {
                            let mut eliminations = Vec::new();
                            for r in 0..9 {
                                if r == row0_idx || r == row1_idx || r == row2_idx || r == row3_idx
                                {
                                    continue;
                                }
                                let row = &ROWS[r];
                                for &c in &all_cols {
                                    let cell_idx = row.cells[c as usize];
                                    if grid.get(cell_idx) == 0
                                        && grid.candidates(cell_idx).has(digit)
                                    {
                                        eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                                    }
                                }
                            }

                            if !eliminations.is_empty() {
                                let desc = format!(
                                    "Jellyfish {} in rows {},{},{},{} and columns {},{},{},{}",
                                    digit,
                                    row0_idx + 1,
                                    row1_idx + 1,
                                    row2_idx + 1,
                                    row3_idx + 1,
                                    all_cols[0] + 1,
                                    all_cols[1] + 1,
                                    all_cols[2] + 1,
                                    all_cols[3] + 1
                                );
                                acc.add(Hint {
                                    hint_type: crate::solver::HintType::Jellyfish,
                                    difficulty: 5.2,
                                    technique_name: "Jellyfish".to_string(),
                                    description: desc,
                                    cell: CellIndex::from(
                                        ROWS[row0_idx].cells[all_cols[0] as usize],
                                    ),
                                    value: 0,
                                    eliminations,
                                });
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    // Column-based Jellyfish: 4 columns with digit in exactly 4 rows
    for col0_idx in 0..9 {
        for col1_idx in (col0_idx + 1)..9 {
            for col2_idx in (col1_idx + 1)..9 {
                for col3_idx in (col2_idx + 1)..9 {
                    let cols = [
                        &COLS[col0_idx],
                        &COLS[col1_idx],
                        &COLS[col2_idx],
                        &COLS[col3_idx],
                    ];

                    for digit in 1..=9u8 {
                        let col_rows: Vec<Vec<u8>> = cols
                            .iter()
                            .map(|col| {
                                col.cells
                                    .iter()
                                    .filter(|&&idx| {
                                        grid.get(idx) == 0 && grid.candidates(idx).has(digit)
                                    })
                                    .map(|&idx| idx / 9)
                                    .collect()
                            })
                            .collect();

                        if col_rows
                            .iter()
                            .any(|rows| rows.is_empty() || rows.len() > 4)
                        {
                            continue;
                        }

                        let mut all_rows: Vec<u8> = Vec::new();
                        for rows in &col_rows {
                            for &r in rows {
                                if !all_rows.contains(&r) {
                                    all_rows.push(r);
                                }
                            }
                        }

                        if all_rows.len() == 4 {
                            let mut eliminations = Vec::new();
                            for c in 0..9 {
                                if c == col0_idx || c == col1_idx || c == col2_idx || c == col3_idx
                                {
                                    continue;
                                }
                                for &r in &all_rows {
                                    let cell_idx = ROWS[r as usize].cells[c];
                                    if grid.get(cell_idx) == 0
                                        && grid.candidates(cell_idx).has(digit)
                                    {
                                        eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                                    }
                                }
                            }

                            if !eliminations.is_empty() {
                                let desc = format!(
                                    "Jellyfish {} in columns {},{},{},{} and rows {},{},{},{}",
                                    digit,
                                    col0_idx + 1,
                                    col1_idx + 1,
                                    col2_idx + 1,
                                    col3_idx + 1,
                                    all_rows[0] + 1,
                                    all_rows[1] + 1,
                                    all_rows[2] + 1,
                                    all_rows[3] + 1
                                );
                                acc.add(Hint {
                                    hint_type: crate::solver::HintType::Jellyfish,
                                    difficulty: 5.2,
                                    technique_name: "Jellyfish".to_string(),
                                    description: desc,
                                    cell: CellIndex::from(
                                        COLS[col0_idx].cells[all_rows[0] as usize],
                                    ),
                                    value: 0,
                                    eliminations,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
