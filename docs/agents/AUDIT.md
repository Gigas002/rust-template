# Auditing a project

A whole-project audit: find the bugs, performance problems, security and robustness gaps,
usability problems and code-quality debt; fix what should be fixed, one commit each, with
before/after evidence; and write the report as `docs/AUDIT.md`, from
[`templates/AUDIT.md`](../templates/AUDIT.md).

An audit changes code only through findings: every commit on the audit branch fixes
something the report lists.

## 1. Baseline

Before reading or changing anything, record where you start. Every "before" number comes
from here.

```sh
git switch -c chore/audit
base=$(git rev-parse --short HEAD)
scratch=$(mktemp -d)                          # outside the repository
git worktree add "$scratch/base" "$base"      # the untouched code, for comparisons
(cd "$scratch/base" && cargo build --release)

scripts/gates.sh 2>&1 | tail -20              # failing gates are findings
scripts/stress.sh 20                          # flaky tests are findings
scripts/coverage.sh | tail -1                 # the coverage baseline
```

Then define the **performance scenarios**: the three to five things users do most, and
anything that handles large input. Each scenario is a command line run against a fixture
you generate (in the scratch dir, never real user data), measured on the base binary with
`scripts/measure.py`.

## 2. Read everything

Read every source file, the docs, the examples, the CI workflows and the manifests. For
each module, ask the questions below; note every suspicion with a file and line. The list
comes from defects that real audits of projects built from this template found.

**Bugs**
- Inputs that panic: `expect`/`unwrap`/indexing on data from outside; map lookups that
  "always succeed".
- Negated conditions: `a != x && b != y` where `!(a == x && b == y)` was meant.
- Empty or degenerate input accepted as success: an empty archive "installed", an empty
  list "processed".
- Operations that fail part-way: is the old state destroyed before the new one is
  complete?
- Text handling: a BOM, non-UTF-8 files, control characters, very long lines.

**Performance**
- Allocations or loops sized by input (`vec![0; n]` with `n` from a header, `for i in
  0..user_number`).
- Whole files loaded or copied when a stream or a slice would do; clones in loops.
- Temporary files and caches that are never cleaned; large copies into `/tmp`.

**Security and robustness**
- External text printed to the terminal unescaped.
- Paths from outside used without a containment check.
- Non-atomic writes of important files; races between processes.
- Secrets in logs or `Debug`; credentials sent to the wrong host.
- `cargo deny check`: advisories, unmaintained crates, licenses.

**Usability**
- Config keys and flags that are silently ignored when misspelled.
- Error messages that don't say what was wrong with which input.
- Ctrl-C, Esc and non-interactive use (no terminal, a pipe).
- Docs that describe behaviour the code no longer has.

**Code quality, tests, hygiene**
- Tests that touch real files, `/tmp` paths shared between tests, `HOME`, the network.
- Wall-clock assertions and sleeps; tests that fail under `stress.sh`.
- Untested modules (from the coverage report) and error paths.
- Build warnings, lints allowed without a reason, stale TODOs.

## 3. Reproduce

A suspicion becomes a finding only once reproduced:

- A **probe test** in the module's `tests.rs`, named `zz_probe_*`, that shows the problem
  (a panic, a wrong result). Probes are never committed: each becomes a regression test
  or is deleted.
- Or the **release binary** run in a scratch environment (`HOME` and `XDG_*` in a temp
  dir) on a fixture that triggers it.
- For performance: the scenario measured on the base binary.

Give each finding a severity:

- **high**: a crash in normal use, data loss, a broken install or state, a security hole
  reachable by normal input.
- **medium**: wrong behaviour in a reachable case, a setting that's ignored, a large
  waste of time or memory.
- **low**: hardening, polish, or a case that needs an unusual setup.

## 4. Fix

Most severe first, one commit per finding:

1. Turn the probe into a regression test with a comment saying what used to happen.
2. Fix the code.
3. **Prove the test**: `git stash push -- <the fixed source files>`, run the test and see
   it fail, `git stash pop`, see it pass.
4. CHANGELOG line under Unreleased (Fixed / Security / Changed); docs that described the
   old behaviour.
5. `scripts/gates.sh` and `scripts/stress.sh 20`, then commit: `fix(scope): ...` with
   what happened, why, and how it was verified (numbers for performance).

Not everything gets fixed. A finding is **accepted** when fixing it is a product decision
(which the user makes), or costs more than it's worth; the report says why. Ask the user
only when a fix would change behaviour they may rely on.

## 5. Measure after

Run the same scenarios on the final tree against the base binary, with
`scripts/measure.py`, in one invocation so the runs interleave:

```sh
scripts/measure.py -n 30 --env HOME="$scratch/home" \
  "base=$scratch/base/target/release/<name> <args>" \
  "final=target/release/<name> <args>"
scripts/coverage.sh | tail -1
```

Every performance row in the report has before and after numbers and says how they were
measured. Say what wasn't measured, and why.

## 6. Report

Write `docs/AUDIT.md` from [`templates/AUDIT.md`](../templates/AUDIT.md): method, one
table per area, every finding with its status (Fixed with the commit, Accepted with the
reason, OK), the measurements, what's left open, and how to re-run the audit. Check the
claims: every commit hash exists, every number came from a run. Commit it:
`docs: add AUDIT.md`.

## 7. Version and wrap up

1. Bump the version as [`RELEASE.md`](RELEASE.md) describes (minor before 1.0), in its own
   commit: `chore: bump crates to X.Y.Z`. The CHANGELOG keeps `## [Unreleased]`: no date.
2. `git worktree remove "$scratch/base"`; delete the scratch dir.
3. Report to the user: the findings by severity, the numbers, what's left for them to
   decide. Don't push.
