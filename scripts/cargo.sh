#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
mkdir -p data
lock=data/cargo.lock
until mkdir "$lock" 2>/dev/null; do sleep 2; done
trap 'rmdir "$lock"' EXIT
"$@"
