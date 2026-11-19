---
name: new-project
description: Build a new application or library from this Rust template, starting from a one-line idea, asking the license first ("I want an app that does X", "make me a CLI for Y", "a library to parse Z"). Bootstraps the project with scripts/init.sh, asks the few decisions that are the user's, writes docs/PLAN.md, implements it step by step with tests and quality gates, verifies it by hand, audits it, and prepares the release.
---

# New project

Follow `docs/agents/WORKFLOW.md`, phases 0 to 8, in order. Read it now, with
`docs/ARCHITECTURE.md` and `docs/agents/PLANNING.md`.

Checklist:

1. Orient: `git status`, `git log`, `scripts/gates.sh` once.
2. License: ask the user first (MIT, `MIT OR Apache-2.0`, Apache-2.0, MPL-2.0,
   GPL-3.0-only, or any SPDX expression) and wait for the answer; never pick one yourself.
3. Bootstrap: `scripts/init.sh <name> --description "<line>" --license "<SPDX>"` (for a
   library only, then delete `<name>/` and its entries); for anything but MIT, put the
   license's official text in `LICENSE`; gates, commit.
4. Clarify: one batch of questions, only for decisions that are the user's (interface,
   scope, platforms, external effects), each with a recommendation. Skip if the request is
   clear; record the rest in `docs/DECISIONS.md`.
5. Plan: `docs/PLAN.md` from `docs/templates/PLAN.md`; commit; summarise it to the user;
   continue.
6. Build: per step, tests first, implement, docs + CHANGELOG, `scripts/gates.sh`,
   `scripts/stress.sh 20` for new tests, measure hot paths, commit, tick Progress.
7. Verify each milestone by running the release binary in a scratch HOME.
8. Audit: `docs/agents/AUDIT.md` → `docs/AUDIT.md`.
9. Release prep: `docs/agents/RELEASE.md`, no tag, no push.
10. Report: what was built, how to try it, what was verified (with numbers), open decisions.
