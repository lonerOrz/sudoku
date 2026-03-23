# Sudoku Core

High-performance Sudoku puzzle generation and solving library.

## Benchmark Results

| Function | Time Complexity | Result (Medium) |
|----------|-----------------|-----------------|
| `generate(Difficulty::Easy)` | O(n!×k) | 64.7 µs |
| `generate(Difficulty::Medium)` | O(n!×k) | 86.6 µs |
| `generate(Difficulty::Hard)` | O(n!×k) | 170 µs |
| `generate(Difficulty::Expert)` | O(n!×k) | 196 µs |
| `solve` | O(n!) | 6.75 µs |
| `count_solutions` | O(n!) | 2.11 µs |
| `compute_conflicts` | O(n²) | 276 ns |
| `find_clue` | O(n²) | 85 ns |
| `find_errors` | O(n²) | 807 ns |
| `possible_values` | O(n) | 3.4 ns |
| `is_solved` | O(1) | 83 ns |
| `is_valid` | O(1) | 2.2 ns |
| `shuffle_chi_square` | O(k·n²) | 8.81 ms |
| `shuffle_entropy` | O(k·n²) | 8.70 ms |

*Note: n=81 (9×9 grid), k=100 (sample count). Benchmark environment: ARM Mac.*

## Run Benchmarks

```bash
cargo bench --package sudoku-core
```

## License

This project is licensed under the BSD 3-Clause License.