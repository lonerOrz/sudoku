use crate::grid::{Cell, Grid};
use crate::solver::{Hint, HintAccumulator};

pub fn x_diagonal_var(grid: &Grid, acc: &mut HintAccumulator) {
    for digit in 1..=9u8 {
        let mut cells_with_digit = Vec::new();
        for cell in 0..81u8 {
            if grid.get(cell) == 0 && grid.candidates(cell).has(digit) {
                cells_with_digit.push(cell);
            }
        }

        for i in 0..cells_with_digit.len() {
            for j in (i + 1)..cells_with_digit.len() {
                let cell1 = cells_with_digit[i];
                let cell2 = cells_with_digit[j];
                let row1 = cell1 / 9;
                let col1 = cell1 % 9;
                let row2 = cell2 / 9;
                let col2 = cell2 % 9;

                if (row1 == col1 && row2 == col2) || (row1 + col1 == 8 && row2 + col2 == 8) {
                    for &target in &[cell1, cell2] {
                        if target != cell1 && target != cell2 {
                            continue;
                        }
                        let mut elim = Vec::new();
                        for d in 1..=9u8 {
                            if d != digit && grid.candidates(target).has(d) {
                                elim.push((Cell::from(target), vec![d]));
                            }
                        }
                        if !elim.is_empty() {
                            acc.add(Hint {
                                hint_type: crate::solver::HintType::XDiagonal,
                                difficulty: 5.5,
                                technique_name: "X-Diagonal".to_string(),
                                description: format!("X-Diagonal: digit {} in diagonal", digit),
                                cell: Cell::from(cell1),
                                value: 0,
                                eliminations: elim,
                            });
                        }
                    }
                }
            }
        }
    }
}
