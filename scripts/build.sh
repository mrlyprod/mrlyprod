#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."

DATA_DIR="data/packages/rs/bindgen"
SUFFIX=dylib
[ "$(uname)" = Darwin ] || SUFFIX=so

export RUSTFLAGS="--remap-path-prefix=$HOME=~ --remap-path-prefix=$PWD=."

uv run --no-project scripts/stamp.py
cargo run -q -p mrly-bindgen

BLOBS=()
wasm=()
for dir in "$DATA_DIR"/blobs/*/; do
  blob="$(basename "$dir")"
  BLOBS+=("$blob")
  wasm+=(-p "mrly-blob-$blob")
done
cargo build -q --manifest-path "$DATA_DIR/Cargo.toml" --profile wasm --target wasm32-unknown-unknown --lib "${wasm[@]}"
cargo build -q --manifest-path "$DATA_DIR/Cargo.toml" --release --lib -p mrly-blob-all
cargo build -q --manifest-path "$DATA_DIR/Cargo.toml" --release -p mrly-cli

mkdir -p "$DATA_DIR/py"
cp "target/release/libmrly_blob_all.$SUFFIX" "$DATA_DIR/py/libmrly.$SUFFIX"
for blob in "${BLOBS[@]}"; do
  cp "target/wasm32-unknown-unknown/wasm/mrly_blob_$blob.wasm" "$DATA_DIR/js/blobs/$blob.wasm"
  printf '%s %s bytes\n' "$blob" "$(wc -c < "$DATA_DIR/js/blobs/$blob.wasm" | tr -d ' ')"
done
