# Releasing

Releases are cut by pushing a version tag; `.github/workflows/deploy.yml` does the rest.
An agent prepares a release (steps 1–3) on its own, but tags and pushes only when the user
asks for that.

## 1. Version

Semantic versioning. Before 1.0: a breaking change (CLI, config format, library API) bumps
the minor version (`0.3.0` → `0.4.0`), anything else the patch.

The version appears in two places; change both:

```toml
# Cargo.toml
[workspace.package]
version = "0.4.0"

# <name>/Cargo.toml
[dependencies]
lib<name> = { path = "../lib<name>", version = "0.4.0" }
```

Then `cargo build` (which updates `Cargo.lock`), `scripts/gates.sh`, and commit
`chore: bump crates to 0.4.0` with `Cargo.toml` and `Cargo.lock`.

## 2. Changelog

`CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com): every change has
already added its line under `## [Unreleased]`. Before a release, read the section as a
user would: one line per change, grouped (Added, Changed, Deprecated, Removed, Fixed,
Security), breaking changes stated as such.

The heading stays `## [Unreleased]` with no date: the maintainer renames it to
`## [0.4.0] - YYYY-MM-DD` when they tag.

## 3. Check the build

```sh
scripts/gates.sh
cargo build --release --locked
cargo publish --workspace --locked --dry-run   # crates.io packaging, nothing uploaded
```

## 4. Tag (the maintainer, or an agent asked to)

```sh
git tag v0.4.0
git push origin v0.4.0
```

`deploy.yml` then:

1. Builds, strips and archives each crate in `BINARY_CRATES` (x86_64 Linux).
2. Creates the GitHub release with the archives and generated release notes.
3. Publishes each crate in `PUBLISH_CRATES`, in that order, to crates.io
   (`CARGO_REGISTRY_TOKEN`).

`workflow_dispatch` builds the archives without releasing, to try the pipeline.

## Repository settings

| Secret | Used for | Without it |
| ------ | -------- | ---------- |
| `CARGO_REGISTRY_TOKEN` | publishing to crates.io (`deploy.yml`) | the publish jobs fail after the GitHub release |
| `CODECOV_TOKEN` | uploading coverage (`test.yml`) | the upload step fails, the job doesn't |
