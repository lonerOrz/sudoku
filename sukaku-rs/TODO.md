# Sukaku-RS Development TODO

## Project Goal

Rewrite `SukakuExplainer` (Java) in Rust - library + CLI only, no GUI.

**Target**: SER-like difficulty rating system for Sudoku/Sukaku puzzles.

---

## Current Status

**Branch**: `sukaku`
**Last Updated**: 2026-03-25
**Version**: 0.1.0

### [x] Completed

| Phase     | Task                           | Status       | Difficulty |
| --------- | ------------------------------ | ------------ | ---------- |
| Phase 0   | MVP Foundation                 | [x] Complete | -          |
| Phase 0   | Test infrastructure (15 tests) | [x] Complete | -          |
| Phase 1.1 | Naked Single (1.6)             | [x] Complete | SE 1.6     |
| Phase 1.1 | Hidden Single (1.2/1.5)        | [x] Complete | SE 1.2-1.5 |
| Phase 1.2 | Naked Pair                     | [x] Complete | SE 3.0     |
| Phase 1.2 | Hidden Pair                    | [x] Complete | SE 2.9     |
| Phase 1.3 | Locked Candidates - Pointing   | [x] Complete | SE 1.7/2.6 |
| Phase 1.3 | Locked Candidates - Claiming   | [x] Complete | SE 1.9/2.8 |
| Phase 1.4 | Naked Triple                   | [x] Complete | SE 3.6     |
| Phase 1.4 | Hidden Triple                  | [x] Complete | SE 4.0     |
| Phase 1.5 | X-Wing                         | [x] Complete | SE 3.2     |

**Current Coverage**: ~80% of common puzzles (ER 1.0-4.0)

---

## Roadmap

### Phase 1: Basic Techniques (SE 1.0-4.0) [COMPLETE] 🎉

| Task                             | Difficulty | Priority | Est. Time | Tests |
| -------------------------------- | ---------- | -------- | --------- | ----- |
| [x] Naked Single                 | 1.6        | P0       | -         | [x]   |
| [x] Hidden Single                | 1.2/1.5    | P0       | -         | [x]   |
| [x] Naked Pair                   | 3.0        | P0       | -         | [x]   |
| [x] Hidden Pair                  | 2.9        | P0       | -         | [x]   |
| [x] Locked Candidates - Pointing | 1.7/2.6    | P0       | -         | [x]   |
| [x] Locked Candidates - Claiming | 1.9/2.8    | P0       | -         | [x]   |
| [x] Naked Triple                 | 3.6        | P1       | -         | [x]   |
| [x] Hidden Triple                | 4.0        | P1       | -         | [x]   |
| [x] X-Wing                       | 3.2        | P1       | -         | [x]   |

**Target**: 80% puzzle coverage ✅ **ACHIEVED**

---

### Phase 2: Intermediate Techniques (SE 3.0-5.0) [NEXT]

**Reference**: SukakuExplainer classes: `Fisherman`, `XYWing`, `NakedSet`, `HiddenSet`, `UniqueLoops`

| Task                          | Difficulty | Priority | Est. Time | Tests | SE Class |
| ----------------------------- | ---------- | -------- | --------- | ----- | -------- |
| [ ] Swordfish                 | 4.0        | P0       | 3h        | [ ]   | `Fisherman(3)` |
| [ ] XY-Wing                   | 4.2        | P0       | 3h        | [ ]   | `XYWing(false)` |
| [ ] XYZ-Wing                  | 4.4        | P0       | 3h        | [ ]   | `XYWing(true)` |
| [ ] Jellyfish                 | 5.4        | P1       | 3h        | [ ]   | `Fisherman(4)` |
| [ ] Naked Quad                | 5.0        | P1       | 2h        | [ ]   | `NakedSet(4)` |
| [ ] Hidden Quad               | 5.4        | P1       | 2h        | [ ]   | `HiddenSet(4)` |
| [ ] Unique Rectangle Type 1-4 | 4.5-5.0    | P0       | 4h        | [ ]   | `UniqueLoops` |
| [ ] BUG+1                     | 5.6        | P1       | 3h        | [ ]   | `BivalueUniversalGrave` |
| [ ] WXYZ-Wing                 | 5.5        | P2       | 4h        | [ ]   | `WXYZWing` |

**Target**: 95% puzzle coverage

---

### Phase 3: Advanced Techniques (SE 5.0-8.0)

**Reference**: SukakuExplainer classes: `StrongLinks`, `Chaining`, `AlignedExclusion`

| Task                         | Difficulty | Priority | SE Class |
| ---------------------------- | ---------- | -------- | -------- |
| [x] Turbot Fish              | 4.0-4.2    | P0       | `StrongLinks(2)` / `TurbotFish` |
| [ ] Skyscraper               | 4.2        | P0       | `StrongLinks(2)` |
| [ ] 2-String Kite            | 4.1        | P1       | `StrongLinks(2)` |
| [ ] 3-Strong-Links           | 5.0        | P1       | `StrongLinks(3)` |
| [ ] X-Cycles (Simple)        | 6.5        | P1       | `Chaining` |
| [ ] Y-Cycles                 | 6.5        | P1       | `Chaining` |
| [ ] Aligned Pair Exclusion   | 6.2        | P2       | `AlignedPairExclusion` |
| [ ] Aligned Triplet Exclusion| 7.5        | P2       | `AlignedExclusion(3)` |
| [ ] Forcing Chains           | 7.0-8.0    | P3       | `Chaining` |
| [ ] Nishio                   | 7.5-8.5    | P3       | `Chaining(..., true, ...)` |

---

### Phase 4: CLI Enhancements

**Reference**: `serate.java` - command line interface

| Task                                       | Difficulty | Priority |
| ------------------------------------------ | ---------- | -------- |
| [ ] Batch processing (`--input=FILE`)      | Low        | P0       |
| [ ] Format strings (`--format="%r/%p/%d"`) | Medium     | P0       |
| [ ] Multi-threading (`--threads=N`)        | Medium     | P1       |
| [ ] HTML output (`--html`)                 | Low        | P2       |
| [ ] Step-by-step verbose output            | Low        | P1       |
| [ ] JSON output (`--json`)                 | Low        | P2       |

**Target**: serate-compatible CLI

---

### Phase 5: Variant Support (Optional)

**Reference**: `Grid.java` - variant cell visibility rules

| Task                        | Difficulty | Priority | SE Support |
| --------------------------- | ---------- | -------- | ---------- |
| [ ] Disjoint Groups (DG)    | Medium     | P2       | `DGVisibleCellIndex` |
| [ ] X-Diagonal (Sudoku X)   | Medium     | P2       | `XVisibleCellIndex` |
| [ ] Non-Consecutive (NC)    | Medium     | P3       | `wazirCellsRegular`, `lockedNC` |
| [ ] Ferz NC (Diagonal NC)   | High       | P3       | `ferzCellsRegular`, `lockedFNC` |
| [ ] Anti-King / Anti-Knight | Medium     | P3       | `knightCellIndex` |
| [ ] Toroidal Board          | High       | P3       | `wazirCellsToroidal` |
| [ ] Asterisk                | Medium     | P3       | `asteriskVisibleCellIndex` |
| [ ] Girandola               | Medium     | P3       | `girandolaVisibleCellIndex` |

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
cargo test -p sukaku-rs test_naked_triple

# Check clippy
cargo clippy -p sukaku-rs

# Format check
cargo fmt -p sukaku-rs --check
```

**Current**: 15 tests + 1 doctest, all passing ✅

---

## Next Steps

### Immediate (Next Session) - Phase 2 Start

1. **Swordfish (4.0)** - Extend `fish.rs` with `Fisherman(3)` logic
   - File: `sukaku-rs/src/rules/fish.rs`
   - Pattern: Generalize X-Wing to 3 rows/columns
   - Test: Find puzzle requiring Swordfish

2. **XY-Wing (4.2)** - New `sukaku-rs/src/rules/wing.rs`
   - File: `sukaku-rs/src/rules/wing.rs`
   - Pattern: 3 cells with XY, XZ, YZ candidates
   - Test: Find puzzle requiring XY-Wing

### Quick Start (Next Session)

```bash
# 1. Verify environment
cargo test -p sukaku-rs

# 2. Check current rules
cat sukaku-rs/src/rules/mod.rs

# 3. Start implementing Swordfish
# Edit: sukaku-rs/src/rules/fish.rs
# Add: pub fn swordfish(...)
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
git diff HEAD~10 sukaku-rs/
```

---

**Last Session**: Phase 1 Complete - All basic techniques implemented (9/9)
**Current Status**: Ready for Phase 2 (Intermediate Techniques)
**Next Session**: Start with Swordfish (4.0) or XY-Wing (4.2)

---

## SukakuExplainer Reference

### Technique to Class Mapping

| Technique | SE Class | File |
|-----------|----------|------|
| Naked Single | `NakedSingle` | `NakedSingle.java` |
| Hidden Single | `HiddenSingle` | `HiddenSingle.java` |
| Locked Candidates | `Locking`, `DirectLockingHint` | `Locking.java` |
| Naked Pair/Triple/Quad | `NakedSet(n)` | `NakedSet.java` |
| Hidden Pair/Triple/Quad | `HiddenSet(n, direct)` | `HiddenSet.java` |
| X-Wing/Swordfish/Jellyfish | `Fisherman(n)` | `Fisherman.java` |
| XY-Wing/XYZ-Wing | `XYWing(xyz)` | `XYWing.java` |
| Turbot Fish/Skyscraper | `StrongLinks(2)`, `TurbotFish` | `StrongLinks.java` |
| Unique Rectangle | `UniqueLoops` | `UniqueLoops.java` |
| BUG | `BivalueUniversalGrave` | `BivalueUniversalGrave.java` |
| WXYZ/UVWXYZ Wing | `WXYZWing`, `UVWXYZWing` | `*.java` |
| Aligned Exclusion | `AlignedPairExclusion`, `AlignedExclusion(n)` | `AlignedExclusion.java` |
| Forcing Chains | `Chaining(...)` | `Chaining.java` |

### Rating System (SER)

```
ER (Experience Rating): Hardest technique required
EP (Entry Point): First technique difficulty  
ED (Entry Difficulty): First step difficulty

Key thresholds:
- 1.0-2.0: Basic (Singles, Locked Candidates)
- 2.0-3.0: Simple (Pairs, X-Wing)
- 3.0-4.0: Intermediate (Triples, Swordfish, XY-Wing)
- 4.0-5.0: Advanced (Quads, Unique Rectangle, BUG)
- 5.0-6.0: Complex (Wings, Strong Links)
- 6.0-7.0: Expert (Cycles, Chains)
- 7.0-8.0: Master (Forcing Chains, Nishio)
- 8.0+: Extreme (Nested Forcing Chains)
```
