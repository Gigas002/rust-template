---
name: audit
description: Audit a project made from this Rust template end to end (bugs, performance, security and robustness, usability, code quality and tests), fix the findings one commit each with regression tests and before/after measurements, and write docs/AUDIT.md. Use when asked to audit, review the whole project, or harden it before a release.
---

# Audit

Follow `docs/agents/AUDIT.md` exactly; the report follows `docs/templates/AUDIT.md`.

1. Baseline: branch `chore/audit`; base worktree and release build in a scratch dir;
   gates, stress, coverage, and the performance scenarios measured on the base.
2. Read every file against the checklist in `docs/agents/AUDIT.md` §2.
3. Reproduce each suspicion (probe test or the binary in a scratch HOME); assign severity.
4. Fix most severe first, one commit each: regression test proven to fail without the fix,
   CHANGELOG, gates, stress.
5. Measure after (same scenarios, `scripts/measure.py`, interleaved with the base), coverage.
6. Write and commit `docs/AUDIT.md`; check every hash and number in it.
7. Bump the version (`docs/agents/RELEASE.md`), CHANGELOG stays Unreleased.
8. Report findings by severity with numbers and open decisions. Don't push.
