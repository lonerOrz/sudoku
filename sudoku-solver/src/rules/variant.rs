use crate::grid::{CellIndex, Grid};
use crate::solver::{Hint, HintAccumulator};

/// X-Diagonal: cells on the same diagonal can't share a value.
/// When a cell is filled, eliminate that value from diagonal peers.
pub fn x_diagonal_var(grid: &Grid, acc: &mut HintAccumulator) {
    for cell in 0..81u8 {
        let val = grid.get(cell);
        if val == 0 {
            continue;
        }
        let row = cell / 9;
        let col = cell % 9;

        // Main diagonal (row == col)
        if row == col {
            for r in 0..9u8 {
                if r != row {
                    let peer = r * 9 + r;
                    if grid.get(peer) == 0 && grid.candidates(peer).has(val) {
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::XDiagonal,
                            difficulty: 5.5,
                            technique_name: "X-Diagonal".to_string(),
                            description: format!(
                                "X-Diagonal: eliminate {} from main diagonal peer",
                                val
                            ),
                            cell: CellIndex::from(cell),
                            value: 0,
                            eliminations: vec![(CellIndex::from(peer), vec![val])],
                        });
                    }
                }
            }
        }

        // Anti-diagonal (row + col == 8)
        if row + col == 8 {
            for r in 0..9u8 {
                let c = 8 - r;
                if r != row {
                    let peer = r * 9 + c;
                    if grid.get(peer) == 0 && grid.candidates(peer).has(val) {
                        acc.add(Hint {
                            hint_type: crate::solver::HintType::XDiagonal,
                            difficulty: 5.5,
                            technique_name: "X-Diagonal".to_string(),
                            description: format!(
                                "X-Diagonal: eliminate {} from anti-diagonal peer",
                                val
                            ),
                            cell: CellIndex::from(cell),
                            value: 0,
                            eliminations: vec![(CellIndex::from(peer), vec![val])],
                        });
                    }
                }
            }
        }
    }
}

/// Disjoint Groups: cells at the same position in each 3x3 box form a group.
/// If a digit can only appear in one cell in a group, eliminate other candidates from that cell.
pub fn disjoint_groups_var(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        for pos_in_box in 0..9u8 {
            let box_row = pos_in_box / 3;
            let box_col = pos_in_box % 3;
            let mut cells_at_position = Vec::new();
            for box_idx in 0..9u8 {
                let box_r = box_idx / 3;
                let box_c = box_idx % 3;
                let cell_row = box_r * 3 + box_row;
                let cell_col = box_c * 3 + box_col;
                let cell = cell_row * 9 + cell_col;
                cells_at_position.push(cell);
            }
            let cells_with_digit: Vec<u8> = cells_at_position
                .iter()
                .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
                .copied()
                .collect();
            if cells_with_digit.len() == 1 {
                let cell = cells_with_digit[0];
                let mut elim = Vec::new();
                for d in 1..=9u8 {
                    if d != digit && grid.candidates(cell).has(d) {
                        elim.push((CellIndex::from(cell), vec![d]));
                    }
                }
                if !elim.is_empty() {
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::DisjointGroups,
                        difficulty: 5.5,
                        technique_name: "Disjoint Groups".to_string(),
                        description: format!(
                            "Disjoint Groups: digit {} restricted to one cell in group",
                            digit
                        ),
                        cell: CellIndex::from(cell),
                        value: digit,
                        eliminations: elim,
                    });
                }
            }
        }
    }
}

/// Windows: 3x3 windows starting at (1,1), (1,4), (4,1), (4,4).
/// If a digit is restricted to one cell in a window, place it there.
pub fn windows_var(grid: &Grid, acc: &mut HintAccumulator) {
    let windows = [(1, 1), (1, 4), (4, 1), (4, 4)];
    for digit in 1..=9u8 {
        for &(wr, wc) in &windows {
            let mut cells_in_window = Vec::new();
            for dr in 0..3u8 {
                for dc in 0..3u8 {
                    let r = wr + dr;
                    let c = wc + dc;
                    let cell = r * 9 + c;
                    cells_in_window.push(cell);
                }
            }
            let cells_with_digit: Vec<u8> = cells_in_window
                .iter()
                .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
                .copied()
                .collect();
            if cells_with_digit.len() == 1 {
                let cell = cells_with_digit[0];
                let mut elim = Vec::new();
                for d in 1..=9u8 {
                    if d != digit && grid.candidates(cell).has(d) {
                        elim.push((CellIndex::from(cell), vec![d]));
                    }
                }
                if !elim.is_empty() {
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::Windows,
                        difficulty: 5.5,
                        technique_name: "Windows".to_string(),
                        description: format!(
                            "Windows: digit {} restricted to one cell in window",
                            digit
                        ),
                        cell: CellIndex::from(cell),
                        value: digit,
                        eliminations: elim,
                    });
                }
            }
        }
    }
}

/// Center Dot: the 9 center cells of each 3x3 box form a group.
/// If a digit is restricted to one center cell, place it there.
pub fn center_dot_var(grid: &Grid, acc: &mut HintAccumulator) {
    let center_dots = [10, 13, 16, 37, 40, 43, 64, 67, 70];
    for digit in 1..=9u8 {
        let cells_with_digit: Vec<u8> = center_dots
            .iter()
            .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
            .copied()
            .collect();
        if cells_with_digit.len() == 1 {
            let cell = cells_with_digit[0];
            let mut elim = Vec::new();
            for d in 1..=9u8 {
                if d != digit && grid.candidates(cell).has(d) {
                    elim.push((CellIndex::from(cell), vec![d]));
                }
            }
            if !elim.is_empty() {
                acc.add(Hint {
                    hint_type: crate::solver::HintType::CenterDot,
                    difficulty: 5.5,
                    technique_name: "Center Dot".to_string(),
                    description: format!(
                        "Center Dot: digit {} restricted to one center cell",
                        digit
                    ),
                    cell: CellIndex::from(cell),
                    value: digit,
                    eliminations: elim,
                });
            }
        }
    }
}

/// Asterisk: 8 specific cells form a group.
/// If a digit is restricted to one asterisk cell, place it there.
pub fn asterisk_var(grid: &Grid, acc: &mut HintAccumulator) {
    let asterisk = [4, 10, 16, 36, 40, 44, 64, 70, 76];
    for digit in 1..=9u8 {
        let cells_with_digit: Vec<u8> = asterisk
            .iter()
            .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
            .copied()
            .collect();
        if cells_with_digit.len() == 1 {
            let cell = cells_with_digit[0];
            let mut elim = Vec::new();
            for d in 1..=9u8 {
                if d != digit && grid.candidates(cell).has(d) {
                    elim.push((CellIndex::from(cell), vec![d]));
                }
            }
            if !elim.is_empty() {
                acc.add(Hint {
                    hint_type: crate::solver::HintType::Asterisk,
                    difficulty: 5.5,
                    technique_name: "Asterisk".to_string(),
                    description: format!(
                        "Asterisk: digit {} restricted to one asterisk cell",
                        digit
                    ),
                    cell: CellIndex::from(cell),
                    value: digit,
                    eliminations: elim,
                });
            }
        }
    }
}

/// Girandola: 4 corner cells of each box form a group.
/// If a digit is restricted to one girandola cell, place it there.
pub fn girandola_var(grid: &Grid, acc: &mut HintAccumulator) {
    let girandola = [0, 8, 20, 40, 60, 72, 80, 24, 56];
    for digit in 1..=9u8 {
        let cells_with_digit: Vec<u8> = girandola
            .iter()
            .filter(|&&c| grid.get(c) == 0 && grid.candidates(c).has(digit))
            .copied()
            .collect();
        if cells_with_digit.len() == 1 {
            let cell = cells_with_digit[0];
            let mut elim = Vec::new();
            for d in 1..=9u8 {
                if d != digit && grid.candidates(cell).has(d) {
                    elim.push((CellIndex::from(cell), vec![d]));
                }
            }
            if !elim.is_empty() {
                acc.add(Hint {
                    hint_type: crate::solver::HintType::Girandola,
                    difficulty: 5.5,
                    technique_name: "Girandola".to_string(),
                    description: format!(
                        "Girandola: digit {} restricted to one girandola cell",
                        digit
                    ),
                    cell: CellIndex::from(cell),
                    value: digit,
                    eliminations: elim,
                });
            }
        }
    }
}

/// Non-Consecutive: orthogonally adjacent cells can't have consecutive values.
/// When a cell is filled with V, eliminate V-1 and V+1 from neighbors.
pub fn non_consecutive_var(grid: &Grid, acc: &mut HintAccumulator) {
    for cell in 0..81u8 {
        let val = grid.get(cell);
        if val == 0 {
            continue;
        }
        let row = cell / 9;
        let col = cell % 9;
        let neighbors = [
            (row.wrapping_sub(1), col),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
        ];
        for &(nr, nc) in &neighbors {
            if nr >= 9 || nc >= 9 {
                continue;
            }
            let neighbor = nr * 9 + nc;
            if grid.get(neighbor) != 0 {
                continue;
            }
            let mut elim = Vec::new();
            if val > 1 && grid.candidates(neighbor).has(val - 1) {
                elim.push((CellIndex::from(neighbor), vec![val - 1]));
            }
            if val < 9 && grid.candidates(neighbor).has(val + 1) {
                elim.push((CellIndex::from(neighbor), vec![val + 1]));
            }
            if !elim.is_empty() {
                acc.add(Hint {
                    hint_type: crate::solver::HintType::NonConsecutive,
                    difficulty: 5.5,
                    technique_name: "Non-Consecutive".to_string(),
                    description: format!(
                        "Non-Consecutive: eliminate consecutive values from neighbor of {}",
                        val
                    ),
                    cell: CellIndex::from(cell),
                    value: 0,
                    eliminations: elim,
                });
            }
        }
    }
}

/// Anti-Knight: cells a knight's move apart can't share a value.
/// When a cell is filled with V, eliminate V from all knight-mate cells.
pub fn anti_knight_var(grid: &Grid, acc: &mut HintAccumulator) {
    let knight_moves = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];
    for cell in 0..81u8 {
        let val = grid.get(cell);
        if val == 0 {
            continue;
        }
        let row = (cell / 9) as i32;
        let col = (cell % 9) as i32;
        for &(dr, dc) in &knight_moves {
            let nr = row + dr;
            let nc = col + dc;
            if (0..9).contains(&nr) && (0..9).contains(&nc) {
                let neighbor = (nr * 9 + nc) as u8;
                if grid.get(neighbor) == 0 && grid.candidates(neighbor).has(val) {
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::AntiKnight,
                        difficulty: 5.5,
                        technique_name: "Anti-Knight".to_string(),
                        description: format!(
                            "Anti-Knight: eliminate {} from knight-mate of filled cell",
                            val
                        ),
                        cell: CellIndex::from(cell),
                        value: 0,
                        eliminations: vec![(CellIndex::from(neighbor), vec![val])],
                    });
                }
            }
        }
    }
}

/// Anti-King: cells adjacent (including diagonals) can't share a value.
/// When a cell is filled with V, eliminate V from all king-adjacent cells.
pub fn anti_king_var(grid: &Grid, acc: &mut HintAccumulator) {
    let king_moves = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for cell in 0..81u8 {
        let val = grid.get(cell);
        if val == 0 {
            continue;
        }
        let row = (cell / 9) as i32;
        let col = (cell % 9) as i32;
        for &(dr, dc) in &king_moves {
            let nr = row + dr;
            let nc = col + dc;
            if (0..9).contains(&nr) && (0..9).contains(&nc) {
                let neighbor = (nr * 9 + nc) as u8;
                if grid.get(neighbor) == 0 && grid.candidates(neighbor).has(val) {
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::AntiKing,
                        difficulty: 5.5,
                        technique_name: "Anti-King".to_string(),
                        description: format!(
                            "Anti-King: eliminate {} from king-adjacent of filled cell",
                            val
                        ),
                        cell: CellIndex::from(cell),
                        value: 0,
                        eliminations: vec![(CellIndex::from(neighbor), vec![val])],
                    });
                }
            }
        }
    }
}

/// Toroidal: cells at adjacent positions (wrapping around edges) can't share a value.
/// When a cell is filled with V, eliminate V from all toroidal-adjacent cells.
pub fn toroidal_var(grid: &Grid, acc: &mut HintAccumulator) {
    let adjacencies = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for cell in 0..81u8 {
        let val = grid.get(cell);
        if val == 0 {
            continue;
        }
        let row = (cell / 9) as i32;
        let col = (cell % 9) as i32;
        for &(dr, dc) in &adjacencies {
            let nr = ((row + dr + 9) % 9) as u8;
            let nc = ((col + dc + 9) % 9) as u8;
            let neighbor = nr * 9 + nc;
            if neighbor == cell {
                continue;
            }
            if grid.get(neighbor) == 0 && grid.candidates(neighbor).has(val) {
                acc.add(Hint {
                    hint_type: crate::solver::HintType::Toroidal,
                    difficulty: 6.0,
                    technique_name: "Toroidal".to_string(),
                    description: format!(
                        "Toroidal: eliminate {} from toroidal-adjacent of filled cell",
                        val
                    ),
                    cell: CellIndex::from(cell),
                    value: 0,
                    eliminations: vec![(CellIndex::from(neighbor), vec![val])],
                });
            }
        }
    }
}

/// Ferz NC: diagonally adjacent cells can't have consecutive values.
/// When a cell is filled with V, eliminate V-1 and V+1 from diagonal neighbors.
pub fn ferz_nc_var(grid: &Grid, acc: &mut HintAccumulator) {
    let ferz_moves = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for cell in 0..81u8 {
        let val = grid.get(cell);
        if val == 0 {
            continue;
        }
        let row = (cell / 9) as i32;
        let col = (cell % 9) as i32;
        for &(dr, dc) in &ferz_moves {
            let nr = row + dr;
            let nc = col + dc;
            if (0..9).contains(&nr) && (0..9).contains(&nc) {
                let neighbor = (nr * 9 + nc) as u8;
                if grid.get(neighbor) != 0 {
                    continue;
                }
                let mut elim = Vec::new();
                if val > 1 && grid.candidates(neighbor).has(val - 1) {
                    elim.push((CellIndex::from(neighbor), vec![val - 1]));
                }
                if val < 9 && grid.candidates(neighbor).has(val + 1) {
                    elim.push((CellIndex::from(neighbor), vec![val + 1]));
                }
                if !elim.is_empty() {
                    acc.add(Hint {
                        hint_type: crate::solver::HintType::FerzNC,
                        difficulty: 6.0,
                        technique_name: "Ferz NC".to_string(),
                        description: format!(
                            "Ferz NC: eliminate consecutive values from diagonal neighbor of {}",
                            val
                        ),
                        cell: CellIndex::from(cell),
                        value: 0,
                        eliminations: elim,
                    });
                }
            }
        }
    }
}
