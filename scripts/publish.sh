#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/publish.sh" "$@"
cd "$HERE/.."

USAGE="usage: publish.sh <rs|py|js>... [--dry]"
DATA_DIR="data/scripts/publish"
MANIFEST=bindings/py/Cargo.toml
WHEELS="$DATA_DIR/wheels"
VERSION="$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -1)"
TAG="v$VERSION"

targets=""
dry=""
for word in "$@"; do
  case "$word" in
    rs|py|js) case " $targets " in *" $word "*) ;; *) targets="$targets $word" ;; esac ;;
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
      py) secret UV_PUBLISH_USERNAME && secret UV_PUBLISH_PASSWORD ;;
      js) secret NPM_CONFIG_TOKEN ;;
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

# PY

wheel() { uvx maturin build --release -m "$MANIFEST" -o "$WHEELS" "$@"; }

smoke() {
  local venv="$DATA_DIR/venv"
  rm -rf "$venv"
  uv venv -q "$venv" || return 1
  uv pip install -q -p "$venv/bin/python" --find-links "$WHEELS" "mrly==$VERSION" || return 1
  "$venv/bin/python" -I -c 'import sys, mrly; sys.exit(mrly.__version__ != sys.argv[1])' "$VERSION"
}

uv_publish() { UV_PUBLISH_USERNAME="$UV_PUBLISH_USERNAME" UV_PUBLISH_PASSWORD="$UV_PUBLISH_PASSWORD" uv publish "$WHEELS"/*; }

py() {
  rm -rf "$WHEELS"
  step "wheel host" wheel
  step "wheel linux x86_64" wheel --zig --target x86_64-unknown-linux-gnu
  step "wheel linux aarch64" wheel --zig --target aarch64-unknown-linux-gnu
  step "sdist" uvx maturin sdist -m "$MANIFEST" -o "$WHEELS"
  step "wheel imports" smoke
  if [ -n "$dry" ]; then
    step "uv publish skipped" ls -1 "$WHEELS"
    return 0
  fi
  step "uv publish" uv_publish
}

# JS

bun_publish() {
  local token="$1"; shift
  (cd bindings/js && NPM_CONFIG_TOKEN="$token" bun publish "$@")
}

js() {
  if [ -n "$dry" ]; then
    step "bun publish --dry-run" bun_publish dry --dry-run
    return 0
  fi
  step "bun publish" bun_publish "$NPM_CONFIG_TOKEN"
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
