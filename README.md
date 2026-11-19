# rust-template

A starting point for Rust projects that a coding agent can take from one sentence to a
tested, audited, releasable program, following the same architecture and quality gates
every time.

Clone it, open your agent, and say what you want:

> I want a CLI that resizes every image in a folder.

The agent reads [`AGENTS.md`](AGENTS.md), sees this is the raw template, and follows
[`docs/agents/WORKFLOW.md`](docs/agents/WORKFLOW.md): it names and initialises the project
(`scripts/init.sh`), asks the few questions that are yours to answer, writes a step-by-step
plan to `docs/PLAN.md`, builds it one gated commit at a time, runs it by hand, audits it,
and prepares the release. It never pushes or publishes unless you ask.

## What's inside

| Part | What it gives you |
| ---- | ----------------- |
| `libskel/`, `skel/` | A library and a binary that already follow the architecture: CLI → config file → settings resolver, logging, exit codes, a feature flag, unit tests in sibling `tests.rs`, integration tests on the real binary in a scratch HOME. `init.sh` renames them. |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | The rules: layout, settings flow, errors, testing (isolation, no wall clock), robustness checklist, performance, dependencies, gates. |
| [`docs/agents/`](docs/agents) | How an agent works here: the workflow, planning, the audit procedure, releasing. |
| [`docs/templates/`](docs/templates) | Skeletons for `docs/PLAN.md` and the audit report `docs/AUDIT.md`. |
| [`docs/DECISIONS.md`](docs/DECISIONS.md) | Why each choice was made; new dependencies get an entry. |
| `AGENTS.md`, `CLAUDE.md`, `.claude/` | Entry points for agents; Claude Code skills (`new-project`, `feature`, `fix`, `audit`, `release`) and a permission allowlist that denies `git push` and `cargo publish`. |
| `scripts/` | `gates.sh` (every gate, as CI runs it), `stress.sh` (flaky-test hunter), `coverage.sh`, `measure.py` (time and peak memory, before vs after), `init.sh`. |
| `.github/` | CI (the gates at three feature levels, stress, coverage, one required check), tag-driven releases (multi-platform binaries, changelog notes, crates.io), daily advisory checks, grouped Dependabot updates, a PR checklist. |

## Starting a project by hand

```sh
scripts/init.sh my-app --description "Resizes images"   # or: NAME --lib-only
scripts/gates.sh
git add -A && git commit -m "chore: initialise my-app from the template"
```

`init.sh` renames the crates (`libmy-app`, `my-app`), fills in `Cargo.toml`, `LICENSE` and
`README.md`, strips the template-only parts, and deletes itself.

## Requirements

- Rust stable with rustfmt and clippy (`rust-toolchain.toml`).
- [`typos`](https://github.com/crate-ci/typos) and
  [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) for the gates;
  [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) for coverage.
- Python 3 for `scripts/measure.py`; Perl for `scripts/init.sh`.

## Repository settings for a project

| Secret | For |
| ------ | --- |
| `CARGO_REGISTRY_TOKEN` | publishing to crates.io on a tag (skipped without it) |
| `CODECOV_TOKEN` | coverage upload (the summary is on the CI page without it) |

Require the **CI passed** check in branch protection.

## License

MIT, see [LICENSE](LICENSE).
