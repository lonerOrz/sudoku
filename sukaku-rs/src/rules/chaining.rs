use crate::grid::{Cell, Grid, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

/// Find X-Cycles patterns (simple implementation).
/// An X-Cycle is a closed chain of alternating strong and weak links.
/// Simple X-Cycles of length 4 manifest as X-Wing patterns.
/// Difficulty: SE 6.5
pub fn x_cycles_simple(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        find_x_cycle_rows(grid, acc, digit);
        find_x_cycle_cols(grid, acc, digit);
    }
}

fn find_x_cycle_rows(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for r1 in 0..9u8 {
        for r2 in (r1 + 1)..9u8 {
            let row1_cands: Vec<u8> = ROWS[r1 as usize]
                .cells
                .iter()
                .copied()
                .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
                .collect();
            let row2_cands: Vec<u8> = ROWS[r2 as usize]
                .cells
                .iter()
                .copied()
                .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
                .collect();

            if row1_cands.is_empty() || row2_cands.is_empty() {
                continue;
            }

            let mut common_cols: Vec<u8> = row1_cands
                .iter()
                .filter(|c1| {
                    let col1 = *c1 % 9;
                    row2_cands.iter().any(|c2| *c2 % 9 == col1)
                })
                .map(|c| *c % 9)
                .collect();
            common_cols.sort();
            common_cols.dedup();

            if common_cols.len() != 2 {
                continue;
            }

            let col1 = common_cols[0];
            let col2 = common_cols[1];

            let cell1 = r1 * 9 + col1;
            let cell2 = r1 * 9 + col2;
            let cell3 = r2 * 9 + col1;
            let cell4 = r2 * 9 + col2;

            if grid.get(cell1) == 0
                && grid.get(cell2) == 0
                && grid.get(cell3) == 0
                && grid.get(cell4) == 0
            {
                let mut eliminations = Vec::new();

                for r in 0..9 {
                    if r == r1 || r == r2 {
                        continue;
                    }
                    for &c in &[col1, col2] {
                        let cell = r * 9 + c;
                        if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                            eliminations.push((Cell::from(cell), vec![digit]));
                        }
                    }
                }

                if !eliminations.is_empty() {
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::XCyclesSimple,
                        difficulty: 6.5,
                        technique_name: "X-Cycles".to_string(),
                        description: format!(
                            "X-Cycle: digit {} in rows {},{} cols {},{}",
                            digit,
                            r1 + 1,
                            r2 + 1,
                            col1 + 1,
                            col2 + 1
                        ),
                        cell: Cell::from(cell1),
                        value: 0,
                        eliminations,
                    });
                }
            }
        }
    }
}

fn find_x_cycle_cols(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for c1 in 0..9u8 {
        for c2 in (c1 + 1)..9u8 {
            let col1_cands: Vec<u8> = COLS[c1 as usize]
                .cells
                .iter()
                .copied()
                .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
                .collect();
            let col2_cands: Vec<u8> = COLS[c2 as usize]
                .cells
                .iter()
                .copied()
                .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
                .collect();

            if col1_cands.is_empty() || col2_cands.is_empty() {
                continue;
            }

            let mut common_rows: Vec<u8> = col1_cands
                .iter()
                .filter(|r1| {
                    let row1 = *r1 / 9;
                    col2_cands.iter().any(|r2| *r2 / 9 == row1)
                })
                .map(|r| *r / 9)
                .collect();
            common_rows.sort();
            common_rows.dedup();

            if common_rows.len() != 2 {
                continue;
            }

            let row1 = common_rows[0];
            let row2 = common_rows[1];

            let cell1 = row1 * 9 + c1;
            let cell2 = row1 * 9 + c2;
            let cell3 = row2 * 9 + c1;
            let cell4 = row2 * 9 + c2;

            if grid.get(cell1) == 0
                && grid.get(cell2) == 0
                && grid.get(cell3) == 0
                && grid.get(cell4) == 0
            {
                let mut eliminations = Vec::new();

                for c in 0..9 {
                    if c == c1 || c == c2 {
                        continue;
                    }
                    for &r in &[row1, row2] {
                        let cell = r * 9 + c;
                        if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                            eliminations.push((Cell::from(cell), vec![digit]));
                        }
                    }
                }

                if !eliminations.is_empty() {
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::XCyclesSimple,
                        difficulty: 6.5,
                        technique_name: "X-Cycles".to_string(),
                        description: format!(
                            "X-Cycle: digit {} in cols {},{} rows {},{}",
                            digit,
                            c1 + 1,
                            c2 + 1,
                            row1 + 1,
                            row2 + 1
                        ),
                        cell: Cell::from(cell1),
                        value: 0,
                        eliminations,
                    });
                }
            }
        }
    }
}
