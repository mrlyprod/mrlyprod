#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."
[ -z "${CARGO_LOCK:-}" ] || exec "$@"
export CARGO_LOCK=held
lock="${TMPDIR:-/tmp}/mrlyprod-cargo.lock"

stale() {
  local holder
  holder="$(cat "$lock/pid" 2>/dev/null || true)"
  if [ -z "$holder" ]; then
    [ -n "$(find "$lock" -maxdepth 0 -mmin +1 2>/dev/null)" ]
    return
  fi
  ! kill -0 "$holder" 2>/dev/null
}

told=""
until mkdir "$lock" 2>/dev/null; do
  if stale; then
    echo "cargo.sh: reclaiming a stale lock at $lock" >&2
    rm -rf "$lock"
    continue
  fi
  [ -n "$told" ] || { echo "cargo.sh: waiting for pid $(cat "$lock/pid" 2>/dev/null || echo '?') to release $lock" >&2; told=1; }
  sleep 2
done
echo $$ > "$lock/pid"
trap 'rm -rf "$lock"' EXIT
"$@"
