#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."
CARGO_BUILD_JOBS=4 wasm-pack build crates/mrlyweb --target web --release --out-dir "$PWD/demos/pkg"
