#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."

DATA_DIR="data/scripts/check"
RUFF="uvx ruff@0.17.2"
JS=(os apps ui designs packages/js/draw packages/js/loader packages/js/sprites collections sim sites/net sites/com sites/org sites/tv sites/store sites/sh)
SITES=(sites/net sites/com sites/org sites/tv sites/store sites/sh)
FULL=0
[ "${1:-}" = "--full" ] && FULL=1

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

mkdir -p "$DATA_DIR"
total=$SECONDS
step prose uv run scripts/prose.py README.md CONTRIBUTING.md sim/testers/
step stamp uv run --no-project scripts/stamp.py --check
if [ "$FULL" = 1 ] || [ ! -d data/packages/rs/bindgen/js/blobs ]; then step generate bash scripts/build.sh; fi
step fmt cargo fmt --all --check
step fmtcli cargo fmt --manifest-path packages/cli/Cargo.toml --check
step clippy cargo clippy --workspace --all-targets -- -D warnings
step clipcli cargo clippy --manifest-path data/packages/rs/bindgen/Cargo.toml -p mrly-cli -- -D warnings
if [ "$FULL" = 1 ]; then step examples uv run --no-project scripts/examples.py --check; fi
step ruff $RUFF check packages/py infra
step ruffmt $RUFF format --check packages/py infra
step install bun install --frozen-lockfile
step rules bun os/scripts/rules.ts
step lint bun run lint
step types each bun run types
step wasm sites bun run wasm
step build sites bun run build
step content sites bun run content
echo "check passed in $((SECONDS - total))s"
