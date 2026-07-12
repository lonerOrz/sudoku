//! Aligned Pair Exclusion techniques.
//!
//! APE works by enumerating all possible candidate combinations in base cells
//! and checking which combinations are "locked out" by exclusion cells.

use crate::grid::{CellIndex, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

/// Check if two cells can see each other (same row, column, or box)
fn is_visible(cell1: u8, cell2: u8) -> bool {
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

/// Get all candidates of a cell as a Vec
fn get_candidates(grid: &Grid, cell: u8) -> Vec<u8> {
    grid.candidates(cell).iter().collect()
}

/// Generate all possible pairs of candidates for two cells
fn generate_pairs(cands1: &[u8], cands2: &[u8]) -> Vec<(u8, u8)> {
    let mut pairs = Vec::new();
    for &c1 in cands1 {
        for &c2 in cands2 {
            if c1 != c2 {
                pairs.push((c1, c2));
            }
        }
    }
    pairs
}

/// Check if a pair of values is "locked out" by an exclusion cell
/// A pair (v1, v2) is locked out if:
/// - The exclusion cell can see both base cells
/// - The exclusion cell shares candidates with both base cells
/// - Using v1 and v2 in base cells would leave the exclusion cell with no candidates
fn is_pair_locked_out(
    grid: &Grid,
    cell1: u8,
    val1: u8,
    cell2: u8,
    val2: u8,
    exclusion_cell: u8,
) -> bool {
    if !is_visible(cell1, exclusion_cell) || !is_visible(cell2, exclusion_cell) {
        return false;
    }

    let excl_cands = get_candidates(grid, exclusion_cell);

    // Check if exclusion cell would have any valid candidates left
    // if cell1=val1 and cell2=val2
    for &excl_val in &excl_cands {
        // Exclusion cell value must not conflict with base cells
        if excl_val != val1 && excl_val != val2 {
            // This is a valid assignment for exclusion cell
            return false;
        }
    }

    // All exclusion cell candidates conflict with base cell values
    true
}

/// Find Aligned Pair Exclusion patterns.
///
/// Algorithm:
/// 1. Find two base cells in the same unit (row/col/box)
/// 2. Generate all possible candidate pairs for the base cells
/// 3. Find exclusion cells that can see both base cells
/// 4. Check which pairs are "locked out" by exclusion cells
/// 5. Eliminate candidates that only appear in locked-out pairs
///
/// Difficulty: SE 6.2
pub fn aligned_pair_exclusion(grid: &Grid, acc: &mut HintAccumulator) {
    // Find base cell pairs in rows
    for row in &ROWS {
        let empty_cells: Vec<u8> = row
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0)
            .collect();

        for i in 0..empty_cells.len() {
            for j in (i + 1)..empty_cells.len() {
                let cell1 = empty_cells[i];
                let cell2 = empty_cells[j];

                check_ape_for_pair(grid, acc, cell1, cell2);
            }
        }
    }

    // Find base cell pairs in columns
    for col in &COLS {
        let empty_cells: Vec<u8> = col
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0)
            .collect();

        for i in 0..empty_cells.len() {
            for j in (i + 1)..empty_cells.len() {
                let cell1 = empty_cells[i];
                let cell2 = empty_cells[j];

                check_ape_for_pair(grid, acc, cell1, cell2);
            }
        }
    }

    // Find base cell pairs in blocks
    for block in &BLOCKS {
        let empty_cells: Vec<u8> = block
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0)
            .collect();

        for i in 0..empty_cells.len() {
            for j in (i + 1)..empty_cells.len() {
                let cell1 = empty_cells[i];
                let cell2 = empty_cells[j];

                check_ape_for_pair(grid, acc, cell1, cell2);
            }
        }
    }
}

/// Check APE for a specific pair of base cells
fn check_ape_for_pair(grid: &Grid, acc: &mut HintAccumulator, cell1: u8, cell2: u8) {
    let cands1 = get_candidates(grid, cell1);
    let cands2 = get_candidates(grid, cell2);

    if cands1.is_empty() || cands2.is_empty() {
        return;
    }

    // Generate all possible pairs
    let all_pairs = generate_pairs(&cands1, &cands2);
    if all_pairs.is_empty() {
        return;
    }

    // Find exclusion cells (cells that can see both base cells)
    let exclusion_cells: Vec<u8> = (0..81)
        .filter(|&c| {
            c != cell1
                && c != cell2
                && grid.get(c) == 0
                && is_visible(c, cell1)
                && is_visible(c, cell2)
        })
        .collect();

    if exclusion_cells.is_empty() {
        return;
    }

    // Check which pairs are locked out
    let mut locked_out_pairs: Vec<(u8, u8)> = Vec::new();
    for &(v1, v2) in &all_pairs {
        let mut is_locked = false;
        for &excl in &exclusion_cells {
            if is_pair_locked_out(grid, cell1, v1, cell2, v2, excl) {
                is_locked = true;
                break;
            }
        }
        if is_locked {
            locked_out_pairs.push((v1, v2));
        }
    }

    // Find candidates that only appear in locked-out pairs
    let mut eliminations_cell1: Vec<u8> = Vec::new();
    let mut eliminations_cell2: Vec<u8> = Vec::new();

    for &cand in &cands1 {
        // Check if this candidate only appears in locked-out pairs for cell1
        let appears_in_valid = all_pairs
            .iter()
            .filter(|&&(v1, _v2)| v1 == cand)
            .any(|&(v1, v2)| !locked_out_pairs.contains(&(v1, v2)));

        if !appears_in_valid && !locked_out_pairs.is_empty() {
            eliminations_cell1.push(cand);
        }
    }

    for &cand in &cands2 {
        let appears_in_valid = all_pairs
            .iter()
            .filter(|&&(_v1, v2)| v2 == cand)
            .any(|&(v1, v2)| !locked_out_pairs.contains(&(v1, v2)));

        if !appears_in_valid && !locked_out_pairs.is_empty() {
            eliminations_cell2.push(cand);
        }
    }

    // Report eliminations
    let mut all_eliminations: Vec<(CellIndex, Vec<u8>)> = Vec::new();

    if !eliminations_cell1.is_empty() {
        all_eliminations.push((CellIndex::from(cell1), eliminations_cell1.clone()));
    }
    if !eliminations_cell2.is_empty() {
        all_eliminations.push((CellIndex::from(cell2), eliminations_cell2.clone()));
    }

    if !all_eliminations.is_empty() {
        let desc = format!(
            "Aligned Pair Exclusion: cells R{}C{} and R{}C{} -> eliminate {:?} and {:?}",
            (cell1 / 9) + 1,
            (cell1 % 9) + 1,
            (cell2 / 9) + 1,
            (cell2 % 9) + 1,
            eliminations_cell1,
            eliminations_cell2
        );

        acc.add(Hint {
            hint_type: crate::solver::HintType::AlignedPairExclusion,
            difficulty: 6.2,
            technique_name: "Aligned Pair Exclusion".to_string(),
            description: desc,
            cell: CellIndex::from(cell1),
            value: 0,
            eliminations: all_eliminations,
        });
    }
}

/// Generate all possible triplets of candidates for three cells
fn generate_triplets(cands1: &[u8], cands2: &[u8], cands3: &[u8]) -> Vec<(u8, u8, u8)> {
    let mut triplets = Vec::new();
    for &c1 in cands1 {
        for &c2 in cands2 {
            for &c3 in cands3 {
                // All three values must be different
                if c1 != c2 && c1 != c3 && c2 != c3 {
                    triplets.push((c1, c2, c3));
                }
            }
        }
    }
    triplets
}

/// A triplet of cell-value pairs for locked out checking
struct TripletCells {
    cell1: u8,
    val1: u8,
    cell2: u8,
    val2: u8,
    cell3: u8,
    val3: u8,
}

/// Check if a triplet of values is "locked out" by an exclusion cell
fn is_triplet_locked_out(grid: &Grid, triplet: TripletCells, exclusion_cell: u8) -> bool {
    let TripletCells {
        cell1,
        val1,
        cell2,
        val2,
        cell3,
        val3,
    } = triplet;

    if !is_visible(cell1, exclusion_cell)
        || !is_visible(cell2, exclusion_cell)
        || !is_visible(cell3, exclusion_cell)
    {
        return false;
    }

    let excl_cands = get_candidates(grid, exclusion_cell);

    for &excl_val in &excl_cands {
        if excl_val != val1 && excl_val != val2 && excl_val != val3 {
            return false;
        }
    }

    true
}

/// Find Aligned Triplet Exclusion patterns.
///
/// Similar to APE but with three base cells instead of two.
///
/// Difficulty: SE 7.5
pub fn aligned_triplet_exclusion(grid: &Grid, acc: &mut HintAccumulator) {
    // Find base cell triplets in rows
    for row in &ROWS {
        let empty_cells: Vec<u8> = row
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0)
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

                    check_ate_for_triplet(grid, acc, cell1, cell2, cell3);
                }
            }
        }
    }

    // Find base cell triplets in columns
    for col in &COLS {
        let empty_cells: Vec<u8> = col
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0)
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

                    check_ate_for_triplet(grid, acc, cell1, cell2, cell3);
                }
            }
        }
    }

    // Find base cell triplets in blocks
    for block in &BLOCKS {
        let empty_cells: Vec<u8> = block
            .cells
            .iter()
            .copied()
            .filter(|&c| grid.get(c) == 0)
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

                    check_ate_for_triplet(grid, acc, cell1, cell2, cell3);
                }
            }
        }
    }
}

/// Check ATE for a specific triplet of base cells
fn check_ate_for_triplet(grid: &Grid, acc: &mut HintAccumulator, cell1: u8, cell2: u8, cell3: u8) {
    let cands1 = get_candidates(grid, cell1);
    let cands2 = get_candidates(grid, cell2);
    let cands3 = get_candidates(grid, cell3);

    if cands1.is_empty() || cands2.is_empty() || cands3.is_empty() {
        return;
    }

    // Generate all possible triplets
    let all_triplets = generate_triplets(&cands1, &cands2, &cands3);
    if all_triplets.is_empty() {
        return;
    }

    // Find exclusion cells
    let exclusion_cells: Vec<u8> = (0..81)
        .filter(|&c| {
            c != cell1
                && c != cell2
                && c != cell3
                && grid.get(c) == 0
                && is_visible(c, cell1)
                && is_visible(c, cell2)
                && is_visible(c, cell3)
        })
        .collect();

    if exclusion_cells.is_empty() {
        return;
    }

    // Check which triplets are locked out
    let mut locked_out_triplets: Vec<(u8, u8, u8)> = Vec::new();
    for &(v1, v2, v3) in &all_triplets {
        let mut is_locked = false;
        for &excl in &exclusion_cells {
            let triplet = TripletCells {
                cell1,
                val1: v1,
                cell2,
                val2: v2,
                cell3,
                val3: v3,
            };
            if is_triplet_locked_out(grid, triplet, excl) {
                is_locked = true;
                break;
            }
        }
        if is_locked {
            locked_out_triplets.push((v1, v2, v3));
        }
    }

    // Find candidates that only appear in locked-out triplets
    let mut elim1: Vec<u8> = Vec::new();
    let mut elim2: Vec<u8> = Vec::new();
    let mut elim3: Vec<u8> = Vec::new();

    for &cand in &cands1 {
        let appears_in_valid = all_triplets
            .iter()
            .filter(|&&(v1, _, _)| v1 == cand)
            .any(|&(v1, v2, v3)| !locked_out_triplets.contains(&(v1, v2, v3)));

        if !appears_in_valid && !locked_out_triplets.is_empty() {
            elim1.push(cand);
        }
    }

    for &cand in &cands2 {
        let appears_in_valid = all_triplets
            .iter()
            .filter(|&&(_, v2, _)| v2 == cand)
            .any(|&(v1, v2, v3)| !locked_out_triplets.contains(&(v1, v2, v3)));

        if !appears_in_valid && !locked_out_triplets.is_empty() {
            elim2.push(cand);
        }
    }

    for &cand in &cands3 {
        let appears_in_valid = all_triplets
            .iter()
            .filter(|&&(_, _, v3)| v3 == cand)
            .any(|&(v1, v2, v3)| !locked_out_triplets.contains(&(v1, v2, v3)));

        if !appears_in_valid && !locked_out_triplets.is_empty() {
            elim3.push(cand);
        }
    }

    // Report eliminations
    let mut all_eliminations: Vec<(CellIndex, Vec<u8>)> = Vec::new();

    if !elim1.is_empty() {
        all_eliminations.push((CellIndex::from(cell1), elim1.clone()));
    }
    if !elim2.is_empty() {
        all_eliminations.push((CellIndex::from(cell2), elim2.clone()));
    }
    if !elim3.is_empty() {
        all_eliminations.push((CellIndex::from(cell3), elim3.clone()));
    }

    if !all_eliminations.is_empty() {
        let desc =
            format!(
            "Aligned Triplet Exclusion: cells R{}C{}, R{}C{}, R{}C{} -> eliminate {:?}, {:?}, {:?}",
            (cell1 / 9) + 1, (cell1 % 9) + 1,
            (cell2 / 9) + 1, (cell2 % 9) + 1,
            (cell3 / 9) + 1, (cell3 % 9) + 1,
            elim1, elim2, elim3
        );

        acc.add(Hint {
            hint_type: crate::solver::HintType::AlignedTripletExclusion,
            difficulty: 7.5,
            technique_name: "Aligned Triplet Exclusion".to_string(),
            description: desc,
            cell: CellIndex::from(cell1),
            value: 0,
            eliminations: all_eliminations,
        });
    }
}
