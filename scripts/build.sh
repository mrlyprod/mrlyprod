#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."

DATA_DIR="data/packages/rs/bridge"
UNITS=(core num math tile life font)
SUFFIX=dylib
[ "$(uname)" = Darwin ] || SUFFIX=so

export RUSTFLAGS="--remap-path-prefix=$HOME=~ --remap-path-prefix=$PWD=."

cargo run -q -p mrly-bridge

wasm=()
for unit in "${UNITS[@]}"; do wasm+=(-p "mrly-$unit-doors"); done
cargo build -q --manifest-path "$DATA_DIR/Cargo.toml" --profile wasm --target wasm32-unknown-unknown --lib "${wasm[@]}"
cargo build -q --manifest-path "$DATA_DIR/Cargo.toml" --release --lib "${wasm[@]}"
cargo build -q --manifest-path "$DATA_DIR/Cargo.toml" --release -p mrly-cli

mkdir -p "$DATA_DIR/py"
for unit in "${UNITS[@]}"; do
  cp "target/wasm32-unknown-unknown/wasm/mrly_${unit}_doors.wasm" "$DATA_DIR/js/$unit/$unit.wasm"
  cp "target/release/libmrly_${unit}_doors.$SUFFIX" "$DATA_DIR/py/libmrly_$unit.$SUFFIX"
  printf '%s %s bytes\n' "$unit" "$(wc -c < "$DATA_DIR/js/$unit/$unit.wasm" | tr -d ' ')"
done
