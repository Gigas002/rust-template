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

### clap (derive) for the command line

**Decision**: `clap` with the `derive` feature. **Why**: the standard; generates `--help`,
`--version`, value validation and shell-friendly errors; `debug_assert` catches definition
mistakes in a test. **Alternatives**: `lexopt`, `pico-args` (smaller, but everything by
hand), `argh`. **Cost**: the largest dependency of the skeleton's binary.

### TOML config with serde, unknown keys denied

**Decision**: `serde` + `toml`, `deny_unknown_fields` on every config struct. **Why**: TOML
is the Rust ecosystem's config format; denying unknown keys turns a typo into an error that
names the key instead of a setting that silently does nothing (found by audits of several
projects). **Alternatives**: accepting unknown keys for forward compatibility; not worth
the silent failures for a user-edited file.

### dirs for the config location

**Decision**: `dirs::config_dir()`. **Why**: the platform's convention on Linux (XDG),
macOS and Windows without code per platform. **Alternatives**: reading `XDG_CONFIG_HOME`
and `HOME` by hand (Linux only).

### tracing for logging

**Decision**: `tracing` in both crates, `tracing-subscriber`'s `fmt` layer to stderr in the
binary, level from `Settings`. **Why**: structured fields and spans; libraries emit
without choosing an output. No `env-filter` feature: the level comes from the settings, and
the feature pulls in a regex engine. **Alternatives**: `log` + `env_logger` (no spans).

### thiserror for errors

**Decision**: one `thiserror` enum per crate. **Why**: typed errors a caller can match on,
with messages that name the input; no boxing. **Alternatives**: `anyhow` in the binary
(easier, but the exit-code mapping in `main` wants to match on variants).

### tempfile for tests

**Decision**: `tempfile` as a dev-dependency. **Why**: every test gets its own directory,
removed afterwards (ARCHITECTURE.md §7.3).

### Quality gates in one script

**Decision**: `scripts/gates.sh` holds the gate commands; CI calls it step by step.
**Why**: the documented gates and CI's jobs drifted apart when they were written twice.
**Alternatives**: `just` or `cargo xtask` (one more tool, or a crate to build first).

### Python for measurements

**Decision**: `scripts/measure.py` (Python 3 standard library). **Why**: peak memory needs
`wait4`, which a shell can't reach portably and `/usr/bin/time` isn't always installed.
**Alternatives**: `hyperfine` (no memory numbers; not always installed).
