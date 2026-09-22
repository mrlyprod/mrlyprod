#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/bridge.sh" "$@"
cd "$HERE/.."

cargo run -q -p bridge -- "$@"
