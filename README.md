# rust-template

Template for rust projects. Configures:

- typos checks via `.typos.toml`: run `typos .`
- MIT `LICENSE` file (`<YEAR>` and `<OWNER>` to fill in)
- `deny.toml` with permissive licenses, advisories, wildcard and source bans: run `cargo deny check`
- ready to use `CHANGELOG` file
- minimalistic workspace-level `Cargo.toml`
- basic `.gitignore` file
- empty `examples/` directory for config/theme/etc examples
- `docs/ARCHITECTURE.md` (structure and rules) and `docs/DECISIONS.md` (why things are the way they are)
- weekly dependabot scans via `.github/dependabot.yml`
- CI/CD jobs handled by GitHub Actions via `.github/workflows`
    - three build jobs: minimal (`--no-default-features`), normal, full (`--all-features`) via `build.yml`
    - deny check job via `deny.yml`, on every push and daily
    - automated deploy to GitHub releases and crates.io on new tag pushes
    - automated crates.io `doc` build checks via `doc.yml`
    - formatters checks: fmt, clippy-minimal (`--no-default-features`), clippy-normal, clippy-full (`--all-features`) via `lint.yml`
    - typos check via `typos.yml`
    - automated tests checks with `codecov` coverage reports:  via `test.yml`

## Scripts

- `scripts/init.sh NAME [--description TEXT] [--owner NAME] [--license SPDX]`: names a new project in one step
  (see below)
- `scripts/gates.sh`: every check the workflows run, locally (fmt, typos, deny, clippy and
  tests at three feature levels, docs); `scripts/gates.sh clippy full` runs one of them
- `scripts/stress.sh 20`: every test binary 20 times in parallel, to catch flaky tests
- `scripts/coverage.sh`: coverage summary via `cargo-llvm-cov`
- `scripts/measure.py`: wall time and peak memory of commands, before vs after

Tools: `typos-cli`, `cargo-deny`, `cargo-llvm-cov` (coverage), Python 3 (`measure.py`).

## Requires manual changes

Run `scripts/init.sh NAME` once after creating a repository from the template: `binary-crate/`
becomes `NAME/`, `crate/` becomes `libNAME/`, and the crate lists, Cargo metadata, LICENSE
and README follow. It doesn't commit; review `git status`. Or look for `TODO` markers and
replace them with your data by hand:

- `test.yml`: specify projects for `codecov` job
- `deploy.yml`: binary and library crates paths to build and publish

## Required environment variables

- `CARGO_REGISTRY_TOKEN` for `deploy` job to crates.io
- `CODECOV_TOKEN` for code coverage reports on `test` job

## Coding agents (optional)

`AGENTS.md`, `CLAUDE.md`, `.claude/` and `docs/agents/` describe how coding agents should
work in a project made from this template: plan, build, audit, release. Nothing else
depends on them; delete them if you don't use agents.
