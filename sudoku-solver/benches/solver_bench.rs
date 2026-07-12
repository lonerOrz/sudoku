use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sudoku_solver::rules::all_rules;
use sudoku_solver::{Grid, Rater, Solver};

const PUZZLE_ER1: &str =
    "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
const PUZZLE_ER3: &str =
    "100007090030020008009600500005300900010080002600004000300000010040000007007000300";
const PUZZLE_ER5: &str =
    "900062700005003000000000006700030000000009000802045009003501028040000005010000000";

fn bench_rater(c: &mut Criterion) {
    let mut group = c.benchmark_group("rater_analyse");
    let puzzles = [
        ("er_1", PUZZLE_ER1),
        ("er_3", PUZZLE_ER3),
        ("er_5", PUZZLE_ER5),
    ];

    for (name, puzzle) in &puzzles {
        group.bench_with_input(BenchmarkId::from_parameter(name), puzzle, |b, p| {
            b.iter(|| {
                let grid = Grid::parse(p).unwrap();
                let mut solver = Solver::new(grid);
                let mut rater = Rater::new(&mut solver);
                rater.analyse()
            });
        });
    }

    group.finish();
}

fn bench_rater_with_counts(c: &mut Criterion) {
    let mut group = c.benchmark_group("rater_with_counts");
    let puzzles = [
        ("er_1", PUZZLE_ER1),
        ("er_3", PUZZLE_ER3),
        ("er_5", PUZZLE_ER5),
    ];

    for (name, puzzle) in &puzzles {
        group.bench_with_input(BenchmarkId::from_parameter(name), puzzle, |b, p| {
            b.iter(|| {
                let grid = Grid::parse(p).unwrap();
                let mut solver = Solver::new(grid);
                let mut rater = Rater::new(&mut solver);
                rater.analyse_with_counts()
            });
        });
    }

    group.finish();
}

fn bench_technique_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("technique_detection");

    let grid = Grid::parse(PUZZLE_ER5).unwrap();
    let mut solver = Solver::new(grid);
    solver.rebuild_candidates();
    let g = solver.grid();

    for rule in &all_rules() {
        group.bench_with_input(
            BenchmarkId::from_parameter(rule.name),
            &rule.func,
            |b, func| {
                b.iter(|| {
                    let mut acc = sudoku_solver::solver::HintAccumulator::new();
                    func(black_box(&g), &mut acc);
                });
            },
        );
    }

    group.finish();
}

fn bench_solver_solve(c: &mut Criterion) {
    let mut group = c.benchmark_group("solver_solve");
    let puzzles = [
        ("er_1", PUZZLE_ER1),
        ("er_3", PUZZLE_ER3),
        ("er_5", PUZZLE_ER5),
    ];

    for (name, puzzle) in &puzzles {
        group.bench_with_input(BenchmarkId::from_parameter(name), puzzle, |b, p| {
            b.iter(|| {
                let grid = Grid::parse(p).unwrap();
                let mut solver = Solver::new(grid);
                solver.solve();
            });
        });
    }

    group.finish();
}

fn bench_generator(c: &mut Criterion) {
    let mut group = c.benchmark_group("generator_generate");
    let configs = [
        ("er_1_2", 1.0f64, 2.0f64),
        ("er_2_4", 2.0, 3.5),
        ("er_4_5", 3.5, 5.0),
    ];

    for (label, min, max) in &configs {
        group.bench_with_input(
            BenchmarkId::from_parameter(label),
            &(*min, *max),
            |b, &(min, max)| {
                b.iter(|| {
                    let mut gen = sudoku_solver::Generator::with_difficulty(min, max);
                    gen.generate()
                });
            },
        );
    }

    group.finish();
}

/// Measure generation success rate and ER distribution.
fn bench_generator_stats(_c: &mut Criterion) {
    let configs = [
        ("er_1_2", 1.0f64, 2.0f64, 20),
        ("er_2_4", 2.0, 3.5, 20),
        ("er_4_5", 3.5, 5.0, 20),
    ];

    eprintln!("\n=== Generator Stats ===");
    for (label, min, max, n) in &configs {
        let mut successes = 0;
        let mut ers = Vec::new();
        let start = std::time::Instant::now();

        for _ in 0..*n {
            let mut gen = sudoku_solver::Generator::with_difficulty(*min, *max);
            if let Ok(puzzle) = gen.generate() {
                successes += 1;
                let mut solver = Solver::new(puzzle);
                let mut rater = Rater::new(&mut solver);
                let rating = rater.analyse();
                ers.push(rating.er);
            }
        }

        let elapsed = start.elapsed();
        let avg_er = if ers.is_empty() {
            0.0
        } else {
            ers.iter().sum::<f64>() / ers.len() as f64
        };

        eprintln!(
            "{}: {}/{} success ({:.0}%), avg ER={:.1}, time={:.1}s",
            label,
            successes,
            n,
            successes as f64 / *n as f64 * 100.0,
            avg_er,
            elapsed.as_secs_f64()
        );
    }
}

/// Show technique breakdown for a sample puzzle.
fn bench_technique_breakdown(_c: &mut Criterion) {
    let puzzles = [
        ("er_1", PUZZLE_ER1),
        ("er_3", PUZZLE_ER3),
        ("er_5", PUZZLE_ER5),
    ];

    eprintln!("\n=== Technique Breakdown ===");
    for (name, puzzle) in &puzzles {
        let grid = Grid::parse(puzzle).unwrap();
        let mut solver = Solver::new(grid);
        let mut rater = Rater::new(&mut solver);
        let (rating, counts) = rater.analyse_with_counts();

        eprintln!("\n{}: ER={:.1} ({})", name, rating.er, rating.er_technique);

        let mut sorted: Vec<_> = counts.into_iter().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
        for (tech, count) in &sorted {
            eprintln!("  {:>3}x {}", count, tech);
        }
    }
}

criterion_group!(
    benches,
    bench_rater,
    bench_rater_with_counts,
    bench_technique_detection,
    bench_solver_solve,
    bench_generator,
    bench_generator_stats,
    bench_technique_breakdown,
);
criterion_main!(benches);
