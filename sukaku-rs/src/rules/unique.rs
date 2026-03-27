use crate::grid::{Grid, ROWS};
use crate::solver::{Hint, HintAccumulator};

pub fn unique_rectangle_type1(grid: &Grid, acc: &mut HintAccumulator) {
    for r1 in 0..9 {
        for r2 in (r1 + 1)..9 {
            for c1 in 0..9 {
                for c2 in (c1 + 1)..9 {
                    let c1_idx = ROWS[r1 as usize].cells[c1 as usize];
                    let c2_idx = ROWS[r1 as usize].cells[c2 as usize];
                    let c3_idx = ROWS[r2 as usize].cells[c1 as usize];
                    let c4_idx = ROWS[r2 as usize].cells[c2 as usize];

                    if grid.get(c1_idx) != 0
                        || grid.get(c2_idx) != 0
                        || grid.get(c3_idx) != 0
                        || grid.get(c4_idx) != 0
                    {
                        continue;
                    }

                    let b1 = c1_idx / 9 / 3 * 3 + c1_idx % 9 / 3;
                    let b2 = c2_idx / 9 / 3 * 3 + c2_idx % 9 / 3;
                    let b3 = c3_idx / 9 / 3 * 3 + c3_idx % 9 / 3;
                    let b4 = c4_idx / 9 / 3 * 3 + c4_idx % 9 / 3;

                    if !(b1 == b2 && b3 == b4 && b1 == b3) {
                        continue;
                    }

                    let cands1 = grid.candidates(c1_idx);
                    let cands2 = grid.candidates(c2_idx);
                    let cands3 = grid.candidates(c3_idx);
                    let cands4 = grid.candidates(c4_idx);

                    let all_pairs: Vec<u8> = (1..=9)
                        .filter(|&d| {
                            cands1.has(d) && cands2.has(d) && cands3.has(d) && cands4.has(d)
                        })
                        .collect();

                    if all_pairs.len() != 2 {
                        continue;
                    }

                    let x = all_pairs[0];
                    let y = all_pairs[1];

                    let c1_is_xy = cands1.cardinality() == 2 && cands1.has(x) && cands1.has(y);
                    let c2_is_xy = cands2.cardinality() == 2 && cands2.has(x) && cands2.has(y);
                    let c3_is_xy = cands3.cardinality() == 2 && cands3.has(x) && cands3.has(y);
                    let c4_is_xy = cands4.cardinality() == 2 && cands4.has(x) && cands4.has(y);

                    let xy_count = [c1_is_xy, c2_is_xy, c3_is_xy, c4_is_xy]
                        .iter()
                        .filter(|&&b| b)
                        .count();

                    if xy_count == 3 {
                        let (pivot_idx, pivot_cands) = if !c1_is_xy {
                            (c1_idx, cands1)
                        } else if !c2_is_xy {
                            (c2_idx, cands2)
                        } else if !c3_is_xy {
                            (c3_idx, cands3)
                        } else {
                            (c4_idx, cands4)
                        };

                        let z = (1..=9).find(|&d| pivot_cands.has(d) && d != x && d != y);

                        if let Some(z) = z {
                            let desc = format!(
                                "Unique Rectangle Type 1: cells ({},{}), ({},{}), ({},{}), ({},{}) have {{ {}, {} }}, pivot cell ({},{}) has extra candidate {} -> must be {}",
                                r1 + 1, c1 + 1, r1 + 1, c2 + 1, r2 + 1, c1 + 1, r2 + 1, c2 + 1,
                                x, y,
                                pivot_idx / 9 + 1, pivot_idx % 9 + 1,
                                z, z
                            );

                            acc.add(Hint {
                                hint_type: crate::solver::HintType::UniqueRectangleType1,
                                difficulty: 4.5,
                                technique_name: "Unique Rectangle Type 1".to_string(),
                                description: desc,
                                cell: crate::grid::Cell::from(pivot_idx),
                                value: z,
                                eliminations: vec![],
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn unique_rectangle_type2(grid: &Grid, acc: &mut HintAccumulator) {
    for r1 in 0..9 {
        for r2 in (r1 + 1)..9 {
            for c1 in 0..9 {
                for c2 in (c1 + 1)..9 {
                    let c1_idx = ROWS[r1 as usize].cells[c1 as usize];
                    let c2_idx = ROWS[r1 as usize].cells[c2 as usize];
                    let c3_idx = ROWS[r2 as usize].cells[c1 as usize];
                    let c4_idx = ROWS[r2 as usize].cells[c2 as usize];

                    if grid.get(c1_idx) != 0
                        || grid.get(c2_idx) != 0
                        || grid.get(c3_idx) != 0
                        || grid.get(c4_idx) != 0
                    {
                        continue;
                    }

                    let b1 = c1_idx / 9 / 3 * 3 + c1_idx % 9 / 3;
                    let b2 = c2_idx / 9 / 3 * 3 + c2_idx % 9 / 3;
                    let b3 = c3_idx / 9 / 3 * 3 + c3_idx % 9 / 3;
                    let b4 = c4_idx / 9 / 3 * 3 + c4_idx % 9 / 3;

                    if !(b1 == b2 && b3 == b4 && b1 == b3) {
                        continue;
                    }

                    let cands1 = grid.candidates(c1_idx);
                    let cands2 = grid.candidates(c2_idx);
                    let cands3 = grid.candidates(c3_idx);
                    let cands4 = grid.candidates(c4_idx);

                    let all_pairs: Vec<u8> = (1..=9)
                        .filter(|&d| {
                            cands1.has(d) && cands2.has(d) && cands3.has(d) && cands4.has(d)
                        })
                        .collect();

                    if all_pairs.len() != 2 {
                        continue;
                    }

                    let x = all_pairs[0];
                    let y = all_pairs[1];

                    let get_extra = |cands: crate::grid::Candidates| -> Vec<u8> {
                        (1..=9)
                            .filter(|&d| d != x && d != y && cands.has(d))
                            .collect()
                    };

                    let extra1 = get_extra(cands1);
                    let extra2 = get_extra(cands2);
                    let extra3 = get_extra(cands3);
                    let extra4 = get_extra(cands4);

                    let cells = [
                        (c1_idx, c1, extra1),
                        (c2_idx, c2, extra2),
                        (c3_idx, c1, extra3),
                        (c4_idx, c2, extra4),
                    ];

                    for i in 0..4 {
                        for j in (i + 1)..4 {
                            let (_, _, ref extra_i) = cells[i];
                            let (_, _, ref extra_j) = cells[j];

                            if extra_i.len() == 1 && extra_j.len() == 1 && extra_i[0] == extra_j[0]
                            {
                                let z = extra_i[0];

                                let same_row = (cells[i].0 / 9) == (cells[j].0 / 9);
                                let same_col = (cells[i].0 % 9) == (cells[j].0 % 9);

                                if same_row || same_col {
                                    let mut eliminations = Vec::new();

                                    if same_row {
                                        let row = cells[i].0 / 9;
                                        for c in 0..9 {
                                            if c == c1 || c == c2 {
                                                continue;
                                            }
                                            let cell_idx = ROWS[row as usize].cells[c as usize];
                                            if grid.get(cell_idx) == 0
                                                && grid.candidates(cell_idx).has(z)
                                            {
                                                eliminations.push((
                                                    crate::grid::Cell::from(cell_idx),
                                                    vec![z],
                                                ));
                                            }
                                        }
                                    } else {
                                        let col = cells[i].0 % 9;
                                        for r in 0..9 {
                                            if r == r1 || r == r2 {
                                                continue;
                                            }
                                            let cell_idx = ROWS[r as usize].cells[col as usize];
                                            if grid.get(cell_idx) == 0
                                                && grid.candidates(cell_idx).has(z)
                                            {
                                                eliminations.push((
                                                    crate::grid::Cell::from(cell_idx),
                                                    vec![z],
                                                ));
                                            }
                                        }
                                    }

                                    if !eliminations.is_empty() {
                                        let desc = format!(
                                            "Unique Rectangle Type 2: digit {} in rows {},{} cols {},{} -> eliminate {} from same {}",
                                            z, r1 + 1, r2 + 1, c1 + 1, c2 + 1, z,
                                            if same_row { "row" } else { "column" }
                                        );

                                        acc.add(Hint {
                                            hint_type:
                                                crate::solver::HintType::UniqueRectangleType2,
                                            difficulty: 4.6,
                                            technique_name: "Unique Rectangle Type 2".to_string(),
                                            description: desc,
                                            cell: crate::grid::Cell::from(c1_idx),
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
    }
}

#[allow(dead_code)]
pub fn unique_rectangle_type3(grid: &Grid, acc: &mut HintAccumulator) {
    for r1 in 0..9 {
        for r2 in (r1 + 1)..9 {
            for c1 in 0..9 {
                for c2 in (c1 + 1)..9 {
                    let c1_idx = ROWS[r1 as usize].cells[c1 as usize];
                    let c2_idx = ROWS[r1 as usize].cells[c2 as usize];
                    let c3_idx = ROWS[r2 as usize].cells[c1 as usize];
                    let c4_idx = ROWS[r2 as usize].cells[c2 as usize];

                    if grid.get(c1_idx) != 0
                        || grid.get(c2_idx) != 0
                        || grid.get(c3_idx) != 0
                        || grid.get(c4_idx) != 0
                    {
                        continue;
                    }

                    let b1 = c1_idx / 9 / 3 * 3 + c1_idx % 9 / 3;
                    let b2 = c2_idx / 9 / 3 * 3 + c2_idx % 9 / 3;
                    let b3 = c3_idx / 9 / 3 * 3 + c3_idx % 9 / 3;
                    let b4 = c4_idx / 9 / 3 * 3 + c4_idx % 9 / 3;

                    if !(b1 == b2 && b3 == b4 && b1 == b3) {
                        continue;
                    }

                    let cands1 = grid.candidates(c1_idx);
                    let cands2 = grid.candidates(c2_idx);
                    let cands3 = grid.candidates(c3_idx);
                    let cands4 = grid.candidates(c4_idx);

                    let all_pairs: Vec<u8> = (1..=9)
                        .filter(|&d| {
                            cands1.has(d) && cands2.has(d) && cands3.has(d) && cands4.has(d)
                        })
                        .collect();

                    if all_pairs.len() != 2 {
                        continue;
                    }

                    let x = all_pairs[0];
                    let y = all_pairs[1];

                    let get_extra = |cands: crate::grid::Candidates| -> Vec<u8> {
                        (1..=9)
                            .filter(|&d| d != x && d != y && cands.has(d))
                            .collect()
                    };

                    let extra1 = get_extra(cands1);
                    let extra2 = get_extra(cands2);
                    let extra3 = get_extra(cands3);
                    let extra4 = get_extra(cands4);

                    let cells = [
                        (c1_idx, c1, extra1),
                        (c2_idx, c2, extra2),
                        (c3_idx, c1, extra3),
                        (c4_idx, c2, extra4),
                    ];

                    let extra_cells: Vec<_> =
                        cells.iter().filter(|&(_, _, e)| !e.is_empty()).collect();

                    if extra_cells.len() == 2 {
                        let extra_a = &extra_cells[0].2;
                        let extra_b = &extra_cells[1].2;

                        let combined: Vec<u8> =
                            extra_a.iter().chain(extra_b.iter()).copied().collect();

                        if combined.len() >= 2 {
                            for &extra_digit in &combined {
                                let mut eliminations = Vec::new();

                                for &(_, col, ref extra) in &cells {
                                    if extra.contains(&extra_digit) {
                                        continue;
                                    }
                                    for r in 0..9 {
                                        if r == r1 || r == r2 {
                                            continue;
                                        }
                                        let idx = ROWS[r as usize].cells[col as usize];
                                        if grid.get(idx) == 0
                                            && grid.candidates(idx).has(extra_digit)
                                        {
                                            eliminations.push((
                                                crate::grid::Cell::from(idx),
                                                vec![extra_digit],
                                            ));
                                        }
                                    }
                                }

                                if !eliminations.is_empty() {
                                    let desc = format!(
                                        "Unique Rectangle Type 3: digit {} in rows {},{} cols {},{} -> eliminate {}",
                                        extra_digit, r1 + 1, r2 + 1, c1 + 1, c2 + 1, extra_digit
                                    );

                                    acc.add(Hint {
                                        hint_type: crate::solver::HintType::UniqueRectangleType3,
                                        difficulty: 4.8,
                                        technique_name: "Unique Rectangle Type 3".to_string(),
                                        description: desc,
                                        cell: crate::grid::Cell::from(c1_idx),
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
    }
}

#[allow(dead_code)]
pub fn unique_rectangle_type4(grid: &Grid, acc: &mut HintAccumulator) {
    for r1 in 0..9 {
        for r2 in (r1 + 1)..9 {
            for c1 in 0..9 {
                for c2 in (c1 + 1)..9 {
                    let c1_idx = ROWS[r1 as usize].cells[c1 as usize];
                    let c2_idx = ROWS[r1 as usize].cells[c2 as usize];
                    let c3_idx = ROWS[r2 as usize].cells[c1 as usize];
                    let c4_idx = ROWS[r2 as usize].cells[c2 as usize];

                    if grid.get(c1_idx) != 0
                        || grid.get(c2_idx) != 0
                        || grid.get(c3_idx) != 0
                        || grid.get(c4_idx) != 0
                    {
                        continue;
                    }

                    let b1 = c1_idx / 9 / 3 * 3 + c1_idx % 9 / 3;
                    let b2 = c2_idx / 9 / 3 * 3 + c2_idx % 9 / 3;
                    let b3 = c3_idx / 9 / 3 * 3 + c3_idx % 9 / 3;
                    let b4 = c4_idx / 9 / 3 * 3 + c4_idx % 9 / 3;

                    if !(b1 == b2 && b3 == b4 && b1 == b3) {
                        continue;
                    }

                    let cands1 = grid.candidates(c1_idx);
                    let cands2 = grid.candidates(c2_idx);
                    let cands3 = grid.candidates(c3_idx);
                    let cands4 = grid.candidates(c4_idx);

                    let all_pairs: Vec<u8> = (1..=9)
                        .filter(|&d| {
                            cands1.has(d) && cands2.has(d) && cands3.has(d) && cands4.has(d)
                        })
                        .collect();

                    if all_pairs.len() != 2 {
                        continue;
                    }

                    let x = all_pairs[0];
                    let y = all_pairs[1];

                    let get_extra = |cands: crate::grid::Candidates| -> Vec<u8> {
                        (1..=9)
                            .filter(|&d| d != x && d != y && cands.has(d))
                            .collect()
                    };

                    let extra1 = get_extra(cands1);
                    let extra2 = get_extra(cands2);
                    let extra3 = get_extra(cands3);
                    let extra4 = get_extra(cands4);

                    let cells = [
                        (c1_idx, c1, extra1),
                        (c2_idx, c2, extra2),
                        (c3_idx, c1, extra3),
                        (c4_idx, c2, extra4),
                    ];

                    let extra_cells: Vec<_> =
                        cells.iter().filter(|&(_, _, e)| !e.is_empty()).collect();

                    if extra_cells.len() >= 2 {
                        for i in 0..extra_cells.len() {
                            for j in (i + 1)..extra_cells.len() {
                                let (idx_i, col_i, extra_i) =
                                    (extra_cells[i].0, extra_cells[i].1, &extra_cells[i].2);
                                let (idx_j, col_j, extra_j) =
                                    (extra_cells[j].0, extra_cells[j].1, &extra_cells[j].2);

                                if col_i == col_j {
                                    for &digit in extra_i {
                                        if !extra_j.contains(&digit) {
                                            continue;
                                        }
                                        let col = col_i as usize;
                                        let mut digit_count = 0;
                                        for r in 0..9 {
                                            let cell_idx = ROWS[r as usize].cells[col];
                                            if grid.get(cell_idx) == 0
                                                && grid.candidates(cell_idx).has(digit)
                                            {
                                                digit_count += 1;
                                            }
                                        }
                                        if digit_count == 2 {
                                            let mut eliminations = Vec::new();
                                            if grid.candidates(idx_i).has(x) {
                                                eliminations.push((
                                                    crate::grid::Cell::from(idx_i),
                                                    vec![x],
                                                ));
                                            }
                                            if grid.candidates(idx_j).has(x) {
                                                eliminations.push((
                                                    crate::grid::Cell::from(idx_j),
                                                    vec![x],
                                                ));
                                            }
                                            if !eliminations.is_empty() {
                                                let desc = format!(
                                                    "Unique Rectangle Type 4: digit {} forms strong link in col {}, rows {},{} -> eliminate {}",
                                                    digit, col + 1, idx_i / 9 + 1, idx_j / 9 + 1, x
                                                );
                                                acc.add(Hint {
                                                    hint_type:
                                                        crate::solver::HintType::UniqueRectangleType4,
                                                    difficulty: 5.0,
                                                    technique_name: "Unique Rectangle Type 4"
                                                        .to_string(),
                                                    description: desc,
                                                    cell: crate::grid::Cell::from(c1_idx),
                                                    value: 0,
                                                    eliminations,
                                                });
                                                return;
                                            }
                                        }
                                    }
                                }

                                let row_i = idx_i / 9;
                                let row_j = idx_j / 9;
                                if row_i == row_j {
                                    for &digit in extra_i {
                                        if !extra_j.contains(&digit) {
                                            continue;
                                        }
                                        let row = row_i as usize;
                                        let mut digit_count = 0;
                                        for c in 0..9 {
                                            let cell_idx = ROWS[row].cells[c as usize];
                                            if grid.get(cell_idx) == 0
                                                && grid.candidates(cell_idx).has(digit)
                                            {
                                                digit_count += 1;
                                            }
                                        }
                                        if digit_count == 2 {
                                            let mut eliminations = Vec::new();
                                            if grid.candidates(idx_i).has(x) {
                                                eliminations.push((
                                                    crate::grid::Cell::from(idx_i),
                                                    vec![x],
                                                ));
                                            }
                                            if grid.candidates(idx_j).has(x) {
                                                eliminations.push((
                                                    crate::grid::Cell::from(idx_j),
                                                    vec![x],
                                                ));
                                            }
                                            if !eliminations.is_empty() {
                                                let desc = format!(
                                                    "Unique Rectangle Type 4: digit {} forms strong link in row {}, cols {},{} -> eliminate {}",
                                                    digit, row + 1, idx_i % 9 + 1, idx_j % 9 + 1, x
                                                );
                                                acc.add(Hint {
                                                    hint_type:
                                                        crate::solver::HintType::UniqueRectangleType4,
                                                    difficulty: 5.0,
                                                    technique_name: "Unique Rectangle Type 4"
                                                        .to_string(),
                                                    description: desc,
                                                    cell: crate::grid::Cell::from(c1_idx),
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
                }
            }
        }
    }
}

/// BUG+1: Bivalue Universal Grave Type 1
///
/// A BUG pattern exists when:
/// - All unsolved cells have exactly 2 candidates, except one cell with 3 candidates
/// - Removing the "BUG-breaking" candidate from the triple-cell would create a deadly pattern
/// - In a deadly pattern, each digit appears exactly 0 or 2 times in each region
///
/// The triple-cell must contain the candidate that breaks the BUG pattern.
pub fn bug_plus_one(grid: &Grid, acc: &mut HintAccumulator) {
    // Step 1: Find the triple-cell (only cell with 3 candidates)
    let mut triple_cell: Option<(u8, crate::grid::Candidates)> = None;

    for i in 0..81 {
        if grid.get(i) == 0 {
            let cands = grid.candidates(i);
            let count = cands.cardinality();

            if count == 3 {
                if triple_cell.is_none() {
                    triple_cell = Some((i, cands));
                } else {
                    return; // More than one triple-cell, not BUG+1
                }
            } else if count != 2 {
                return; // Cell with != 2 or 3 candidates, not BUG+1
            }
        }
    }

    let (pivot_idx, pivot_cands) = match triple_cell {
        Some(cell) => cell,
        None => return, // No triple-cell found
    };

    // Step 2: Try each candidate in the triple-cell as the BUG-breaking value
    for d in pivot_cands.iter() {
        // Step 3: Check if removing d creates a BUG pattern
        if is_bug_pattern_without(grid, pivot_idx, d) {
            // Step 4: Verify this is a real BUG pattern (not a false positive)
            // The excluded digit must appear exactly 3 times in some region(s)
            // when counting the pivot's full candidates (before exclusion)
            if !is_real_bug_trigger(grid, pivot_idx, d) {
                continue;
            }

            // Found BUG+1: d must be the solution
            let desc =
                format!(
                "BUG+1: Cell ({},{}) with extra candidate {} breaks deadly pattern -> must be {}",
                pivot_idx / 9 + 1, pivot_idx % 9 + 1, d, d
            );

            acc.add(Hint {
                hint_type: crate::solver::HintType::BUGPlusOne,
                difficulty: 5.6,
                technique_name: "BUG+1".to_string(),
                description: desc,
                cell: crate::grid::Cell::from(pivot_idx),
                value: d,
                eliminations: vec![],
            });
            return;
        }
    }
}

/// Verify that excluding this digit creates a real BUG pattern.
///
/// A real BUG pattern requires that the excluded digit appears an odd number
/// of times (typically 3) in at least one region when counting all candidates.
/// This prevents false positives where the pattern happens to satisfy the
/// 0-or-2 rule but isn't actually a deadly pattern.
fn is_real_bug_trigger(grid: &Grid, _pivot_idx: u8, exclude_value: u8) -> bool {
    use crate::grid::{BLOCKS, COLS, ROWS};

    // Count occurrences of exclude_value in each region type
    // In a true BUG+1, the exclude_value should appear 3 times in some regions
    // (2 from the BUG pattern + 1 from the pivot's extra candidate)

    let regions: Vec<&[u8]> = ROWS
        .iter()
        .map(|r| r.cells.as_slice())
        .chain(COLS.iter().map(|c| c.cells.as_slice()))
        .chain(BLOCKS.iter().map(|b| b.cells.as_slice()))
        .collect();

    let mut found_anomaly = false;

    for &region in &regions {
        let mut count = 0;
        for &cell_idx in region {
            if grid.get(cell_idx) == 0 && grid.candidates(cell_idx).has(exclude_value) {
                count += 1;
            }
        }

        // In BUG+1, the exclude_value typically appears 3 times in regions
        // containing the pivot (2 from BUG + 1 extra = 3, which is odd)
        if count == 3 {
            found_anomaly = true;
        } else if count != 2 && count != 0 {
            // Other counts are also anomalies (1, 4, 5, etc.)
            if count == 1 || count > 3 {
                found_anomaly = true;
            }
        }
    }

    found_anomaly
}

/// Check if removing a candidate from the pivot cell creates a BUG pattern.
///
/// A BUG pattern requires:
/// 1. All unsolved cells have exactly 2 candidates
/// 2. Each digit appears exactly 0 or 2 times in every region (row, column, block)
fn is_bug_pattern_without(grid: &Grid, pivot_idx: u8, exclude_value: u8) -> bool {
    use crate::grid::{BLOCKS, COLS, ROWS};

    // Step 1: Verify all cells have exactly 2 candidates (excluding the pivot value)
    for i in 0..81 {
        if grid.get(i) == 0 {
            let cands = grid.candidates(i);
            let count = if i == pivot_idx {
                // For pivot, exclude the BUG-breaking value
                cands.iter().filter(|&v| v != exclude_value).count()
            } else {
                cands.cardinality() as usize
            };

            if count != 2 {
                return false;
            }
        }
    }

    // Step 2: Verify each digit appears 0 or 2 times in all regions
    let regions: Vec<&[u8]> = ROWS
        .iter()
        .map(|r| r.cells.as_slice())
        .chain(COLS.iter().map(|c| c.cells.as_slice()))
        .chain(BLOCKS.iter().map(|b| b.cells.as_slice()))
        .collect();

    for digit in 1..=9u8 {
        for &region in &regions {
            let mut count = 0;

            for &cell_idx in region {
                if grid.get(cell_idx) == 0 {
                    let cands = grid.candidates(cell_idx);
                    let has_digit = if cell_idx == pivot_idx {
                        // For pivot, check if digit is present (excluding the BUG-breaking value)
                        cands.has(digit) && digit != exclude_value
                    } else {
                        cands.has(digit)
                    };

                    if has_digit {
                        count += 1;
                    }
                }
            }

            // Each digit must appear 0 or 2 times in each region
            if count != 0 && count != 2 {
                return false;
            }
        }
    }

    true
}

/// BUG+2: Bivalue Universal Grave Type 2
///
/// A BUG+2 pattern exists when:
/// - All unsolved cells have exactly 2 candidates, except TWO cells with 3 candidates
/// - Both triple-cells share the SAME extra candidate (not in the BUG pattern)
/// - The extra candidate can be eliminated from cells visible to BOTH triple-cells
///
/// Difficulty: SE 5.8
pub fn bug_plus_two(grid: &Grid, acc: &mut HintAccumulator) {
    // Step 1: Find all triple-cells (cells with 3 candidates)
    let mut triple_cells: Vec<(u8, crate::grid::Candidates)> = Vec::new();

    for i in 0..81 {
        if grid.get(i) == 0 {
            let cands = grid.candidates(i);
            let count = cands.cardinality();

            if count == 3 {
                triple_cells.push((i, cands));
            } else if count != 2 {
                return; // Cell with != 2 or 3 candidates, not BUG+2
            }
        }
    }

    // Must have exactly 2 triple-cells
    if triple_cells.len() != 2 {
        return;
    }

    let (cell1_idx, cell1_cands) = triple_cells[0];
    let (cell2_idx, cell2_cands) = triple_cells[1];

    // Step 2: Find the extra values in each triple-cell
    // The BUG values are the intersection of both cells' candidates
    let bug_values: Vec<u8> = cell1_cands.iter().filter(|&v| cell2_cands.has(v)).collect();

    // Must have exactly 2 BUG values
    if bug_values.len() != 2 {
        return;
    }

    // Step 3: Find the extra value (the one not in BUG pattern)
    // Both cells should share the same extra value
    let extra1: Vec<u8> = cell1_cands
        .iter()
        .filter(|&v| !bug_values.contains(&v))
        .collect();
    let extra2: Vec<u8> = cell2_cands
        .iter()
        .filter(|&v| !bug_values.contains(&v))
        .collect();

    // Both must have exactly 1 extra value, and it must be the same
    if extra1.len() != 1 || extra2.len() != 1 || extra1[0] != extra2[0] {
        return;
    }

    let extra_value = extra1[0];

    // Step 4: Verify this is a real BUG pattern
    // Check that removing extra_value from both cells creates a BUG pattern
    if !is_bug_pattern_without_two_cells(grid, cell1_idx, cell2_idx, extra_value) {
        return;
    }

    // Step 5: Find common visible cells for elimination
    let cell1 = crate::grid::Cell::from(cell1_idx);

    let mut eliminations = Vec::new();

    // Find cells visible to both triple-cells
    for i in 0..81 {
        if grid.get(i) == 0 && i != cell1_idx && i != cell2_idx {
            let cands = grid.candidates(i);
            if cands.has(extra_value) {
                // Check if this cell is visible to both triple-cells
                if is_visible_cell(cell1_idx, i) && is_visible_cell(cell2_idx, i) {
                    eliminations.push((crate::grid::Cell::from(i), vec![extra_value]));
                }
            }
        }
    }

    if !eliminations.is_empty() {
        let desc = format!(
            "BUG+2: Two cells ({},{}) and ({},{}) with extra candidate {} -> eliminate {} from common peers",
            cell1_idx / 9 + 1,
            cell1_idx % 9 + 1,
            cell2_idx / 9 + 1,
            cell2_idx % 9 + 1,
            extra_value,
            extra_value
        );

        acc.add(Hint {
            hint_type: crate::solver::HintType::BUGPlusTwo,
            difficulty: 5.8,
            technique_name: "BUG+2".to_string(),
            description: desc,
            cell: cell1,
            value: 0,
            eliminations,
        });
    }
}

/// Check if two cells are visible to each other.
fn is_visible_cell(cell1: u8, cell2: u8) -> bool {
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

/// Check if removing a candidate from two cells creates a BUG pattern.
fn is_bug_pattern_without_two_cells(
    grid: &Grid,
    cell1_idx: u8,
    cell2_idx: u8,
    exclude_value: u8,
) -> bool {
    use crate::grid::{BLOCKS, COLS, ROWS};

    // Step 1: Verify all cells have exactly 2 candidates (excluding the extra value)
    for i in 0..81 {
        if grid.get(i) == 0 {
            let cands = grid.candidates(i);
            let count = if i == cell1_idx || i == cell2_idx {
                // For triple-cells, exclude the extra value
                cands.iter().filter(|&v| v != exclude_value).count()
            } else {
                cands.cardinality() as usize
            };

            if count != 2 {
                return false;
            }
        }
    }

    // Step 2: Verify each digit appears 0 or 2 times in all regions
    let regions: Vec<&[u8]> = ROWS
        .iter()
        .map(|r| r.cells.as_slice())
        .chain(COLS.iter().map(|c| c.cells.as_slice()))
        .chain(BLOCKS.iter().map(|b| b.cells.as_slice()))
        .collect();

    for digit in 1..=9u8 {
        for &region in &regions {
            let mut count = 0;

            for &cell_idx in region {
                if grid.get(cell_idx) == 0 {
                    let cands = grid.candidates(cell_idx);
                    let has_digit = if cell_idx == cell1_idx || cell_idx == cell2_idx {
                        cands.has(digit) && digit != exclude_value
                    } else {
                        cands.has(digit)
                    };

                    if has_digit {
                        count += 1;
                    }
                }
            }

            if count != 0 && count != 2 {
                return false;
            }
        }
    }

    true
}

/// BUG+3: Bivalue Universal Grave Type 3
///
/// A BUG+3 pattern exists when:
/// - All unsolved cells have exactly 2 candidates, except THREE cells with 3 candidates
/// - All three triple-cells share the SAME extra candidate (not in the BUG pattern)
/// - The extra candidate can be eliminated from cells visible to ALL triple-cells
///
/// Difficulty: SE 6.0
pub fn bug_plus_three(grid: &Grid, acc: &mut HintAccumulator) {
    // Step 1: Find all triple-cells (cells with 3 candidates)
    let mut triple_cells: Vec<(u8, crate::grid::Candidates)> = Vec::new();

    for i in 0..81 {
        if grid.get(i) == 0 {
            let cands = grid.candidates(i);
            let count = cands.cardinality();

            if count == 3 {
                triple_cells.push((i, cands));
            } else if count != 2 {
                return; // Cell with != 2 or 3 candidates, not BUG+3
            }
        }
    }

    // Must have exactly 3 triple-cells
    if triple_cells.len() != 3 {
        return;
    }

    let (cell1_idx, cell1_cands) = triple_cells[0];
    let (cell2_idx, cell2_cands) = triple_cells[1];
    let (cell3_idx, cell3_cands) = triple_cells[2];

    // Step 2: Find the BUG values (intersection of all three cells' candidates)
    let bug_values: Vec<u8> = cell1_cands
        .iter()
        .filter(|&v| cell2_cands.has(v) && cell3_cands.has(v))
        .collect();

    // Must have exactly 2 BUG values
    if bug_values.len() != 2 {
        return;
    }

    // Step 3: Find the extra value (the one not in BUG pattern)
    // All three cells should share the same extra value
    let extra1: Vec<u8> = cell1_cands
        .iter()
        .filter(|&v| !bug_values.contains(&v))
        .collect();
    let extra2: Vec<u8> = cell2_cands
        .iter()
        .filter(|&v| !bug_values.contains(&v))
        .collect();
    let extra3: Vec<u8> = cell3_cands
        .iter()
        .filter(|&v| !bug_values.contains(&v))
        .collect();

    // All must have exactly 1 extra value, and it must be the same
    if extra1.len() != 1 || extra2.len() != 1 || extra3.len() != 1 {
        return;
    }
    if extra1[0] != extra2[0] || extra2[0] != extra3[0] {
        return;
    }

    let extra_value = extra1[0];

    // Step 4: Verify this is a real BUG pattern
    // Check that removing extra_value from all three cells creates a BUG pattern
    if !is_bug_pattern_without_three_cells(grid, cell1_idx, cell2_idx, cell3_idx, extra_value) {
        return;
    }

    // Step 5: Find common visible cells for elimination
    let cell1 = crate::grid::Cell::from(cell1_idx);

    let mut eliminations = Vec::new();

    // Find cells visible to all three triple-cells
    for i in 0..81 {
        if grid.get(i) == 0 && i != cell1_idx && i != cell2_idx && i != cell3_idx {
            let cands = grid.candidates(i);
            if cands.has(extra_value) {
                // Check if this cell is visible to all three triple-cells
                if is_visible_cell(cell1_idx, i)
                    && is_visible_cell(cell2_idx, i)
                    && is_visible_cell(cell3_idx, i)
                {
                    eliminations.push((crate::grid::Cell::from(i), vec![extra_value]));
                }
            }
        }
    }

    if !eliminations.is_empty() {
        let desc = format!(
            "BUG+3: Three cells ({},{}), ({},{}), ({},{}) with extra candidate {} -> eliminate {} from common peers",
            cell1_idx / 9 + 1,
            cell1_idx % 9 + 1,
            cell2_idx / 9 + 1,
            cell2_idx % 9 + 1,
            cell3_idx / 9 + 1,
            cell3_idx % 9 + 1,
            extra_value,
            extra_value
        );

        acc.add(Hint {
            hint_type: crate::solver::HintType::BUGPlusThree,
            difficulty: 6.0,
            technique_name: "BUG+3".to_string(),
            description: desc,
            cell: cell1,
            value: 0,
            eliminations,
        });
    }
}

/// Check if removing a candidate from three cells creates a BUG pattern.
fn is_bug_pattern_without_three_cells(
    grid: &Grid,
    cell1_idx: u8,
    cell2_idx: u8,
    cell3_idx: u8,
    exclude_value: u8,
) -> bool {
    use crate::grid::{BLOCKS, COLS, ROWS};

    // Step 1: Verify all cells have exactly 2 candidates (excluding the extra value)
    for i in 0..81 {
        if grid.get(i) == 0 {
            let cands = grid.candidates(i);
            let count = if i == cell1_idx || i == cell2_idx || i == cell3_idx {
                // For triple-cells, exclude the extra value
                cands.iter().filter(|&v| v != exclude_value).count()
            } else {
                cands.cardinality() as usize
            };

            if count != 2 {
                return false;
            }
        }
    }

    // Step 2: Verify each digit appears 0 or 2 times in all regions
    let regions: Vec<&[u8]> = ROWS
        .iter()
        .map(|r| r.cells.as_slice())
        .chain(COLS.iter().map(|c| c.cells.as_slice()))
        .chain(BLOCKS.iter().map(|b| b.cells.as_slice()))
        .collect();

    for digit in 1..=9u8 {
        for &region in &regions {
            let mut count = 0;

            for &cell_idx in region {
                if grid.get(cell_idx) == 0 {
                    let cands = grid.candidates(cell_idx);
                    let has_digit = if cell_idx == cell1_idx
                        || cell_idx == cell2_idx
                        || cell_idx == cell3_idx
                    {
                        cands.has(digit) && digit != exclude_value
                    } else {
                        cands.has(digit)
                    };

                    if has_digit {
                        count += 1;
                    }
                }
            }

            if count != 0 && count != 2 {
                return false;
            }
        }
    }

    true
}
