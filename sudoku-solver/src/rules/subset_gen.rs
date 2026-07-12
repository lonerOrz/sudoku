use crate::grid::{CellIndex, Grid, BLOCKS, COLS, ROWS};
use crate::solver::{Hint, HintAccumulator};

/// Find Generalized Naked Sets: N cells in a region containing exactly N candidates.
/// This generalizes Naked Pair (N=2), Naked Triple (N=3), etc. to N=2-6.
/// Difficulty: varies 3.0-5.8
pub fn generalized_naked_set(grid: &Grid, acc: &mut HintAccumulator, n: usize) {
    if !(2..=6).contains(&n) {
        return;
    }

    let regions: Vec<_> = ROWS
        .iter()
        .chain(COLS.iter())
        .chain(BLOCKS.iter())
        .collect();

    for region in regions {
        let empty_cells: Vec<u8> = region
            .cells
            .iter()
            .copied()
            .filter(|&idx| {
                grid.get(idx) == 0 && (grid.candidates(idx).cardinality() as usize) <= n + 1
            })
            .collect();

        if empty_cells.len() < n {
            continue;
        }

        find_gns_in_cells(grid, acc, n, &empty_cells, region);
    }
}

fn find_gns_in_cells(
    grid: &Grid,
    acc: &mut HintAccumulator,
    n: usize,
    empty_cells: &[u8],
    region: &crate::grid::Region,
) {
    let combinations = generate_combinations(empty_cells.len(), n);
    for combo_indices in combinations {
        let cells: Vec<u8> = combo_indices.iter().map(|&i| empty_cells[i]).collect();

        let mut union_cands = crate::grid::Candidates::empty();
        for &cell in &cells {
            union_cands = union_cands.union(grid.candidates(cell));
        }

        if union_cands.cardinality() as usize == n {
            let candidate_values: Vec<u8> = union_cands.iter().collect();

            let mut eliminations = Vec::new();
            for &cell in &region.cells {
                if cells.contains(&cell) {
                    continue;
                }
                if grid.get(cell) != 0 {
                    continue;
                }

                let cell_cands = grid.candidates(cell);
                let to_remove: Vec<u8> = candidate_values
                    .iter()
                    .filter(|&&v| cell_cands.has(v))
                    .copied()
                    .collect();

                if !to_remove.is_empty() {
                    eliminations.push((CellIndex::from(cell), to_remove));
                }
            }

            if !eliminations.is_empty() {
                let difficulty = match n {
                    2 => 3.0,
                    3 => 3.6,
                    4 => 5.0,
                    5 => 5.4,
                    6 => 5.8,
                    _ => 5.0,
                };
                let name = format!("Generalized Naked Set ({})", n);

                acc.add(Hint {
                    hint_type: crate::solver::HintType::GeneralizedNakedSet,
                    difficulty,
                    technique_name: name,
                    description: format!(
                        "Generalized Naked Set: {} cells with {} candidates in {:?}",
                        n, n, region.region_type
                    ),
                    cell: CellIndex::from(cells[0]),
                    value: 0,
                    eliminations,
                });
            }
        }
    }
}

fn generate_combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    fn gen(
        n: usize,
        k: usize,
        start: usize,
        depth: usize,
        current: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if depth == k {
            result.push(current.clone());
            return;
        }
        for i in start..n {
            current.push(i);
            gen(n, k, i + 1, depth + 1, current, result);
            current.pop();
        }
    }

    gen(n, k, 0, 0, &mut Vec::with_capacity(k), &mut result);
    result
}

pub fn generalized_naked_pair(grid: &Grid, acc: &mut HintAccumulator) {
    generalized_naked_set(grid, acc, 2);
}

pub fn generalized_naked_triple(grid: &Grid, acc: &mut HintAccumulator) {
    generalized_naked_set(grid, acc, 3);
}

pub fn generalized_naked_quad(grid: &Grid, acc: &mut HintAccumulator) {
    generalized_naked_set(grid, acc, 4);
}

pub fn generalized_naked_quint(grid: &Grid, acc: &mut HintAccumulator) {
    generalized_naked_set(grid, acc, 5);
}

pub fn generalized_naked_sext(grid: &Grid, acc: &mut HintAccumulator) {
    generalized_naked_set(grid, acc, 6);
}
