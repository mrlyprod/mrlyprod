#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

font=$(mktemp)
trap 'rm -f "$font"' EXIT

rm -rf "$PWD/site/pkg"
wasm-pack build crates/mrlydemo --target web --release --out-dir "$PWD/site/pkg"
cargo run -q -p mrlyrs --example book > "$font"
install -m 644 "$font" site/kit/font/font.json
