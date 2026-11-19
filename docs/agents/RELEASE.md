# Releasing

Releases are cut by pushing a version tag; `.github/workflows/release.yml` does the rest.
An agent prepares a release (steps 1–3) on its own, but tags and pushes only when the user
asks for that.

## 1. Version

Semantic versioning. Before 1.0: a breaking change (CLI, config format, library API) bumps
the minor version (`0.3.0` → `0.4.0`), anything else the patch.

The version appears in two places in the root `Cargo.toml`; change both:

```toml
[workspace.package]
version = "0.4.0"

[workspace.dependencies]
lib<name> = { path = "lib<name>", version = "0.4.0", default-features = false }
```

Then `cargo build` (which updates `Cargo.lock`), `scripts/gates.sh`, and commit
`chore: bump crates to 0.4.0` with `Cargo.toml` and `Cargo.lock`.

## 2. Changelog

`CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com): every change has
already added its line under `## [Unreleased]`. Before a release, read the section as a
user would: one line per change, grouped (Added, Changed, Deprecated, Removed, Fixed,
Security), breaking changes stated as such.

The heading stays `## [Unreleased]` with no date: the maintainer renames it to
`## [0.4.0] - YYYY-MM-DD` when they tag. The release workflow uses the section matching
the version, and falls back to Unreleased.

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

The workflow then:

1. Runs CI on the tag.
2. Fails if the tag isn't `v` + the crates' version.
3. Builds every binary for Linux (x86_64, arm64), macOS (arm64) and Windows (x86_64),
   archived with README, LICENSE, CHANGELOG and `examples/`, plus SHA-256 checksums.
4. Creates the GitHub release with the changelog section as notes.
5. Publishes every crate to crates.io in dependency order, if the `CARGO_REGISTRY_TOKEN`
   secret is set. A crate with `publish = false` stays off crates.io.

`workflow_dispatch` on the Release workflow builds everything without releasing, to try
the pipeline.

## Repository settings

| Secret | Used for | Without it |
| ------ | -------- | ---------- |
| `CARGO_REGISTRY_TOKEN` | publishing to crates.io | the release stops at GitHub |
| `CODECOV_TOKEN` | uploading coverage | coverage is shown on the CI run page only |

Branch protection: require the **CI passed** check.
