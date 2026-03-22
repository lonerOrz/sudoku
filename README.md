# Sudoku Core

High-performance Sudoku puzzle generation and solving library.

## Benchmark Results

| Function | Time Complexity | Result (Medium) |
|----------|-----------------|-----------------|
| `generate(Difficulty::Easy)` | O(n!×k) | 67.1 µs |
| `generate(Difficulty::Medium)` | O(n!×k) | 87.0 µs |
| `generate(Difficulty::Hard)` | O(n!×k) | 121 µs |
| `generate(Difficulty::Expert)` | O(n!×k) | 184 µs |
| `solve` | O(n!) | 2.50 µs |
| `count_solutions` | O(n!) | 3.15 µs |
| `compute_conflicts` | O(n²) | 296 ns |
| `find_clue` | O(n²) | 90 ns |
| `shuffle_chi_square` | O(k·n²) | 8.73 ms |
| `shuffle_entropy` | O(k·n²) | 8.67 ms |

*Note: n=81 (9×9 grid), k=100 (sample count). Benchmark environment: ARM Mac.*

## Run Benchmarks

```bash
cargo bench --package sudoku-core
```

## License

This project is licensed under the BSD 3-Clause License.