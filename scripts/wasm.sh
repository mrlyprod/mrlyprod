#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

font=$(mktemp)
trap 'rm -f "$font"' EXIT

units=$(cat bridge/units.txt)
built=target/wasm32-unknown-unknown/release

cargo build --release --target wasm32-unknown-unknown --lib -p demos $(printf -- '-p mrlyjs_%s ' $units)

rm -rf pkgs/mrlyjs/pkg site/pkg
for unit in $units; do
  wasm-bindgen "$built/mrlyjs_$unit.wasm" --target web --out-dir "pkgs/mrlyjs/pkg/$unit"
done
wasm-bindgen "$built/demos.wasm" --target web --out-dir site/pkg
cargo run -q -p mrlyrs --example book > "$font"
install -m 644 "$font" site/kit/font/font.json

echo "unit bytes"
for unit in $units; do
  printf '%s %s\n' "$unit" "$(wc -c < "pkgs/mrlyjs/pkg/$unit/mrlyjs_${unit}_bg.wasm" | tr -d ' ')"
done
printf 'demos %s\n' "$(wc -c < site/pkg/demos_bg.wasm | tr -d ' ')"
