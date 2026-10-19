#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

[ -d bindings/js/units ] || { echo "wasm.sh: no bindings/js/units; run scripts/bridge.sh" >&2; exit 1; }
units=$(ls bindings/js/units)
target=$(cargo metadata --format-version 1 --no-deps | sed -n 's/.*"target_directory":"\([^"]*\)".*/\1/p')
built="$target/wasm32-unknown-unknown/wasm"
locked=$(grep -A1 '^name = "wasm-bindgen"$' Cargo.lock | sed -n 's/^version = "\(.*\)"$/\1/p')
install="cargo install wasm-bindgen-cli --version $locked --locked"
command -v wasm-bindgen > /dev/null || { echo "wasm.sh: wasm-bindgen is not installed; $install" >&2; exit 1; }
cli=$(wasm-bindgen --version | awk '{print $2}')
[ "$cli" = "$locked" ] || { echo "wasm.sh: wasm-bindgen $cli is installed but Cargo.lock wants $locked; $install" >&2; exit 1; }

cargo build --profile wasm --target wasm32-unknown-unknown --lib $(printf -- '-p mrly-js-%s ' $units)

rm -rf bindings/js/pkg
for unit in $units; do
  wasm-bindgen "$built/mrly_js_$unit.wasm" --target web --out-dir "bindings/js/pkg/$unit"
done

echo "unit bytes"
for unit in $units; do
  printf '%s %s\n' "$unit" "$(wc -c < "bindings/js/pkg/$unit/mrly_js_${unit}_bg.wasm" | tr -d ' ')"
done
