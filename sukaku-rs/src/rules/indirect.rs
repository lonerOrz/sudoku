use crate::grid::{Cell, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

/// Find Hidden Pairs: two digits that only appear in exactly two cells in a region.
/// These two cells can only contain those two digits, so other candidates can be removed.
pub fn hidden_pair(grid: &Grid, acc: &mut HintAccumulator) {
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

        if empty_cells.len() < 2 {
            continue;
        }

        for val1 in 1..=9u8 {
            let cells_with_val1: Vec<u8> = empty_cells
                .iter()
                .filter(|&&cell| grid.candidates(cell).has(val1))
                .copied()
                .collect();

            if cells_with_val1.len() != 2 {
                continue;
            }

            for val2 in (val1 + 1)..=9 {
                let cells_with_val2: Vec<u8> = empty_cells
                    .iter()
                    .filter(|&&cell| grid.candidates(cell).has(val2))
                    .copied()
                    .collect();

                if cells_with_val2.len() != 2 {
                    continue;
                }

                if cells_with_val1 == cells_with_val2 {
                    let cell1 = cells_with_val1[0];
                    let cell2 = cells_with_val1[1];
                    let cands1 = grid.candidates(cell1);
                    let cands2 = grid.candidates(cell2);

                    if cands1 == cands2 {
                        continue;
                    }

                    let elim1: Vec<u8> =
                        cands1.iter().filter(|v| *v != val1 && *v != val2).collect();
                    let elim2: Vec<u8> =
                        cands2.iter().filter(|v| *v != val1 && *v != val2).collect();

                    if !elim1.is_empty() || !elim2.is_empty() {
                        let desc = format!(
                            "Hidden Pair ({},{}) in {:?}",
                            val1, val2, region.region_type
                        );
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::HiddenPair,
                            difficulty: 2.9,
                            technique_name: "Hidden Pair".to_string(),
                            description: desc,
                            cell: Cell::from(cell1),
                            value: 0,
                            eliminations: vec![
                                (Cell::from(cell1), elim1),
                                (Cell::from(cell2), elim2),
                            ],
                        });
                    }
                }
            }
        }
    }
}

pub fn naked_pair(grid: &Grid, acc: &mut HintAccumulator) {
    let regions: Vec<_> = ROWS
        .iter()
        .chain(COLS.iter())
        .chain(BLOCKS.iter())
        .collect();

    for region in regions {
        let empty_cells: Vec<u8> = region
            .cells
            .iter()
            .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).cardinality() == 2)
            .copied()
            .collect();

        for (i, cell1) in empty_cells.iter().enumerate() {
            let cands1 = grid.candidates(*cell1);

            for cell2 in empty_cells.iter().skip(i + 1) {
                let cands2 = grid.candidates(*cell2);

                if cands1 == cands2 {
                    let pair_values: Vec<u8> = cands1.iter().collect();
                    if pair_values.len() == 2 {
                        for cell3 in region.cells.iter() {
                            if *cell3 != *cell1 && *cell3 != *cell2 && grid.get(*cell3) == 0 {
                                let cell3_cands = grid.candidates(*cell3);
                                if cell3_cands.has(pair_values[0])
                                    || cell3_cands.has(pair_values[1])
                                {
                                    let mut new_cands = cell3_cands;
                                    new_cands.remove(pair_values[0]);
                                    new_cands.remove(pair_values[1]);
                                    if new_cands != cell3_cands {
                                        let removed: Vec<u8> = pair_values
                                            .iter()
                                            .filter(|&&v| cell3_cands.has(v) && !new_cands.has(v))
                                            .copied()
                                            .collect();
                                        if !removed.is_empty() {
                                            let desc = format!(
                                                "Naked Pair in {:?}: eliminated from {:?}",
                                                region.region_type,
                                                Cell::from(*cell3)
                                            );
                                            acc.add(Hint {
                                                hint_type: crate::solver::HintType::NakedPair,
                                                difficulty: 3.0,
                                                technique_name: "Naked Pair".to_string(),
                                                description: desc,
                                                cell: Cell::from(*cell3),
                                                value: 0,
                                                eliminations: vec![(Cell::from(*cell3), removed)],
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
}
