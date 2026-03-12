#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RESULTS_DIR="$SCRIPT_DIR/results"
RUST_BIN="$SCRIPT_DIR/rust/target/release"
DATA_DIR="$SCRIPT_DIR/data"

mkdir -p "$RESULTS_DIR"

# Detect GNU time
if command -v /usr/bin/time &>/dev/null; then
    GNU_TIME="/usr/bin/time"
elif command -v gtime &>/dev/null; then
    GNU_TIME="gtime"
else
    echo "WARNING: GNU time not found. Memory measurement will be skipped."
    GNU_TIME=""
fi

# Resolve Haskell binary paths
resolve_hs_bin() {
    local name="$1"
    cabal -v0 list-bin "$name" --project-dir="$SCRIPT_DIR/haskell" 2>/dev/null || echo ""
}

echo "=== Building ==="
echo "Building Rust..."
(cd "$SCRIPT_DIR/rust" && cargo build --release --quiet)

echo "Building Haskell..."
(cd "$SCRIPT_DIR/haskell" && cabal build all --quiet 2>/dev/null)

echo ""
echo "=== Running Benchmarks ==="
echo ""

# --- Algorithm Benchmarks (steady-state: large enough to dominate startup) ---

echo "--- Fibonacci (naive, n=35) ---"
hyperfine --warmup 3 --min-runs 10 \
    --export-json "$RESULTS_DIR/fib_naive_35.json" \
    --export-markdown "$RESULTS_DIR/fib_naive_35.md" \
    -n "Rust" "$RUST_BIN/fibonacci --n 35 --mode naive" \
    -n "Haskell" "$(resolve_hs_bin fibonacci) --n 35 --mode naive +RTS -N1 -RTS"

echo ""
echo "--- Fibonacci (optimized, n=1000000) ---"
hyperfine --warmup 3 --min-runs 10 \
    --export-json "$RESULTS_DIR/fib_iter_1m.json" \
    --export-markdown "$RESULTS_DIR/fib_iter_1m.md" \
    -n "Rust" "$RUST_BIN/fibonacci --n 1000000 --mode iter" \
    -n "Haskell" "$(resolve_hs_bin fibonacci) --n 1000000 --mode iter +RTS -N1 -RTS"

echo ""
echo "--- Merge Sort (n=1000000) ---"
hyperfine --warmup 3 --min-runs 5 \
    --export-json "$RESULTS_DIR/merge_sort_1m.json" \
    --export-markdown "$RESULTS_DIR/merge_sort_1m.md" \
    -n "Rust" "$RUST_BIN/merge_sort --n 1000000" \
    -n "Haskell" "$(resolve_hs_bin merge-sort) --n 1000000 +RTS -N1 -RTS"

echo ""
echo "--- Sieve (n=10000000) ---"
hyperfine --warmup 3 --min-runs 10 \
    --export-json "$RESULTS_DIR/sieve_10m.json" \
    --export-markdown "$RESULTS_DIR/sieve_10m.md" \
    -n "Rust" "$RUST_BIN/sieve --n 10000000" \
    -n "Haskell" "$(resolve_hs_bin sieve) --n 10000000 +RTS -N1 -RTS"

# --- Data Processing Benchmarks ---

echo ""
echo "--- Word Count (10MB text) ---"
if [ -f "$DATA_DIR/large_text.txt" ]; then
    hyperfine --warmup 2 --min-runs 5 \
        --export-json "$RESULTS_DIR/word_count.json" \
        --export-markdown "$RESULTS_DIR/word_count.md" \
        -n "Rust" "$RUST_BIN/word_count --input $DATA_DIR/large_text.txt" \
        -n "Haskell" "$(resolve_hs_bin word-count) --input $DATA_DIR/large_text.txt +RTS -N1 -RTS"
else
    echo "SKIP: $DATA_DIR/large_text.txt not found. Run setup.sh first."
fi

echo ""
echo "--- JSON Filter (100K users) ---"
if [ -f "$DATA_DIR/users_100k.json" ]; then
    hyperfine --warmup 2 --min-runs 5 \
        --export-json "$RESULTS_DIR/json_filter.json" \
        --export-markdown "$RESULTS_DIR/json_filter.md" \
        -n "Rust" "$RUST_BIN/json_filter --input $DATA_DIR/users_100k.json" \
        -n "Haskell" "$(resolve_hs_bin json-filter) --input $DATA_DIR/users_100k.json +RTS -N1 -RTS"
else
    echo "SKIP: $DATA_DIR/users_100k.json not found. Run setup.sh first."
fi

echo ""
echo "--- Group By (100K sales) ---"
if [ -f "$DATA_DIR/sales_100k.csv" ]; then
    hyperfine --warmup 2 --min-runs 5 \
        --export-json "$RESULTS_DIR/group_by.json" \
        --export-markdown "$RESULTS_DIR/group_by.md" \
        -n "Rust" "$RUST_BIN/group_by --input $DATA_DIR/sales_100k.csv" \
        -n "Haskell" "$(resolve_hs_bin group-by) --input $DATA_DIR/sales_100k.csv +RTS -N1 -RTS"
else
    echo "SKIP: $DATA_DIR/sales_100k.csv not found. Run setup.sh first."
fi

# --- Concurrency Benchmarks ---

CORES=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

echo ""
echo "--- Parallel Map (n=1000000, ${CORES} threads) ---"
hyperfine --warmup 3 --min-runs 5 \
    --export-json "$RESULTS_DIR/parallel_map.json" \
    --export-markdown "$RESULTS_DIR/parallel_map.md" \
    -n "Rust" "$RUST_BIN/parallel_map --n 1000000 --threads $CORES" \
    -n "Haskell" "$(resolve_hs_bin parallel-map) --n 1000000 +RTS -N${CORES} -RTS"

echo ""
echo "--- Channel Throughput (n=1000000) ---"
hyperfine --warmup 3 --min-runs 5 \
    --export-json "$RESULTS_DIR/channel_throughput.json" \
    --export-markdown "$RESULTS_DIR/channel_throughput.md" \
    -n "Rust" "$RUST_BIN/channel_throughput --n 1000000" \
    -n "Haskell" "$(resolve_hs_bin channel-throughput) --n 1000000 +RTS -N2 -RTS"

echo ""
echo "=== All benchmarks complete ==="
echo "Results saved to: $RESULTS_DIR/"
echo ""
echo "View individual results: cat $RESULTS_DIR/*.md"
