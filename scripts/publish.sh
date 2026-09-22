#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/publish.sh" "$@"
cd "$HERE/.."

USAGE="usage: publish.sh <rs|py|js> [--dry]"
MANIFEST=pkgs/mrlypy/Cargo.toml
WHEELS=target/wheels
VERSION="$(sed -n 's/^version = "\(.*\)"$/\1/p' pkgs/mrlyrs/Cargo.toml | head -1)"
TAG="v$VERSION"

target=""
dry=""
for word in "$@"; do
  case "$word" in
    rs|py|js) target="$word" ;;
    --dry) dry=1 ;;
    *) echo "$USAGE" >&2; exit 1 ;;
  esac
done
[ -n "$target" ] || { echo "$USAGE" >&2; exit 1; }

# SHELL

step() {
  local name="$1"; shift
  echo "== $name"
  "$@" || { echo "publish stopped at $name" >&2; exit 1; }
}

need() {
  [ -n "${!1:-}" ] || { echo "publish needs $1 in the environment" >&2; exit 1; }
}

tokens() {
  if [ "$target" = js ]; then need NPM_CONFIG_TOKEN; fi
  if [ -n "$dry" ]; then return 0; fi
  if [ "$target" = rs ]; then need CARGO_REGISTRY_TOKEN; fi
  if [ "$target" = py ]; then need UV_PUBLISH_USERNAME; need UV_PUBLISH_PASSWORD; fi
}

# GATES

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
  if ! git rev-parse -q --verify "refs/tags/$TAG" > /dev/null; then
    git tag -a "$TAG" -m "mrlyrs, mrlypy and mrlyjs $VERSION"
  fi
  git push origin "$TAG"
}

tests() {
  cargo test -p mrlyrs -p bridge
  uv run --no-project --with pytest --with ./pkgs/mrlypy pytest pkgs/mrlypy
  bun --cwd pkgs/mrlyjs test
}

# RS

rs() {
  if [ -n "$dry" ]; then
    step "cargo publish --dry-run" cargo publish -p mrlyrs --dry-run
    return 0
  fi
  step "cargo publish" cargo publish -p mrlyrs
}

# PY

wheel() { uvx maturin build --release -m "$MANIFEST" "$@"; }

uv_publish() { uv publish "$WHEELS"/*; }

py() {
  rm -rf "$WHEELS"
  step "wheel macos arm64" wheel
  step "wheel linux x86_64" wheel --zig --target x86_64-unknown-linux-gnu
  step "wheel linux aarch64" wheel --zig --target aarch64-unknown-linux-gnu
  step "sdist" uvx maturin sdist -m "$MANIFEST"
  if [ -n "$dry" ]; then
    step "uv publish skipped" ls -1 "$WHEELS"
    return 0
  fi
  step "uv publish" uv_publish
}

# JS

bun_publish() { (cd pkgs/mrlyjs && bun publish "$@"); }

js() {
  step "wasm" scripts/wasm.sh
  if [ -n "$dry" ]; then
    step "bun publish --dry-run" bun_publish --dry-run
    return 0
  fi
  step "bun publish" bun_publish
}

# MAIN

step "tokens" tokens
step "bridge" scripts/bridge.sh
step "clean" clean
step "bump" scripts/bridge.sh bump
step "tag" tagged
step "tests" tests
"$target"
[ -n "$dry" ] || step "tag $TAG" release
