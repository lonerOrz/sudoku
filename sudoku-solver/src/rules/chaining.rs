use crate::grid::{CellIndex, Grid};
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
            let mut impl_grid = *grid;
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
                            cell: CellIndex::from(cell),
                            value: 0,
                            eliminations: vec![(CellIndex::from(other), vec![d])],
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
            let mut impl_grid = *grid;
            impl_grid.set(cell, cand);
            impl_grid.rebuild_candidates();
            for other in 0..81u8 {
                if other == cell || impl_grid.get(other) != 0 {
                    continue;
                }
                let impl_cands = impl_grid.candidates(other);
                if impl_cands.cardinality() == 1 {
                    let forced_val = impl_cands.iter().next().unwrap();
                    let mut impl_grid2 = impl_grid;
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
                                    cell: CellIndex::from(cell),
                                    value: 0,
                                    eliminations: vec![(CellIndex::from(third), vec![d])],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn nested_forcing_chain_2(grid: &Grid, acc: &mut HintAccumulator) {
    nested_forcing_chain(
        grid,
        acc,
        2,
        9.5,
        "Nested Forcing Chain (2-level)",
        crate::solver::HintType::NestedForcingChain2,
    );
}

pub fn nested_forcing_chain_3(grid: &Grid, acc: &mut HintAccumulator) {
    nested_forcing_chain(
        grid,
        acc,
        3,
        10.0,
        "Nested Forcing Chain (3-level)",
        crate::solver::HintType::NestedForcingChain3,
    );
}

pub fn nested_forcing_chain_4(grid: &Grid, acc: &mut HintAccumulator) {
    nested_forcing_chain(
        grid,
        acc,
        4,
        10.5,
        "Nested Forcing Chain (4-level)",
        crate::solver::HintType::NestedForcingChain4,
    );
}

fn nested_forcing_chain(
    grid: &Grid,
    acc: &mut HintAccumulator,
    depth: usize,
    difficulty: f64,
    name: &str,
    hint_type: crate::solver::HintType,
) {
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let cands: Vec<u8> = grid.candidates(cell).iter().collect();
        if cands.len() != 2 {
            continue;
        }
        let mut eliminations = Vec::new();
        for &cand in &cands {
            let mut impl_grid = *grid;
            impl_grid.set(cell, cand);
            impl_grid.rebuild_candidates();
            if find_nested_implications(&impl_grid, depth - 1, &mut eliminations) {
                break;
            }
        }
        if !eliminations.is_empty() {
            acc.add(Hint {
                hint_type: hint_type.clone(),
                difficulty,
                technique_name: name.to_string(),
                description: format!("{}: cell ({},{})", name, cell / 9 + 1, cell % 9 + 1),
                cell: CellIndex::from(cell),
                value: 0,
                eliminations,
            });
        }
    }
}

fn find_nested_implications(
    grid: &Grid,
    depth: usize,
    eliminations: &mut Vec<(CellIndex, Vec<u8>)>,
) -> bool {
    if depth == 0 {
        for other in 0..81u8 {
            if grid.get(other) == 0 && grid.candidates(other).is_empty() {
                eliminations.push((CellIndex::from(other), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
                return true;
            }
        }
        return false;
    }

    for other in 0..81u8 {
        if grid.get(other) != 0 {
            continue;
        }
        let cands = grid.candidates(other);
        if cands.cardinality() == 1 {
            let val = cands.iter().next().unwrap();
            let mut next_grid = *grid;
            next_grid.set(other, val);
            next_grid.rebuild_candidates();
            if find_nested_implications(&next_grid, depth - 1, eliminations) {
                return true;
            }
        }
    }

    false
}
