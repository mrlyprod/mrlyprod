#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."
export CARGO_LOCK=held
lock="${TMPDIR:-/tmp}/mrlyprod-cargo.lock"
until mkdir "$lock" 2>/dev/null; do sleep 2; done
trap 'rmdir "$lock"' EXIT
"$@"
