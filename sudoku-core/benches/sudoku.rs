use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use sudoku_core::{Difficulty, Grid, PEERS, generate};

fn generate_all_difficulties(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate");

    for difficulty in [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Expert,
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(difficulty.label()),
            &difficulty,
            |b, &d| {
                b.iter(|| generate(black_box(d)));
            },
        );
    }

    group.finish();
}

fn solve_grid(c: &mut Criterion) {
    let (puzzle, _solution) = generate(Difficulty::Expert);
    let grid = puzzle;

    c.bench_function("solve_expert_puzzle", |b| {
        b.iter(|| {
            let mut g = grid;
            solve_backtrack(&mut g);
        });
    });
}

fn solve_backtrack(grid: &mut Grid) -> bool {
    if let Some(idx) = find_empty(grid) {
        for val in 1..=9 {
            if is_valid(grid, idx, val) {
                grid[idx / 9][idx % 9] = sudoku_core::Cell::Given(val);
                if solve_backtrack(grid) {
                    return true;
                }
                grid[idx / 9][idx % 9] = sudoku_core::Cell::Empty;
            }
        }
        false
    } else {
        true
    }
}

fn find_empty(grid: &Grid) -> Option<usize> {
    for idx in 0..81 {
        if grid[idx / 9][idx % 9].value().is_none() {
            return Some(idx);
        }
    }
    None
}

fn is_valid(grid: &Grid, idx: usize, val: u8) -> bool {
    for &peer in &PEERS[idx] {
        if peer == u8::MAX {
            break;
        }
        if let Some(v) = grid[(peer / 9) as usize][(peer % 9) as usize].value() {
            if v == val {
                return false;
            }
        }
    }
    true
}

fn uniqueness_check(c: &mut Criterion) {
    let (puzzle, _solution) = generate(Difficulty::Medium);
    let grid = puzzle;

    c.bench_function("count_solutions_medium", |b| {
        b.iter(|| {
            let mut g = grid;
            let mut count = 0;
            count_solutions_inner(&mut g, &mut count, 2);
        });
    });
}

fn count_solutions_inner(grid: &mut Grid, count: &mut usize, max_count: usize) {
    if *count >= max_count {
        return;
    }

    if let Some(idx) = find_empty(grid) {
        for val in 1..=9 {
            if is_valid(grid, idx, val) {
                grid[idx / 9][idx % 9] = sudoku_core::Cell::Given(val);
                count_solutions_inner(grid, count, max_count);
                grid[idx / 9][idx % 9] = sudoku_core::Cell::Empty;
            }
        }
    } else {
        *count += 1;
    }
}

fn shuffle_quality(c: &mut Criterion) {
    c.bench_function("shuffle_chi_square", |b| {
        b.iter(|| {
            let mut counts = [0u32; 81];
            for _ in 0..100 {
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
            chi_square
        });
    });
}

fn shuffle_entropy(c: &mut Criterion) {
    c.bench_function("shuffle_entropy", |b| {
        b.iter(|| {
            let mut counts = [0u32; 81];
            for _ in 0..100 {
                let (puzzle, _) = generate(Difficulty::Medium);
                for idx in 0..81 {
                    if puzzle[idx / 9][idx % 9].value().is_some() {
                        counts[idx] += 1;
                    }
                }
            }
            let total: f64 = counts.iter().map(|&c| c as f64).sum();
            let entropy: f64 = counts
                .iter()
                .map(|&c| {
                    let p = c as f64 / total;
                    if p > 0.0 { -p * p.log2() } else { 0.0 }
                })
                .sum::<f64>()
                / 81.0;
            entropy
        });
    });
}

criterion_group!(
    benches,
    generate_all_difficulties,
    solve_grid,
    uniqueness_check,
);
criterion_main!(benches);
