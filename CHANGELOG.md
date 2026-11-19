# Changelog

All notable changes to this project are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com), and the project uses
[Semantic Versioning](https://semver.org).

## [Unreleased]
<!-- template:start -->

Changes to the template itself; `scripts/init.sh` removes them from new projects.

### Added

- A skeleton workspace (`libskel`, `skel`) that follows `docs/ARCHITECTURE.md`: settings
  resolver, TOML config with unknown keys denied, logging, exit codes, a feature flag,
  unit and integration tests.
- `scripts/init.sh` to turn the template into a named project.
- `scripts/gates.sh`, `stress.sh`, `coverage.sh` and `measure.py`.
- Agent docs: `AGENTS.md`, `CLAUDE.md`, `docs/agents/` (workflow, planning, audit,
  release), `docs/templates/` (plan, audit report), `docs/DECISIONS.md`, Claude Code
  skills.
- CI on `gates.sh` with stress and coverage jobs; tag-driven releases with binaries for
  four targets; daily advisory checks; a template CI that initialises and gates a copy.

### Changed

- `docs/ARCHITECTURE.md` rewritten: errors, robustness, performance, documentation,
  versioning, and the testing rules (isolation, determinism, stress) learned from audits.
- `deny.toml` checks advisories, bans and sources, not only licenses, and allows the
  common permissive licenses.
- `LICENSE` holds the MIT text.

### Removed

- The seven per-check workflows and their `TODO` edits.
<!-- template:end -->
