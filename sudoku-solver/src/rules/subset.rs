use crate::grid::{CellIndex, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

/// Find three cells in a region with exactly three candidates total, eliminating those candidates from other cells.
pub fn naked_triple(grid: &Grid, acc: &mut HintAccumulator) {
    let regions: Vec<_> = ROWS
        .iter()
        .chain(COLS.iter())
        .chain(BLOCKS.iter())
        .collect();

    for region in regions {
        let empty_cells: Vec<u8> = region
            .cells
            .iter()
            .filter(|&&idx| {
                grid.get(idx) == 0
                    && grid.candidates(idx).cardinality() >= 2
                    && grid.candidates(idx).cardinality() <= 3
            })
            .copied()
            .collect();

        if empty_cells.len() < 3 {
            continue;
        }

        for i in 0..empty_cells.len() {
            for j in (i + 1)..empty_cells.len() {
                for k in (j + 1)..empty_cells.len() {
                    let cell1 = empty_cells[i];
                    let cell2 = empty_cells[j];
                    let cell3 = empty_cells[k];

                    let cands1 = grid.candidates(cell1);
                    let cands2 = grid.candidates(cell2);
                    let cands3 = grid.candidates(cell3);

                    let union = cands1.union(cands2).union(cands3);

                    if union.cardinality() == 3 {
                        let triple_values: Vec<u8> = union.iter().collect();

                        let mut eliminations = Vec::new();
                        for &cell in &region.cells {
                            if cell == cell1 || cell == cell2 || cell == cell3 {
                                continue;
                            }
                            if grid.get(cell) != 0 {
                                continue;
                            }

                            let cell_cands = grid.candidates(cell);
                            let mut to_remove = Vec::new();
                            for &v in &triple_values {
                                if cell_cands.has(v) {
                                    to_remove.push(v);
                                }
                            }

                            if !to_remove.is_empty() {
                                eliminations.push((CellIndex::from(cell), to_remove));
                            }
                        }

                        if !eliminations.is_empty() {
                            let desc = format!(
                                "Naked Triple ({}) in {:?}",
                                triple_values
                                    .iter()
                                    .map(|v| v.to_string())
                                    .collect::<Vec<_>>()
                                    .join(","),
                                region.region_type
                            );
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::NakedTriple,
                                difficulty: 3.6,
                                technique_name: "Naked Triple".to_string(),
                                description: desc,
                                cell: CellIndex::from(cell1),
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

/// Find four cells in a region with exactly four candidates total, eliminating those candidates from other cells.
pub fn naked_quad(grid: &Grid, acc: &mut HintAccumulator) {
    let regions: Vec<_> = ROWS
        .iter()
        .chain(COLS.iter())
        .chain(BLOCKS.iter())
        .collect();

    for region in regions {
        let empty_cells: Vec<u8> = region
            .cells
            .iter()
            .filter(|&&idx| {
                grid.get(idx) == 0
                    && grid.candidates(idx).cardinality() >= 2
                    && grid.candidates(idx).cardinality() <= 4
            })
            .copied()
            .collect();

        if empty_cells.len() < 4 {
            continue;
        }

        for i in 0..empty_cells.len() {
            for j in (i + 1)..empty_cells.len() {
                for k in (j + 1)..empty_cells.len() {
                    for l in (k + 1)..empty_cells.len() {
                        let cell1 = empty_cells[i];
                        let cell2 = empty_cells[j];
                        let cell3 = empty_cells[k];
                        let cell4 = empty_cells[l];

                        let cands1 = grid.candidates(cell1);
                        let cands2 = grid.candidates(cell2);
                        let cands3 = grid.candidates(cell3);
                        let cands4 = grid.candidates(cell4);

                        let union = cands1.union(cands2).union(cands3).union(cands4);

                        if union.cardinality() == 4 {
                            let quad_values: Vec<u8> = union.iter().collect();

                            let mut eliminations = Vec::new();
                            for &cell in &region.cells {
                                if cell == cell1 || cell == cell2 || cell == cell3 || cell == cell4
                                {
                                    continue;
                                }
                                if grid.get(cell) != 0 {
                                    continue;
                                }

                                let cell_cands = grid.candidates(cell);
                                let mut to_remove = Vec::new();
                                for &v in &quad_values {
                                    if cell_cands.has(v) {
                                        to_remove.push(v);
                                    }
                                }

                                if !to_remove.is_empty() {
                                    eliminations.push((CellIndex::from(cell), to_remove));
                                }
                            }

                            if !eliminations.is_empty() {
                                let desc = format!(
                                    "Naked Quad ({}) in {:?}",
                                    quad_values
                                        .iter()
                                        .map(|v| v.to_string())
                                        .collect::<Vec<_>>()
                                        .join(","),
                                    region.region_type
                                );
                                acc.add(Hint {
                                    hint_type: crate::solver::HintType::NakedQuad,
                                    difficulty: 5.0,
                                    technique_name: "Naked Quad".to_string(),
                                    description: desc,
                                    cell: CellIndex::from(cell1),
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

/// Find three digits that only appear in exactly three cells in a region, eliminating other candidates from those cells.
pub fn hidden_triple(grid: &Grid, acc: &mut HintAccumulator) {
    let regions: Vec<_> = ROWS
        .iter()
        .chain(COLS.iter())
        .chain(BLOCKS.iter())
        .collect();

    for region in regions {
        let empty_cells: Vec<u8> = region
            .cells
            .iter()
            .filter(|&&idx| grid.get(idx) == 0)
            .copied()
            .collect();

        if empty_cells.len() < 3 {
            continue;
        }

        for val1 in 1..=9u8 {
            for val2 in (val1 + 1)..=9 {
                for val3 in (val2 + 1)..=9 {
                    // Find cells that contain each value
                    let cells_with_val1: Vec<u8> = empty_cells
                        .iter()
                        .copied()
                        .filter(|&cell| grid.candidates(cell).has(val1))
                        .collect();
                    let cells_with_val2: Vec<u8> = empty_cells
                        .iter()
                        .copied()
                        .filter(|&cell| grid.candidates(cell).has(val2))
                        .collect();
                    let cells_with_val3: Vec<u8> = empty_cells
                        .iter()
                        .copied()
                        .filter(|&cell| grid.candidates(cell).has(val3))
                        .collect();

                    // Each value must appear in at least 1 and at most 3 cells
                    if cells_with_val1.is_empty()
                        || cells_with_val1.len() > 3
                        || cells_with_val2.is_empty()
                        || cells_with_val2.len() > 3
                        || cells_with_val3.is_empty()
                        || cells_with_val3.len() > 3
                    {
                        continue;
                    }

                    // The union of cells must be exactly 3 cells
                    let mut candidate_cells: Vec<u8> = cells_with_val1.clone();
                    for &c in &cells_with_val2 {
                        if !candidate_cells.contains(&c) {
                            candidate_cells.push(c);
                        }
                    }
                    for &c in &cells_with_val3 {
                        if !candidate_cells.contains(&c) {
                            candidate_cells.push(c);
                        }
                    }

                    if candidate_cells.len() != 3 {
                        continue;
                    }

                    let cell1 = candidate_cells[0];
                    let cell2 = candidate_cells[1];
                    let cell3 = candidate_cells[2];

                    let cands1 = grid.candidates(cell1);
                    let cands2 = grid.candidates(cell2);
                    let cands3 = grid.candidates(cell3);

                    let union = cands1.union(cands2).union(cands3);
                    if union.cardinality() != 3 {
                        continue;
                    }

                    let mut eliminations = Vec::new();
                    for (cell, cands) in [(cell1, cands1), (cell2, cands2), (cell3, cands3)] {
                        let to_remove: Vec<u8> = cands
                            .iter()
                            .filter(|&v| v != val1 && v != val2 && v != val3)
                            .collect();

                        if !to_remove.is_empty() {
                            eliminations.push((CellIndex::from(cell), to_remove));
                        }
                    }

                    if !eliminations.is_empty() {
                        let desc = format!(
                            "Hidden Triple ({},{},{}) in {:?}",
                            val1, val2, val3, region.region_type
                        );
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::HiddenTriple,
                            difficulty: 4.0,
                            technique_name: "Hidden Triple".to_string(),
                            description: desc,
                            cell: CellIndex::from(cell1),
                            value: 0,
                            eliminations,
                        });
                    }
                }
            }
        }
    }
}

/// Find four digits that only appear in exactly four cells in a region, eliminating other candidates from those cells.
pub fn hidden_quad(grid: &Grid, acc: &mut HintAccumulator) {
    let regions: Vec<_> = ROWS
        .iter()
        .chain(COLS.iter())
        .chain(BLOCKS.iter())
        .collect();

    for region in regions {
        let empty_cells: Vec<u8> = region
            .cells
            .iter()
            .filter(|&&idx| grid.get(idx) == 0)
            .copied()
            .collect();

        if empty_cells.len() < 4 {
            continue;
        }

        for val1 in 1..=9u8 {
            for val2 in (val1 + 1)..=9 {
                for val3 in (val2 + 1)..=9 {
                    for val4 in (val3 + 1)..=9 {
                        let cells_with_val1: Vec<u8> = empty_cells
                            .iter()
                            .copied()
                            .filter(|&cell| grid.candidates(cell).has(val1))
                            .collect();
                        let cells_with_val2: Vec<u8> = empty_cells
                            .iter()
                            .copied()
                            .filter(|&cell| grid.candidates(cell).has(val2))
                            .collect();
                        let cells_with_val3: Vec<u8> = empty_cells
                            .iter()
                            .copied()
                            .filter(|&cell| grid.candidates(cell).has(val3))
                            .collect();
                        let cells_with_val4: Vec<u8> = empty_cells
                            .iter()
                            .copied()
                            .filter(|&cell| grid.candidates(cell).has(val4))
                            .collect();

                        // Each value must appear in at least 1 and at most 4 cells
                        if cells_with_val1.is_empty()
                            || cells_with_val1.len() > 4
                            || cells_with_val2.is_empty()
                            || cells_with_val2.len() > 4
                            || cells_with_val3.is_empty()
                            || cells_with_val3.len() > 4
                            || cells_with_val4.is_empty()
                            || cells_with_val4.len() > 4
                        {
                            continue;
                        }

                        // The union of cells must be exactly 4 cells
                        let mut candidate_cells: Vec<u8> = cells_with_val1.clone();
                        for &c in &cells_with_val2 {
                            if !candidate_cells.contains(&c) {
                                candidate_cells.push(c);
                            }
                        }
                        for &c in &cells_with_val3 {
                            if !candidate_cells.contains(&c) {
                                candidate_cells.push(c);
                            }
                        }
                        for &c in &cells_with_val4 {
                            if !candidate_cells.contains(&c) {
                                candidate_cells.push(c);
                            }
                        }

                        if candidate_cells.len() != 4 {
                            continue;
                        }

                        let cell1 = candidate_cells[0];
                        let cell2 = candidate_cells[1];
                        let cell3 = candidate_cells[2];
                        let cell4 = candidate_cells[3];

                        let cands1 = grid.candidates(cell1);
                        let cands2 = grid.candidates(cell2);
                        let cands3 = grid.candidates(cell3);
                        let cands4 = grid.candidates(cell4);

                        let union = cands1.union(cands2).union(cands3).union(cands4);
                        if union.cardinality() != 4 {
                            continue;
                        }

                        let mut eliminations = Vec::new();
                        for (cell, cands) in [
                            (cell1, cands1),
                            (cell2, cands2),
                            (cell3, cands3),
                            (cell4, cands4),
                        ] {
                            let to_remove: Vec<u8> = cands
                                .iter()
                                .filter(|&v| v != val1 && v != val2 && v != val3 && v != val4)
                                .collect();

                            if !to_remove.is_empty() {
                                eliminations.push((CellIndex::from(cell), to_remove));
                            }
                        }

                        if !eliminations.is_empty() {
                            let desc = format!(
                                "Hidden Quad ({},{},{},{}) in {:?}",
                                val1, val2, val3, val4, region.region_type
                            );
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::HiddenQuad,
                                difficulty: 5.4,
                                technique_name: "Hidden Quad".to_string(),
                                description: desc,
                                cell: CellIndex::from(cell1),
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
