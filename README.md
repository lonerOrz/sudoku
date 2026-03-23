# Sudoku Core

High-performance Sudoku puzzle generation and solving library.

## Benchmark Results

| Function                       | Time Complexity | Result (Medium) |
| ------------------------------ | --------------- | --------------- |
| `generate(Difficulty::Easy)`   | O(n!×k)         | 66.3 µs         |
| `generate(Difficulty::Medium)` | O(n!×k)         | 87.0 µs         |
| `generate(Difficulty::Hard)`   | O(n!×k)         | 125 µs          |
| `generate(Difficulty::Expert)` | O(n!×k)         | 183 µs          |
| `solve`                        | O(n!)           | 2.45 µs         |
| `count_solutions`              | O(n!)           | 2.09 µs         |
| `compute_conflicts`            | O(n²)           | 282 ns          |
| `find_clue`                    | O(n²)           | 92 ns           |
| `find_errors`                  | O(n²)           | 552 ns          |
| `possible_values`              | O(n)            | 3.3 ns          |
| `is_solved`                    | O(1)            | 75-134 ns       |
| `is_valid`                     | O(1)            | 6.6 ns          |

_Note: n=81 (9×9 grid), k=100 (sample count). Benchmark environment: ARM Mac._

## Run Benchmarks

```bash
cargo bench --package sudoku-core
```

## License

This project is licensed under the BSD 3-Clause License.
