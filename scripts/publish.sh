#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/publish.sh" "$@"
cd "$HERE/.."

USAGE="usage: publish.sh rs [--dry]"
VERSION="$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -1)"
TAG="v$VERSION"

targets=""
dry=""
for word in "$@"; do
  case "$word" in
    rs) case " $targets " in *" $word "*) ;; *) targets="$targets $word" ;; esac ;;
    py|js) echo "publish.sh: $word ships per leaf from crates/<unit>/$word; its release step is not written yet" >&2; exit 1 ;;
    --dry) dry=1 ;;
    *) echo "$USAGE" >&2; exit 1 ;;
  esac
done
[ -n "$targets" ] || { echo "$USAGE" >&2; exit 1; }

# SHELL

step() {
  local name="$1"; shift
  echo "== $name"
  "$@" || { echo "publish stopped at $name" >&2; exit 1; }
}

secret() {
  local name="$1"
  [ -n "${!name:-}" ] || [ ! -f .env ] || printf -v "$name" '%s' "$(. ./.env > /dev/null 2>&1; printf '%s' "${!name:-}")"
  [ -n "${!name:-}" ] || { echo "publish needs $name in the environment or .env" >&2; return 1; }
}

tokens() {
  local target
  [ -z "$dry" ] || return 0
  for target in $targets; do
    case "$target" in
      rs) secret CARGO_REGISTRY_TOKEN ;;
    esac || return 1
  done
}

# GATES

remote() {
  git fetch --tags origin main || { echo "git fetch --tags origin main failed; a local tag may differ from origin" >&2; return 1; }
  git merge-base --is-ancestor HEAD origin/main && return 0
  echo "HEAD is not on origin/main; push main first" >&2
  return 1
}

clean() {
  local dirty
  dirty="$(git status --porcelain)"
  [ -n "$dirty" ] || return 0
  echo "$dirty"
  echo "the tree is dirty; commit or rerun the generator" >&2
  return 1
}

tagged() {
  git rev-parse -q --verify "refs/tags/$TAG" > /dev/null || return 0
  [ "$(git rev-parse "$TAG^{commit}")" = "$(git rev-parse HEAD)" ] && return 0
  echo "$TAG is at another commit; every target of $VERSION ships from it" >&2
  return 1
}

release() {
  git rev-parse -q --verify "refs/tags/$TAG" > /dev/null || git tag -a "$TAG" -m "mrly $VERSION" || return 1
  git push -q origin "$TAG" && return 0
  echo "$VERSION is published but $TAG is not pushed; run git push origin $TAG" >&2
  return 1
}

# RS

rs() {
  if [ -n "$dry" ]; then
    step "cargo publish --dry-run" cargo publish --workspace --dry-run
    return 0
  fi
  step "cargo publish" env CARGO_REGISTRY_TOKEN="$CARGO_REGISTRY_TOKEN" cargo publish --workspace
}

# MAIN

step "tokens" tokens
[ -n "$dry" ] || step "remote" remote
step "bridge" scripts/bridge.sh
step "clean" clean
step "semver" scripts/bridge.sh bump
step "tag" tagged
step "check" scripts/check.sh
step "still clean" clean
for target in $targets; do
  "$target"
  [ -n "$dry" ] || step "tag $TAG" release
done
