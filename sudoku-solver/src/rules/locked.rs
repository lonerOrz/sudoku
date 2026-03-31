use crate::grid::{Cell, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

/// Locked Candidates - Pointing: When a candidate in a block appears only in
/// a single row or column within that block, it can be eliminated from that
/// row/column in other blocks.
pub fn locked_pointing(grid: &Grid, acc: &mut HintAccumulator) {
    for block in &BLOCKS {
        let empty_cells: Vec<u8> = block
            .cells
            .iter()
            .copied()
            .filter(|&idx| grid.get(idx) == 0)
            .collect();

        if empty_cells.len() < 2 {
            continue;
        }

        for digit in 1..=9u8 {
            let cells_with_digit: Vec<u8> = empty_cells
                .iter()
                .copied()
                .filter(|&cell| grid.candidates(cell).has(digit))
                .collect();

            if cells_with_digit.len() < 2 || cells_with_digit.len() > 3 {
                continue;
            }

            // Check if all cells are in the same row
            let rows: Vec<u8> = cells_with_digit.iter().map(|&cell| cell / 9).collect();
            let unique_rows: Vec<_> = rows
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .iter()
                .cloned()
                .collect();

            if unique_rows.len() == 1 {
                let row_idx = rows[0];
                let row = &ROWS[row_idx as usize];

                // Eliminate from other cells in this row outside this block
                let mut eliminations = Vec::new();
                for &row_cell in &row.cells {
                    // Skip cells in the same block
                    if block.cells.contains(&row_cell) {
                        continue;
                    }
                    // Skip solved cells
                    if grid.get(row_cell) != 0 {
                        continue;
                    }
                    if grid.candidates(row_cell).has(digit) {
                        eliminations.push((Cell::from(row_cell), vec![digit]));
                    }
                }

                if !eliminations.is_empty() {
                    let desc = format!("Locked Pointing {} in block {}", digit, block.index);
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::LockedPointing,
                        difficulty: 1.7,
                        technique_name: "Locked Pointing".to_string(),
                        description: desc,
                        cell: Cell::from(cells_with_digit[0]),
                        value: 0,
                        eliminations,
                    });
                }
            }

            let cols: Vec<u8> = cells_with_digit.iter().map(|&cell| cell % 9).collect();
            let unique_cols: Vec<_> = cols
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .iter()
                .cloned()
                .collect();

            if unique_cols.len() == 1 {
                let col_idx = cols[0];
                let col = &COLS[col_idx as usize];

                let mut eliminations = Vec::new();
                for &col_cell in &col.cells {
                    if block.cells.contains(&col_cell) {
                        continue;
                    }
                    if grid.get(col_cell) != 0 {
                        continue;
                    }
                    if grid.candidates(col_cell).has(digit) {
                        eliminations.push((Cell::from(col_cell), vec![digit]));
                    }
                }

                if !eliminations.is_empty() {
                    let desc = format!("Locked Pointing {} in block {}", digit, block.index);
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::LockedPointing,
                        difficulty: 2.6,
                        technique_name: "Locked Pointing".to_string(),
                        description: desc,
                        cell: Cell::from(cells_with_digit[0]),
                        value: 0,
                        eliminations,
                    });
                }
            }
        }
    }
}

/// Locked Candidates - Claiming: When a candidate in a row or column appears
/// only in a single block, it can be eliminated from other cells in that block.
pub fn locked_claiming(grid: &Grid, acc: &mut HintAccumulator) {
    for row in &ROWS {
        let empty_cells: Vec<u8> = row
            .cells
            .iter()
            .copied()
            .filter(|&idx| grid.get(idx) == 0)
            .collect();

        if empty_cells.len() < 2 {
            continue;
        }

        for digit in 1..=9u8 {
            let cells_with_digit: Vec<u8> = empty_cells
                .iter()
                .copied()
                .filter(|&cell| grid.candidates(cell).has(digit))
                .collect();

            if cells_with_digit.len() < 2 || cells_with_digit.len() > 3 {
                continue;
            }

            // Check if all cells are in the same block
            let block_indices: Vec<u8> = cells_with_digit
                .iter()
                .map(|&cell| (cell / 27) * 3 + (cell % 9) / 3)
                .collect();
            let unique_blocks: Vec<_> = block_indices
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .iter()
                .cloned()
                .collect();

            if unique_blocks.len() == 1 {
                let block_idx = block_indices[0];
                let block = &BLOCKS[block_idx as usize];
                let mut eliminations = Vec::new();
                for &block_cell in &block.cells {
                    if block_cell / 9 == row.index {
                        continue;
                    }
                    if grid.get(block_cell) != 0 {
                        continue;
                    }
                    if grid.candidates(block_cell).has(digit) {
                        eliminations.push((Cell::from(block_cell), vec![digit]));
                    }
                }

                if !eliminations.is_empty() {
                    let desc = format!("Locked Claiming {} in row {}", digit, row.index);
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::LockedClaiming,
                        difficulty: 1.9,
                        technique_name: "Locked Claiming".to_string(),
                        description: desc,
                        cell: Cell::from(cells_with_digit[0]),
                        value: 0,
                        eliminations,
                    });
                }
            }
        }
    }

    for col in &COLS {
        let empty_cells: Vec<u8> = col
            .cells
            .iter()
            .copied()
            .filter(|&idx| grid.get(idx) == 0)
            .collect();

        if empty_cells.len() < 2 {
            continue;
        }

        for digit in 1..=9u8 {
            let cells_with_digit: Vec<u8> = empty_cells
                .iter()
                .copied()
                .filter(|&cell| grid.candidates(cell).has(digit))
                .collect();

            if cells_with_digit.len() < 2 || cells_with_digit.len() > 3 {
                continue;
            }

            // Check if all cells are in the same block
            let block_indices: Vec<u8> = cells_with_digit
                .iter()
                .map(|&cell| (cell / 27) * 3 + (cell % 9) / 3)
                .collect();
            let unique_blocks: Vec<_> = block_indices
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .iter()
                .cloned()
                .collect();

            if unique_blocks.len() == 1 {
                let block_idx = block_indices[0];
                let block = &BLOCKS[block_idx as usize];
                let mut eliminations = Vec::new();
                for &block_cell in &block.cells {
                    if block_cell % 9 == col.index {
                        continue;
                    }
                    if grid.get(block_cell) != 0 {
                        continue;
                    }
                    if grid.candidates(block_cell).has(digit) {
                        eliminations.push((Cell::from(block_cell), vec![digit]));
                    }
                }

                if !eliminations.is_empty() {
                    let desc = format!("Locked Claiming {} in col {}", digit, col.index);
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::LockedClaiming,
                        difficulty: 2.8,
                        technique_name: "Locked Claiming".to_string(),
                        description: desc,
                        cell: Cell::from(cells_with_digit[0]),
                        value: 0,
                        eliminations,
                    });
                }
            }
        }
    }
}
