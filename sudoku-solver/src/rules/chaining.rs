use crate::grid::{CellIndex, Grid};
use crate::solver::{Hint, HintAccumulator};

pub fn x_cycles_simple(grid: &Grid, acc: &mut HintAccumulator) {
    use crate::grid::{BLOCKS, COLS, ROWS};
    for d in 1..=9u8 {
        // Find all cells where d is a candidate
        let d_cells: Vec<u8> = (0..81u8)
            .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(d))
            .collect();
        if d_cells.len() < 2 {
            continue;
        }

        // Find strong links: pairs of cells that are the only 2 with d in some unit
        let units: &[&[crate::grid::Region]] = &[&ROWS, &COLS, &BLOCKS];
        let mut strong_links: Vec<(u8, u8)> = Vec::new();
        for unit_group in units {
            for region in unit_group.iter() {
                let in_unit: Vec<u8> = region
                    .cells
                    .iter()
                    .copied()
                    .filter(|&c| d_cells.contains(&c))
                    .collect();
                if in_unit.len() == 2 {
                    let a = in_unit[0];
                    let b = in_unit[1];
                    if !strong_links
                        .iter()
                        .any(|&(x, y)| (x == a && y == b) || (x == b && y == a))
                    {
                        strong_links.push((a, b));
                    }
                }
            }
        }

        if strong_links.is_empty() {
            continue;
        }

        // Simple X-Cycle: for each strong link (a,b), any cell c that sees both a and b
        // can have d eliminated (since exactly one of a,b has d, c can't have d if it sees both).
        // Also check 3-cell chains: a—b—c where a-c strong link exists and b sees both a and c.
        let mut eliminations: std::collections::HashSet<(u8, u8)> =
            std::collections::HashSet::new();

        for &(a, b) in &strong_links {
            // Direct: cell seeing both ends of a strong link
            for &c in &d_cells {
                if c == a || c == b {
                    continue;
                }
                if cells_see_each_other(a, c) && cells_see_each_other(b, c) {
                    eliminations.insert((c, d));
                }
            }
        }

        // Chains of length 3: a—b strong, b—c strong, a and c in same unit
        for &(a, b) in &strong_links {
            for &(b2, c) in &strong_links {
                if b != b2 || a == c || a == b2 || b == c {
                    continue;
                }
                // a—b—c chain: if a and c see each other, this is an odd cycle
                if cells_see_each_other(a, c) {
                    // Any cell seeing both a and c can be eliminated
                    for &x in &d_cells {
                        if x == a || x == c {
                            continue;
                        }
                        if cells_see_each_other(a, x) && cells_see_each_other(c, x) {
                            eliminations.insert((x, d));
                        }
                    }
                }
            }
        }

        if !eliminations.is_empty() {
            let mut grouped: std::collections::HashMap<u8, Vec<u8>> =
                std::collections::HashMap::new();
            for &(c, val) in &eliminations {
                grouped.entry(c).or_default().push(val);
            }
            let elims: Vec<(CellIndex, Vec<u8>)> = grouped
                .into_iter()
                .map(|(c, vs)| (CellIndex::from(c), vs))
                .collect();
            acc.add(Hint {
                hint_type: crate::solver::HintType::XCyclesSimple,
                difficulty: 6.5,
                technique_name: "X-Cycles Simple".to_string(),
                description: format!(
                    "X-Cycles Simple on digit {} eliminates {:?}",
                    d,
                    elims
                        .iter()
                        .map(|(c, vs)| format!("{}:rem{:?}", c.index, vs))
                        .collect::<Vec<_>>()
                ),
                cell: CellIndex::from(0u8),
                value: 0,
                eliminations: elims,
            });
        }
    }
}

pub fn y_cycles_simple(grid: &Grid, acc: &mut HintAccumulator) {
    use crate::grid::{BLOCKS, COLS, ROWS};
    for d in 1..=9u8 {
        let d_cells: Vec<u8> = (0..81u8)
            .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(d))
            .collect();
        if d_cells.len() < 2 {
            continue;
        }

        let units: &[&[crate::grid::Region]] = &[&ROWS, &COLS, &BLOCKS];
        let mut adj: std::collections::HashMap<u8, Vec<u8>> = std::collections::HashMap::new();
        for unit_group in units {
            for region in unit_group.iter() {
                let in_unit: Vec<u8> = region
                    .cells
                    .iter()
                    .copied()
                    .filter(|&c| d_cells.contains(&c))
                    .collect();
                if in_unit.len() == 2 {
                    let a = in_unit[0];
                    let b = in_unit[1];
                    adj.entry(a).or_default().push(b);
                    adj.entry(b).or_default().push(a);
                }
            }
        }

        if adj.is_empty() {
            continue;
        }

        // BFS coloring with per-component tracking.
        // Bug fix: each connected component is independent — elimination
        // must check "sees both colors" within the SAME component only.
        let mut color: std::collections::HashMap<u8, u8> = std::collections::HashMap::new();
        let mut comp_of: std::collections::HashMap<u8, usize> = std::collections::HashMap::new();
        let mut next_comp = 0usize;
        // Track which components have an odd-cycle conflict and which color is false
        let mut comp_conflict: std::collections::HashMap<usize, u8> =
            std::collections::HashMap::new();

        for &start in &d_cells {
            if color.contains_key(&start) {
                continue;
            }
            if !adj.contains_key(&start) {
                continue;
            }
            let comp = next_comp;
            next_comp += 1;
            let mut queue = std::collections::VecDeque::new();
            color.insert(start, 1);
            comp_of.insert(start, comp);
            queue.push_back(start);
            while let Some(curr) = queue.pop_front() {
                let curr_color = color[&curr];
                let next_color = 3 - curr_color;
                if let Some(neighbors) = adj.get(&curr) {
                    for &nbr in neighbors {
                        if let Some(&nc) = color.get(&nbr) {
                            if nc == curr_color && !comp_conflict.contains_key(&comp) {
                                // Odd cycle detected in this component
                                comp_conflict.insert(comp, curr_color);
                            }
                        } else {
                            color.insert(nbr, next_color);
                            comp_of.insert(nbr, comp);
                            queue.push_back(nbr);
                        }
                    }
                }
            }
        }

        let mut eliminations: Vec<(CellIndex, Vec<u8>)> = Vec::new();

        // Process each component independently
        for comp in 0..next_comp {
            if let Some(&fc) = comp_conflict.get(&comp) {
                // Conflict: eliminate d from all false-colored cells in THIS component
                for &c in &d_cells {
                    if comp_of.get(&c) == Some(&comp) && color.get(&c) == Some(&fc) {
                        eliminations.push((CellIndex::from(c), vec![d]));
                    }
                }
            } else {
                // No conflict: collect true/false cells for THIS component only
                let comp_true: Vec<u8> = d_cells
                    .iter()
                    .copied()
                    .filter(|&c| comp_of.get(&c) == Some(&comp) && color.get(&c) == Some(&1))
                    .collect();
                let comp_false: Vec<u8> = d_cells
                    .iter()
                    .copied()
                    .filter(|&c| comp_of.get(&c) == Some(&comp) && color.get(&c) == Some(&2))
                    .collect();
                if comp_true.is_empty() || comp_false.is_empty() {
                    continue;
                }
                for &c in &d_cells {
                    if color.contains_key(&c) {
                        continue;
                    }
                    let sees_true = comp_true.iter().any(|&t| cells_see_each_other(c, t));
                    let sees_false = comp_false.iter().any(|&f| cells_see_each_other(c, f));
                    if sees_true && sees_false {
                        eliminations.push((CellIndex::from(c), vec![d]));
                    }
                }
            }
        }

        if !eliminations.is_empty() {
            eliminations.sort_by_key(|(c, _)| c.index);
            eliminations.dedup_by_key(|(c, _)| c.index);
            acc.add(Hint {
                hint_type: crate::solver::HintType::YCyclesSimple,
                difficulty: 6.5,
                technique_name: "Y-Cycles Simple".to_string(),
                description: format!("Y-Cycles Simple on digit {}", d),
                cell: CellIndex::from(0u8),
                value: 0,
                eliminations,
            });
        }
    }
}

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

pub fn nishio_forcing_chain(grid: &Grid, acc: &mut HintAccumulator) {
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let cands: Vec<u8> = grid.candidates(cell).iter().collect();
        if cands.len() < 2 {
            continue;
        }
        let mut eliminations: Vec<(CellIndex, Vec<u8>)> = Vec::new();
        for &v in &cands {
            let mut test_grid = *grid;
            test_grid.set(cell, v);
            test_grid.rebuild_candidates();
            let mut elims = Vec::new();
            if find_nested_implications(&test_grid, 3, &mut elims) {
                eliminations.push((CellIndex::from(cell), vec![v]));
            }
        }
        if !eliminations.is_empty() {
            acc.add(Hint {
                hint_type: crate::solver::HintType::NishioForcingChain,
                difficulty: 7.5,
                technique_name: "Nishio Forcing Chain".to_string(),
                description: format!(
                    "Nishio on ({},{}) eliminates {:?}",
                    cell / 9 + 1,
                    cell % 9 + 1,
                    eliminations
                        .iter()
                        .map(|(_, vs)| format!("{:?}", vs))
                        .collect::<Vec<_>>()
                ),
                cell: CellIndex::from(cell),
                value: 0,
                eliminations,
            });
        }
    }
}

pub fn multiple_forcing_chain(grid: &Grid, acc: &mut HintAccumulator) {
    // Find all bi-value cells
    let bivalue: Vec<u8> = (0..81u8)
        .filter(|&c| grid.get(c) == 0 && grid.candidates(c).cardinality() == 2)
        .collect();
    if bivalue.len() < 2 {
        return;
    }
    // Try pairs of bi-value cells (limit to avoid blowup)
    let max_pairs = 50;
    let mut pair_count = 0;
    for i in 0..bivalue.len() {
        if pair_count >= max_pairs {
            break;
        }
        for j in (i + 1)..bivalue.len() {
            if pair_count >= max_pairs {
                break;
            }
            let a = bivalue[i];
            let b = bivalue[j];
            let a_cands: Vec<u8> = grid.candidates(a).iter().collect();
            let b_cands: Vec<u8> = grid.candidates(b).iter().collect();
            // Skip if they share a candidate (not useful for multiple forcing)
            if a_cands.iter().any(|v| b_cands.contains(v)) {
                continue;
            }
            // Try all 4 combinations
            let mut branch_elims: Vec<std::collections::HashSet<(u8, u8)>> = Vec::new();
            for &av in &a_cands {
                for &bv in &b_cands {
                    let mut impl_grid = *grid;
                    impl_grid.set(a, av);
                    impl_grid.set(b, bv);
                    impl_grid.rebuild_candidates();
                    let mut elims = std::collections::HashSet::new();
                    for other in 0..81u8 {
                        if other == a || other == b || impl_grid.get(other) != 0 {
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
            }
            // Intersection of all 4
            if branch_elims.len() == 4 {
                let mut common = branch_elims[0].clone();
                for item in branch_elims.iter().skip(1) {
                    common = common.intersection(item).copied().collect();
                }
                if !common.is_empty() {
                    let mut grouped: std::collections::HashMap<u8, Vec<u8>> =
                        std::collections::HashMap::new();
                    for &(c, d) in &common {
                        grouped.entry(c).or_default().push(d);
                    }
                    let eliminations: Vec<(CellIndex, Vec<u8>)> = grouped
                        .into_iter()
                        .map(|(c, vs)| (CellIndex::from(c), vs))
                        .collect();
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::MultipleForcingChain,
                        difficulty: 8.0,
                        technique_name: "Multiple Forcing Chain".to_string(),
                        description: format!(
                            "Multiple forcing chain from ({},{}) and ({},{}) eliminates {:?}",
                            a / 9 + 1,
                            a % 9 + 1,
                            b / 9 + 1,
                            b % 9 + 1,
                            eliminations
                                .iter()
                                .map(|(c, vs)| format!("{}:rem{:?}", c.index, vs))
                                .collect::<Vec<_>>()
                        ),
                        cell: CellIndex::from(a),
                        value: 0,
                        eliminations,
                    });
                }
            }
            pair_count += 1;
        }
    }
}

pub fn dynamic_forcing_chain(grid: &Grid, acc: &mut HintAccumulator) {
    for cell in 0..81u8 {
        if grid.get(cell) != 0 {
            continue;
        }
        let cands: Vec<u8> = grid.candidates(cell).iter().collect();
        if cands.len() < 2 {
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
        if branch_elims.len() >= 2 {
            let mut common = branch_elims[0].clone();
            for item in branch_elims.iter().skip(1) {
                common = common.intersection(item).copied().collect();
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
                    hint_type: crate::solver::HintType::DynamicForcingChain,
                    difficulty: 8.5,
                    technique_name: "Dynamic Forcing Chain".to_string(),
                    description: format!(
                        "Dynamic forcing chain from ({},{}) eliminates {:?}",
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
                    let Some(forced_val) = impl_cands.iter().next() else {
                        continue;
                    };
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
            for item in branch_elims.iter().skip(1) {
                common = common.intersection(item).copied().collect();
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
        if cands.len() < 2 || cands.len() > 4 {
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
        // If exactly one branch does NOT contradict, the other N-1 all do → that one is forced
        let non_contradicting: Vec<usize> = contradiction_branches
            .iter()
            .enumerate()
            .filter(|(_, (c, _))| !*c)
            .map(|(i, _)| i)
            .collect();
        let forced_cand = if non_contradicting.len() == 1 {
            Some(cands[non_contradicting[0]])
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

    // Phase 1: propagate ALL naked singles and hidden singles iteratively until no more.
    let mut g = *grid;
    loop {
        let mut progress = false;
        // Naked singles: cell with exactly 1 candidate
        for other in 0..81u8 {
            if g.get(other) != 0 {
                continue;
            }
            let cands = g.candidates(other);
            if cands.is_empty() {
                return true;
            }
            if cands.cardinality() == 1 {
                let Some(val) = cands.iter().next() else {
                    return false;
                };
                g.set(other, val);
                g.rebuild_candidates();
                progress = true;
                break; // restart scan — candidates changed
            }
        }
        if progress {
            continue;
        }
        // Hidden singles: digit only in one cell per unit
        use crate::grid::{BLOCKS, COLS, ROWS};
        'outer: for &region in ROWS.iter().chain(COLS.iter()).chain(BLOCKS.iter()) {
            for value in 1..=9u8 {
                let positions: Vec<u8> = region
                    .cells
                    .iter()
                    .copied()
                    .filter(|&idx| g.get(idx) == 0 && g.candidates(idx).has(value))
                    .collect();
                if positions.len() == 1 {
                    g.set(positions[0], value);
                    g.rebuild_candidates();
                    progress = true;
                    break 'outer;
                }
            }
        }
        if !progress {
            break;
        }
        // Locked candidates: digit confined to one row/col within a block → eliminate from rest
        for box_region in &BLOCKS {
            for value in 1..=9u8 {
                let box_positions: Vec<u8> = box_region
                    .cells
                    .iter()
                    .copied()
                    .filter(|&idx| g.get(idx) == 0 && g.candidates(idx).has(value))
                    .collect();
                if box_positions.is_empty() || box_positions.len() == 1 {
                    continue;
                }
                // Pointing: all in same row within block
                let rows: Vec<u8> = box_positions.iter().map(|&c| c / 9).collect();
                if rows.iter().all(|&r| r == rows[0]) {
                    let target_row = rows[0];
                    for col in 0..9u8 {
                        let idx = target_row * 9 + col;
                        if g.get(idx) == 0
                            && g.candidates(idx).has(value)
                            && !box_positions.contains(&idx)
                        {
                            g.remove_candidate(idx, value);
                            progress = true;
                            break;
                        }
                    }
                }
                // Pointing: all in same col within block
                let cols: Vec<u8> = box_positions.iter().map(|&c| c % 9).collect();
                if cols.iter().all(|&c| c == cols[0]) {
                    let target_col = cols[0];
                    for row in 0..9u8 {
                        let idx = row * 9 + target_col;
                        if g.get(idx) == 0
                            && g.candidates(idx).has(value)
                            && !box_positions.contains(&idx)
                        {
                            g.remove_candidate(idx, value);
                            progress = true;
                            break;
                        }
                    }
                }
            }
        }
        if progress {
            continue;
        }
    }

    // Phase 2: no more naked singles. Pick MRV cell and branch.
    let mut best: Option<(u8, crate::grid::Candidates)> = None;
    for other in 0..81u8 {
        if g.get(other) == 0 {
            let cands = g.candidates(other);
            match &best {
                None => best = Some((other, cands)),
                Some((_, prev)) if cands.cardinality() < prev.cardinality() => {
                    best = Some((other, cands))
                }
                _ => {}
            }
        }
    }

    if let Some((idx, cands)) = best {
        // ALL candidates must lead to contradiction for the branch to be forced
        for v in cands.iter() {
            let mut next_grid = g;
            next_grid.set(idx, v);
            next_grid.rebuild_candidates();
            if !find_nested_implications(&next_grid, depth - 1, eliminations) {
                return false; // This candidate doesn't contradict → branch not forced
            }
        }
        return true; // All candidates contradicted → branch is forced
    }

    false
}

fn cells_see_each_other(a: u8, b: u8) -> bool {
    let ra = a / 9;
    let ca = a % 9;
    let rb = b / 9;
    let cb = b % 9;
    ra == rb || ca == cb || (ra / 3 == rb / 3 && ca / 3 == cb / 3)
}
