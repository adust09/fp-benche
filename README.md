# Functional Programming: Study & Benchmarks

A multi-language functional programming study project with three pillars:

1. **Rust vs Haskell Benchmarks** — Cross-language idiomatic performance comparison
2. **Rust Imperative vs Functional** — Same-language style comparison (imperative / FP / rustica PersistentVector)
3. **Elixir FP Curriculum** — Hands-on lessons covering core functional programming concepts

## Project Structure

```
.
├── rust/                        # Rust benchmarks (imperative, FP, rustica variants)
│   ├── src/algorithms/          # Fibonacci, Merge Sort, Sieve
│   ├── src/data_processing/     # Word Count, JSON Filter, Group By
│   ├── src/concurrency/         # Parallel Map, Channel Throughput
│   ├── src/bin/                  # CLI entry points (clap)
│   └── benches/                 # Criterion benchmarks
├── haskell/                     # Haskell benchmarks
│   ├── src/                     # Library modules (Algorithms, DataProcessing, Concurrency)
│   ├── app/                     # CLI entry points (optparse-applicative)
│   └── bench/                   # Criterion benchmarks
├── 01_basics.exs                # Elixir Lesson 1: Immutability & Data Types
├── 02_pattern_matching.exs      # Elixir Lesson 2: Pattern Matching & Guards
├── 03_higher_order.exs          # Elixir Lesson 3: Higher-order Functions
├── 04_pipe_and_compose.exs      # Elixir Lesson 4: Pipe Operator & Composition
├── 05_recursion.exs             # Elixir Lesson 5: Recursion & TCO
├── docs/                        # Analysis documents
├── data/                        # Generated test data (gitignored)
├── fixtures/                    # Small test data for development
├── results/                     # Benchmark results (gitignored)
├── setup.sh                     # Dependency check + build
└── run-all.sh                   # Run all hyperfine benchmarks
```

### Branch Structure

| Branch | Description |
|--------|-------------|
| `main` | Base branch |
| `feature/elixir-fp-curriculum` | Default upstream. Elixir curriculum track |
| `feat/rust-haskell-benchmarks` | Rust vs Haskell benchmark suite |

## Rust vs Haskell Benchmarks

Cross-language comparison where each language uses its natural, idiomatic style — not the same algorithm forced into both, but the best-practice approach each community would use.

### Algorithms

| Benchmark | Rust | Haskell |
|-----------|------|---------|
| Fibonacci (naive) | Recursive, no memo | Recursive, no memo |
| Fibonacci (optimized) | Iterator + fold | Lazy infinite list (zipWith) |
| Merge Sort | Recursive on `Vec<i64>` | Recursive on `Data.Vector Int` |
| Sieve of Eratosthenes | Mutable `Vec<u8>` | ST monad + `MVector Word8` |

### Data Processing

| Benchmark | Rust | Haskell |
|-----------|------|---------|
| Word Count | BufReader + HashMap | Strict ByteString + HashMap |
| JSON Filter | serde_json | aeson |
| Group-by Aggregation | HashMap + iterators | HashMap.Strict + foldl' |

### Concurrency

| Benchmark | Rust | Haskell |
|-----------|------|---------|
| Parallel Map | rayon (work-stealing) | parallel Strategies (sparks) |
| Channel Throughput | crossbeam bounded channel | STM TBQueue |

## Rust: Imperative vs Functional Style

Each benchmark has three variants within Rust to compare programming styles:

| Variant | Description |
|---------|-------------|
| **Standard (imperative)** | Mutable state, loops, in-place mutation |
| **FP** | Iterators, closures, immutable transformations |
| **Rustica** | Persistent data structures via [rustica](https://crates.io/crates/rustica) `PersistentVector` |

Benchmark suites: `cargo bench --bench functional_comparison` / `cargo bench --bench rustica_comparison`

Analysis (Japanese):
- [`rust/functional-comparison-results.md`](rust/functional-comparison-results.md)
- [`rust/rustica-comparison-results.md`](rust/rustica-comparison-results.md)

## Elixir FP Curriculum

Self-contained lessons that teach core functional programming concepts through Elixir. Each file is a runnable script with exercises.

| Lesson | Topic | Key Concepts |
|--------|-------|-------------|
| `01_basics.exs` | Immutability & Data Types | Atoms, Tuples, Lists, Variable Binding |
| `02_pattern_matching.exs` | Pattern Matching | Destructuring, Guards, Case Statements |
| `03_higher_order.exs` | Higher-order Functions | First-class Functions, Anonymous Functions |
| `04_pipe_and_compose.exs` | Pipe & Composition | `\|>` Operator, Data Transformation Pipelines |
| `05_recursion.exs` | Recursion | Tail-call Optimization, Accumulators |

```bash
# Run a lesson
elixir 01_basics.exs
```

## Documentation

Analysis documents covering concurrency patterns, memory design, and persistence strategies:

| Document | Topic |
|----------|-------|
| [`actor_model_in_rust.md`](docs/actor_model_in_rust.md) | Actor pattern implementation in Rust |
| [`ethlambda_actor_pattern_analysis.md`](docs/ethlambda_actor_pattern_analysis.md) | Actor model pattern analysis |
| [`memory_design_elixir_vs_rust.md`](docs/memory_design_elixir_vs_rust.md) | Memory management comparison (Elixir vs Rust) |
| [`elixir_ets.md`](docs/elixir_ets.md) | Erlang Term Storage (ETS) — concurrent key-value store |
| [`rocksdb.md`](docs/rocksdb.md) | RocksDB integration & persistent storage |

## Setup

### Prerequisites

- **Rust** >= 1.70: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **GHC** >= 9.6 + cabal: `curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh`
- **Elixir** >= 1.14: `brew install elixir` (macOS) or [elixir-lang.org/install](https://elixir-lang.org/install.html)
- **hyperfine**: `sudo apt install hyperfine` (Debian) or `cargo install hyperfine`
- **Python 3**: for test data generation
- **GNU time**: `sudo apt install time` (Debian) or `brew install gnu-time` (macOS)

### Quick Start

```bash
./setup.sh      # Check deps, generate data, build both projects
./run-all.sh    # Run all hyperfine comparisons
```

### In-language benchmarks (criterion)

```bash
# Rust
cd rust && cargo bench

# Haskell
cd haskell && cabal bench
```

## Measurement

Three layers of measurement for accuracy:

1. **criterion** (in-language): Hot-loop micro-benchmarks with statistical rigor (100+ samples, outlier detection)
2. **hyperfine** (cross-language): End-to-end CLI execution including startup time
3. **GNU time** (`/usr/bin/time -v`): Peak RSS memory measurement (unified across both languages)

## Results

After running `./run-all.sh`, results are saved to `results/`:
- `*.json`: Machine-readable hyperfine output
- `*.md`: Markdown tables for each benchmark
