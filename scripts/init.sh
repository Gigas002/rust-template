#!/usr/bin/env bash
# Turns the template into a named project. Run once, right after creating the repository
# from the template; it deletes itself when done.
#
#   scripts/init.sh NAME [--description TEXT] [--owner NAME] [--license SPDX]
#
# NAME          the binary's name: lowercase letters, digits and dashes (my-app).
#               binary-crate/ becomes NAME/, crate/ becomes libNAME/ (Rust path libmy_app).
# --description one line for Cargo.toml and the README.
# --owner       the copyright holder in LICENSE (default: git config user.name).
# --license     an SPDX expression: MIT (default), Apache-2.0, "MIT OR Apache-2.0",
#               GPL-3.0-only, MPL-2.0, ... Set in Cargo.toml and allowed in deny.toml
#               (cargo deny checks the project's own crates too). Anything but MIT leaves
#               LICENSE as a TODO for the license's official text.
#
# It renames the crates everywhere they're named (manifests, code, the crate lists in
# deploy.yml and test.yml), fills in Cargo.toml's description and repository and the
# LICENSE, writes a project README, removes the template-only sections of the docs, and
# refreshes Cargo.lock. It doesn't commit: review `git status` first.
#
# A library-only project: run it, then delete NAME/ and its entries in Cargo.toml,
# deploy.yml and test.yml.
set -euo pipefail
cd "$(dirname "$0")/.."

usage() { sed -n '2,22p' "$0" | sed 's/^# \{0,1\}//'; exit "${1:-0}"; }

name="" description="" owner="" license="MIT"
while [ $# -gt 0 ]; do
    case "$1" in
        --description) description="${2:?--description needs a value}"; shift 2 ;;
        --owner) owner="${2:?--owner needs a value}"; shift 2 ;;
        --license) license="${2:?--license needs a value}"; shift 2 ;;
        -h | --help) usage ;;
        -*) echo "unknown option $1" >&2; usage 2 ;;
        *) [ -z "$name" ] || { echo "one NAME only" >&2; usage 2; }; name="$1"; shift ;;
    esac
done

[ -n "$name" ] || usage 2
[[ "$name" =~ ^[a-z][a-z0-9]*(-[a-z0-9]+)*$ ]] || { echo "NAME must be lowercase letters, digits and single dashes, starting with a letter: $name" >&2; exit 2; }
case "$name" in crate | binary-crate | lib*) echo "NAME is the binary's name; the library gets the lib prefix itself: $name" >&2; exit 2 ;; esac
[ -d crate ] && [ -d binary-crate ] || { echo "already initialised (no crate/ and binary-crate/)" >&2; exit 2; }
[[ "$license" =~ ^[A-Za-z0-9.+()\ -]+$ ]] || { echo "--license must be an SPDX expression, like MIT or \"MIT OR Apache-2.0\": $license" >&2; exit 2; }
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

echo "==> crates: $lib (library), $name (binary)"
git mv crate "$lib"
git mv binary-crate "$name"

echo "==> manifests, code and workflows"
LIB="$lib" NAME="$name" perl -pi -e '
    s/^name = "crate"$/name = "$ENV{LIB}"/;
    s/^name = "binary-crate"$/name = "$ENV{NAME}"/;
    s/^library-crate = \{ package = "crate", path = "\.\.\/crate"/$ENV{LIB} = { path = "..\/$ENV{LIB}"/;
    s/"library-crate\//"$ENV{LIB}\//g;
' "$lib/Cargo.toml" "$name/Cargo.toml"
LIB="$lib" NAME="$name" perl -pi -e '
    s/^members = \["binary-crate", "crate"\]/members = ["$ENV{NAME}", "$ENV{LIB}"]/;
' Cargo.toml
LIB="$lib" NAME="$name" perl -pi -e '
    s/^  # TODO: configure your crates \(dependency order for publishing\)$/  # Crates to build, and to publish in dependency order./;
    s/^  BINARY_CRATES: .*/  BINARY_CRATES: \x27["$ENV{NAME}"]\x27/;
    s/^  PUBLISH_CRATES: .*/  PUBLISH_CRATES: \x27["$ENV{LIB}", "$ENV{NAME}"]\x27/;
' .github/workflows/deploy.yml
LIB="$lib" NAME="$name" perl -pi -e '
    s/^        # TODO: add your crates here$/        # The crates to report coverage for./;
    s/^        package: \[binary-crate, crate\]$/        package: [$ENV{NAME}, $ENV{LIB}]/;
' .github/workflows/test.yml
# Rust paths, then any other mention (docs, scripts, comments) of the template's names.
find "$lib" "$name" -name '*.rs' -print0 | xargs -0 perl -pi -e "s/\\blibrary_crate\\b/$lib_ident/g"
mapfile -t files < <(git ls-files | grep -v -e '^scripts/init.sh$' -e '^LICENSE$' | while read -r f; do
    [ -f "$f" ] && grep -Iq . "$f" 2>/dev/null && echo "$f"
done)
for f in "${files[@]}"; do
    perl -pi -e "s/\\bbinary-crate\\b/$name/g" "$f"
done

echo "==> Cargo.toml metadata"
desc_toml=${description//\\/\\\\}
desc_toml=${desc_toml//\"/\\\"}
DESC="$desc_toml" REPO="$repository" LICENSE_EXPR="$license" perl -pi -e '
    s/^description = .*/description = "$ENV{DESC}"/;
    s/^repository = .*/repository = "$ENV{REPO}"/;
    s/^license = .*/license = "$ENV{LICENSE_EXPR}"/;
' Cargo.toml

echo "==> LICENSE ($license)"
if [ "$license" = MIT ]; then
    YEAR="$year" OWNER="$owner" perl -pi -e 's/<YEAR>/$ENV{YEAR}/; s/<OWNER>/$ENV{OWNER}/' LICENSE
else
    echo "TODO: replace this file with the full text of $license, copyright $year $owner." > LICENSE
    echo "warning: LICENSE needs the official text of $license (see the SPDX license list);" >&2
    [[ "$license" == *" OR "* ]] && echo "         for a choice of licenses, the Rust convention is one file each (LICENSE-MIT, LICENSE-APACHE)." >&2
fi
# Each license in the expression (split on OR/AND; "X WITH Y" stays whole) must be allowed,
# since cargo deny checks the workspace's own crates too.
perl -e '
    my $expr = shift; $expr =~ s/[()]/ /g;
    my @ids = grep { length } map { s/^\s+|\s+$//gr } split /\s+(?:OR|AND)\s+/, $expr;
    local $/; open my $fh, "<", "deny.toml" or die; my $toml = <$fh>; close $fh;
    for my $id (@ids) {
        next if $toml =~ /^\s*"\Q$id\E",\s*$/m;
        $toml =~ s/^(allow = \[\n)/$1    "$id",\n/m or die "no allow list in deny.toml\n";
        print "    allowed in deny.toml: $id\n";
    }
    open $fh, ">", "deny.toml" or die; print $fh $toml;
' "$license"

echo "==> README.md"
cat > README.md <<EOF
# $name

$description

## Install

\`\`\`sh
cargo install --path $name
\`\`\`

## Usage

\`\`\`sh
$name --help
\`\`\`

## Development

\`\`\`sh
scripts/gates.sh      # every check CI runs: fmt, typos, deny, clippy and tests x3, docs
scripts/stress.sh 20  # every test binary 20 times in parallel
\`\`\`

[\`docs/ARCHITECTURE.md\`](docs/ARCHITECTURE.md) describes the layout and the rules.

## License

$license, see [LICENSE](LICENSE).
EOF

echo "==> template-only parts"
# Markdown sections between <!-- template:start --> and <!-- template:end -->.
for f in $(git ls-files '*.md'); do
    perl -0pi -e 's/<!-- template:start -->.*?<!-- template:end -->\n?//gs; s/\n{3,}/\n\n/g' "$f"
done
git rm -q scripts/init.sh

echo "==> Cargo.lock"
cargo update --workspace --quiet

if leftovers=$(git grep -n -E -e 'library[-_]crate|binary-crate|name = "crate"' -- . ':!scripts/init.sh'); then
    echo "warning: the template's crate names are still here:" >&2
    echo "$leftovers" >&2
fi

echo
echo "Done. Next: review \`git status\`, run scripts/gates.sh, and commit."
