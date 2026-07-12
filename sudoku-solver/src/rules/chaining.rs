use crate::grid::{CellIndex, Grid};
use crate::solver::{Hint, HintAccumulator};

/// STUB: Not yet implemented. Returns no hints.
pub fn x_cycles_simple(_grid: &Grid, _acc: &mut HintAccumulator) {}

/// STUB: Not yet implemented. Returns no hints.
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
        // Collect elimination sets from each branch (each candidate)
        let mut branch_elims: Vec<std::collections::HashSet<(u8, u8)>> = Vec::new();
        for &cand in &cands {
            let mut impl_grid = *grid;
            impl_grid.set(cell, cand);
            impl_grid.rebuild_candidates();
            let mut elims = std::collections::HashSet::new();
            for other in 0..81u8 {
                if other == cell || impl_grid.get(other) != 0 {
                    continue;
                }
                let impl_cands = impl_grid.candidates(other);
                let orig_cands = grid.candidates(other);
                for d in 1..=9u8 {
                    if orig_cands.has(d) && !impl_cands.has(d) {
                        elims.insert((other, d));
                    }
                }
            }
            branch_elims.push(elims);
        }
        // Only eliminate if ALL branches agree
        if branch_elims.len() == 2 {
            let common: Vec<(u8, u8)> = branch_elims[0]
                .intersection(&branch_elims[1])
                .copied()
                .collect();
            // Group by cell
            let mut grouped: std::collections::HashMap<u8, Vec<u8>> =
                std::collections::HashMap::new();
            for &(c, d) in &common {
                grouped.entry(c).or_default().push(d);
            }
            if !grouped.is_empty() {
                let eliminations: Vec<(CellIndex, Vec<u8>)> = grouped
                    .into_iter()
                    .map(|(c, vs)| (CellIndex::from(c), vs))
                    .collect();
                acc.add(Hint {
                    hint_type: crate::solver::HintType::ForcingChain,
                    difficulty: 7.0,
                    technique_name: "Forcing Chain".to_string(),
                    description: format!(
                        "Forcing chain from ({},{}) eliminates {:?}",
                        cell / 9 + 1,
                        cell % 9 + 1,
                        eliminations
                            .iter()
                            .map(|(c, vs)| format!("{}:rem{:?}", c.index, vs))
                            .collect::<Vec<_>>()
                    ),
                    cell: CellIndex::from(cell),
                    value: 0,
                    eliminations,
                });
            }
        }
    }
}

/// STUB: Not yet implemented. Returns no hints.
pub fn nishio_forcing_chain(_grid: &Grid, _acc: &mut HintAccumulator) {}

/// STUB: Not yet implemented. Returns no hints.
pub fn multiple_forcing_chain(_grid: &Grid, _acc: &mut HintAccumulator) {}

/// STUB: Not yet implemented. Returns no hints.
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
        // Collect elimination sets from each top-level branch
        let mut branch_elims: Vec<std::collections::HashSet<(u8, u8)>> = Vec::new();
        for &cand in &cands {
            let mut impl_grid = *grid;
            impl_grid.set(cell, cand);
            impl_grid.rebuild_candidates();
            let mut elims = std::collections::HashSet::new();
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
                                elims.insert((third, d));
                            }
                        }
                    }
                }
            }
            branch_elims.push(elims);
        }
        // Only eliminate if ALL branches agree
        if branch_elims.len() >= 2 {
            let mut common = branch_elims[0].clone();
            for i in 1..branch_elims.len() {
                common = common.intersection(&branch_elims[i]).copied().collect();
            }
            let mut grouped: std::collections::HashMap<u8, Vec<u8>> =
                std::collections::HashMap::new();
            for &(c, d) in &common {
                grouped.entry(c).or_default().push(d);
            }
            if !grouped.is_empty() {
                let eliminations: Vec<(CellIndex, Vec<u8>)> = grouped
                    .into_iter()
                    .map(|(c, vs)| (CellIndex::from(c), vs))
                    .collect();
                acc.add(Hint {
                    hint_type: crate::solver::HintType::DynamicForcingChainPlus,
                    difficulty: 9.0,
                    technique_name: "Dynamic Forcing Chain+".to_string(),
                    description: format!(
                        "Dynamic forcing chain+ from ({},{}) eliminates {:?}",
                        cell / 9 + 1,
                        cell % 9 + 1,
                        eliminations
                            .iter()
                            .map(|(c, vs)| format!("{}:rem{:?}", c.index, vs))
                            .collect::<Vec<_>>()
                    ),
                    cell: CellIndex::from(cell),
                    value: 0,
                    eliminations,
                });
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
        // Check each branch for contradiction
        let mut contradiction_branches = Vec::new();
        for &cand in &cands {
            let mut impl_grid = *grid;
            impl_grid.set(cell, cand);
            impl_grid.rebuild_candidates();
            let mut elims = Vec::new();
            let contradicts = find_nested_implications(&impl_grid, depth - 1, &mut elims);
            contradiction_branches.push((contradicts, elims));
        }
        // If exactly one branch contradicts, the other is forced → compute eliminations
        let (c0, _) = &contradiction_branches[0];
        let (c1, _) = &contradiction_branches[1];
        let forced_cand = if *c0 && !*c1 {
            Some(cands[1])
        } else if *c1 && !*c0 {
            Some(cands[0])
        } else {
            None
        };
        if let Some(forced_val) = forced_cand {
            // Compute eliminations by placing the forced value and comparing candidates
            let mut forced_grid = *grid;
            forced_grid.set(cell, forced_val);
            forced_grid.rebuild_candidates();
            let mut forced_elims: Vec<(CellIndex, Vec<u8>)> = Vec::new();
            for other in 0..81u8 {
                if other == cell || grid.get(other) != 0 {
                    continue;
                }
                let orig_cands: Vec<u8> = grid.candidates(other).iter().collect();
                let new_cands: Vec<u8> = forced_grid.candidates(other).iter().collect();
                let eliminated: Vec<u8> = orig_cands
                    .iter()
                    .filter(|v| !new_cands.contains(v))
                    .copied()
                    .collect();
                if !eliminated.is_empty() {
                    forced_elims.push((CellIndex::from(other), eliminated));
                }
            }
            if !forced_elims.is_empty() {
                acc.add(Hint {
                    hint_type: hint_type.clone(),
                    difficulty,
                    technique_name: name.to_string(),
                    description: format!("{}: cell ({},{})", name, cell / 9 + 1, cell % 9 + 1),
                    cell: CellIndex::from(cell),
                    value: 0,
                    eliminations: forced_elims,
                });
            }
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
