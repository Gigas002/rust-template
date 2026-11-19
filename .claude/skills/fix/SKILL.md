---
name: fix
description: Fix a bug in a project made from this Rust template ("X crashes when...", "Y prints the wrong thing", a failing test or CI job). Reproduces it with a test that fails, fixes it, proves the test fails without the fix, and commits with gates passing.
---

# Fix

Rules: `docs/ARCHITECTURE.md` §6–§8 and `docs/agents/WORKFLOW.md` (Commits, Hard limits).

1. Orient: `git status`, `git log`; `scripts/gates.sh` to see the starting state.
2. Reproduce: a test in the affected module's `tests.rs` (or `tests/` for the binary) that
   fails the way the bug report says. If it can't be reproduced, say so and stop.
3. Fix the cause, not the symptom. Check the same mistake elsewhere
   (ARCHITECTURE.md §8 lists the usual suspects).
4. Prove the test: `git stash push -- <fixed files>`, test fails; `git stash pop`, passes.
5. CHANGELOG line under `## [Unreleased]` → Fixed; docs if they described the bug.
6. `scripts/gates.sh`, `scripts/stress.sh 20`, commit `fix(scope): ...` saying what
   happened, why, and how it was verified.
7. Report. Don't push.
