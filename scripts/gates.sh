#!/usr/bin/env bash
# Quality gates (docs/ARCHITECTURE.md §8). CI runs these same steps, so a green local
# run means a green CI run. Nothing may be committed while a gate fails.
#
#   scripts/gates.sh                 every gate, every feature level
#   scripts/gates.sh STEP [LEVEL]    one gate, e.g. `clippy full` or `test minimal`
#
# Steps:  fmt typos deny clippy test doc
# Levels: minimal (--no-default-features), default, full (--all-features); clippy and
#         test run all three when LEVEL is omitted.
set -uo pipefail
cd "$(dirname "$0")/.."

export CARGO_TERM_COLOR="${CARGO_TERM_COLOR:-always}"
# In CI, Cargo.lock must already be up to date.
LOCKED=()
[ -n "${CI:-}" ] && LOCKED=(--locked)

level_flag() {
    case "$1" in
        minimal) echo "--no-default-features" ;;
        default) echo "" ;;
        full) echo "--all-features" ;;
        *) echo "unknown level '$1' (minimal, default, full)" >&2; exit 2 ;;
    esac
}

failed=()
run() {
    local name="$1"; shift
    echo "==> $name: $*"
    if "$@"; then
        echo "PASS $name"
    else
        echo "FAIL $name"
        failed+=("$name")
    fi
}

need() {
    command -v "$1" >/dev/null 2>&1 || { echo "FAIL $1 is not installed: $2" >&2; failed+=("$1"); return 1; }
}

step() {
    local step="$1" levels=("${@:2}")
    [ ${#levels[@]} -eq 0 ] && levels=(minimal default full)
    case "$step" in
        fmt) run fmt cargo fmt --all -- --check ;;
        typos) need typos "cargo install typos-cli" && run typos typos ;;
        deny) need cargo-deny "cargo install cargo-deny" && run deny cargo deny check ;;
        clippy)
            for level in "${levels[@]}"; do
                local flag; flag=$(level_flag "$level") || exit 2
                # shellcheck disable=SC2086
                run "clippy $level" cargo clippy "${LOCKED[@]}" --workspace --all-targets $flag -- -D warnings
            done ;;
        test)
            for level in "${levels[@]}"; do
                local flag; flag=$(level_flag "$level") || exit 2
                # shellcheck disable=SC2086
                run "test $level" cargo test "${LOCKED[@]}" --workspace $flag
            done ;;
        doc) RUSTDOCFLAGS="${RUSTDOCFLAGS:--D warnings}" run doc cargo doc "${LOCKED[@]}" --workspace --no-deps --all-features ;;
        *) echo "unknown step '$step' (fmt typos deny clippy test doc)" >&2; exit 2 ;;
    esac
}

if [ $# -eq 0 ]; then
    for s in fmt typos deny clippy test doc; do step "$s"; done
else
    step "$@"
fi

echo
if [ ${#failed[@]} -eq 0 ]; then
    echo "All gates passed."
else
    echo "Failed: ${failed[*]}"
    exit 1
fi
