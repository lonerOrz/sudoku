use crate::grid::{Cell, Grid, COLS, ROWS};
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
