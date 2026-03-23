# Sudoku Core

High-performance Sudoku puzzle generation and solving library.

## Benchmark Results

| Function | Time Complexity | Result (Medium) |
|----------|-----------------|-----------------|
| `generate(Difficulty::Easy)` | O(n!×k) | ~65 µs |
| `generate(Difficulty::Medium)` | O(n!×k) | 96 µs |
| `generate(Difficulty::Hard)` | O(n!×k) | ~170 µs |
| `generate(Difficulty::Expert)` | O(n!×k) | ~195 µs |
| `solve` | O(n!) | 2.52 µs |
| `count_solutions` | O(n!) | 4.25 µs |
| `compute_conflicts` | O(n²) | 298 ns |
| `find_clue` | O(n²) | 93 ns |
| `find_errors` | O(n²) | 557 ns |
| `possible_values` | O(n) | 120 ns |
| `is_solved` | O(1) | 146 ns |
| `is_valid` | O(1) | 37 ns |

*Note: n=81 (9×9 grid), k=100 (sample count). Benchmark environment: ARM Mac.*

## Run Benchmarks

```bash
cargo bench --package sudoku-core
```

## License

This project is licensed under the BSD 3-Clause License.