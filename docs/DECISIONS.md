# Decisions

Why the project is the way it is. One entry per decision: dependencies above all
(ARCHITECTURE.md §10.2), but also layouts, formats and trade-offs a newcomer would
question. Newest last. When a decision is reversed, add a new entry and point the old one
to it; don't delete history.

Format:

```markdown
## <YYYY-MM-DD> <short title>

**Decision**: <what>. **Why**: <the reason, the constraint>. **Alternatives**: <what else
was considered, and why not>. **Cost**: <weight, binary size, lock-in>, if any.
```

---

## Template defaults

These came with the template; revisit any that doesn't fit the project.

### Workspace: a library and a binary

**Decision**: domain logic in `lib<name>`, the program in `<name>`. **Why**: the library
can be tested without a process, reused, and published on its own; the boundary keeps
CLI and config concerns out of the logic (ARCHITECTURE.md §14). **Alternatives**: a single
binary crate; fine for a small tool, but splitting later is harder than starting split.

### Quality gates in one script

**Decision**: `scripts/gates.sh` runs locally the commands the lint, typos, deny, test and
doc workflows run in CI. **Why**: one command before committing instead of nine, with a
PASS/FAIL summary. When a workflow's command changes, change the script too.
**Alternatives**: `just` or `cargo xtask` (one more tool, or a crate to build first).

### Python for measurements

**Decision**: `scripts/measure.py` (Python 3 standard library). **Why**: peak memory needs
`wait4`, which a shell can't reach portably and `/usr/bin/time` isn't always installed.
**Alternatives**: `hyperfine` (no memory numbers; not always installed).
