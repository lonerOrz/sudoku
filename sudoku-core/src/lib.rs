// sudoku-core: 数独核心算法库

pub mod board;
pub mod checker;
pub mod difficulty;
pub mod generator;
pub mod hints;
pub mod solver;

pub use board::{BitmaskGrid, Cell, Grid, PEERS, Solution, is_valid};

pub fn clear_peers(pencil_marks: &mut [[Vec<u8>; 9]; 9], row: usize, col: usize, val: u8) {
    for &peer_idx in &PEERS[row * 9 + col] {
        if peer_idx == u8::MAX {
            break;
        }
        let r = peer_idx as usize / 9;
        let c = peer_idx as usize % 9;
        pencil_marks[r][c].retain(|&v| v != val);
    }
}
pub use checker::{find_conflicts_at, find_errors, has_empty, is_solved, possible_values};
pub use difficulty::Difficulty;
pub use generator::generate;
pub use hints::{find_hidden_single, find_naked_single};
pub use solver::{count_solutions, solve};

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_range_loop)]
    use super::*;

    #[test]
    fn test_generate() {
        let (puzzle, solution) = generate(Difficulty::Easy);

        let mut has_empty = false;
        for r in 0..9 {
            for c in 0..9 {
                if puzzle[r][c].value().is_none() {
                    has_empty = true;
                    break;
                }
            }
        }
        assert!(has_empty);

        let solution_grid: Grid =
            core::array::from_fn(|r| core::array::from_fn(|c| Cell::Given(solution[r][c])));
        assert!(is_solved(&solution_grid));
    }

    #[test]
    fn test_count_solutions() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        assert!(solve(&mut grid));
        assert_eq!(count_solutions(&mut grid), 1);
    }

    #[test]
    fn test_solve() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        assert!(solve(&mut grid));
        assert!(is_solved(&grid));
    }

    #[test]
    fn test_is_valid_same_row() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);

        assert!(!is_valid(&grid, 3, 5));
        assert!(is_valid(&grid, 3, 3));
    }

    #[test]
    fn test_is_valid_same_col() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);

        assert!(!is_valid(&grid, 45, 5));
        assert!(is_valid(&grid, 45, 3));
    }

    #[test]
    fn test_is_valid_same_box() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);

        assert!(!is_valid(&grid, 20, 5));
        assert!(is_valid(&grid, 40, 5));
    }

    #[test]
    fn test_is_solved_empty() {
        let grid: Grid = [[Cell::Empty; 9]; 9];
        assert!(!is_solved(&grid));
    }

    #[test]
    fn test_cell_value() {
        assert_eq!(Cell::Given(5).value(), Some(5));
        assert_eq!(Cell::UserInput(3).value(), Some(3));
        assert_eq!(Cell::Empty.value(), None);
    }

    #[test]
    fn test_possible_values() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);
        grid[1][1] = Cell::Given(3);

        let candidates = possible_values(&grid, 0, 1);
        assert!(candidates.contains(&1));
        assert!(!candidates.contains(&5));
        assert!(!candidates.contains(&3));
    }

    #[test]
    fn test_find_errors() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(5);
        grid[0][1] = Cell::Given(5);

        let errors = find_errors(&grid);
        assert!(errors.contains(&(0, 0)), "First 5 in row should be error");
        assert!(errors.contains(&(0, 1)), "Second 5 in row should be error");
    }

    #[test]
    fn test_find_errors_column() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(3);
        grid[4][0] = Cell::Given(3);

        let errors = find_errors(&grid);
        assert!(
            errors.contains(&(0, 0)),
            "First 3 in column should be error"
        );
        assert!(
            errors.contains(&(4, 0)),
            "Second 3 in column should be error"
        );
    }

    #[test]
    fn test_find_errors_box() {
        let mut grid: Grid = [[Cell::Empty; 9]; 9];
        grid[0][0] = Cell::Given(7);
        grid[1][2] = Cell::Given(7);

        let errors = find_errors(&grid);
        assert!(errors.contains(&(0, 0)), "First 7 in box should be error");
        assert!(errors.contains(&(1, 2)), "Second 7 in box should be error");
    }

    #[test]
    fn test_find_errors_empty() {
        let grid: Grid = [[Cell::Empty; 9]; 9];
        let errors = find_errors(&grid);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_shuffle_quality() {
        let mut counts = [0u32; 81];
        let samples = 1000;

        for _ in 0..samples {
            let (puzzle, _) = generate(Difficulty::Medium);
            for idx in 0..81 {
                if puzzle[idx / 9][idx % 9].value().is_some() {
                    counts[idx] += 1;
                }
            }
        }

        let total: f64 = counts.iter().map(|&c| c as f64).sum();
        let expected = total / 81.0;

        let chi_square: f64 = counts
            .iter()
            .map(|&c| {
                let observed = c as f64;
                (observed - expected).powi(2) / expected
            })
            .sum();

        let entropy: f64 = counts
            .iter()
            .map(|&c| {
                let p = c as f64 / total;
                if p > 0.0 { -p * p.log2() } else { 0.0 }
            })
            .sum::<f64>()
            / 81.0;

        println!("\n=== Shuffle Quality Report ===");
        println!("Samples: {}", samples);
        println!("Mean Given count per cell: {:.2}", expected);
        println!(
            "Chi-square: {:.2} (df=80, p<0.05 threshold≈101)",
            chi_square
        );
        println!("Normalized entropy: {:.6} (1.0 = perfect uniform)", entropy);

        let min_count = *counts.iter().min().unwrap();
        let max_count = *counts.iter().max().unwrap();
        println!("Min/Max counts: {}/{}", min_count, max_count);

        assert!(chi_square < 101.0, "Chi-square too high: {:.2}", chi_square);
    }
}
