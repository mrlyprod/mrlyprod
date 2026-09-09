#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

font=$(mktemp)
trap 'rm -f "$font"' EXIT

rm -rf "$PWD/sites/net/pkg"
wasm-pack build crates/mrlydemo --target web --release --out-dir "$PWD/sites/net/pkg"
cargo run -q -p mrlyfont --example book > "$font"
install -m 644 "$font" sites/kit/ui/font.json
