#!/usr/bin/env bash
# Runs every test binary N times in parallel (default 20) to catch flaky tests: shared
# temp paths, ordering assumptions, timing. Run it on any commit that adds or changes
# tests. A test that fails here is removed or rewritten, never retried
# (docs/ARCHITECTURE.md §6.4).
#
#   scripts/stress.sh [N] [cargo test flags...]     e.g. scripts/stress.sh 30 --all-features
set -uo pipefail
cd "$(dirname "$0")/.."

runs="${1:-20}"
shift || true
flags=("$@")
[ ${#flags[@]} -eq 0 ] && flags=(--all-features)

mapfile -t binaries < <(
    cargo test --workspace "${flags[@]}" --no-run --message-format=json 2>/dev/null |
        grep -E '"profile":\{[^}]*"test":true' | grep -o '"executable":"[^"]*"' | sed 's/"executable":"//; s/"$//'
)
[ ${#binaries[@]} -gt 0 ] || { echo "no test binaries built" >&2; exit 2; }

logs=$(mktemp -d)
trap 'rm -rf "$logs"' EXIT
for i in $(seq "$runs"); do
    for bin in "${binaries[@]}"; do
        ( "$bin" -q >"$logs/$i-$(basename "$bin").log" 2>&1 || touch "$logs/$i-$(basename "$bin").failed" ) &
    done
done
wait

failures=$(find "$logs" -name '*.failed' | wc -l)
if [ "$failures" -gt 0 ]; then
    echo "Failing runs:"
    for f in "$logs"/*.failed; do
        log="${f%.failed}.log"
        grep -h -E "panicked at|^test .* FAILED" "$log" | sed -E 's/ \([0-9]+\)//' | head -5
    done | sort | uniq -c
fi
echo "runs: $runs x ${#binaries[@]} binaries, failures: $failures"
[ "$failures" -eq 0 ]
