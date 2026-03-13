#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "=== Checking dependencies ==="

MISSING=0

check_cmd() {
    if ! command -v "$1" &>/dev/null; then
        echo "MISSING: $1 — $2"
        MISSING=1
    else
        echo "OK: $1 ($("$1" --version 2>&1 | head -1))"
    fi
}

check_cmd rustc "Install via: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
check_cmd cargo "Installed with rustup (see above)"
check_cmd ghc "Install via: curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh"
check_cmd cabal "Installed with ghcup (see above)"
check_cmd python3 "Install via: sudo apt install python3 (Debian/Ubuntu) or brew install python3 (macOS)"

if ! command -v hyperfine &>/dev/null; then
    echo "MISSING: hyperfine — Install via: sudo apt install hyperfine (Debian) or cargo install hyperfine"
    MISSING=1
else
    echo "OK: hyperfine ($(hyperfine --version))"
fi

# Check /usr/bin/time -v support (GNU time, not shell builtin)
if command -v /usr/bin/time &>/dev/null; then
    echo "OK: /usr/bin/time (GNU time for memory measurement)"
elif command -v gtime &>/dev/null; then
    echo "OK: gtime (GNU time for macOS)"
else
    echo "MISSING: GNU time — Install via: sudo apt install time (Debian) or brew install gnu-time (macOS)"
    MISSING=1
fi

if [ $MISSING -ne 0 ]; then
    echo ""
    echo "ERROR: Some dependencies are missing. Install them and re-run this script."
    exit 1
fi

echo ""
echo "=== Generating test data ==="
python3 "$SCRIPT_DIR/data/generate-test-data.py"

echo ""
echo "=== Building Rust benchmarks ==="
cd "$SCRIPT_DIR/rust"
cargo build --release

echo ""
echo "=== Building Haskell benchmarks ==="
cd "$SCRIPT_DIR/haskell"
cabal update
cabal build all

echo ""
echo "=== Setup complete ==="
echo "Run './run-all.sh' to execute benchmarks."
