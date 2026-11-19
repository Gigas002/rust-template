# Agent workflow

How an agent takes a request from one sentence to a finished, tested, audited project.
[`ARCHITECTURE.md`](../ARCHITECTURE.md) says what the code must look like; this file says
in what order to work.

## Which phases a request needs

| Request | Phases |
| ------- | ------ |
| "I want an app (or a library) that does X", in the raw template | 0 → 8, all of them |
| "I want feature X", in a project | 0, 2, 3 (a new milestone in `docs/PLAN.md`), 4, 5, 8 |
| "Fix bug X" | 0, then reproduce with a failing test, fix, gates (phase 4 rules), 8 |
| "Audit the project" | [`AUDIT.md`](AUDIT.md) |
| "Prepare a release" | [`RELEASE.md`](RELEASE.md) |
| "Continue" / a new session in the middle of work | 0, then the next unchecked step of `docs/PLAN.md` |

Work autonomously through the phases: don't stop for approval between steps. Stop and ask
only for the decisions in phase 2, when something blocks the plan, or before anything
outward-facing (§ Hard limits).

---

## 0. Orient

Before changing anything:

1. Read `AGENTS.md`, then [`ARCHITECTURE.md`](../ARCHITECTURE.md).
2. `git status` and `git log --oneline -15`: what state the work is in, uncommitted
   changes included. Don't overwrite someone's uncommitted work.
3. If `docs/PLAN.md` exists, read it: its Progress section says what's done and what's
   next. It's the memory that survives between sessions.
4. If `scripts/init.sh` exists, this is still the raw template: phase 1 comes first.
5. Run `scripts/gates.sh` once: know whether the tree is green before you start. A failure
   that predates you gets fixed in its own commit first (or reported, if it isn't yours to
   fix).

## 1. Bootstrap (raw template only)

1. Choose the project name: the user's, else a short lowercase name derived from the
   request (`imgshrink`, `todo-sync`). It becomes the binary's name and the library's
   (`lib<name>`).
2. `scripts/init.sh <name> --description "<one line>"`, adding `--lib-only` for a
   library. Check its output for warnings, and `git status`.
3. `scripts/gates.sh`, then commit: `chore: initialise <name> from the template`.

The greeting code in the skeleton is a placeholder. Replace it as the plan's first steps
implement real behaviour; delete what's left of it once nothing uses it.

## 2. Clarify

Collect the decisions that are the user's to make and ask them **once, together, up
front**, each with a recommended answer. Typical ones:

- The interface: CLI, TUI or GUI; one command or subcommands.
- Scope of the first version: what's in, what's explicitly out.
- Platforms (Linux only? Windows?), and whether it's published to crates.io.
- Anything with outside effects: which external services, accounts or files it touches.

Everything else (crate choices, module layout, file formats, defaults) is yours to decide:
choose sensibly, and record it in [`DECISIONS.md`](../DECISIONS.md). If the request is
clear enough, skip the questions and say which defaults you chose.

## 3. Plan

Write `docs/PLAN.md` from [`templates/PLAN.md`](../templates/PLAN.md), following
[`PLANNING.md`](PLANNING.md). In short:

- The goal and the user-visible interface (commands, flags, config keys, output, exit codes).
- How it maps onto the architecture: modules per crate, Cargo features, dependencies.
- Risks and unknowns, each with a spike step early in the plan.
- Milestones, each a working program, split into steps of one commit each.

Commit the plan (`docs: plan <name>`), show the user a short summary (milestones and the
decisions you made), and carry on with the first step unless they asked to review first.

## 4. Build: one step at a time

For each step of the plan:

1. **Tests first** where the behaviour is clear: write the test, see it fail.
2. **Implement**, following ARCHITECTURE.md: module folders with `tests.rs`, errors that
   name their input, nothing that panics on input, `Settings` only below the resolver.
3. **Docs in the same change**: README, `examples/` (and their tests), `CHANGELOG.md`
   under Unreleased, `DECISIONS.md` for any new dependency or design choice.
4. **Gates**: `scripts/gates.sh`. Fix everything it reports; never skip a gate.
5. **Stress** new or changed tests: `scripts/stress.sh 20`.
6. **Measure** if the step touches a hot path: `scripts/measure.py`, before and after,
   numbers in the commit message.
7. **Commit** (§ Commits), then tick the step in `docs/PLAN.md`'s Progress section, with
   the commit hash and anything the next step needs to know. Commit that update with the
   step itself, or as `docs: update plan progress`.

If a step turns out wrong or too big, change the plan first (and say why in its log),
then continue. Keep the tree green between steps: every commit passes the gates.

## 5. Verify

When a milestone is complete, use the program the way a user would:

- Build the release binary and run it in a scratch environment (`HOME` and `XDG_*`
  pointed at a temporary directory), never against the user's real config or data.
- Go through the README's usage section command by command: the output, `--help`, the
  error messages, the exit codes, piping (`| head`).
- Fix what's wrong as new steps (test first, as in phase 4).

## 6. Audit

When the plan is done, audit the whole project: [`AUDIT.md`](AUDIT.md). It ends with
`docs/AUDIT.md` committed and its findings fixed.

## 7. Release prep

Follow [`RELEASE.md`](RELEASE.md) up to, not including, the tag: version bump, changelog,
release build checked. Tag and push only when the user asks.

## 8. Report

End with a short report to the user:

- What was built (or fixed), and how to try it: the one or two commands that show it.
- What was verified and how: gates, stress runs, the scenarios run by hand, measurements
  with numbers.
- Decisions you made that they may want to revisit, and what's left open.
- The branch and commits; say plainly that nothing was pushed.

---

## Commits

- **One logical change per commit**, leaving the gates green. A fix, its test, its
  changelog line and its docs travel together.
- Messages follow Conventional Commits: `feat(scope): ...`, `fix`, `perf`, `refactor`,
  `test`, `docs`, `ci`, `chore`; `!` for breaking (`refactor!: ...`). The body says why,
  what changed, and how it was verified (and the numbers, for performance).
- Stage files by name (`git add path ...`), never `git add -A` or `git add .`. Check
  `git status` for strays first: `*.profraw`, `lcov.info`, probe files, editor files.
- Never commit secrets or files from outside the project.
- Work on a branch (`feat/<topic>`, `fix/<topic>`, `chore/audit`), not the default branch.

## Hard limits

Never, without the user explicitly asking for that specific action:

- push, tag, publish to crates.io, open or merge pull requests;
- delete branches, rewrite pushed history, force-push;
- touch anything outside the repository: the user's config files, running processes,
  real services and accounts, other repositories;
- run tests or the program against real user data or live APIs (use scratch dirs and
  mock servers, ARCHITECTURE.md §7.3).

Approval for one of these doesn't carry over to the next one.

## When things go wrong

- **A test fails intermittently**: it's a bug in the test or the code, never noise.
  Reproduce with `scripts/stress.sh`, fix the cause, or remove the test.
- **A gate fails on code you didn't touch** (new clippy lint, new advisory): fix it in a
  separate commit before your change, or report it.
- **You're unsure whether something is a bug or intended**: write down both readings,
  pick the one the docs and tests support, and mention it in the report.
- **You made a mistake** (a wrong claim, a broken commit): say so plainly in the report
  and fix it; don't hide it in a later commit.
