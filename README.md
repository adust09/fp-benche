# Rust vs Haskell: Idiomatic Performance Benchmarks

Comparing performance of idiomatic Rust and Haskell across algorithms, data processing, and concurrency.

**Goal**: Measure performance when each language uses its natural, idiomatic style -- not the same algorithm forced into both, but the best-practice approach each community would use.

## Benchmarks

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

## Setup

### Prerequisites

- **Rust** >= 1.70: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **GHC** >= 9.6 + cabal: `curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh`
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
