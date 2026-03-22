# Sudoku Core

High-performance Sudoku puzzle generation and solving library.

## Benchmark Results

| Function | Time Complexity | Result (Medium) |
|----------|-----------------|-----------------|
| `generate(Difficulty::Easy)` | O(n²) | 64.5 µs |
| `generate(Difficulty::Medium)` | O(n²) | 86.5 µs |
| `generate(Difficulty::Hard)` | O(n²) | 118 µs |
| `generate(Difficulty::Expert)` | O(n²) | 178 µs |
| `solve` | O(n!) | 99.8 µs |
| `count_solutions` | O(n!) | 2.05 µs |
| `compute_conflicts` | O(n²) | 909 ns |
| `find_clue` | O(n²) | 96 ns |
| `shuffle_chi_square` | O(k·n²) | 8.59 ms |
| `shuffle_entropy` | O(k·n²) | 8.55 ms |

*Note: n=81 (9×9 grid), k=100 (sample count). Benchmark environment: ARM Mac.*

## Run Benchmarks

```bash
cargo bench --package sudoku-core
```

## License

This project is licensed under the BSD 3-Clause License.