#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
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
  [ -n "${MRLY_LEAK:-}" ] && [ -f "$MRLY_LEAK" ] || { echo "leak: no list in MRLY_LEAK, the push tool runs it"; return 0; }
  hits="$( { echo "$LEAKS"; sed '/^$/d' "$MRLY_LEAK"; } | git grep --untracked -nIE -f - -- ':!scripts/check.sh')" || code=$?
  [ "$code" -le 1 ] || { echo "git grep failed with $code" >&2; return 1; }
  [ -z "$hits" ] || { echo "$hits"; echo "private paths or token shapes in the tree" >&2; return 1; }
}

mkdir -p "$DATA_DIR"
total=$SECONDS
step leak leak
step install bun install --frozen-lockfile
step lint bun run lint
step types bun run types
step wasm bun run wasm --copy
step test bun run test
step build bun run build
step content bun run content
echo "check passed in $((SECONDS - total))s"
