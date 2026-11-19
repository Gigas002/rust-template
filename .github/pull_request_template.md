## What and why

<!-- What changes for the user, and why. Link the issue or the docs/PLAN.md step. -->

## Checklist

- [ ] `scripts/gates.sh` passes (fmt, typos, deny, clippy and tests at three feature levels, docs)
- [ ] New or changed tests pass `scripts/stress.sh 20`
- [ ] A bug fix has a regression test that failed before the fix
- [ ] Performance-sensitive changes have before/after numbers (`scripts/measure.py`)
- [ ] `CHANGELOG.md` has an entry under Unreleased; docs and `examples/` match the change
- [ ] New dependencies are current, maintained, and recorded in `docs/DECISIONS.md`
