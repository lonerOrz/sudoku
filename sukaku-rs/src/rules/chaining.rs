use crate::grid::{Cell, Grid};
use crate::solver::{Hint, HintAccumulator};

pub fn x_cycles_simple(_grid: &Grid, _acc: &mut HintAccumulator) {}

pub fn y_cycles_simple(_grid: &Grid, _acc: &mut HintAccumulator) {}

pub fn forcing_chain(grid: &Grid, acc: &mut HintAccumulator) {
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let cands: Vec<u8> = grid.candidates(cell).iter().collect();
        if cands.len() != 2 {
            continue;
        }
        for &cand in &cands {
            let mut impl_grid = grid.clone();
            impl_grid.set(cell, cand);
            impl_grid.rebuild_candidates();
            for other in 0..81u8 {
                if other == cell || impl_grid.get(other) != 0 {
                    continue;
                }
                let impl_cands = impl_grid.candidates(other);
                let orig_cands = grid.candidates(other);
                for d in 1..=9u8 {
                    if orig_cands.has(d) && !impl_cands.has(d) {
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::ForcingChain,
                            difficulty: 7.0,
                            technique_name: "Forcing Chain".to_string(),
                            description: format!(
                                "Forcing chain eliminates {} from ({},{})",
                                d,
                                other / 9 + 1,
                                other % 9 + 1
                            ),
                            cell: Cell::from(cell),
                            value: 0,
                            eliminations: vec![(Cell::from(other), vec![d])],
                        });
                    }
                }
            }
        }
    }
}

pub fn nishio_forcing_chain(_grid: &Grid, _acc: &mut HintAccumulator) {}

pub fn multiple_forcing_chain(_grid: &Grid, _acc: &mut HintAccumulator) {}

pub fn dynamic_forcing_chain(_grid: &Grid, _acc: &mut HintAccumulator) {}

pub fn dynamic_forcing_chain_plus(grid: &Grid, acc: &mut HintAccumulator) {
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let cands: Vec<u8> = grid.candidates(cell).iter().collect();
        if cands.len() < 2 {
            continue;
        }
        for &cand in &cands {
            let mut impl_grid = grid.clone();
            impl_grid.set(cell, cand);
            impl_grid.rebuild_candidates();
            for other in 0..81u8 {
                if other == cell || impl_grid.get(other) != 0 {
                    continue;
                }
                let impl_cands = impl_grid.candidates(other);
                if impl_cands.cardinality() == 1 {
                    let forced_val = impl_cands.iter().next().unwrap();
                    let mut impl_grid2 = impl_grid.clone();
                    impl_grid2.set(other, forced_val);
                    impl_grid2.rebuild_candidates();
                    for third in 0..81u8 {
                        if third == cell || third == other || impl_grid2.get(third) != 0 {
                            continue;
                        }
                        let impl_cands2 = impl_grid2.candidates(third);
                        let orig_cands = grid.candidates(third);
                        for d in 1..=9u8 {
                            if orig_cands.has(d) && !impl_cands2.has(d) {
                                acc.add(Hint {
                                    hint_type: crate::solver::HintType::DynamicForcingChainPlus,
                                    difficulty: 9.0,
                                    technique_name: "Dynamic Forcing Chain+".to_string(),
                                    description: format!(
                                        "Dynamic forcing chain+ eliminates {} from ({},{})",
                                        d,
                                        third / 9 + 1,
                                        third % 9 + 1
                                    ),
                                    cell: Cell::from(cell),
                                    value: 0,
                                    eliminations: vec![(Cell::from(third), vec![d])],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
