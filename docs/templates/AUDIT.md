<!--
Template for docs/AUDIT.md, the report of an audit (docs/agents/AUDIT.md).
Keep the structure and the table format; replace every <...>. The example rows show the
level of detail expected: what happened, the evidence, what was done. Delete them, and
this comment.
-->

# Audit

A whole-application audit of <name> (<YYYY-MM-DD>, from <base commit>) in five parts:
bugs (§2), performance (§3), security and robustness (§4), usability (§5), and code quality,
tests and project hygiene (§6). §7 lists what was left open, and §8 is the checklist to
re-run the audit.

Each finding says what was found, how serious it is, and what was done: **fixed** (with
the commit), **accepted** (with the reason), or **OK** (checked, nothing to do). Every fix
is its own commit, with a regression test that fails without it where one can be written.

Severity: **high** (a crash in normal use, data loss, a broken state, a security hole
reachable by normal input), **medium** (wrong behavior in a reachable case, an ignored
setting, a large waste of time or memory), **low** (hardening, polish, or needs an unusual
setup).

## 1. Method

<What was read (every module? anything only skimmed?). How findings were reproduced:
probe tests, the release binary in a scratch HOME, fixtures, mock servers. What was never
touched: real user data, live services. Builds and static checks run: the three feature
levels, clippy, cargo deny, typos, cargo llvm-cov.>

## 2. Bugs

| Area | Finding | Status |
| ---- | ------- | ------ |
| <Replace with dependencies> | <`replace` looked up each candidate's old package with `expect(...)`; a dependency has none, so replacing an addon that declared one panicked, after the old one was already removed. Dependencies are now installed alongside.> | Fixed, high (<commit>) |
| <Lock file saves> | <Written to a temporary file and renamed, so an interrupted save leaves the old file.> | OK |

## 3. Performance

| Area | Finding | Status |
| ---- | ------- | ------ |
| <Ranged reads> | <The 25 KB tail was copied into a buffer of the full asset size. For a 200 MB asset: **212.7 MB → 10.9 MB** peak RSS, **15.1 → 0.75 ms** (median of 30, scripts/measure.py).> | Fixed, medium (<commit>) |
| <Startup> | <`list` on 500 packages: **17.6 / 17.6 ms**, base / final. Unchanged.> | OK |

Not measured: <what, and why>.

## 4. Security and robustness

| Area | Finding | Status |
| ---- | ------- | ------ |
| <Terminal escapes> | <...> | <...> |
| Dependencies | <`cargo deny check` result.> | <...> |

## 5. Usability

| Area | Finding | Status |
| ---- | ------- | ------ |
| <Unknown config keys> | <...> | <...> |

## 6. Code quality, tests and hygiene

| Area | Finding | Status |
| ---- | ------- | ------ |
| Coverage | <X % → Y % of lines (`scripts/coverage.sh`).> | <...> |
| Gates | <fmt, typos, deny, clippy and tests at three levels, docs: pass on every commit? Which failed at the base, until which commit?> | OK |
| Flaky tests | <`scripts/stress.sh 20` at the base and after each commit.> | OK |

## 7. Left open

- **<Topic>.** <What the problem is, the options, and why it's the user's decision or not
  worth it now.>

## 8. Re-running the audit

```sh
scripts/gates.sh
scripts/stress.sh 20
scripts/coverage.sh
<the measurement commands for §3's scenarios, with how the fixtures were made>
```
