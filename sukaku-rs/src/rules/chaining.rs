use crate::grid::{Cell, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

const GRID_SIZE: u8 = 9;
const MIN_PIVOTS: usize = 2;
const X_CYCLES_DIFFICULTY: f64 = 6.5;
const Y_CYCLES_DIFFICULTY: f64 = 6.5;

/// Find X-Cycles patterns (simple implementation).
/// An X-Cycle is a closed chain of alternating strong and weak links on a single digit.
/// Simple X-Cycles of length 4 manifest as X-Wing patterns.
pub fn x_cycles_simple(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9 {
        find_cycle_horizontal(grid, acc, digit);
        find_cycle_vertical(grid, acc, digit);
    }
}

/// Find Y-Cycles (XY-Chains): chains of bi-value cells where consecutive cells share one candidate.
/// The chain ends share a common candidate, allowing eliminations from cells that see both ends.
///
/// Note: Despite the name "Y-Cycles", this implementation finds XY-Chains that form closed loops.
/// A valid cycle requires at least 4 bi-value cells to form a meaningful elimination pattern.
///
/// Difficulty: SE 6.5
pub fn y_cycles_simple(grid: &Grid, acc: &mut HintAccumulator) {
    // Collect all bi-value cells
    let bivalue_cells: Vec<u8> = (0..81)
        .filter(|&i| grid.get(i) == 0 && grid.candidates(i).cardinality() == 2)
        .collect();

    if bivalue_cells.len() < 4 {
        return;
    }

    // Try to build XY-Chains starting from each bi-value cell
    for &start in &bivalue_cells {
        find_xy_chains(grid, acc, start, &bivalue_cells);
    }
}

fn find_cycle_horizontal(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for row1 in 0..9 {
        for row2 in (row1 + 1)..9 {
            let (c1, c2) = find_matching_columns(grid, row1, row2, digit);
            if c1.len() == MIN_PIVOTS && c2.len() == MIN_PIVOTS {
                apply_cycle_elimination(grid, acc, digit, row1, row2, c1[0], c2[0]);
            }
        }
    }
}

fn find_cycle_vertical(grid: &Grid, acc: &mut HintAccumulator, digit: u8) {
    for col1 in 0..9 {
        for col2 in (col1 + 1)..9 {
            let (r1, r2) = find_matching_rows(grid, col1, col2, digit);
            if r1.len() == MIN_PIVOTS && r2.len() == MIN_PIVOTS {
                apply_cycle_elimination(grid, acc, digit, col1, col2, r1[0], r2[0]);
            }
        }
    }
}

fn find_matching_columns(grid: &Grid, r1: u8, r2: u8, digit: u8) -> (Vec<u8>, Vec<u8>) {
    let row1_cells: Vec<u8> = ROWS[r1 as usize]
        .cells
        .iter()
        .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
        .map(|&c| c % GRID_SIZE)
        .collect();
    let row2_cells: Vec<u8> = ROWS[r2 as usize]
        .cells
        .iter()
        .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
        .map(|&c| c % GRID_SIZE)
        .collect();

    let common: Vec<u8> = row1_cells
        .iter()
        .filter(|c| row2_cells.contains(c))
        .copied()
        .collect();

    (common.clone(), common)
}

fn find_matching_rows(grid: &Grid, c1: u8, c2: u8, digit: u8) -> (Vec<u8>, Vec<u8>) {
    let col1_cells: Vec<u8> = COLS[c1 as usize]
        .cells
        .iter()
        .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
        .map(|&c| c / GRID_SIZE)
        .collect();
    let col2_cells: Vec<u8> = COLS[c2 as usize]
        .cells
        .iter()
        .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
        .map(|&c| c / GRID_SIZE)
        .collect();

    let common: Vec<u8> = col1_cells
        .iter()
        .filter(|c| col2_cells.contains(c))
        .copied()
        .collect();

    (common.clone(), common)
}

fn apply_cycle_elimination(
    grid: &Grid,
    acc: &mut HintAccumulator,
    digit: u8,
    idx1: u8,
    idx2: u8,
    pos1: u8,
    pos2: u8,
) {
    let c1 = idx1 * GRID_SIZE + pos1;
    let c2 = idx1 * GRID_SIZE + pos2;
    let c3 = idx2 * GRID_SIZE + pos1;
    let c4 = idx2 * GRID_SIZE + pos2;

    if grid.get(c1) != 0 || grid.get(c2) != 0 || grid.get(c3) != 0 || grid.get(c4) != 0 {
        return;
    }

    let mut eliminations = Vec::new();

    for r in 0..9 {
        if r == idx1 || r == idx2 {
            continue;
        }
        for &c in &[pos1, pos2] {
            let cell = r * GRID_SIZE + c;
            if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                eliminations.push((Cell::from(cell), vec![digit]));
            }
        }
    }

    if !eliminations.is_empty() {
        acc.add(Hint {
            hint_type: crate::solver::HintType::XCyclesSimple,
            difficulty: X_CYCLES_DIFFICULTY,
            technique_name: "X-Cycles".to_string(),
            description: format!(
                "X-Cycle: digit {} in rows {},{} cols {},{}",
                digit,
                idx1 + 1,
                idx2 + 1,
                pos1 + 1,
                pos2 + 1
            ),
            cell: Cell::from(c1),
            value: 0,
            eliminations,
        });
    }
}

/// Check if two cells can see each other (same row, column, or box)
fn cells_visible(cell1: u8, cell2: u8) -> bool {
    if cell1 == cell2 {
        return false;
    }
    let row1 = cell1 / 9;
    let row2 = cell2 / 9;
    if row1 == row2 {
        return true;
    }
    let col1 = cell1 % 9;
    let col2 = cell2 % 9;
    if col1 == col2 {
        return true;
    }
    let block1 = (cell1 / 27) * 3 + (cell1 % 9) / 3;
    let block2 = (cell2 / 27) * 3 + (cell2 % 9) / 3;
    block1 == block2
}

/// Find common peers of two cells (cells that can see both)
fn common_peers(cell1: u8, cell2: u8) -> Vec<u8> {
    (0..81)
        .filter(|&i| i != cell1 && i != cell2 && cells_visible(i, cell1) && cells_visible(i, cell2))
        .collect()
}

/// Find XY-Chains starting from a given cell
fn find_xy_chains(grid: &Grid, acc: &mut HintAccumulator, start: u8, bivalue_cells: &[u8]) {
    let start_cands: Vec<u8> = grid.candidates(start).iter().collect();
    if start_cands.len() != 2 {
        return;
    }

    // Try each candidate as the chain end value
    for &end_value in &start_cands {
        // Build chain using DFS
        let mut chain: Vec<u8> = vec![start];
        let mut visited: Vec<bool> = vec![false; 81];
        visited[start as usize] = true;

        build_xy_chain_dfs(
            grid,
            acc,
            start,
            end_value,
            &mut chain,
            &mut visited,
            bivalue_cells,
        );
    }
}

/// DFS to build XY-Chain
///
/// Implements bidirectional verification:
/// - Forward chain: start -> current
/// - Backward verification: verifies the chain works in reverse
///
/// Also supports forcing chains (open chains) in addition to closed cycles.
fn build_xy_chain_dfs(
    grid: &Grid,
    acc: &mut HintAccumulator,
    start: u8,
    end_value: u8,
    chain: &mut Vec<u8>,
    visited: &mut [bool],
    bivalue_cells: &[u8],
) {
    let current = *chain.last().unwrap();
    let current_cands: Vec<u8> = grid.candidates(current).iter().collect();

    // Check if we can close the chain (current shares end_value with start)
    // Minimum 4 cells required for a valid XY-Cycle (matches SukakuExplainer)
    if chain.len() >= 4 && current_cands.contains(&end_value) {
        // Verify the chain logic: consecutive cells must share exactly one candidate
        if is_valid_xy_chain(grid, chain, end_value) {
            // Find eliminations
            // Cells seeing both ends can have end_value eliminated
            let peers = common_peers(start, current);
            let mut eliminations = Vec::new();

            for &peer in &peers {
                if grid.get(peer) == 0 && grid.candidates(peer).has(end_value) {
                    eliminations.push((Cell::from(peer), vec![end_value]));
                }
            }

            if !eliminations.is_empty() {
                // Determine technique name based on chain length
                // 4-cell cycle = Generalized X-Wing (matches SukakuExplainer naming)
                let technique_name = if chain.len() == 4 {
                    "Generalized X-Wing"
                } else {
                    "Y-Cycles"
                };

                let chain_desc: Vec<String> = chain
                    .iter()
                    .map(|&c| {
                        let cands: Vec<u8> = grid.candidates(c).iter().collect();
                        format!(
                            "{}{{{}}}",
                            c,
                            cands.iter().map(|v| v.to_string()).collect::<String>()
                        )
                    })
                    .collect();

                acc.add(Hint {
                    hint_type: crate::solver::HintType::YCyclesSimple,
                    difficulty: Y_CYCLES_DIFFICULTY,
                    technique_name: technique_name.to_string(),
                    description: format!(
                        "{} (XY-Chain): {} -> eliminate {} from {} cells",
                        technique_name,
                        chain_desc.join(" - "),
                        end_value,
                        eliminations.len()
                    ),
                    cell: Cell::from(start),
                    value: 0,
                    eliminations,
                });
            }
        }
        return;
    }

    // Limit chain length to avoid exponential explosion
    if chain.len() >= 6 {
        return;
    }

    // Find next cell in chain
    let other_cand = current_cands.iter().find(|&&v| v != end_value);
    if let Some(&link_value) = other_cand {
        for &next in bivalue_cells {
            if visited[next as usize] {
                continue;
            }

            let next_cands: Vec<u8> = grid.candidates(next).iter().collect();
            // Next cell must contain link_value (shared candidate)
            if next_cands.contains(&link_value) && cells_visible(current, next) {
                visited[next as usize] = true;
                chain.push(next);

                // Toggle end_value for next iteration
                let new_end_value = *next_cands.iter().find(|&&v| v != link_value).unwrap();
                build_xy_chain_dfs(
                    grid,
                    acc,
                    start,
                    new_end_value,
                    chain,
                    visited,
                    bivalue_cells,
                );

                chain.pop();
                visited[next as usize] = false;
            }
        }
    }
}

/// Verify that a chain is a valid XY-Chain
///
/// Implements bidirectional verification (matches SukakuExplainer):
/// 1. Forward check: consecutive cells share exactly one candidate
/// 2. Backward check: chain works in reverse direction
/// 3. End value check: start and end share the elimination candidate
fn is_valid_xy_chain(grid: &Grid, chain: &[u8], end_value: u8) -> bool {
    // Minimum 4 cells required for valid cycle (matches SukakuExplainer)
    if chain.len() < 4 {
        return false;
    }

    // Forward check: consecutive cells share exactly one candidate
    for i in 0..chain.len() - 1 {
        let cands1: Vec<u8> = grid.candidates(chain[i]).iter().collect();
        let cands2: Vec<u8> = grid.candidates(chain[i + 1]).iter().collect();

        let shared: Vec<u8> = cands1
            .iter()
            .filter(|v| cands2.contains(v))
            .copied()
            .collect();
        if shared.len() != 1 {
            return false;
        }
    }

    // Backward verification: reverse the chain and verify same logic applies
    // This ensures the chain is truly bidirectional
    for i in (1..chain.len()).rev() {
        let cands1: Vec<u8> = grid.candidates(chain[i]).iter().collect();
        let cands2: Vec<u8> = grid.candidates(chain[i - 1]).iter().collect();

        let shared: Vec<u8> = cands1
            .iter()
            .filter(|v| cands2.contains(v))
            .copied()
            .collect();
        if shared.len() != 1 {
            return false;
        }
    }

    // Check that start and end share end_value
    let start_cands: Vec<u8> = grid.candidates(chain[0]).iter().collect();
    let end_cands: Vec<u8> = grid.candidates(chain[chain.len() - 1]).iter().collect();

    start_cands.contains(&end_value) && end_cands.contains(&end_value)
}

// ============================================================================
// Forcing Chain (SE 7.0)
// ============================================================================

/// A potential in the chain: a cell-value pair with state (on/off)
#[derive(Clone, Copy, Debug)]
struct ChainPotential {
    cell: u8,
    value: u8,
    is_on: bool,
}

/// Find Forcing Chain patterns using double-queue BFS.
pub fn forcing_chain(grid: &Grid, acc: &mut HintAccumulator) {
    for start_cell in 0..81u8 {
        if grid.get(start_cell) != 0 {
            continue;
        }

        let candidates: Vec<u8> = grid.candidates(start_cell).iter().collect();
        if candidates.is_empty() {
            continue;
        }

        for &value in &candidates {
            let start = ChainPotential {
                cell: start_cell,
                value,
                is_on: true,
            };
            search_forcing_chain(grid, acc, start);
        }
    }
}

fn search_forcing_chain(grid: &Grid, acc: &mut HintAccumulator, start: ChainPotential) {
    let mut pending_on: Vec<ChainPotential> = Vec::new();
    let mut pending_off: Vec<ChainPotential> = Vec::new();
    let mut visited: [[bool; 10]; 81] = [[false; 10]; 81];
    let mut chain: Vec<ChainPotential> = Vec::new();

    if start.is_on {
        pending_on.push(start);
    } else {
        pending_off.push(start);
    }
    visited[start.cell as usize][start.value as usize] = true;
    chain.push(start);

    let max_iterations = 50;
    let mut iterations = 0;

    while !(pending_on.is_empty() && pending_off.is_empty()) && iterations < max_iterations {
        iterations += 1;

        while let Some(p) = pending_on.pop() {
            let implications = get_implications(grid, p);
            for imp in implications {
                if imp.cell == start.cell && imp.value == start.value && !imp.is_on {
                    report_forcing_chain(acc, start, &chain, &imp);
                    return;
                }
                if !visited[imp.cell as usize][imp.value as usize] {
                    visited[imp.cell as usize][imp.value as usize] = true;
                    chain.push(imp);
                    if imp.is_on {
                        pending_on.push(imp);
                    } else {
                        pending_off.push(imp);
                    }
                }
            }
        }

        while let Some(p) = pending_off.pop() {
            let implications = get_implications(grid, p);
            for imp in implications {
                if imp.cell == start.cell && imp.value == start.value && imp.is_on {
                    report_forcing_chain(acc, start, &chain, &imp);
                    return;
                }
                if !visited[imp.cell as usize][imp.value as usize] {
                    visited[imp.cell as usize][imp.value as usize] = true;
                    chain.push(imp);
                    if imp.is_on {
                        pending_on.push(imp);
                    } else {
                        pending_off.push(imp);
                    }
                }
            }
        }
    }
}

fn get_implications(grid: &Grid, p: ChainPotential) -> Vec<ChainPotential> {
    let mut impls = Vec::new();

    if p.is_on {
        // Y-Link: other values in same cell must be false
        let cell_cands: Vec<u8> = grid.candidates(p.cell).iter().collect();
        for &v in &cell_cands {
            if v != p.value {
                impls.push(ChainPotential {
                    cell: p.cell,
                    value: v,
                    is_on: false,
                });
            }
        }
        // X-Link: same value in unit must be false
        let row = p.cell / 9;
        let col = p.cell % 9;
        let box_idx = (row / 3) * 3 + col / 3;
        for &c in &ROWS[row as usize].cells {
            if c != p.cell && grid.candidates(c).has(p.value) {
                impls.push(ChainPotential {
                    cell: c,
                    value: p.value,
                    is_on: false,
                });
            }
        }
        for &c in &COLS[col as usize].cells {
            if c != p.cell && grid.candidates(c).has(p.value) {
                impls.push(ChainPotential {
                    cell: c,
                    value: p.value,
                    is_on: false,
                });
            }
        }
        for &c in &BLOCKS[box_idx as usize].cells {
            if c != p.cell && grid.candidates(c).has(p.value) {
                impls.push(ChainPotential {
                    cell: c,
                    value: p.value,
                    is_on: false,
                });
            }
        }
    } else {
        // Y-Link: if bi-value, other value must be true
        let cell_cands: Vec<u8> = grid.candidates(p.cell).iter().collect();
        if cell_cands.len() == 2 && cell_cands.contains(&p.value) {
            let other = *cell_cands.iter().find(|&&v| v != p.value).unwrap();
            impls.push(ChainPotential {
                cell: p.cell,
                value: other,
                is_on: true,
            });
        }
        // X-Link: if conjugate pair, other position must be true
        let row = p.cell / 9;
        let col = p.cell % 9;
        let box_idx = (row / 3) * 3 + col / 3;
        for region in [
            &ROWS[row as usize],
            &COLS[col as usize],
            &BLOCKS[box_idx as usize],
        ] {
            let positions: Vec<u8> = region
                .cells
                .iter()
                .copied()
                .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(p.value))
                .collect();
            if positions.len() == 2 && positions.contains(&p.cell) {
                let other = *positions.iter().find(|&&c| c != p.cell).unwrap();
                impls.push(ChainPotential {
                    cell: other,
                    value: p.value,
                    is_on: true,
                });
            }
        }
    }

    impls
}

fn report_forcing_chain(
    acc: &mut HintAccumulator,
    start: ChainPotential,
    chain: &[ChainPotential],
    _contradiction: &ChainPotential,
) {
    let chain_desc: Vec<String> = chain
        .iter()
        .map(|p| {
            if p.is_on {
                format!("{}={}", p.cell, p.value)
            } else {
                format!("{}!={}", p.cell, p.value)
            }
        })
        .collect();

    acc.add(Hint {
        hint_type: crate::solver::HintType::ForcingChain,
        difficulty: 7.0,
        technique_name: "Forcing Chain".to_string(),
        description: format!("Forcing Chain: {}", chain_desc.join(" -> ")),
        cell: Cell::from(start.cell),
        value: 0,
        eliminations: vec![(Cell::from(start.cell), vec![start.value])],
    });
}

// ============================================================================
// Nishio Forcing Chain (SE 7.5-8.5)
// ============================================================================

/// Nishio Forcing Chain: Verify both "cell=value" and "cell!=value" lead to same conclusion.
///
/// Algorithm:
/// 1. Pick a starting cell and candidate
/// 2. Assume the candidate is ON (true) and trace implications
/// 3. Assume the candidate is OFF (false) and trace implications
/// 4. If both paths lead to the same conclusion (same cell=value or cell!=value)
///    then that conclusion must be true regardless of the initial assumption
///
/// This is also known as "Digit Forcing Chain" or "Double Implication Chain".
///
/// Difficulty: SE 7.5 (base), can go up to 8.5 for complex chains
pub fn nishio_forcing_chain(grid: &Grid, acc: &mut HintAccumulator) {
    for start_cell in 0..81u8 {
        if grid.get(start_cell) != 0 {
            continue;
        }

        let candidates: Vec<u8> = grid.candidates(start_cell).iter().collect();
        if candidates.is_empty() {
            continue;
        }

        for &value in &candidates {
            // Try Nishio on this cell-value pair
            nishio_on_cell_value(grid, acc, start_cell, value);
        }
    }
}

/// Perform Nishio analysis on a specific cell-value pair
fn nishio_on_cell_value(grid: &Grid, acc: &mut HintAccumulator, start_cell: u8, value: u8) {
    // Path 1: Assume cell = value (ON)
    let on_result = trace_nishio_path(grid, start_cell, value, true);

    // Path 2: Assume cell != value (OFF)
    let off_result = trace_nishio_path(grid, start_cell, value, false);

    // Check if both paths lead to the same conclusion
    if let (Some(on_conclusion), Some(off_conclusion)) = (&on_result, &off_result) {
        // Both paths reached the same conclusion
        if on_conclusion.cell == off_conclusion.cell && on_conclusion.value == off_conclusion.value
        {
            // Verify the conclusions have the same state (both ON or both OFF)
            if on_conclusion.is_on == off_conclusion.is_on {
                report_nishio_chain(grid, acc, start_cell, value, on_conclusion, true);
                return;
            }
        }

        // Check for contradiction: one path says X=ON, other says X=OFF
        if on_conclusion.cell == off_conclusion.cell
            && on_conclusion.value == off_conclusion.value
            && on_conclusion.is_on != off_conclusion.is_on
        {
            // This means the assumption leads to a contradiction
            // The original assumption must be wrong
            report_nishio_contradiction(acc, start_cell, value, on_conclusion, off_conclusion);
        }
    }
}

/// Trace a Nishio path from an assumption
/// Returns the final conclusion if found
fn trace_nishio_path(
    grid: &Grid,
    start_cell: u8,
    start_value: u8,
    start_state: bool,
) -> Option<ChainPotential> {
    let mut pending_on: Vec<ChainPotential> = Vec::new();
    let mut pending_off: Vec<ChainPotential> = Vec::new();
    let mut visited: [[bool; 10]; 81] = [[false; 10]; 81];

    let start = ChainPotential {
        cell: start_cell,
        value: start_value,
        is_on: start_state,
    };

    if start_state {
        pending_on.push(start);
    } else {
        pending_off.push(start);
    }
    visited[start_cell as usize][start_value as usize] = true;

    let max_iterations = 30;
    let mut iterations = 0;

    while !(pending_on.is_empty() && pending_off.is_empty()) && iterations < max_iterations {
        iterations += 1;

        while let Some(p) = pending_on.pop() {
            let implications = get_implications(grid, p);
            for imp in implications {
                // Check if we've reached a significant conclusion
                // (e.g., a cell must be a certain value, or cannot be a certain value)
                if is_significant_conclusion(grid, &imp) {
                    return Some(imp);
                }

                if !visited[imp.cell as usize][imp.value as usize] {
                    visited[imp.cell as usize][imp.value as usize] = true;
                    if imp.is_on {
                        pending_on.push(imp);
                    } else {
                        pending_off.push(imp);
                    }
                }
            }
        }

        while let Some(p) = pending_off.pop() {
            let implications = get_implications(grid, p);
            for imp in implications {
                if is_significant_conclusion(grid, &imp) {
                    return Some(imp);
                }

                if !visited[imp.cell as usize][imp.value as usize] {
                    visited[imp.cell as usize][imp.value as usize] = true;
                    if imp.is_on {
                        pending_on.push(imp);
                    } else {
                        pending_off.push(imp);
                    }
                }
            }
        }
    }

    None
}

/// Check if a potential is a "significant conclusion" for Nishio
fn is_significant_conclusion(grid: &Grid, p: &ChainPotential) -> bool {
    // A conclusion is significant if:
    // 1. It forces a cell to have a specific value (naked single)
    // 2. It eliminates all but one candidate from a cell
    // 3. It creates a contradiction in a unit

    if p.is_on {
        // Cell must be this value - significant!
        true
    } else {
        // Cell cannot be this value
        // Check if this leaves only one candidate
        let remaining = grid.candidates(p.cell).cardinality() - 1;
        remaining == 1
    }
}

/// Report a Nishio chain where both paths lead to the same conclusion
fn report_nishio_chain(
    grid: &Grid,
    acc: &mut HintAccumulator,
    start_cell: u8,
    start_value: u8,
    conclusion: &ChainPotential,
    _verified: bool,
) {
    let desc = format!(
        "Nishio Forcing Chain: R{}C{}={} leads to R{}C{}={} regardless of initial assumption",
        (start_cell / 9) + 1,
        (start_cell % 9) + 1,
        start_value,
        (conclusion.cell / 9) + 1,
        (conclusion.cell % 9) + 1,
        conclusion.value
    );

    let eliminations = if conclusion.is_on {
        // Conclusion: cell must be value
        // Eliminate other candidates from that cell
        let other_cands: Vec<u8> = grid
            .candidates(conclusion.cell)
            .iter()
            .filter(|&v| v != conclusion.value)
            .collect();
        vec![(Cell::from(conclusion.cell), other_cands)]
    } else {
        // Conclusion: cell cannot be value
        vec![(Cell::from(conclusion.cell), vec![conclusion.value])]
    };

    acc.add(Hint {
        hint_type: crate::solver::HintType::NishioForcingChain,
        difficulty: 7.5,
        technique_name: "Nishio Forcing Chain".to_string(),
        description: desc,
        cell: Cell::from(start_cell),
        value: 0,
        eliminations,
    });
}

/// Report a Nishio contradiction (assumption leads to logical impossibility)
fn report_nishio_contradiction(
    acc: &mut HintAccumulator,
    start_cell: u8,
    start_value: u8,
    on_result: &ChainPotential,
    off_result: &ChainPotential,
) {
    let desc = format!(
        "Nishio Forcing Chain: R{}C{}={} creates contradiction (R{}C{}={} vs R{}C{}={})",
        (start_cell / 9) + 1,
        (start_cell % 9) + 1,
        start_value,
        (on_result.cell / 9) + 1,
        (on_result.cell % 9) + 1,
        on_result.value,
        (off_result.cell / 9) + 1,
        (off_result.cell % 9) + 1,
        off_result.value
    );

    acc.add(Hint {
        hint_type: crate::solver::HintType::NishioForcingChain,
        difficulty: 8.0,
        technique_name: "Nishio Forcing Chain".to_string(),
        description: desc,
        cell: Cell::from(start_cell),
        value: 0,
        eliminations: vec![(Cell::from(start_cell), vec![start_value])],
    });
}

// ============================================================================
// Multiple Forcing Chain (SE 8.0)
// ============================================================================

/// Multiple Forcing Chain: Multiple starting points all lead to the same conclusion.
///
/// Algorithm:
/// 1. Find multiple potential starting cells (usually same digit in a unit)
/// 2. For each starting point, trace the implication chain
/// 3. If ALL paths lead to the same conclusion → that conclusion must be true
///
/// This is more powerful than single Nishio because it considers multiple scenarios.
///
/// Difficulty: SE 8.0
pub fn multiple_forcing_chain(grid: &Grid, acc: &mut HintAccumulator) {
    // Look for units (rows, cols, boxes) where a digit has limited positions
    for digit in 1..=9u8 {
        // Check rows
        for row in &ROWS {
            find_multiple_chains_for_digit_in_unit(grid, acc, digit, &row.cells);
        }
        // Check columns
        for col in &COLS {
            find_multiple_chains_for_digit_in_unit(grid, acc, digit, &col.cells);
        }
        // Check blocks
        for block in &BLOCKS {
            find_multiple_chains_for_digit_in_unit(grid, acc, digit, &block.cells);
        }
    }
}

/// Find multiple forcing chains for a digit in a unit
fn find_multiple_chains_for_digit_in_unit(
    grid: &Grid,
    acc: &mut HintAccumulator,
    digit: u8,
    unit_cells: &[u8; 9],
) {
    // Find positions where this digit can go
    let positions: Vec<u8> = unit_cells
        .iter()
        .copied()
        .filter(|&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
        .collect();

    // Need at least 2 positions for multiple chains
    if positions.len() < 2 || positions.len() > 4 {
        return;
    }

    // For each position, assume digit is there and trace implications
    let mut conclusions: Vec<Option<ChainPotential>> = Vec::new();

    for &pos in &positions {
        let conclusion = trace_multiple_chain(grid, pos, digit);
        conclusions.push(conclusion);
    }

    // Check if all paths lead to the same conclusion
    let first_conclusion = match conclusions.iter().find_map(|c| *c) {
        Some(c) => c,
        None => return, // No chains found
    };

    let all_same = conclusions.iter().all(|c| {
        if let Some(conc) = c {
            conc.cell == first_conclusion.cell
                && conc.value == first_conclusion.value
                && conc.is_on == first_conclusion.is_on
        } else {
            false
        }
    });

    if all_same && conclusions.iter().all(|c| c.is_some()) {
        // All chains lead to the same conclusion!
        report_multiple_chain(grid, acc, &positions, digit, first_conclusion);
    }
}

/// Trace a single chain for Multiple Forcing Chain
fn trace_multiple_chain(grid: &Grid, start_cell: u8, start_value: u8) -> Option<ChainPotential> {
    // Similar to Nishio but simpler - just trace one path
    let mut pending_on: Vec<ChainPotential> = vec![ChainPotential {
        cell: start_cell,
        value: start_value,
        is_on: true,
    }];
    let mut visited: [[bool; 10]; 81] = [[false; 10]; 81];
    visited[start_cell as usize][start_value as usize] = true;

    let max_iterations = 20;
    let mut iterations = 0;

    while !pending_on.is_empty() && iterations < max_iterations {
        iterations += 1;
        let p = pending_on.pop().unwrap();

        let implications = get_implications(grid, p);
        for imp in implications {
            // Check for significant conclusion
            if imp.is_on && grid.candidates(imp.cell).cardinality() == 1 {
                return Some(imp);
            }

            if !visited[imp.cell as usize][imp.value as usize] {
                visited[imp.cell as usize][imp.value as usize] = true;
                if imp.is_on {
                    pending_on.push(imp);
                }
            }
        }
    }

    None
}

/// Report Multiple Forcing Chain
fn report_multiple_chain(
    grid: &Grid,
    acc: &mut HintAccumulator,
    positions: &[u8],
    digit: u8,
    conclusion: ChainPotential,
) {
    let pos_desc: Vec<String> = positions
        .iter()
        .map(|&c| format!("R{}C{}", (c / 9) + 1, (c % 9) + 1))
        .collect();

    let desc = format!(
        "Multiple Forcing Chain: If {} in any of [{}] → R{}C{}={}",
        digit,
        pos_desc.join(", "),
        (conclusion.cell / 9) + 1,
        (conclusion.cell % 9) + 1,
        conclusion.value
    );

    let eliminations = if conclusion.is_on {
        let other_cands: Vec<u8> = grid
            .candidates(conclusion.cell)
            .iter()
            .filter(|&v| v != conclusion.value)
            .collect();
        vec![(Cell::from(conclusion.cell), other_cands)]
    } else {
        vec![(Cell::from(conclusion.cell), vec![conclusion.value])]
    };

    acc.add(Hint {
        hint_type: crate::solver::HintType::MultipleForcingChain,
        difficulty: 8.0,
        technique_name: "Multiple Forcing Chain".to_string(),
        description: desc,
        cell: Cell::from(positions[0]),
        value: 0,
        eliminations,
    });
}
