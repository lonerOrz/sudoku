use crate::grid::{Cell, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

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
                                eliminations.push((Cell::from(cell), to_remove));
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
                                cell: Cell::from(cell1),
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
