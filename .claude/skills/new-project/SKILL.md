---
name: new-project
description: Build a new application or library from this Rust template, starting from a one-line idea ("I want an app that does X", "make me a CLI for Y", "a library to parse Z"). Bootstraps the project with scripts/init.sh, asks the few decisions that are the user's, writes docs/PLAN.md, implements it step by step with tests and quality gates, verifies it by hand, audits it, and prepares the release.
---

# New project

Follow `docs/agents/WORKFLOW.md`, phases 0 to 8, in order. Read it now, with
`docs/ARCHITECTURE.md` and `docs/agents/PLANNING.md`.

Checklist:

1. Orient: `git status`, `git log`, `scripts/gates.sh` once.
2. Bootstrap: `scripts/init.sh <name> --description "<line>"` (`--lib-only` for a
   library), gates, commit.
3. Clarify: one batch of questions, only for decisions that are the user's (interface,
   scope, platforms, external effects), each with a recommendation. Skip if the request is
   clear; record the rest in `docs/DECISIONS.md`.
4. Plan: `docs/PLAN.md` from `docs/templates/PLAN.md`; commit; summarise it to the user;
   continue.
5. Build: per step, tests first, implement, docs + CHANGELOG, `scripts/gates.sh`,
   `scripts/stress.sh 20` for new tests, measure hot paths, commit, tick Progress.
6. Verify each milestone by running the release binary in a scratch HOME.
7. Audit: `docs/agents/AUDIT.md` → `docs/AUDIT.md`.
8. Release prep: `docs/agents/RELEASE.md`, no tag, no push.
9. Report: what was built, how to try it, what was verified (with numbers), open decisions.
