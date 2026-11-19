# AGENTS.md

Instructions for coding agents working in this repository. Read this first, then follow
the links it gives for the task at hand.

<!-- template:start -->
## This is the raw template

`scripts/init.sh` exists, so no project has been made from this repository yet. Whatever
the user asks for ("I want an app that does X", "a library for Y"), start with
[docs/agents/WORKFLOW.md](docs/agents/WORKFLOW.md). Its first step is to ask the user which
license the project uses; then it bootstraps the project (`scripts/init.sh <name> --license
<SPDX>`), plans it, builds it step by step, audits it, and prepares the release. The crates `crate/` and `binary-crate/` are placeholders that `init.sh` renames;
their greeting code is replaced as real features land.

If the user wants to change the template itself, work on it directly: keep `init.sh`
working (initialise a copy and run `scripts/gates.sh` on it).
<!-- template:end -->

## Where to look

| You need to... | Read |
| -------------- | ---- |
| Build something from a request ("I want X") | [docs/agents/WORKFLOW.md](docs/agents/WORKFLOW.md) |
| Write or update the plan | [docs/agents/PLANNING.md](docs/agents/PLANNING.md), [docs/templates/PLAN.md](docs/templates/PLAN.md) |
| Know how code must be structured (always) | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| Know why something is the way it is | [docs/DECISIONS.md](docs/DECISIONS.md) |
| Resume work in progress | `docs/PLAN.md` (Progress section), `git log` |
| Audit the project | [docs/agents/AUDIT.md](docs/agents/AUDIT.md), [docs/templates/AUDIT.md](docs/templates/AUDIT.md) |
| Prepare a release | [docs/agents/RELEASE.md](docs/agents/RELEASE.md) |
| Find the last audit's results | `docs/AUDIT.md` |

## Commands

```sh
scripts/gates.sh                 # all quality gates (fmt, typos, deny, clippy x3, test x3, doc)
scripts/gates.sh test full       # one gate at one level: minimal | default | full
scripts/stress.sh 20             # every test binary 20x in parallel: catches flaky tests
scripts/coverage.sh              # coverage summary (cargo-llvm-cov)
scripts/measure.py -n 30 'a=CMD' 'b=CMD'   # time and peak memory, before vs after
cargo run -p <name> -- --help    # the binary
```

Tools the gates need: `typos` (`cargo install typos-cli`), `cargo-deny`, and for coverage
`cargo-llvm-cov`. Python 3 for `measure.py`.

## Rules

These hold for every change. Details and reasons are in the linked docs.

1. **Gates before every commit.** `scripts/gates.sh` must pass; never skip or weaken a gate
   to get a commit through. CI's workflows run the same commands.
2. **One logical change per commit**, with its tests, docs and CHANGELOG line, in
   Conventional Commits style. Stage files by name; never `git add -A`; check for stray
   files (`*.profraw`, `lcov.info`, probes) before committing.
3. **Architecture is not optional.** Module folders with sibling `tests.rs`, slim `main`,
   `Settings` only below the resolver, errors that name their input, no panics on input,
   optional capabilities behind Cargo features passing at all three levels
   ([ARCHITECTURE.md](docs/ARCHITECTURE.md)).
4. **Every bug fix has a regression test that fails without the fix.** Prove it by
   reverting the fix and running the test.
5. **Tests are isolated and deterministic**: their own temp dirs, never the real `HOME`,
   config, services or network; no wall-clock assertions or sleeps. New tests pass
   `scripts/stress.sh 20`; a flaky test is fixed or removed, never retried.
6. **Measure, don't guess**: performance changes come with before/after numbers from
   `scripts/measure.py`, in the commit message.
7. **Dependencies**: latest version, maintained (released within a year, not archived, no
   advisory), only needed features, justified in [DECISIONS.md](docs/DECISIONS.md).
8. **Docs move with the code**: README, `examples/` (tested), `CHANGELOG.md` under
   `## [Unreleased]` (never add a release date), `docs/PLAN.md` progress.
9. **Ask only what's the user's to decide** (scope, interface, platforms, external
   effects), once, up front, with a recommendation. Decide the rest and record it.
10. **Never without an explicit request**: push, tag, publish, open PRs, force-push,
    rewrite history, or touch anything outside the repository (user config, real data,
    running processes, live services).
11. **Report honestly**: what was verified and how, with numbers; what wasn't; anything
    that went wrong. Never claim a check you didn't run.
