#!/usr/bin/env bash
# Line and region coverage of the whole workspace, all features (cargo-llvm-cov).
#
#   scripts/coverage.sh                   summary table
#   scripts/coverage.sh --lcov FILE       also write an lcov report (CI uploads it)
#
# On distributions whose Rust has no llvm-tools component (Arch's `rust` package), point
# cargo-llvm-cov at the system LLVM: LLVM_COV=/usr/bin/llvm-cov LLVM_PROFDATA=/usr/bin/llvm-profdata
set -euo pipefail
cd "$(dirname "$0")/.."

command -v cargo-llvm-cov >/dev/null || { echo "cargo-llvm-cov is not installed: cargo install cargo-llvm-cov" >&2; exit 2; }

if [ "${1:-}" = "--lcov" ]; then
    cargo llvm-cov --workspace --all-features --lcov --output-path "${2:?--lcov needs a file}"
    cargo llvm-cov report --summary-only
else
    cargo llvm-cov --workspace --all-features --summary-only
fi
