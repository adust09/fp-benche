# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A functional programming study project comparing idiomatic Rust and Haskell across algorithms, data processing, and concurrency benchmarks. Also includes an Elixir FP curriculum track (on `feature/elixir-fp-curriculum`). The Rust side additionally compares imperative vs functional style within the same language.

## Build & Run Commands

### Full Setup (from repo root)
```bash
./setup.sh      # Check deps, generate test data, build Rust + Haskell
./run-all.sh    # Run all hyperfine cross-language benchmarks
```

### Rust (from `rust/`)
```bash
cargo build --release          # Build all binaries
cargo bench                    # Run all criterion benchmarks
cargo bench --bench algorithms # Run a single benchmark suite (algorithms | data_processing | concurrency | functional_comparison)
```

### Haskell (from `haskell/`)
```bash
cabal build all    # Build all executables + library
cabal bench        # Run criterion benchmarks
```

### Test Data Generation
```bash
python3 data/generate-test-data.py   # Generates large_text.txt, users_100k.json, sales_100k.csv (seed=42)
```

### Running Individual Binaries
```bash
# Rust
rust/target/release/fibonacci --n 35 --mode naive
rust/target/release/word_count --input data/large_text.txt

# Haskell (resolve path first)
cabal -v0 list-bin fibonacci --project-dir=haskell
$(cabal -v0 list-bin fibonacci --project-dir=haskell) --n 35 --mode naive +RTS -N1 -RTS
```

## Prerequisites

Rust >= 1.70, GHC >= 9.6 + cabal, Python 3, hyperfine, GNU time (`/usr/bin/time -v`).

## Architecture

Benchmark code lives at the repository root:

- **`rust/src/algorithms/`**, **`rust/src/data_processing/`**, **`rust/src/concurrency/`** — Library modules with benchmark implementations. Each domain has standard (imperative) and `_fp` (functional-style) variants (e.g., `merge_sort.rs` vs `merge_sort_fp.rs`).
- **`rust/src/bin/`** — CLI entry points wrapping library functions (clap-based). These are what `run-all.sh` and hyperfine invoke.
- **`rust/benches/`** — Criterion benchmark harnesses. `functional_comparison.rs` specifically compares imperative vs FP style within Rust.
- **`haskell/src/`** — Library modules mirroring the Rust structure (`Algorithms.*`, `DataProcessing.*`, `Concurrency.*`).
- **`haskell/app/`** — CLI entry points (optparse-applicative). Named differently from Rust: `merge-sort` (hyphen) vs `merge_sort` (underscore).
- **`haskell/bench/Main.hs`** — Criterion benchmarks for Haskell.
- **`data/`** — Generated test data (gitignored). `generate-test-data.py` is the source of truth.
- **`fixtures/`** — Small checked-in test data for development/testing without full data generation.
- **`results/`** — Hyperfine output (gitignored except `.gitkeep`).

## Key Conventions

- Rust release builds use aggressive optimization: `lto = true`, `codegen-units = 1`, `opt-level = 3`.
- Haskell builds use `-O2` and `-threaded -rtsopts` for all executables.
- Haskell RTS flags (`+RTS -N1 -RTS`) control thread count per benchmark in `run-all.sh`.
- Analysis docs are written in Japanese (see `rust/functional-comparison-results.md`).
- Three measurement layers: criterion (in-language micro), hyperfine (cross-language CLI), GNU time (peak RSS memory).

## Branch Structure

- `main` — Base branch
- `feature/elixir-fp-curriculum` — Default upstream branch, Elixir curriculum track
- `feat/rust-haskell-benchmarks` — Rust vs Haskell benchmark suite
