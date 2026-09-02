#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
wasm-pack build crates/mrlyweb --target web --release --out-dir "$PWD/sites/net/pkg"
cargo run -q -p mrlyfont --example book > sites/ui/font.json
cargo run -q -p mrlyfont --example cycle > sites/ui/mark.json
