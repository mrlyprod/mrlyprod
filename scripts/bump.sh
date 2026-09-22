#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
SOURCE="$HERE/../pkgs/mrlyrs/Cargo.toml"
USAGE="usage: bump.sh <patch|minor|major|X.Y.Z>"

[ $# -eq 1 ] || { echo "$USAGE" >&2; exit 1; }
old="$(sed -n 's/^version = "\(.*\)"$/\1/p' "$SOURCE" | head -1)"
IFS=. read -r major minor patch <<< "$old"
case "$1" in
  patch) new="$major.$minor.$((patch + 1))" ;;
  minor) new="$major.$((minor + 1)).0" ;;
  major) new="$((major + 1)).0.0" ;;
  *) [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "$USAGE" >&2; exit 1; }; new="$1" ;;
esac

text="$(awk -v new="$new" '!done && /^version = "/ { print "version = \"" new "\""; done = 1; next } { print }' "$SOURCE")"
printf '%s\n' "$text" > "$SOURCE"
"$HERE/bridge.sh" > /dev/null
echo "mrlyrs, mrlypy and mrlyjs $old -> $new"
