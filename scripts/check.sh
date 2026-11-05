#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."

DATA_DIR="data/scripts/check"
RUFF="uvx ruff@0.17.2"
UNITS=(core num math tile life font)
JS=(os apps ui designs packages/draw sites/net)
SITES=(sites/net)

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

each() {
  local dir
  for dir in "${JS[@]}"; do
    (cd "$dir" && "$@") || { echo "failed in $dir"; return 1; }
  done
}

sites() {
  local dir
  for dir in "${SITES[@]}"; do
    (cd "$dir" && "$@") || { echo "failed in $dir"; return 1; }
  done
}

bridge() {
  local unit
  for unit in "${UNITS[@]}"; do bash scripts/bridge.sh "$unit" check || return 1; done
}

lint_py() {
  local unit
  for unit in "${UNITS[@]}"; do
    $RUFF format --check "packages/$unit/py" && $RUFF check "packages/$unit/py" || return 1
  done
}

units_wasm() {
  local unit
  for unit in "${UNITS[@]}"; do bash scripts/wasm.sh "$unit" || return 1; done
}

mkdir -p "$DATA_DIR"
total=$SECONDS
step fmt cargo fmt --all --check
step clippy cargo clippy --workspace --all-targets --all-features -- -D warnings
step bridge bridge
step ruff lint_py
step units units_wasm
step install bun install --frozen-lockfile
step rules bun os/scripts/rules.ts
step lint bun run lint
step types each bun run types
step wasm sites bun run wasm
step build sites bun run build
step content sites bun run content
echo "check passed in $((SECONDS - total))s"
