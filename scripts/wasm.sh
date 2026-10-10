#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

font=$(mktemp)
trap 'rm -f "$font"' EXIT

units=$(cat pkgs/bridge/units.txt)
used=$(grep -hoE 'mrlyjs[/_][a-z]+' $(ls figures/*.ts | grep -v '\.test\.ts$') $(find site/apps -name '*.js' -o -name '*.jsx' -o -name '*.ts' | grep -v '\.test\.') | cut -c8- | sort -u)
live=$(for unit in $units; do if grep -qx "$unit" <<< "$used"; then echo "$unit"; fi; done)
built=target/wasm32-unknown-unknown/release

cargo build --release --target wasm32-unknown-unknown --lib $(printf -- '-p mrlyjs_%s ' $units)

rm -rf pkgs/mrlyjs/pkg site/pkg
for unit in $units; do
  wasm-bindgen "$built/mrlyjs_$unit.wasm" --target web --out-dir "pkgs/mrlyjs/pkg/$unit"
done
for unit in $live; do
  mkdir -p "site/pkg/$unit"
  cp "pkgs/mrlyjs/pkg/$unit/mrlyjs_$unit.js" "pkgs/mrlyjs/pkg/$unit/mrlyjs_${unit}_bg.wasm" "site/pkg/$unit/"
done
cargo run -q -p mrlyrs --example book > "$font"
install -m 644 "$font" site/kit/font/font.json

echo "unit bytes"
for unit in $units; do
  printf '%s %s\n' "$unit" "$(wc -c < "pkgs/mrlyjs/pkg/$unit/mrlyjs_${unit}_bg.wasm" | tr -d ' ')"
done
