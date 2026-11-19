---
name: feature
description: Add a feature or change to a project made from this Rust template ("I want feature X", "add a --json flag", "support Windows"). Plans it as a milestone in docs/PLAN.md, implements it in gated, tested, one-change commits, updates docs and the changelog, and verifies it by running the program.
---

# Feature

Follow `docs/agents/WORKFLOW.md` phases 0, 2, 3, 4, 5 and 8. Read it, with
`docs/ARCHITECTURE.md`.

1. Orient: `git status`, `git log`, `docs/PLAN.md` if present, `scripts/gates.sh` once.
2. Clarify only what changes what the user gets; otherwise decide and record in
   `docs/DECISIONS.md`.
3. Add a milestone to `docs/PLAN.md` (create it from `docs/templates/PLAN.md` if missing):
   interface changes, modules touched, Cargo feature if the capability is optional, steps.
4. Build step by step: tests first, implement, README/examples/CHANGELOG, gates, stress new
   tests, measure if it touches a hot path, commit, tick Progress.
5. Verify by running the release binary in a scratch HOME, as the README says to use it.
6. Report: the change, how to try it, what was verified. Don't push.
