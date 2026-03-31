# Sudoku TUI

A complete Sudoku puzzle generation, solving, and difficulty rating ecosystem written in Rust.

## Project Structure

This workspace contains three crates:

- **sudoku-core** - Core data structures and high-performance algorithms
- **sudoku-solver** - Complete solving engine with 59 techniques and SE-compatible difficulty rating
- **sudoku-tui** - Terminal-based interactive Sudoku game

## Quick Start

```bash
# Build all crates
cargo build --release

# Run tests
cargo test --workspace

# Run benchmarks
cargo bench --package sudoku-core
```

## Usage

### Rate a Puzzle

```bash
cargo run -p sudoku-solver -- rate -p "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79"
```

### Generate Puzzles

```bash
cargo run -p sudoku-solver -- generate --min-diff 5.0 --max-diff 6.0 --count 10
```

### Launch TUI

```bash
cargo run -p sudoku-tui
```

## Features

- 59 solving techniques (ER 1.0-10.0+)
- SE-compatible difficulty rating (ER/EP/ED)
- 12 variant rules (X-Diagonal, Disjoint Groups, Non-Consecutive, etc.)
- Multi-threaded puzzle generation with symmetry filtering
- Interactive terminal UI

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

## License

BSD 3-Clause License
