#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/check.sh" "$@"
cd "$HERE/.."

DATA_DIR="data/scripts/check"
LEAKS='/Users/|Developer/|AKIA[0-9A-Z]{16}|npm_[A-Za-z0-9]{36}|pypi-[A-Za-z0-9_-]{40,}|cio[A-Za-z0-9]{32}'

step() {
  local name="$1"; shift
  local log="$DATA_DIR/$name.log"
  local start=$SECONDS
  if "$@" > "$log" 2>&1; then
    printf 'ok   %-12s %4ss\n' "$name" "$((SECONDS - start))"
    return 0
  fi
  printf 'FAIL %-12s %4ss\n' "$name" "$((SECONDS - start))"
  tail -n 60 "$log" >&2
  echo "check stopped at $name; full log in $log" >&2
  exit 1
}

leak() {
  local hits code=0
  hits="$( { echo "$LEAKS"; [ ! -f .leak ] || sed '/^$/d' .leak; } | git grep --untracked -nIE -f - -- ':!scripts/check.sh' ':!.leak')" || code=$?
  [ "$code" -le 1 ] || { echo "git grep failed with $code" >&2; return 1; }
  [ -z "$hits" ] || { echo "$hits"; echo "private paths or token shapes in the tree" >&2; return 1; }
}

pytest_py() { uv run --no-project --with pytest --with ./bindings/py pytest bindings/py; }

mkdir -p "$DATA_DIR"
total=$SECONDS
step leak leak
step fmt cargo fmt --all --check
step bridge scripts/bridge.sh check
step clippy cargo clippy --workspace -- -D warnings
step test cargo test
step ignored cargo test -p mrly-math -- --ignored
step doc env RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --workspace
step wasm scripts/wasm.sh
step js bun --cwd bindings/js test
step node node bindings/js/node.smoke.js
step py pytest_py
step site-install bun install --cwd site --frozen-lockfile
step site-wasm bun run --cwd site wasm --no-build
step site-types bun run --cwd site types
step site-test bun --cwd site test
step site-build bun run --cwd site build
step site-check bun run --cwd site check
echo "check passed in $((SECONDS - total))s"
