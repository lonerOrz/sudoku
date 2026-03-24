use crate::grid::RegionType;
use crate::grid::{Cell, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

pub fn naked_single(grid: &Grid, acc: &mut HintAccumulator) {
    for i in 0..81 {
        if grid.get(i) == 0 {
            let cands = grid.candidates(i);
            if cands.cardinality() == 1 {
                if let Some(v) = cands.first() {
                    acc.add(Hint::naked_single(Cell::from(i), v));
                }
            }
        }
    }
}

pub fn hidden_single(grid: &Grid, acc: &mut HintAccumulator) {
    for &region in ROWS.iter().chain(COLS.iter()).chain(BLOCKS.iter()) {
        for value in 1..=9u8 {
            let positions: Vec<_> = region
                .cells
                .iter()
                .filter(|&&idx| grid.get(idx) == 0 && grid.candidates(idx).has(value))
                .collect();

            if positions.len() == 1 {
                let difficulty = if region.region_type == RegionType::Block {
                    1.2
                } else {
                    1.5
                };
                acc.add(Hint::hidden_single(
                    Cell::from(*positions[0]),
                    value,
                    difficulty,
                ));
            }
        }
    }
}
