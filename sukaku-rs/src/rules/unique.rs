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

pub fn bug_plus_one(grid: &Grid, acc: &mut HintAccumulator) {
    let mut bivalue_cells = 0;
    let mut triple_cell: Option<(u8, crate::grid::Candidates)> = None;

    for i in 0..81 {
        if grid.get(i) == 0 {
            let cands = grid.candidates(i);
            let count = cands.cardinality();

            if count == 2 {
                bivalue_cells += 1;
            } else if count == 3 {
                if triple_cell.is_none() {
                    triple_cell = Some((i, cands));
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }

    let empty_count = (0..81).filter(|&i| grid.get(i) == 0).count();
    if empty_count > 1 && bivalue_cells == empty_count - 1 {
        if let Some((pivot_idx, pivot_cands)) = triple_cell {
            for &d in &[1, 2, 3] {
                if !pivot_cands.has(d) {
                    continue;
                }

                let in_bivalue: usize = (1..=9)
                    .filter(|&x| {
                        if x == d {
                            return false;
                        }
                        let mut count = 0;
                        for i in 0..81 {
                            if grid.get(i) == 0
                                && grid.candidates(i).has(x)
                                && grid.candidates(i).cardinality() == 2
                            {
                                count += 1;
                            }
                        }
                        count >= 2
                    })
                    .count();

                if in_bivalue == 1 {
                    let desc = format!(
                        "BUG+1: Only cell ({},{}) has 3 candidates, digit {} breaks BUG pattern -> must be {}",
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
    }
}
