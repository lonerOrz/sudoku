use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sudoku_core::{count_solutions, generate, solve, Difficulty};

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

    c.bench_function("solve_expert_puzzle", |b| {
        b.iter(|| {
            let mut g = puzzle;
            solve(black_box(&mut g));
        });
    });
}

fn uniqueness_check(c: &mut Criterion) {
    let (puzzle, _solution) = generate(Difficulty::Medium);

    c.bench_function("count_solutions_medium", |b| {
        b.iter(|| {
            let mut g = puzzle;
            count_solutions(black_box(&mut g));
        });
    });
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
                    if p > 0.0 {
                        -p * p.log2()
                    } else {
                        0.0
                    }
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
    shuffle_quality,
    shuffle_entropy,
);
criterion_main!(benches);
