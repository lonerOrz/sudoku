use crate::grid::{Cell, Grid, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

/// Find VLocking (Generalized Intersections) patterns.
/// This technique looks for intersections between rows/columns and blocks where
/// a digit is confined to the intersection cells, allowing eliminations elsewhere.
/// Difficulty: Variable (typically 4.0-6.5 depending on configuration)
pub fn vlocking(grid: &Grid, acc: &mut HintAccumulator) {
    // Check for row-block intersections (pointing variants)
    for digit in 1..=9u8 {
        // Check each row for pointing opportunities
        for (row_idx, row) in ROWS.iter().enumerate() {
            // Find cells in this row that contain the digit
            let mut cells_in_row: Vec<u8> = Vec::new();
            for &cell_idx in &row.cells {
                if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                    cells_in_row.push(cell_idx);
                }
            }

            // If the digit appears in 2 or 3 cells in this row
            if cells_in_row.len() >= 2 && cells_in_row.len() <= 3 {
                // Check if all these cells are in the same block
                let first_block = (cells_in_row[0] / 27) * 3 + (cells_in_row[0] % 9) / 3;
                let mut same_block = true;
                for &cell_idx in &cells_in_row {
                    let block = (cell_idx / 27) * 3 + (cell_idx % 9) / 3;
                    if block != first_block {
                        same_block = false;
                        break;
                    }
                }

                if same_block {
                    // This is a pointing opportunity: digit is confined to this block within the row
                    // Eliminate from other cells in the block that are not in this row
                    let mut eliminations = Vec::new();
                    let block_start = (first_block / 3) * 27 + (first_block % 3) * 3;
                    for r in 0..3 {
                        for c in 0..3 {
                            let cell_idx = block_start + r * 9 + c;
                            // Skip cells that are in the row
                            let cell_row = cell_idx / 9;
                            if cell_row == row_idx as u8 {
                                continue;
                            }
                            if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                eliminations.push((Cell::from(cell_idx), vec![digit]));
                            }
                        }
                    }

                    if !eliminations.is_empty() {
                        let desc = format!(
                            "VLocking (Pointing): digit {} confined to row {} in block {}",
                            digit,
                            row_idx + 1,
                            first_block + 1
                        );
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::VLocking,
                            difficulty: 4.0 + (cells_in_row.len() - 2) as f64 * 0.5, // 2 cells -> 4.0, 3 cells -> 4.5
                            technique_name: "VLocking".to_string(),
                            description: desc,
                            cell: Cell::from(cells_in_row[0]),
                            value: 0,
                            eliminations,
                        });
                    }
                }
            }
        }

        // Check each column for claiming opportunities (column-block intersections)
        for (col_idx, col) in COLS.iter().enumerate() {
            // Find cells in this column that contain the digit
            let mut cells_in_col: Vec<u8> = Vec::new();
            for &cell_idx in &col.cells {
                if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                    cells_in_col.push(cell_idx);
                }
            }

            // If the digit appears in 2 or 3 cells in this column
            if cells_in_col.len() >= 2 && cells_in_col.len() <= 3 {
                // Check if all these cells are in the same block
                let first_block = (cells_in_col[0] / 27) * 3 + (cells_in_col[0] % 9) / 3;
                let mut same_block = true;
                for &cell_idx in &cells_in_col {
                    let block = (cell_idx / 27) * 3 + (cell_idx % 9) / 3;
                    if block != first_block {
                        same_block = false;
                        break;
                    }
                }

                if same_block {
                    // This is a claiming opportunity: digit is confined to this block within the column
                    // Eliminate from other cells in the block that are not in this column
                    let mut eliminations = Vec::new();
                    let block_start = (first_block / 3) * 27 + (first_block % 3) * 3;
                    for r in 0..3 {
                        for c in 0..3 {
                            let cell_idx = block_start + r * 9 + c;
                            // Skip cells that are in the column
                            let cell_col = cell_idx % 9;
                            if cell_col == col_idx as u8 {
                                continue;
                            }
                            if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                eliminations.push((Cell::from(cell_idx), vec![digit]));
                            }
                        }
                    }

                    if !eliminations.is_empty() {
                        let desc = format!(
                            "VLocking (Claiming): digit {} confined to column {} in block {}",
                            digit,
                            col_idx + 1,
                            first_block + 1
                        );
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::VLocking,
                            difficulty: 4.0 + (cells_in_col.len() - 2) as f64 * 0.5, // 2 cells -> 4.0, 3 cells -> 4.5
                            technique_name: "VLocking".to_string(),
                            description: desc,
                            cell: Cell::from(cells_in_col[0]),
                            value: 0,
                            eliminations,
                        });
                    }
                }
            }
        }
    }

    // Additionally, we can look for more complex locked set interactions
    // For simplicity, we'll implement the basic pointing/claiming which covers most VLocking use cases
}
