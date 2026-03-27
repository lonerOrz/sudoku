use crate::grid::{Cell, Grid, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

pub fn skyscraper(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        for row_idx in 0..9 {
            find_skyscraper_row(grid, acc, digit, row_idx);
        }
        for col_idx in 0..9 {
            find_skyscraper_col(grid, acc, digit, col_idx);
        }
    }
}

/// Find Skyscraper pattern in a row.
///
/// A Skyscraper consists of two strong links in the same row
/// with end cells visible to each other in different columns.
fn find_skyscraper_row(grid: &Grid, acc: &mut HintAccumulator, digit: u8, row_idx: usize) {
    let row = &ROWS[row_idx];
    let strong_links: Vec<u8> = row
        .cells
        .iter()
        .copied()
        .filter(|&cell| {
            grid.get(cell) == 0
                && grid.candidates(cell).has(digit)
                && grid.candidates(cell).cardinality() == 2
        })
        .collect();

    if strong_links.len() < 2 {
        return;
    }

    for i in 0..strong_links.len() {
        for j in (i + 1)..strong_links.len() {
            let base1 = strong_links[i];
            let base2 = strong_links[j];

            let base1_col = base1 % 9;
            let base2_col = base2 % 9;
            if base1_col == base2_col {
                continue;
            }

            // Find strong link end in column: must be the ONLY other cell with digit
            if let (Some(end1), Some(end2)) = (
                find_strong_link_end_in_column(grid, base1, digit),
                find_strong_link_end_in_column(grid, base2, digit),
            ) {
                if end1 == end2 {
                    continue;
                }
                if is_visible(end1, end2) {
                    let eliminations = find_eliminations(grid, base1, base2, end1, end2, digit);
                    if !eliminations.is_empty() {
                        add_hint_unique(
                            acc,
                            crate::solver::HintType::Skyscraper,
                            4.0,
                            "Skyscraper",
                            format!("Skyscraper: digit {} in row {}", digit, row_idx + 1),
                            Cell::from(base1),
                            eliminations,
                        );
                    }
                }
            }
        }
    }
}

/// Find Skyscraper pattern in a column.
fn find_skyscraper_col(grid: &Grid, acc: &mut HintAccumulator, digit: u8, col_idx: usize) {
    let col = &COLS[col_idx];
    let strong_links: Vec<u8> = col
        .cells
        .iter()
        .copied()
        .filter(|&cell| {
            grid.get(cell) == 0
                && grid.candidates(cell).has(digit)
                && grid.candidates(cell).cardinality() == 2
        })
        .collect();

    if strong_links.len() < 2 {
        return;
    }

    for i in 0..strong_links.len() {
        for j in (i + 1)..strong_links.len() {
            let base1 = strong_links[i];
            let base2 = strong_links[j];

            let base1_row = base1 / 9;
            let base2_row = base2 / 9;
            if base1_row == base2_row {
                continue;
            }

            // Find strong link end in row: must be the ONLY other cell with digit
            if let (Some(end1), Some(end2)) = (
                find_strong_link_end_in_row(grid, base1, digit),
                find_strong_link_end_in_row(grid, base2, digit),
            ) {
                if end1 == end2 {
                    continue;
                }
                if is_visible(end1, end2) {
                    let eliminations = find_eliminations(grid, base1, base2, end1, end2, digit);
                    if !eliminations.is_empty() {
                        add_hint_unique(
                            acc,
                            crate::solver::HintType::Skyscraper,
                            4.0,
                            "Skyscraper",
                            format!("Skyscraper: digit {} in column {}", digit, col_idx + 1),
                            Cell::from(base1),
                            eliminations,
                        );
                    }
                }
            }
        }
    }
}

pub fn two_string_kite(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        for row_idx in 0..9 {
            for col_idx in 0..9 {
                find_kite(grid, acc, digit, row_idx, col_idx);
            }
        }
    }
}

/// Find 2-String Kite pattern: a strong link in a row and another in a column
/// that share the same block.
fn find_kite(grid: &Grid, acc: &mut HintAccumulator, digit: u8, row_idx: usize, col_idx: usize) {
    let row = &ROWS[row_idx];
    let col = &COLS[col_idx];

    let row_strong: Vec<u8> = row
        .cells
        .iter()
        .copied()
        .filter(|&cell| {
            grid.get(cell) == 0
                && grid.candidates(cell).has(digit)
                && grid.candidates(cell).cardinality() == 2
        })
        .collect();

    let col_strong: Vec<u8> = col
        .cells
        .iter()
        .copied()
        .filter(|&cell| {
            grid.get(cell) == 0
                && grid.candidates(cell).has(digit)
                && grid.candidates(cell).cardinality() == 2
        })
        .collect();

    if row_strong.len() < 2 || col_strong.len() < 2 {
        return;
    }

    for &base1 in &row_strong {
        for &base2 in &col_strong {
            let base1_block = (base1 / 27) * 3 + (base1 % 9) / 3;
            let base2_block = (base2 / 27) * 3 + (base2 % 9) / 3;

            if base1_block != base2_block {
                continue;
            }

            // end1: same row as base1, at col_idx (must be strong link in column)
            // end2: same column as base2, at row_idx (must be strong link in row)
            let end1 = (base1 / 9) * 9 + col_idx as u8;
            let end2 = row_idx as u8 * 9 + (base2 % 9);

            // Verify end1 forms strong link in its column
            if !is_strong_link_in_column(grid, end1, base2, digit) {
                continue;
            }
            // Verify end2 forms strong link in its row
            if !is_strong_link_in_row(grid, end2, base1, digit) {
                continue;
            }

            if grid.get(end1) != 0 || !grid.candidates(end1).has(digit) {
                continue;
            }
            if grid.get(end2) != 0 || !grid.candidates(end2).has(digit) {
                continue;
            }

            if is_visible(end1, end2) {
                let eliminations = find_eliminations(grid, base1, base2, end1, end2, digit);
                if !eliminations.is_empty() {
                    add_hint_unique(
                        acc,
                        crate::solver::HintType::TwoStringKite,
                        4.1,
                        "2-String Kite",
                        format!("2-String Kite: digit {}", digit),
                        Cell::from(base1),
                        eliminations,
                    );
                }
            }
        }
    }
}

/// Check if two cells form a strong link in a column (only these two have the digit).
fn is_strong_link_in_column(grid: &Grid, cell1: u8, cell2: u8, digit: u8) -> bool {
    let col = cell1 % 9;
    let mut count = 0;
    for row in 0..9u8 {
        let c = row * 9 + col;
        if grid.get(c) == 0 && grid.candidates(c).has(digit) {
            if c == cell2 {
                continue;
            }
            count += 1;
            if count > 1 {
                return false;
            }
        }
    }
    count == 1
}

/// Check if two cells form a strong link in a row.
fn is_strong_link_in_row(grid: &Grid, cell1: u8, cell2: u8, digit: u8) -> bool {
    let row = cell1 / 9;
    let mut count = 0;
    for col in 0..9u8 {
        let c = row * 9 + col;
        if grid.get(c) == 0 && grid.candidates(c).has(digit) {
            if c == cell2 {
                continue;
            }
            count += 1;
            if count > 1 {
                return false;
            }
        }
    }
    count == 1
}

/// Find end cell in column that forms a strong link (only other cell with digit).
fn find_strong_link_end_in_column(grid: &Grid, base: u8, digit: u8) -> Option<u8> {
    let base_col = base % 9;
    let base_row = base / 9;

    let mut end_cell = None;
    let mut count = 0;

    for row in 0..9u8 {
        if row == base_row {
            continue;
        }
        let cell = row * 9 + base_col;
        if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
            count += 1;
            if count > 1 {
                return None;
            }
            end_cell = Some(cell);
        }
    }

    if count == 1 {
        end_cell
    } else {
        None
    }
}

/// Find end cell in row that forms a strong link.
fn find_strong_link_end_in_row(grid: &Grid, base: u8, digit: u8) -> Option<u8> {
    let base_row = base / 9;
    let base_col = base % 9;

    let mut end_cell = None;
    let mut count = 0;

    for col in 0..9u8 {
        if col == base_col {
            continue;
        }
        let cell = base_row * 9 + col;
        if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
            count += 1;
            if count > 1 {
                return None;
            }
            end_cell = Some(cell);
        }
    }

    if count == 1 {
        end_cell
    } else {
        None
    }
}

/// Find elimination cells: visible to both end cells but not to base cells.
fn find_eliminations(
    grid: &Grid,
    base1: u8,
    base2: u8,
    end1: u8,
    end2: u8,
    digit: u8,
) -> Vec<(Cell, Vec<u8>)> {
    let mut eliminations = Vec::new();

    for cell in 0..81u8 {
        if grid.get(cell) != 0 || !grid.candidates(cell).has(digit) {
            continue;
        }
        if [base1, base2, end1, end2].contains(&cell) {
            continue;
        }

        if is_visible(end1, cell)
            && is_visible(end2, cell)
            && !is_visible(base1, cell)
            && !is_visible(base2, cell)
        {
            eliminations.push((Cell::from(cell), vec![digit]));
        }
    }

    eliminations
}

/// Check if two cells are visible to each other (same row, column, or block).
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

/// Add hint only if no equivalent hint already exists.
fn add_hint_unique(
    acc: &mut HintAccumulator,
    hint_type: crate::solver::HintType,
    difficulty: f64,
    technique_name: &str,
    description: String,
    cell: Cell,
    eliminations: Vec<(Cell, Vec<u8>)>,
) {
    // Check if an equivalent hint already exists
    for existing in acc.hints() {
        if existing.hint_type == hint_type
            && existing.difficulty == difficulty
            && existing.eliminations.len() == eliminations.len()
        {
            // Check if all elimination targets are the same
            let same_elims = existing
                .eliminations
                .iter()
                .zip(eliminations.iter())
                .all(|(e1, e2)| e1.0 == e2.0 && e1.1 == e2.1);
            if same_elims {
                return;
            }
        }
    }

    acc.add(Hint {
        hint_type,
        difficulty,
        technique_name: technique_name.to_string(),
        description,
        cell,
        value: 0,
        eliminations,
    });
}
