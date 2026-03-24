use crate::grid::{Cell, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

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
                                                "Naked Pair in {:?}: {} removed from {:?}",
                                                region.region_type,
                                                removed
                                                    .iter()
                                                    .map(|v| v.to_string())
                                                    .collect::<Vec<_>>()
                                                    .join(","),
                                                Cell::from(*cell3)
                                            );
                                            acc.add(Hint {
                                                hint_type: crate::solver::HintType::NakedPair,
                                                difficulty: 3.0,
                                                technique_name: "Naked Pair".to_string(),
                                                description: desc,
                                                cell: Cell::from(*cell3),
                                                value: 0,
                                                eliminated_candidates: removed,
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
