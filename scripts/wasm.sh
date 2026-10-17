#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

font=$(mktemp)
trap 'rm -f "$font"' EXIT

units=$(ls bindings/js/units)
used=$(grep -hoE '(@mrlyprod/mrly/|mrly_js_)[a-z]+' $(bun -e 'for (const one of (await import("./site/scripts/build.ts")).roster()) console.log(one.file)') $(find site/src -name '*.js' -o -name '*.jsx' -o -name '*.ts' | grep -v '\.test\.') | sed -E 's/^(@mrlyprod\/mrly\/|mrly_js_)//' | sort -u)
live=$(for unit in $units; do if grep -qx "$unit" <<< "$used"; then echo "$unit"; fi; done)
built=target/wasm32-unknown-unknown/release

cargo build --release --target wasm32-unknown-unknown --lib $(printf -- '-p mrly-js-%s ' $units)

rm -rf bindings/js/pkg site/wasm
for unit in $units; do
  wasm-bindgen "$built/mrly_js_$unit.wasm" --target web --out-dir "bindings/js/pkg/$unit"
done
for unit in $live; do
  mkdir -p "site/wasm/$unit"
  cp "bindings/js/pkg/$unit/mrly_js_$unit.js" "bindings/js/pkg/$unit/mrly_js_${unit}_bg.wasm" "site/wasm/$unit/"
done
cargo run -q -p mrly --example book > "$font"
install -m 644 "$font" site/src/lib/font/font.gen.json

echo "unit bytes"
for unit in $units; do
  printf '%s %s\n' "$unit" "$(wc -c < "bindings/js/pkg/$unit/mrly_js_${unit}_bg.wasm" | tr -d ' ')"
done
