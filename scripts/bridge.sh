#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
unit="${1:?usage: bridge.sh <unit> [check|bump]}"
shift
cd "$ROOT/packages/$unit"

cargo run -q -p mrly-bridge -- "$@"
