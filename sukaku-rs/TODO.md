# Sukaku-RS Development TODO

> **Goal**: Complete 100% rewrite of `SukakuExplainer` (Java) in Rust - library + CLI only, no GUI.
>
> **Target**: Full SER-like difficulty rating system (SE 1.0-10.0+) for Sudoku/Sukaku puzzles.

---

## Current Status

| Item | Status |
|------|--------|
| **Branch** | `sukaku` |
| **Version** | 0.1.0 |
| **Last Updated** | 2026-03-27 |
| **Overall Progress** | 32/62 techniques (52%) |
| **Puzzle Coverage** | ~95% (ER 1.0-5.5) |
| **Tests** | 35 passing |
| **CLI** | generate, rate, direct input |

---

## Roadmap Overview

| Phase | Focus | Techniques | Progress | Target |
|-------|-------|------------|----------|--------|
| **Phase 1** | Basic (SE 1.0-4.0) | 11 | ✅ 100% | Complete |
| **Phase 2** | Intermediate (SE 4.0-5.5) | 11 | ✅ **100%** | Complete |
| **Phase 3** | Advanced (SE 5.5-7.0) | 20 | 🔄 0% | 2026-Q3 |
| **Phase 4** | Chaining (SE 6.5-10.0+) | 10 | ⏳ 0% | 2026-Q4 |
| **Phase 5** | Variant Support | 12 | ⏳ 8% | Optional |
| **Phase 6** | CLI Enhancement | 20 | 🔄 40% | 2026-Q3 |
| **Phase 7** | Generator | 6 | 🔄 67% | Optional |
| **Phase 8** | Rating Enhancement | 4 | ⏳ 0% | Optional |

**Total**: 62 solving techniques + 12 variants + 20 CLI features

---

## Phase 1: Basic Techniques (SE 1.0-4.0) ✅ COMPLETE

**Coverage**: 80% of common puzzles | **Status**: 🎉 DONE

| # | Technique | Difficulty | Priority | File | Tests | SE Class |
|---|-----------|------------|----------|------|-------|----------|
| 1.1 | Naked Single | 1.6 | P0 | `direct.rs` | [x] | `NakedSingle` |
| 1.2 | Hidden Single (block) | 1.2 | P0 | `direct.rs` | [x] | `HiddenSingle` |
| 1.3 | Hidden Single (line) | 1.5 | P0 | `direct.rs` | [x] | `HiddenSingle` |
| 1.4 | Naked Pair | 3.0 | P0 | `subset.rs` | [x] | `NakedSet(2)` |
| 1.5 | Hidden Pair | 2.9 | P0 | `subset.rs` | [x] | `HiddenSet(2)` |
| 1.6 | Locked Pointing | 1.7/2.6 | P0 | `locked.rs` | [x] | `Locking` |
| 1.7 | Locked Claiming | 1.9/2.8 | P0 | `locked.rs` | [x] | `Locking` |
| 1.8 | Naked Triple | 3.6 | P1 | `subset.rs` | [x] | `NakedSet(3)` |
| 1.9 | Hidden Triple | 4.0 | P1 | `subset.rs` | [x] | `HiddenSet(3)` |
| 1.10 | X-Wing | 3.2 | P1 | `fish.rs` | [x] | `Fisherman(2)` |
| 1.11 | Swordfish | 4.0 | P0 | `fish.rs` | [x] | `Fisherman(3)` |

---

## Phase 2: Intermediate Techniques (SE 4.0-5.5)

**Coverage**: 95% of puzzles | **Target**: 2026-Q2

| # | Technique | Difficulty | Priority | Est. | File | Tests | SE Class |
|---|-----------|------------|----------|------|------|-------|----------|
| 2.1 | XY-Wing | 4.2 | P0 | 3h | `wing.rs` | [x] | `XYWing(false)` |
| 2.2 | XYZ-Wing | 4.4 | P0 | 3h | `wing.rs` | [x] | `XYWing(true)` |
| 2.3 | WXYZ-Wing | 5.5 | P1 | 4h | `wing.rs` | [x] | `WXYZWing` |
| 2.4 | Unique Rectangle Type 1 | 4.5 | P0 | 2h | `unique.rs` | [x] | `UniqueLoops` |
| 2.5 | Unique Rectangle Type 2 | 4.6 | P0 | 2h | `unique.rs` | [x] | `UniqueLoops` |
| 2.6 | Unique Rectangle Type 3 | 4.8 | P1 | 3h | `unique.rs` | [x] | `UniqueLoops` |
| 2.7 | Unique Rectangle Type 4 | 5.0 | P1 | 2h | `unique.rs` | [x] | `UniqueLoops` |
| 2.8 | BUG+1 | 5.6 | P1 | 3h | `unique.rs` | [x] | `BivalueUniversalGrave` |
| 2.9 | Naked Quad | 5.0 | P1 | 2h | `subset.rs` | [x] | `NakedSet(4)` |
| 2.10 | Hidden Quad | 5.4 | P1 | 2h | `subset.rs` | [x] | `HiddenSet(4)` |
| 2.11 | Jellyfish | 5.2 | P1 | 3h | `fish.rs` | [x] | `Fisherman(4)` |

**Status**: 11/11 techniques implemented ✅ | Phase 2 complete!

**Files Created**:
- ✅ `src/rules/wing.rs` - XY/XYZ/WXYZ-Wing implementations
- ✅ `src/rules/unique.rs` - Unique Rectangle Type 1-4

---

## Phase 3: Advanced Techniques (SE 5.5-7.0)

**Target**: 98% puzzle coverage | **Target**: 2026-Q3

| # | Technique | Difficulty | Priority | Est. | File | Tests | SE Class |
|---|-----------|------------|----------|------|------|-------|----------|
| 3.1 | Skyscraper | 4.0 | P0 | 2h | `strong_link.rs` | [x] | `StrongLinks(2)` |
| 3.2 | 2-String Kite | 4.1 | P0 | 2h | `strong_link.rs` | [x] | `StrongLinks(2)` |
| 3.3 | 3-Strong-Links Fish | 5.4 | P1 | 3h | `strong_link.rs` | [x] | `StrongLinks(3)` |
| 3.4 | 4-Strong-Links Fish | 5.8 | P2 | 4h | `strong_link.rs` | [x] | `StrongLinks(4)` |
| 3.5 | 5-Strong-Links Fish | 6.0 | P2 | 4h | `strong_link.rs` | `StrongLinks(5)` |
| 3.6 | 6-Strong-Links Fish | 6.2 | P2 | 4h | `strong_link.rs` | `StrongLinks(6)` |
| 3.7 | VWXYZ-Wing | 6.2 | P2 | 4h | `wing.rs` | `VWXYZWing` |
| 3.8 | UVWXYZ-Wing | 6.6 | P2 | 5h | `wing.rs` | `UVWXYZWing` |
| 3.9 | TUVWXYZ-Wing | 7.0 | P3 | 5h | `wing.rs` | `TUVWXYZWing` |
| 3.10 | Aligned Pair Exclusion | 6.2 | P2 | 4h | `exclusion.rs` | `AlignedPairExclusion` |
| 3.11 | Aligned Triplet Exclusion | 7.5 | P3 | 6h | `exclusion.rs` | `AlignedExclusion(3)` |
| 3.12 | BUG+2 | 5.8 | P2 | 3h | `unique.rs` | [x] | `BivalueUniversalGrave` |
| 3.13 | BUG+3 | 6.0 | P2 | 3h | `unique.rs` | [x] | `BivalueUniversalGrave` |
| 3.14 | BUG+4 | 6.2 | P2 | 3h | `unique.rs` | [x] | `BivalueUniversalGrave` |
| 3.15 | Generalized Naked Pair | 3.0 | P3 | 2h | `subset_gen.rs` | `NakedSetGen(2)` |
| 3.16 | Generalized Naked Triplet | 3.6 | P3 | 3h | `subset_gen.rs` | `NakedSetGen(3)` |
| 3.17 | Generalized Naked Quad | 5.0 | P3 | 3h | `subset_gen.rs` | `NakedSetGen(4)` |
| 3.18 | Generalized Naked Quint | 5.4 | P3 | 4h | `subset_gen.rs` | `NakedSetGen(5)` |
| 3.19 | Generalized Naked Sext | 5.8 | P3 | 4h | `subset_gen.rs` | `NakedSetGen(6)` |
| 3.20 | VLocking (Generalized Intersections) | Variable | P3 | 4h | `vlocking.rs` | `VLocking` |

**New Files Required**:
- `src/rules/strong_link.rs` - Skyscraper, Kite, Strong-Links Fish
- `src/rules/exclusion.rs` - Aligned Pair/Triplet Exclusion
- `src/rules/subset_gen.rs` - Generalized Naked Sets
- `src/rules/vlocking.rs` - Generalized Intersections

---

## Phase 4: Chaining System (SE 6.5-10.0+)

**Target**: 100% puzzle coverage (including extreme) | **Target**: 2026-Q4

| # | Technique | Difficulty | Priority | Est. | File | SE Class |
|---|-----------|------------|----------|------|------|----------|
| 4.1 | X-Cycles (Simple) | 6.5 | P0 | 6h | `chaining.rs` | `Chaining` |
| 4.2 | Y-Cycles | 6.5 | P0 | 6h | `chaining.rs` | `Chaining` |
| 4.3 | Forcing Chain Cycle | 7.0 | P0 | 8h | `chaining.rs` | `Chaining(false,false,false)` |
| 4.4 | Nishio Forcing Chain | 7.5-8.5 | P1 | 10h | `chaining.rs` | `Chaining(...,true,...)` |
| 4.5 | Multiple Forcing Chain | 8.0 | P1 | 12h | `chaining.rs` | `Chaining(true,false,false)` |
| 4.6 | Dynamic Forcing Chain | 8.5 | P1 | 14h | `chaining.rs` | `Chaining(true,true,false)` |
| 4.7 | Dynamic Forcing Chain+ | 9.0 | P2 | 16h | `chaining.rs` | `Chaining(true,true,false,1)` |
| 4.8 | Nested Forcing Chain (2-level) | 9.5 | P2 | 20h | `chaining.rs` | `Chaining(...,2)` |
| 4.9 | Nested Forcing Chain (3-level) | 10.0 | P3 | 24h | `chaining.rs` | `Chaining(...,3)` |
| 4.10 | Nested Forcing Chain (4-level) | 10.5+ | P3 | 30h | `chaining.rs` | `Chaining(...,4)` |

**New Files Required**:
- `src/rules/chaining.rs` - Complete chaining engine (largest single file)
- `src/rules/chaining_hint.rs` - Chain-specific hint structures

**Implementation Notes**:
- Requires recursive implication tracking
- Needs efficient candidate state save/restore
- May require parallel processing for nested chains

**Time Estimate**: ~146 hours total (actual time may vary based on complexity)

---

## Phase 5: Variant Support

**Reference**: `Grid.java` visibility rules | **Status**: Optional

### 5.1 Core Variant Infrastructure

| # | Task | Priority | Est. | File | SE Support |
|---|------|----------|------|------|------------|
| 5.1.1 | Variant visibility trait | P0 | 4h | `grid/variant.rs` | Base infrastructure |
| 5.1.2 | Variant configuration in Grid | P0 | 2h | `grid/mod.rs` | `VariantConfig` struct |
| 5.1.3 | Variant-aware candidate rebuild | P0 | 3h | `grid/candidates.rs` | `rebuild_candidates_variant()` |

### 5.2 Region-Based Variants

| # | Variant | Difficulty | Priority | Est. | File | SE Support |
|---|---------|------------|----------|------|------|------------|
| 5.2.1 | X-Diagonal (Sudoku X) | Medium | P1 | 3h | `grid/variant.rs` | `XVisibleCellIndex` |
| 5.2.2 | Disjoint Groups (DG) | Medium | P1 | 3h | `grid/variant.rs` | `DGVisibleCellIndex` |
| 5.2.3 | Windows (Windoku) | Medium | P2 | 3h | `grid/variant.rs` | `windowsVisibleCellIndex` |
| 5.2.4 | Center Dot (CD) | Medium | P2 | 2h | `grid/variant.rs` | `CDVisibleCellIndex` |
| 5.2.5 | Asterisk | Medium | P2 | 2h | `grid/variant.rs` | `asteriskVisibleCellIndex` |
| 5.2.6 | Girandola | Medium | P2 | 2h | `grid/variant.rs` | `girandolaVisibleCellIndex` |

### 5.3 Adjacency-Based Variants

| # | Variant | Difficulty | Priority | Est. | File | SE Support |
|---|---------|------------|----------|------|------|------------|
| 5.3.1 | Toroidal Board | High | P3 | 4h | `grid/variant.rs` | `wazirCellsToroidal` |
| 5.3.2 | Non-Consecutive (NC) | Medium | P2 | 4h | `grid/variant.rs` | `wazirCellsRegular`, `lockedNC` |
| 5.3.3 | Ferz NC (Diagonal NC) | High | P3 | 4h | `grid/variant.rs` | `ferzCellsRegular`, `lockedFNC` |
| 5.3.4 | Anti-Knight | Medium | P3 | 3h | `grid/variant.rs` | `knightCellIndex` |
| 5.3.5 | Anti-King | Low | P3 | 2h | `grid/variant.rs` | `ferzCellIndex` |

### 5.4 Variant-Specific Techniques

| # | Technique | Variant | Priority | File | SE Support |
|---|-----------|---------|----------|------|------------|
| 5.4.1 | forcingCellNC | NC | P3 | `rules/variant_nc.rs` | `forcingCellNC` |
| 5.4.2 | lockedNC | NC | P3 | `rules/variant_nc.rs` | `lockedNC` |
| 5.4.3 | forcingCellFNC | Ferz NC | P3 | `rules/variant_fnc.rs` | `forcingCellFNC` |
| 5.4.4 | lockedFNC | Ferz NC | P3 | `rules/variant_fnc.rs` | `lockedFNC` |

**New Files Required**:
- `src/grid/variant.rs` - All variant visibility rules
- `src/rules/variant_nc.rs` - Non-Consecutive techniques
- `src/rules/variant_fnc.rs` - Ferz Non-Consecutive techniques

---

## Phase 6: CLI Enhancement

**Reference**: `serate.java` | **Target**: serate-compatible CLI

### 6.1 Core CLI Options

| # | Option | Format | Priority | Est. | Status | File |
|---|--------|--------|----------|------|--------|------|
| 6.1.1 | Input file | `--input=FILE` | P0 | 2h | ✅ | `main.rs` |
| 6.1.2 | Output file | `--output=FILE` | P0 | 2h | ✅ | `main.rs` |
| 6.1.3 | Format string | `--format=FORMAT` | P0 | 4h | ✅ | `main.rs` |
| 6.1.4 | Thread count | `--threads=N` | P1 | 3h | ⏳ | `main.rs` |
| 6.1.5 | Start format | `--start=FORMAT` | P1 | 2h | ⏳ | `main.rs` |
| 6.1.6 | Before format | `--before=FORMAT` | P1 | 2h | ⏳ | `main.rs` |
| 6.1.7 | After format | `--after=FORMAT` | P1 | 2h | ⏳ | `main.rs` |

### 6.2 Format Specifiers (20 total)

| Spec | Description | Priority | Status |
|------|-------------|----------|--------|
| `%d` | Diamond rating (ED) | P0 | ✅ |
| `%D` | Diamond technique name | P0 | ⏳ |
| `%e` | Elapsed time | P0 | ⏳ |
| `%g` | Input puzzle (81-char) | P0 | ✅ |
| `%h` | Step description (HTML) | P1 | ⏳ |
| `%i` | Puzzle grid (81-digit) | P0 | ⏳ |
| `%l` | Newline | P0 | ⏳ |
| `%m` | Pencilmarks (729-char) | P1 | ⏳ |
| `%M` | Pencilmarks (multi-line) | P1 | ⏳ |
| `%n` | Puzzle ordinal | P0 | ⏳ |
| `%p` | Pearl rating (EP) | P0 | ✅ |
| `%P` | Pearl technique name | P0 | ⏳ |
| `%r` | Puzzle rating (ER) | P0 | ✅ |
| `%R` | Rating technique name | P0 | ⏳ |
| `%s` | Step description (short) | P1 | ⏳ |
| `%S` | Rating technique (short) | P1 | ⏳ |
| `%t` | Tab character | P0 | ⏳ |
| `%T` | Pearl technique (short) | P1 | ⏳ |
| `%U` | Diamond technique (short) | P1 | ⏳ |
| `%%` | Literal % | P0 | ⏳ |

### 6.3 Rating Options

| # | Option | Priority | Est. | Status | Description |
|---|--------|----------|------|--------|-------------|
| 6.3.1 | `--pearl` | P1 | 1h | ⏳ | Terminate if not pearl |
| 6.3.2 | `--diamond` | P1 | 1h | ⏳ | Terminate if not diamond |
| 6.3.3 | `--revisedRating=N` | P2 | 2h | ⏳ | Revised rating scheme |
| 6.3.4 | `--batch=N` | P2 | 3h | ⏳ | Batch solving mode |

### 6.4 Variant CLI Options

| # | Option | Priority | Est. | Description |
|---|--------|----------|------|-------------|
| 6.4.1 | `--isBlocks=N` | P2 | 1h | Enable/disable blocks |
| 6.4.2 | `--isX=N` | P2 | 1h | X-Diagonal variant |
| 6.4.3 | `--isDG=N` | P2 | 1h | Disjoint Groups |
| 6.4.4 | `--isWindows=N` | P2 | 1h | Windoku variant |
| 6.4.5 | `--isAsterisk=N` | P3 | 1h | Asterisk variant |
| 6.4.6 | `--isGirandola=N` | P3 | 1h | Girandola variant |
| 6.4.7 | `--isCD=N` | P3 | 1h | Center Dot variant |
| 6.4.8 | `--isToroidal=N` | P3 | 1h | Toroidal board |
| 6.4.9 | `--isNC=N` | P3 | 2h | Non-Consecutive (0-4) |
| 6.4.10 | `--isAntiKnight=N` | P3 | 1h | Anti-Knight variant |
| 6.4.11 | `--isAntiKing=N` | P3 | 1h | Anti-King variant |

### 6.5 Advanced Options

| # | Option | Priority | Est. | Description |
|---|--------|----------|------|-------------|
| 6.5.1 | `--techs=TECHSTRING` | P3 | 4h | Technique selection |
| 6.5.2 | `--showArguments` | P1 | 1h | Show parameters |
| 6.5.3 | `--version` | P0 | 1h | Version info |
| 6.5.4 | `--html` | P2 | 2h | HTML output mode |
| 6.5.5 | `--json` | P2 | 3h | JSON output |
| 6.5.6 | `--totalTime` | P1 | 1h | Total processing time |
| 6.5.7 | `--man` | P2 | 2h | Manual/help |

**New Files Required**:
- `src/cli/format.rs` - Format string parser and formatter
- `src/cli/args.rs` - Argument parsing structure
- `src/output/json.rs` - JSON output format
- `src/output/html.rs` - HTML output format

---

## Phase 7: Generator Enhancement

**Reference**: `Generator.java`, `Symmetry.java` | **Status**: In Progress

| # | Feature | Priority | Est. | Status | File | Description |
|---|---------|----------|------|--------|------|-------------|
| 7.1 | Symmetry types enum | P2 | 3h | ✅ | `generator.rs` | 8 symmetry types |
| 7.2 | Difficulty range filter | P2 | 4h | ✅ | `generator.rs` | Min/max difficulty |
| 7.3 | Technique exclusion | P3 | 4h | ⏳ | `generator/mod.rs` | Exclude techniques |
| 7.4 | Technique inclusion | P3 | 4h | ⏳ | `generator/mod.rs` | Include techniques |
| 7.5 | Unique solution verification | P1 | 6h | ✅ | `generator.rs` | Fast verification |
| 7.6 | Multi-threaded generation | P3 | 8h | ⏳ | `generator/mod.rs` | Parallel generation |

**Implemented Symmetry Types**:
```rust
pub enum Symmetry {
    None,
    Rotational180,
    Rotational90,
    Horizontal,
    Vertical,
    DiagonalMain,
    DiagonalAnti,
    Full,  // D4 group (all 8 symmetries)
}
```

**Generator Features**:
- ✅ SukakuExplainer algorithm (6 rounds of removal)
- ✅ Difficulty-to-clue mapping (ER 1-2→30-40, ER 2-3→25-30, ER 3-5→22-26, ER 5+→17-22)
- ✅ Unique solution verification
- ✅ Symmetric puzzle generation
- ✅ CLI integration with `generate` subcommand

---

## Phase 8: Rating System Enhancement

**Reference**: `Rule.java`, `Solver.java`, `Settings.java`

| # | Feature | Priority | Est. | File | Description |
|---|---------|----------|------|------|-------------|
| 8.1 | Short technique names | P1 | 2h | `solver/hint.rs` | `%S/%T/%U` support |
| 8.2 | Revised rating system | P2 | 4h | `rating.rs` | `revisedRating` toggle |
| 8.3 | Technique name localization | P3 | 4h | `rating.rs` | Multi-language support |
| 8.4 | Custom difficulty overrides | P3 | 2h | `rating.rs` | User-defined ratings |

---

## Architecture (Complete)

```
sukaku-rs/src/
├── lib.rs                      # Public API
├── main.rs                     # CLI entry point
├── error.rs                    # Error types
├── generator/                  # Puzzle generation
│   ├── mod.rs
│   └── symmetry.rs             # Symmetry types
├── rating.rs                   # Difficulty rating (ER/EP/ED)
├── grid/                       # Core data structures
│   ├── mod.rs
│   ├── cell.rs
│   ├── candidates.rs           # Bitmask candidate tracking
│   ├── region.rs               # Rows, Cols, Blocks
│   └── variant.rs              # Variant visibility rules
├── rules/                      # Solving techniques (59 total)
│   ├── mod.rs
│   ├── direct.rs               # Naked/Hidden Single
│   ├── locked.rs               # Pointing/Claiming
│   ├── subset.rs               # Naked/Hidden Pair/Triple/Quad
│   ├── subset_gen.rs           # Generalized Naked Sets
│   ├── fish.rs                 # X-Wing/Swordfish/Jellyfish
│   ├── wing.rs                 # XY/XYZ/WXYZ/VWXYZ/UVWXYZ/TUVWXYZ
│   ├── unique.rs               # Unique Rectangle + BUG
│   ├── strong_link.rs          # Skyscraper/Kite/Strong-Links Fish
│   ├── exclusion.rs            # Aligned Pair/Triplet Exclusion
│   ├── vlocking.rs             # Generalized Intersections
│   ├── chaining.rs             # All chain techniques
│   ├── variant_nc.rs           # Non-Consecutive techniques
│   └── variant_fnc.rs          # Ferz NC techniques
├── solver/                     # Rule-based solver
│   ├── mod.rs
│   ├── accumulator.rs          # HintAccumulator
│   └── hint.rs                 # Hint structure
├── cli/                        # CLI handling
│   ├── mod.rs
│   ├── args.rs                 # Argument parsing
│   └── format.rs               # Format string processing
└── output/                     # Output formats
    ├── mod.rs
    ├── json.rs                 # JSON output
    └── html.rs                 # HTML output
```

---

## Testing Strategy

```bash
# Run all tests
cargo test -p sukaku-rs

# Run specific phase tests
cargo test -p sukaku-rs phase2      # Intermediate techniques
cargo test -p sukaku-rs chaining    # Chain techniques
cargo test -p sukaku-rs variant     # Variant support

# Performance benchmarks
cargo bench -p sukaku-rs

# Coverage report
cargo tarpaulin -p sukaku-rs --out html
```

**Test Coverage Goals**:
- Phase 1-3: 90%+ unit test coverage
- Phase 4 (Chaining): 85%+ (complex logic)
- Phase 5 (Variant): 80%+ per variant
- CLI: Integration tests for all format specifiers

---

## Implementation Priority Matrix

```
                    High Impact
                        │
    ┌───────────────────┼───────────────────┐
    │  Phase 2          │  Phase 4          │
    │  (Intermediate)   │  (Chaining)       │
    │  SE 4.0-5.5       │  SE 6.5-10.0+     │
    │                   │                   │
    │  Priority: P0     │  Priority: P1-P3  │
    │  Impact: 95%      │  Impact: 100%     │
    ────────────────────┼───────────────────
    │  Phase 6          │  Phase 5          │
    │  (CLI)            │  (Variant)        │
    │  Compatibility    │  Optional         │
    │                   │                   │
    │  Priority: P1-P2  │  Priority: P2-P3  │
    │  Impact: UX       │  Impact: Niche    │
    └───────────────────┼───────────────────┘
                        │
                    Low Impact
```

---

## Next Steps (Immediate)

### Session 1: Phase 2 Start
```bash
# 1. Verify environment
cargo test -p sukaku-rs

# 2. Create wing.rs
touch sukaku-rs/src/rules/wing.rs

# 3. Implement XY-Wing
# Reference: SukakuExplainer/diuf/sudoku/solver/rules/XYWing.java
```

### Session 2: Phase 2 Continue
- Implement XYZ-Wing
- Add tests for Wing techniques

### Session 3: Phase 2 Complete
- Implement Unique Rectangle (Type 1-4)
- Implement BUG+1

---

## SukakuExplainer Reference

### Complete Technique Mapping (59 techniques)

| Category | Count | Techniques |
|----------|-------|------------|
| Direct | 6 | Hidden Single, Naked Single, Direct Pointing/Claiming, Direct Hidden Pair/Triplet |
| Indirect Basic | 8 | Pointing, Claiming, Naked/Hidden Pair/Triple, X-Wing, Swordfish |
| Wing | 7 | XY/XYZ/WXYZ/VWXYZ/UVWXYZ/TUVWXYZ-Wing |
| Fish | 4 | X-Wing, Swordfish, Jellyfish, Strong-Links Fish (3-6) |
| Unique | 8 | Unique Rectangle (4 types), BUG+1/2/3/4 |
| Subset | 6 | Naked/Hidden Quad, Generalized Naked Set (2-6) |
| Exclusion | 2 | Aligned Pair/Triplet Exclusion |
| Chain | 10 | X/Y-Cycles, Forcing Chains (7 types), Nested Chains (3 levels) |
| Variant | 4 | forcingCellNC, lockedNC, forcingCellFNC, lockedFNC |
| Other | 4 | Turbot Fish, Skyscraper, 2-String Kite, VLocking |

### Rating System Reference

```
ER (Experience Rating): Hardest technique required
EP (Entry Point): First technique difficulty  
ED (Entry Difficulty): First step difficulty

Difficulty Scale:
┌─────────────────────────────────────────────────────────────┐
│ 1.0-2.0  │ Basic         │ Singles, Locked Candidates       │
│ 2.0-3.0  │ Simple        │ Pairs, X-Wing                    │
│ 3.0-4.0  │ Intermediate  │ Triples, Swordfish, XY-Wing      │
│ 4.0-5.0  │ Advanced      │ Quads, Unique Rectangle, BUG     │
│ 5.0-6.0  │ Complex       │ Wings, Strong Links              │
│ 6.0-7.0  │ Expert        │ Cycles, Simple Chains            │
│ 7.0-8.0  │ Master        │ Forcing Chains, Nishio           │
│ 8.0-9.0  │ Expert+       │ Multiple Chains                  │
│ 9.0-10.0 │ Master+       │ Dynamic Chains                   │
│ 10.0+    │ Extreme       │ Nested Forcing Chains            │
└─────────────────────────────────────────────────────────────┘
```

---

## Progress Tracking

### Completed
- [x] Phase 1: Basic Techniques (11/11) ✅
- [x] Project infrastructure (Cargo, tests, CI)
- [x] Phase 2: XY-Wing, XYZ-Wing, WXYZ-Wing
- [x] Phase 2: Unique Rectangle Type 1-4
- [x] Phase 2: BUG+1
- [x] Phase 2: Naked Quad, Hidden Quad
- [x] Phase 2: Jellyfish
- [x] Phase 3: Skyscraper (SE 4.0)
- [x] Phase 3: 2-String Kite (SE 4.1)
- [x] Generator: Symmetry types (8 types)
- [x] Generator: Difficulty-to-clue mapping
- [x] Generator: Unique solution verification
- [x] CLI: generate subcommand
- [x] CLI: rate subcommand with format strings
- [x] CLI: JSON output support

### In Progress
- [ ] Phase 6: Additional format specifiers
- [ ] Phase 6: Thread count support

### Pending
- [ ] Phase 3: Advanced Techniques (2/20)
- [ ] Phase 4: Chaining System (0/10)
- [ ] Phase 5: Variant Support (1/12)
- [ ] Phase 8: Rating Enhancement (0/4)

---

## Commit History Template

```bash
# View progress
git log --oneline sukaku

# Phase-specific history
git log --oneline --grep="Phase 2" sukaku-rs/

# Technique-specific history
git log --oneline --grep="XY-Wing" sukaku-rs/
```

---

**Last Updated**: 2026-03-27
**Next Milestone**: Phase 3 Start - Skyscraper Implementation
**Final Goal**: 100% SukakuExplainer feature parity
