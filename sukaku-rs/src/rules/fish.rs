use crate::grid::{Cell, Grid, COLS, ROWS};
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

                if row1_cols.len() >= 2
                    && row1_cols.len() <= 3
                    && row2_cols.len() >= 2
                    && row2_cols.len() <= 3
                {
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
                                    eliminations.push((Cell::from(cell_idx), vec![digit]));
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
                                cell: Cell::from(row1.cells[col1 as usize]),
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

                if col1_rows.len() >= 2
                    && col1_rows.len() <= 3
                    && col2_rows.len() >= 2
                    && col2_rows.len() <= 3
                {
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
                                eliminations.push((Cell::from(cell_idx), vec![digit]));
                            }
                        }

                        for c in 0..9 {
                            if c == col1_idx || c == col2_idx {
                                continue;
                            }
                            let row = &ROWS[row2 as usize];
                            let cell_idx = row.cells[c];
                            if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                eliminations.push((Cell::from(cell_idx), vec![digit]));
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
                                cell: Cell::from(COLS[col1_idx].cells[row1 as usize]),
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
