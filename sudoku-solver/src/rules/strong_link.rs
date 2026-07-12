use crate::grid::{CellIndex, Grid, COLS, ROWS};
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
                            CellIndex::from(base1),
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
                            CellIndex::from(base1),
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
                        CellIndex::from(base1),
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
) -> Vec<(CellIndex, Vec<u8>)> {
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
            eliminations.push((CellIndex::from(cell), vec![digit]));
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

/// Find 3-Strong-Links Fish pattern: 3 rows or columns with strong links
/// that share exactly 3 common columns or rows.
///
/// This is a generalization of X-Wing (2-2) and Swordfish (3-3) using strong links.
/// Difficulty: SE 5.4
pub fn strong_links_fish_3(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        // Row-based: 3 rows with strong links sharing 3 columns
        find_strong_links_fish_rows(grid, acc, digit, 3);
        // Column-based: 3 columns with strong links sharing 3 rows
        find_strong_links_fish_cols(grid, acc, digit, 3);
    }
}

/// Find 4-Strong-Links Fish pattern: 4 rows or columns with strong links
/// that share exactly 4 common columns or rows.
///
/// This is also known as Jellyfish with strong links.
/// Difficulty: SE 5.8
pub fn strong_links_fish_4(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        find_strong_links_fish_4_rows(grid, acc, digit);
        find_strong_links_fish_4_cols(grid, acc, digit);
    }
}

/// Find 4-Strong-Links Fish pattern in rows.
///
/// Similar to Jellyfish but focuses on strong link patterns.
/// Unlike standard Jellyfish, we don't restrict candidate count per row.
fn find_strong_links_fish_4_rows(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    // Iterate on all combinations of 4 rows
    for (r1_idx, r1) in ROWS.iter().enumerate() {
        for (r2_idx, r2) in ROWS.iter().enumerate().skip(r1_idx + 1) {
            for (r3_idx, r3) in ROWS.iter().enumerate().skip(r2_idx + 1) {
                for (r4_idx, r4) in ROWS.iter().enumerate().skip(r3_idx + 1) {
                    let rows = [r1_idx, r2_idx, r3_idx, r4_idx];
                    let rows_data = [r1, r2, r3, r4];

                    // Find columns where each row has the digit
                    let mut row_cols: [Vec<u8>; 4] =
                        [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

                    let mut valid = true;
                    for (i, row) in rows_data.iter().enumerate() {
                        row_cols[i] = row
                            .cells
                            .iter()
                            .copied()
                            .filter(|&cell| grid.get(cell) == 0 && grid.candidates(cell).has(digit))
                            .map(|cell| cell % 9)
                            .collect();

                        // Each row must have at least 1 candidate
                        if row_cols[i].is_empty() {
                            valid = false;
                            break;
                        }
                    }

                    if !valid {
                        continue;
                    }

                    // Find union of all columns
                    let mut all_cols: Vec<u8> = Vec::new();
                    for cols in &row_cols {
                        for &c in cols {
                            if !all_cols.contains(&c) {
                                all_cols.push(c);
                            }
                        }
                    }

                    // Must have exactly 4 common columns
                    if all_cols.len() != 4 {
                        continue;
                    }

                    // Find eliminations in other rows
                    let mut eliminations = Vec::new();
                    for (r, row) in ROWS.iter().enumerate() {
                        if rows.contains(&r) {
                            continue;
                        }
                        for &c in &all_cols {
                            let cell_idx = row.cells[c as usize];
                            if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                            }
                        }
                    }

                    if !eliminations.is_empty() {
                        let desc = format!(
                            "4-Strong-Links Fish: digit {} in rows {},{},{},{} and columns {},{},{},{}",
                            digit,
                            r1_idx + 1,
                            r2_idx + 1,
                            r3_idx + 1,
                            r4_idx + 1,
                            all_cols[0] + 1,
                            all_cols[1] + 1,
                            all_cols[2] + 1,
                            all_cols[3] + 1
                        );
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::StrongLinksFish,
                            difficulty: 5.8,
                            technique_name: "4-Strong-Links Fish".to_string(),
                            description: desc,
                            cell: CellIndex::from(r1.cells[all_cols[0] as usize]),
                            value: 0,
                            eliminations,
                        });
                    }
                }
            }
        }
    }
}

/// Find 4-Strong-Links Fish pattern in columns.
///
/// Similar to Jellyfish but focuses on strong link patterns.
/// Unlike standard Jellyfish, we don't restrict candidate count per column.
fn find_strong_links_fish_4_cols(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    // Iterate on all combinations of 4 columns
    for (c1_idx, c1) in COLS.iter().enumerate() {
        for (c2_idx, c2) in COLS.iter().enumerate().skip(c1_idx + 1) {
            for (c3_idx, c3) in COLS.iter().enumerate().skip(c2_idx + 1) {
                for (c4_idx, c4) in COLS.iter().enumerate().skip(c3_idx + 1) {
                    let cols = [c1_idx, c2_idx, c3_idx, c4_idx];
                    let cols_data = [c1, c2, c3, c4];

                    let mut col_rows: [Vec<u8>; 4] =
                        [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

                    let mut valid = true;
                    for (i, col) in cols_data.iter().enumerate() {
                        col_rows[i] = col
                            .cells
                            .iter()
                            .copied()
                            .filter(|&cell| grid.get(cell) == 0 && grid.candidates(cell).has(digit))
                            .map(|cell| cell / 9)
                            .collect();

                        if col_rows[i].is_empty() {
                            valid = false;
                            break;
                        }
                    }

                    if !valid {
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

                    if all_rows.len() != 4 {
                        continue;
                    }

                    let mut eliminations = Vec::new();
                    for (c, col) in COLS.iter().enumerate() {
                        if cols.contains(&c) {
                            continue;
                        }
                        for &r in &all_rows {
                            let cell_idx = col.cells[r as usize];
                            if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                                eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                            }
                        }
                    }

                    if !eliminations.is_empty() {
                        let desc = format!(
                            "4-Strong-Links Fish: digit {} in columns {},{},{},{} and rows {},{},{},{}",
                            digit,
                            c1_idx + 1,
                            c2_idx + 1,
                            c3_idx + 1,
                            c4_idx + 1,
                            all_rows[0] + 1,
                            all_rows[1] + 1,
                            all_rows[2] + 1,
                            all_rows[3] + 1
                        );
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::StrongLinksFish,
                            difficulty: 5.8,
                            technique_name: "4-Strong-Links Fish".to_string(),
                            description: desc,
                            cell: CellIndex::from(c1.cells[all_rows[0] as usize]),
                            value: 0,
                            eliminations,
                        });
                    }
                }
            }
        }
    }
}

/// Find 5-Strong-Links Fish pattern.
/// Difficulty: SE 6.0
pub fn strong_links_fish_5(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        find_strong_links_fish_n_rows(grid, acc, digit, 5, 6.0, "5-Strong-Links Fish");
        find_strong_links_fish_n_cols(grid, acc, digit, 5, 6.0, "5-Strong-Links Fish");
    }
}

/// Find 6-Strong-Links Fish pattern.
/// Difficulty: SE 6.2
pub fn strong_links_fish_6(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        find_strong_links_fish_n_rows(grid, acc, digit, 6, 6.2, "6-Strong-Links Fish");
        find_strong_links_fish_n_cols(grid, acc, digit, 6, 6.2, "6-Strong-Links Fish");
    }
}

/// Generic N-Strong-Links Fish pattern for rows.
fn find_strong_links_fish_n_rows(
    grid: &Grid,
    acc: &mut HintAccumulator,
    digit: u8,
    n: usize,
    difficulty: f64,
    technique_name: &str,
) {
    if !(3..=6).contains(&n) {
        return;
    }

    let combinations = generate_combinations(9, n);
    for rows_idx in combinations {
        let rows_data: Vec<_> = rows_idx.iter().map(|&i| &ROWS[i]).collect();

        let mut row_cols: Vec<Vec<u8>> = Vec::new();
        for row in &rows_data {
            let cols: Vec<u8> = row
                .cells
                .iter()
                .copied()
                .filter(|&cell| grid.get(cell) == 0 && grid.candidates(cell).has(digit))
                .map(|cell| cell % 9)
                .collect();
            if cols.is_empty() {
                row_cols = Vec::new();
                break;
            }
            row_cols.push(cols);
        }

        if row_cols.len() != n {
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

        if all_cols.len() != n {
            continue;
        }

        let mut eliminations = Vec::new();
        for (r, row) in ROWS.iter().enumerate() {
            if rows_idx.contains(&r) {
                continue;
            }
            for &c in &all_cols {
                let cell_idx = row.cells[c as usize];
                if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                    eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                }
            }
        }

        if !eliminations.is_empty() {
            let rows_str = rows_idx
                .iter()
                .map(|r| (r + 1).to_string())
                .collect::<Vec<_>>()
                .join(",");
            let cols_str = all_cols
                .iter()
                .map(|c| (c + 1).to_string())
                .collect::<Vec<_>>()
                .join(",");

            let desc = format!(
                "{}: digit {} in rows {} and columns {}",
                technique_name, digit, rows_str, cols_str
            );
            acc.add(Hint {
                hint_type: crate::solver::HintType::StrongLinksFish,
                difficulty,
                technique_name: technique_name.to_string(),
                description: desc,
                cell: CellIndex::from(rows_data[0].cells[all_cols[0] as usize]),
                value: 0,
                eliminations,
            });
        }
    }
}

/// Generic N-Strong-Links Fish pattern for columns.
fn find_strong_links_fish_n_cols(
    grid: &Grid,
    acc: &mut HintAccumulator,
    digit: u8,
    n: usize,
    difficulty: f64,
    technique_name: &str,
) {
    if !(3..=6).contains(&n) {
        return;
    }

    let combinations = generate_combinations(9, n);
    for cols_idx in combinations {
        let cols_data: Vec<_> = cols_idx.iter().map(|&i| &COLS[i]).collect();

        let mut col_rows: Vec<Vec<u8>> = Vec::new();
        for col in &cols_data {
            let rows: Vec<u8> = col
                .cells
                .iter()
                .copied()
                .filter(|&cell| grid.get(cell) == 0 && grid.candidates(cell).has(digit))
                .map(|cell| cell / 9)
                .collect();
            if rows.is_empty() {
                col_rows = Vec::new();
                break;
            }
            col_rows.push(rows);
        }

        if col_rows.len() != n {
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

        if all_rows.len() != n {
            continue;
        }

        let mut eliminations = Vec::new();
        for (c, col) in COLS.iter().enumerate() {
            if cols_idx.contains(&c) {
                continue;
            }
            for &r in &all_rows {
                let cell_idx = col.cells[r as usize];
                if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                    eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                }
            }
        }

        if !eliminations.is_empty() {
            let cols_str = cols_idx
                .iter()
                .map(|c| (c + 1).to_string())
                .collect::<Vec<_>>()
                .join(",");
            let rows_str = all_rows
                .iter()
                .map(|r| (r + 1).to_string())
                .collect::<Vec<_>>()
                .join(",");

            let desc = format!(
                "{}: digit {} in columns {} and rows {}",
                technique_name, digit, cols_str, rows_str
            );
            acc.add(Hint {
                hint_type: crate::solver::HintType::StrongLinksFish,
                difficulty,
                technique_name: technique_name.to_string(),
                description: desc,
                cell: CellIndex::from(cols_data[0].cells[all_rows[0] as usize]),
                value: 0,
                eliminations,
            });
        }
    }
}

/// Generate combinations of n elements from 0..m.
fn generate_combinations(m: usize, n: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    fn gen(
        m: usize,
        n: usize,
        start: usize,
        depth: usize,
        current: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if depth == n {
            result.push(current.clone());
            return;
        }
        for i in start..m {
            if depth >= current.len() {
                current.push(i);
            } else {
                current[depth] = i;
            }
            gen(m, n, i + 1, depth + 1, current, result);
            if depth + 1 < current.len() && current[depth + 1] == 0 && depth < n - 1 {
                current.pop();
            }
        }
    }

    gen(m, n, 0, 0, &mut Vec::with_capacity(n), &mut result);
    result
}

/// Find 3-Strong-Links Fish pattern in rows.
fn find_strong_links_fish_rows(grid: &Grid, acc: &mut HintAccumulator, digit: u8, _degree: usize) {
    // Iterate on all combinations of 3 rows
    for (r1_idx, r1) in ROWS.iter().enumerate() {
        for (r2_idx, r2) in ROWS.iter().enumerate().skip(r1_idx + 1) {
            for (r3_idx, r3) in ROWS.iter().enumerate().skip(r2_idx + 1) {
                let rows = [r1_idx, r2_idx, r3_idx];
                let rows_data = [r1, r2, r3];

                // Find columns where each row has the digit
                let mut row_cols: [Vec<u8>; 3] = [Vec::new(), Vec::new(), Vec::new()];

                for (i, &_row_idx) in rows.iter().enumerate() {
                    let row = rows_data[i];
                    row_cols[i] = row
                        .cells
                        .iter()
                        .copied()
                        .filter(|&cell| grid.get(cell) == 0 && grid.candidates(cell).has(digit))
                        .map(|cell| cell % 9)
                        .collect();

                    // Each row must have the digit in 2-3 columns
                    if row_cols[i].len() < 2 || row_cols[i].len() > 3 {
                        continue;
                    }
                }

                // Check if all 3 rows have valid candidates
                if row_cols.iter().any(|c| c.is_empty()) {
                    continue;
                }

                // Find union of all columns
                let mut all_cols: Vec<u8> = Vec::new();
                for cols in &row_cols {
                    for &c in cols {
                        if !all_cols.contains(&c) {
                            all_cols.push(c);
                        }
                    }
                }

                // Must have exactly 3 common columns
                if all_cols.len() != 3 {
                    continue;
                }

                // Find eliminations in other rows
                let mut eliminations = Vec::new();
                for (r, row) in ROWS.iter().enumerate() {
                    if rows.contains(&r) {
                        continue;
                    }
                    for &c in &all_cols {
                        let cell_idx = row.cells[c as usize];
                        if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                            eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                        }
                    }
                }

                if !eliminations.is_empty() {
                    let desc = format!(
                        "3-Strong-Links Fish: digit {} in rows {},{},{} and columns {},{},{}",
                        digit,
                        r1_idx + 1,
                        r2_idx + 1,
                        r3_idx + 1,
                        all_cols[0] + 1,
                        all_cols[1] + 1,
                        all_cols[2] + 1
                    );
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::StrongLinksFish,
                        difficulty: 5.4,
                        technique_name: "3-Strong-Links Fish".to_string(),
                        description: desc,
                        cell: CellIndex::from(r1.cells[all_cols[0] as usize]),
                        value: 0,
                        eliminations,
                    });
                }
            }
        }
    }
}

/// Find 3-Strong-Links Fish pattern in columns.
fn find_strong_links_fish_cols(grid: &Grid, acc: &mut HintAccumulator, digit: u8, _degree: usize) {
    // Iterate on all combinations of 3 columns
    for (c1_idx, c1) in COLS.iter().enumerate() {
        for (c2_idx, c2) in COLS.iter().enumerate().skip(c1_idx + 1) {
            for (c3_idx, c3) in COLS.iter().enumerate().skip(c2_idx + 1) {
                let cols = [c1_idx, c2_idx, c3_idx];
                let cols_data = [c1, c2, c3];

                // Find rows where each column has the digit
                let mut col_rows: [Vec<u8>; 3] = [Vec::new(), Vec::new(), Vec::new()];

                for (i, &_col_idx) in cols.iter().enumerate() {
                    let col = cols_data[i];
                    col_rows[i] = col
                        .cells
                        .iter()
                        .copied()
                        .filter(|&cell| grid.get(cell) == 0 && grid.candidates(cell).has(digit))
                        .map(|cell| cell / 9)
                        .collect();

                    // Each column must have the digit in 2-3 rows
                    if col_rows[i].len() < 2 || col_rows[i].len() > 3 {
                        continue;
                    }
                }

                // Check if all 3 columns have valid candidates
                if col_rows.iter().any(|c| c.is_empty()) {
                    continue;
                }

                // Find union of all rows
                let mut all_rows: Vec<u8> = Vec::new();
                for rows in &col_rows {
                    for &r in rows {
                        if !all_rows.contains(&r) {
                            all_rows.push(r);
                        }
                    }
                }

                // Must have exactly 3 common rows
                if all_rows.len() != 3 {
                    continue;
                }

                // Find eliminations in other columns
                let mut eliminations = Vec::new();
                for (c, col) in COLS.iter().enumerate() {
                    if cols.contains(&c) {
                        continue;
                    }
                    for &r in &all_rows {
                        let cell_idx = col.cells[r as usize];
                        if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(digit) {
                            eliminations.push((CellIndex::from(cell_idx), vec![digit]));
                        }
                    }
                }

                if !eliminations.is_empty() {
                    let desc = format!(
                        "3-Strong-Links Fish: digit {} in columns {},{},{} and rows {},{},{}",
                        digit,
                        c1_idx + 1,
                        c2_idx + 1,
                        c3_idx + 1,
                        all_rows[0] + 1,
                        all_rows[1] + 1,
                        all_rows[2] + 1
                    );
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::StrongLinksFish,
                        difficulty: 5.4,
                        technique_name: "3-Strong-Links Fish".to_string(),
                        description: desc,
                        cell: CellIndex::from(c1.cells[all_rows[0] as usize]),
                        value: 0,
                        eliminations,
                    });
                }
            }
        }
    }
}

/// Add hint only if no equivalent hint already exists.
fn add_hint_unique(
    acc: &mut HintAccumulator,
    hint_type: crate::solver::HintType,
    difficulty: f64,
    technique_name: &str,
    description: String,
    cell: CellIndex,
    eliminations: Vec<(CellIndex, Vec<u8>)>,
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
