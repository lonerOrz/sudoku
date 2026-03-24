# Sukaku-RS Development TODO

## Project Goal

Rewrite `SukakuExplainer` (Java) in Rust - library + CLI only, no GUI.

**Target**: SER-like difficulty rating system for Sudoku/Sukaku puzzles.

---

## Current Status

**Branch**: `sukaku`
**Last Updated**: 2026-03-24
**Version**: 0.1.0

### [x] Completed

| Phase     | Task                           | Status       | Difficulty |
| --------- | ------------------------------ | ------------ | ---------- |
| Phase 0   | MVP Foundation                 | [x] Complete | -          |
| Phase 0   | Test infrastructure (11 tests) | [x] Complete | -          |
| Phase 1.1 | Naked Single (1.6)             | [x] Complete | SE 1.6     |
| Phase 1.1 | Hidden Single (1.2/1.5)        | [x] Complete | SE 1.2-1.5 |
| Phase 1.2 | Naked Pair                     | [x] Complete | SE 3.0     |
| Phase 1.2 | Hidden Pair                    | [x] Complete | SE 2.9     |

**Current Coverage**: ~25% of common puzzles (ER 1.0-3.0)

---

## Roadmap

### Phase 1: Basic Techniques (SE 1.0-3.0) [IN PROGRESS]

| Task                             | Difficulty | Priority | Est. Time | Tests |
| -------------------------------- | ---------- | -------- | --------- | ----- |
| [x] Naked Single                 | 1.6        | P0       | -         | [x]   |
| [x] Hidden Single                | 1.2/1.5    | P0       | -         | [x]   |
| [x] Naked Pair                   | 3.0        | P0       | -         | [x]   |
| [x] Hidden Pair                  | 2.9        | P0       | -         | [x]   |
| [ ] Locked Candidates - Pointing | 1.7/2.6    | P0       | 2h        | [ ]   |
| [ ] Locked Candidates - Claiming | 1.9/2.8    | P0       | 2h        | [ ]   |
| [ ] Naked Triple                 | 3.6        | P1       | 2h        | [ ]   |
| [ ] Hidden Triple                | 4.0        | P1       | 2h        | [ ]   |
| [ ] X-Wing                       | 3.2        | P1       | 3h        | [ ]   |

**Target**: 80% puzzle coverage

---

### Phase 2: Intermediate Techniques (SE 3.0-5.0)

| Task                          | Difficulty | Priority | Est. Time |
| ----------------------------- | ---------- | -------- | --------- |
| [ ] XY-Wing                   | 4.2        | P0       | 3h        |
| [ ] XYZ-Wing                  | 4.4        | P0       | 3h        |
| [ ] WXYZ-Wing                 | 5.5        | P1       | 4h        |
| [ ] Swordfish                 | 4.0        | P1       | 3h        |
| [ ] Unique Rectangle Type 1-4 | 4.5-5.0    | P0       | 4h        |
| [ ] BUG+1                     | 5.6        | P1       | 3h        |
| [ ] Jellyfish                 | 5.4        | P2       | 3h        |

**Target**: 95% puzzle coverage

---

### Phase 3: Advanced Techniques (SE 5.0-8.0)

| Task                         | Difficulty | Priority |
| ---------------------------- | ---------- | -------- |
| [ ] Skyscraper / Turbot Fish | 4.0-4.2    | P0       |
| [ ] 2-String Kite            | 4.1        | P1       |
| [ ] X-Cycles                 | 6.5-7.5    | P1       |
| [ ] Y-Cycles                 | 6.5-7.5    | P1       |
| [ ] Forcing Chains           | 7.0-8.0    | P2       |
| [ ] Aligned Pair Exclusion   | 6.2        | P2       |
| [ ] Nishio                   | 7.5-8.5    | P3       |

---

### Phase 4: CLI Enhancements

| Task                                       | Difficulty | Priority |
| ------------------------------------------ | ---------- | -------- |
| [ ] Batch processing (`--input=FILE`)      | Low        | P0       |
| [ ] Format strings (`--format="%r/%p/%d"`) | Medium     | P0       |
| [ ] Multi-threading (`--threads=N`)        | Medium     | P1       |
| [ ] HTML output (`--html`)                 | Low        | P2       |
| [ ] Step-by-step verbose output            | Low        | P1       |

**Target**: serate-compatible CLI

---

### Phase 5: Variant Support (Optional)

| Task                        | Difficulty | Priority |
| --------------------------- | ---------- | -------- |
| [ ] Disjoint Groups (DG)    | Medium     | P2       |
| [ ] X-Diagonal (Sudoku X)   | Medium     | P2       |
| [ ] Non-Consecutive (NC)    | Medium     | P3       |
| [ ] Anti-King / Anti-Knight | Medium     | P3       |
| [ ] Toroidal Board          | High       | P3       |

---

## Architecture

```
sukaku-rs/src/
├── lib.rs              # Public API
├── main.rs             # CLI entry point
├── error.rs            # Error types
├── generator.rs        # Puzzle generation
├── rating.rs           # Difficulty rating (ER/EP/ED)
├── grid/               # Core data structures
│   ├── mod.rs
│   ├── cell.rs
│   ├── candidates.rs   # Bitmask candidate tracking
│   └── region.rs       # Rows, Cols, Blocks
├── rules/              # Solving techniques
│   ├── mod.rs
│   ├── direct.rs       # Naked/Hidden Single
│   └── indirect.rs     # Naked/Hidden Pair
└── solver/             # Rule-based solver
    ├── mod.rs
    ├── accumulator.rs  # HintAccumulator
    └── hint.rs         # Hint structure
```

### Key Design

```rust
// Hint represents a solving technique
pub struct Hint {
    pub hint_type: HintType,
    pub difficulty: f64,
    pub technique_name: String,
    pub description: String,
    pub cell: Cell,
    pub value: u8,                        // For filling techniques
    pub eliminations: Vec<(Cell, Vec<u8>)>, // For elimination techniques
}

// apply_hint handles both:
// - value > 0: fill cell
// - eliminations: remove candidates from specified cells
```

---

## Testing

```bash
# Run all tests
cargo test -p sukaku-rs

# Run specific test
cargo test -p sukaku-rs test_naked_pair

# Check clippy
cargo clippy -p sukaku-rs

# Format check
cargo fmt -p sukaku-rs --check
```

**Current**: 11 tests + 1 doctest, all passing

---

## Next Steps

### Immediate (Next Session)

1. **Locked Candidates - Pointing (1.7/2.6)**
   - File: `rules/direct.rs` or new `rules/locked.rs`
   - Test: Find puzzle requiring Pointing
   - Difficulty: Medium

2. **Locked Candidates - Claiming (1.9/2.8)**
   - File: `rules/direct.rs` or new `rules/locked.rs`
   - Test: Find puzzle requiring Claiming
   - Difficulty: Medium

### Quick Start (Next Session)

```bash
# 1. Verify environment
cargo test -p sukaku-rs

# 2. Check current rules
cat sukaku-rs/src/rules/mod.rs

# 3. Start implementing Locked Candidates
# Edit: sukaku-rs/src/rules/direct.rs
# Add: pub fn locked_pointing(...)
# Add: pub fn locked_claiming(...)
```

---

## References

- [SudokuWiki - Solving Techniques](https://www.sudokuwiki.org/)
- [SukakuExplainer (Original Java)](../SukakuExplainer/)
- [SER Rating System](http://forum.enjoysudoku.com/sudoku-explainer-1-2-1-t3195.html)

---

## Commit History

```bash
# View recent commits
git log --oneline sukaku

# View changes
git diff HEAD~5 sukaku-rs/
```

---

**Last Session**: Implemented Naked Pair & Hidden Pair with proper elimination handling.
**Next Session**: Start Locked Candidates (Pointing/Claiming).
