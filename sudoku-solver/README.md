# sudoku-solver

A high-performance Sudoku/Sukaku puzzle solver and difficulty rater written in Rust.

## Overview

`sudoku-solver` implements a complete difficulty rating system (ER 1.0-10.0+) compatible with **Sudoku Explainer** (SE), supporting:

- **59 solving techniques** from basic Singles to complex Nested Forcing Chains
- **12 variant rules** (X-Diagonal, Disjoint Groups, Non-Consecutive, etc.)
- **Puzzle generation** with symmetric patterns and difficulty filtering
- **CLI tool** with SE-compatible output format specifiers

## Features

### Solving Techniques

| Category | Techniques | SE Range |
|----------|-----------|----------|
| Basic | Naked/Hidden Single, Locked Candidates, Pairs/Triples | 1.0-4.0 |
| Intermediate | XY/XYZ/WXYZ-Wing, Unique Rectangle, BUG | 4.0-5.5 |
| Advanced | Skyscraper, Kite, Aligned Pair Exclusion | 5.5-7.0 |
| Chaining | X/Y-Cycles, Forcing Chains, Nested Chains | 6.5-10.0+ |

### Variant Support

- **Region-based**: X-Diagonal, Disjoint Groups, Windows, Center Dot, Asterisk, Girandola
- **Adjacency-based**: Non-Consecutive, Ferz NC, Anti-Knight, Anti-King, Toroidal

### Puzzle Generation

- Symmetric patterns (8 symmetry types)
- Difficulty range filtering (ER 1.0-10.0+)
- Multi-threaded generation
- Unique solution verification

## Installation

```bash
cargo install --path sudoku-solver
```

## Usage

### Rate a Puzzle

```bash
# Basic rating
sudoku-solver rate -p "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79"

# Custom output format
sudoku-solver rate -p "53.." --format "ER=%r, Technique=%D"

# JSON output
sudoku-solver rate -p "53.." --json
```

### Generate Puzzles

```bash
# Generate 10 puzzles with difficulty 5.0-6.0
sudoku-solver generate --min-diff 5.0 --max-diff 6.0 --count 10

# With rotational symmetry
sudoku-solver generate --min-diff 4.0 --max-diff 5.0 --symmetry rotational180
```

### Format Specifiers

| Specifier | Description |
|-----------|-------------|
| `%r` | ER rating |
| `%p` | EP rating |
| `%d` | ED rating |
| `%D` | Technique name |
| `%i` | Puzzle string |
| `%e` | Elapsed time |
| `%l` | Newline |
| `%%` | Literal % |

## Library Usage

```rust
use sudoku_solver::{Grid, Solver, Rater};

let puzzle = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
let grid = Grid::parse(puzzle)?;

// Solve
let mut solver = Solver::new(grid);
let solution = solver.solve();

// Rate difficulty
let mut rater = Rater::new(&mut solver);
let rating = rater.analyse();
println!("ER: {}, EP: {}, ED: {}", rating.er, rating.ep, rating.ed);
```

## Project Structure

```
sudoku-solver/
├── src/
│   ├── lib.rs           # Public API
│   ├── main.rs          # CLI entry point
│   ├── grid/            # Core data structures
│   ├── rules/           # 59 solving techniques
│   ├── solver/          # Rule-based solver
│   ├── rating.rs        # Difficulty rating
│   └── generator/       # Puzzle generation
├── tests/               # Integration tests
└── Cargo.toml
```

## Performance

- Solves most puzzles (ER 1.0-8.5) in <100ms
- Generates symmetric puzzles with target difficulty
- Bitmask-based candidate tracking for efficiency

## Testing

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test chaining
cargo test variant
```

## License

MIT

## Acknowledgments

- Algorithm design参考 from [SukakuExplainer](https://github.com/coloin/sukakuExplainer)
- Difficulty rating system compatible with [Sudoku Explainer](https://www.enjoysudoku.com/sudoku-explainer.html)
