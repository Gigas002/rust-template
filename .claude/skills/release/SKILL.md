---
name: release
description: Prepare a release of a project made from this Rust template - version bump, changelog review, release build and crates.io dry run - and, only when the user explicitly asks, tag it so .github/workflows/release.yml publishes it.
---

# Release

Follow `docs/agents/RELEASE.md`.

1. Pick the version (semver; before 1.0 a breaking change bumps the minor).
2. Bump both places in the root `Cargo.toml`, `cargo build`, gates, commit
   `chore: bump crates to X.Y.Z`.
3. Review `CHANGELOG.md`'s `## [Unreleased]` section as a user would; keep the heading
   (no date: the maintainer sets it).
4. `cargo build --release --locked`, `cargo publish --workspace --locked --dry-run`.
5. Tag (`git tag vX.Y.Z`) and push the tag **only if the user asked for it in this
   conversation**. Otherwise give them the two commands.
