#!/usr/bin/env bash
# Turns the raw template into a named project. Run once, right after cloning; it deletes
# itself when done (docs/agents/WORKFLOW.md, phase 1).
#
#   scripts/init.sh NAME [--description TEXT] [--owner NAME] [--lib-only]
#
# NAME          the project and binary name: lowercase letters, digits and dashes (my-app).
#               The library crate becomes libNAME (Rust path libmy_app).
# --description one line for Cargo.toml and the README.
# --owner       the copyright holder in LICENSE (default: git config user.name).
# --lib-only    drop the binary crate: the project is a library.
#
# It renames the crates, fills in Cargo.toml, LICENSE and README.md, removes the
# template-only parts (this script, the template CI workflow, the "template" sections of
# the docs), and refreshes Cargo.lock. It doesn't commit: review `git status` first.
set -euo pipefail
cd "$(dirname "$0")/.."

usage() { sed -n '2,16p' "$0" | sed 's/^# \{0,1\}//'; exit "${1:-0}"; }

name="" description="" owner="" lib_only=false
while [ $# -gt 0 ]; do
    case "$1" in
        --description) description="${2:?--description needs a value}"; shift 2 ;;
        --owner) owner="${2:?--owner needs a value}"; shift 2 ;;
        --lib-only) lib_only=true; shift ;;
        -h | --help) usage ;;
        -*) echo "unknown option $1" >&2; usage 2 ;;
        *) [ -z "$name" ] || { echo "one NAME only" >&2; usage 2; }; name="$1"; shift ;;
    esac
done

[ -n "$name" ] || usage 2
[[ "$name" =~ ^[a-z][a-z0-9]*(-[a-z0-9]+)*$ ]] || { echo "NAME must be lowercase letters, digits and single dashes, starting with a letter: $name" >&2; exit 2; }
case "$name" in skel | lib*) echo "NAME is the binary's name; the library gets the lib prefix itself (and skel is the placeholder): $name" >&2; exit 2 ;; esac
[ -d libskel ] && [ -d skel ] || { echo "this repository is already initialised (no libskel/ and skel/)" >&2; exit 2; }
command -v perl >/dev/null || { echo "perl is needed" >&2; exit 2; }

lib="lib$name"
lib_ident="lib${name//-/_}"
description="${description:-$name}"
owner="${owner:-$(git config user.name 2>/dev/null || true)}"
owner="${owner:-The $name authors}"
year=$(date +%Y)
repository=""
if remote=$(git remote get-url origin 2>/dev/null); then
    repository=$(sed -E 's#^git@([^:]+):#https://\1/#; s#\.git$##' <<<"$remote")
    # Still pointing at the template itself: leave it for the user to set.
    [[ "$repository" == */rust-template ]] && repository=""
fi

echo "==> crates: $lib (library)$($lib_only || echo ", $name (binary)")"
git mv libskel "$lib"
if $lib_only; then
    git rm -rq skel examples/config.toml
else
    git mv skel "$name"
fi

echo "==> names in files"
# Every tracked text file except this script; Rust paths use the underscore form.
mapfile -t files < <(git ls-files | grep -v -e '^scripts/init.sh$' -e '^LICENSE$' | while read -r f; do
    [ -f "$f" ] && grep -Iq . "$f" 2>/dev/null && echo "$f"
done)
for f in "${files[@]}"; do
    case "$f" in
        *.rs) perl -pi -e "s/\\blibskel\\b/$lib_ident/g; s/\\bCARGO_BIN_EXE_skel\\b/CARGO_BIN_EXE_$name/g; s/\\bskel\\b/$name/g" "$f" ;;
        *) perl -pi -e "s/\\blibskel\\b/$lib/g; s/\\bskel\\b/$name/g" "$f" ;;
    esac
done

echo "==> Cargo.toml"
desc_toml=${description//\\/\\\\}
desc_toml=${desc_toml//\"/\\\"}
DESC="$desc_toml" REPO="$repository" perl -pi -e '
    s/^description = .*/description = "$ENV{DESC}"/;
    s/^repository = .*/repository = "$ENV{REPO}"/;
' Cargo.toml
if $lib_only; then
    perl -0pi -e "s/members = \\[\"$lib\", \"$name\"\\]/members = [\"$lib\"]/" Cargo.toml
    # The binary's dependencies.
    perl -ni -e 'print unless /^(clap|dirs|serde|tempfile|toml|tracing-subscriber) = /' Cargo.toml
fi

echo "==> LICENSE"
YEAR="$year" OWNER="$owner" perl -pi -e 's/<YEAR>/$ENV{YEAR}/; s/<OWNER>/$ENV{OWNER}/' LICENSE

echo "==> README.md"
{
    echo "# $name"
    echo
    echo "$description"
    echo
    if $lib_only; then
        cat <<EOF
## Usage

\`\`\`toml
[dependencies]
$lib = "0.1"
\`\`\`

EOF
    else
        cat <<EOF
## Install

\`\`\`sh
cargo install --path $name
\`\`\`

## Usage

\`\`\`sh
$name --help
\`\`\`

## Configuration

\`$name\` reads \`~/.config/$name/config.toml\` (or the file given with \`--config\`).
[\`examples/config.toml\`](examples/config.toml) lists every key with its default; the
command line overrides the file.

EOF
    fi
    cat <<EOF
## Development

\`\`\`sh
scripts/gates.sh      # every quality gate, as CI runs them
scripts/stress.sh 20  # every test binary 20 times in parallel
\`\`\`

[\`docs/ARCHITECTURE.md\`](docs/ARCHITECTURE.md) describes the layout and the rules;
[\`AGENTS.md\`](AGENTS.md) is the entry point for coding agents.

## License

MIT, see [LICENSE](LICENSE).
EOF
} > README.md

echo "==> template-only parts"
# Markdown sections between <!-- template:start --> and <!-- template:end -->.
for f in $(git ls-files '*.md'); do
    perl -0pi -e 's/<!-- template:start -->.*?<!-- template:end -->\n?//gs; s/\n{3,}/\n\n/g' "$f"
done
git rm -q --ignore-unmatch .github/workflows/template.yml
git rm -q scripts/init.sh

echo "==> Cargo.lock"
cargo update --workspace --quiet

if leftovers=$(git grep -n -E -e 'libskel|(^|[^a-zA-Z])skel([^a-zA-Z]|$)' -- . ':!scripts/init.sh'); then
    echo "warning: the template's placeholder name is still here:" >&2
    echo "$leftovers" >&2
fi

echo
echo "Done. Next: review \`git status\`, run scripts/gates.sh, and commit."
