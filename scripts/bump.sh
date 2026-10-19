#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/bump.sh" "$@"
SOURCE="$HERE/../Cargo.toml"
USAGE="usage: bump.sh <patch|minor|major|X.Y.Z>"
SEMVER='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'

[ $# -eq 1 ] || { echo "$USAGE" >&2; exit 1; }
old="$(sed -n 's/^version = "\(.*\)"$/\1/p' "$SOURCE" | head -1)"
[[ "$old" =~ $SEMVER ]] || { echo "bump.sh: the Cargo.toml version '$old' is not X.Y.Z" >&2; exit 1; }
IFS=. read -r major minor patch <<< "$old"
case "$1" in
  patch) new="$major.$minor.$((patch + 1))" ;;
  minor) new="$major.$((minor + 1)).0" ;;
  major) new="$((major + 1)).0.0" ;;
  *) [[ "$1" =~ $SEMVER ]] || { echo "$USAGE" >&2; exit 1; }; new="$1" ;;
esac
IFS=. read -r to_major to_minor to_patch <<< "$new"
(( to_major > major || (to_major == major && (to_minor > minor || (to_minor == minor && to_patch > patch))) )) || { echo "bump.sh: $new is not above $old" >&2; exit 1; }

text="$(awk -v old="$old" -v new="$new" '
  !done && /^version = "/ { print "version = \"" new "\""; done = 1; next }
  !/path = "/ { print; next }
  { pin = "\"=" old "\""; out = ""; rest = $0; while ((at = index(rest, pin)) > 0) { out = out substr(rest, 1, at - 1) "\"=" new "\""; rest = substr(rest, at + length(pin)) } print out rest }
' "$SOURCE")"
printf '%s\n' "$text" > "$SOURCE"
"$HERE/bridge.sh" > /dev/null || { echo "bump.sh: Cargo.toml is at $new but the bridge failed; fix it and rerun scripts/bridge.sh" >&2; exit 1; }
echo "mrly $old -> $new"
