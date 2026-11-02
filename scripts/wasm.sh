#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

leaves=$(for toml in crates/*/js/Cargo.toml; do basename "$(dirname "$(dirname "$toml")")"; done)
[ -n "$leaves" ] || { echo "wasm.sh: no crates/*/js; run scripts/bridge.sh" >&2; exit 1; }
target=$(cargo metadata --format-version 1 --no-deps | sed -n 's/.*"target_directory":"\([^"]*\)".*/\1/p')
built="$target/wasm32-unknown-unknown/wasm"
locked=$(grep -A1 '^name = "wasm-bindgen"$' Cargo.lock | sed -n 's/^version = "\(.*\)"$/\1/p')
install="cargo install wasm-bindgen-cli --version $locked --locked"
command -v wasm-bindgen > /dev/null || { echo "wasm.sh: wasm-bindgen is not installed; $install" >&2; exit 1; }
cli=$(wasm-bindgen --version | awk '{print $2}')
[ "$cli" = "$locked" ] || { echo "wasm.sh: wasm-bindgen $cli is installed but Cargo.lock wants $locked; $install" >&2; exit 1; }

cargo build --profile wasm --target wasm32-unknown-unknown --lib $(printf -- '-p mrly-%s-js ' $leaves)

for leaf in $leaves; do
  rm -rf "crates/$leaf/js/pkg"
  wasm-bindgen "$built/mrly_${leaf}_js.wasm" --target web --out-dir "crates/$leaf/js/pkg"
done

echo "leaf bytes"
for leaf in $leaves; do
  printf '%s %s\n' "$leaf" "$(wc -c < "crates/$leaf/js/pkg/mrly_${leaf}_js_bg.wasm" | tr -d ' ')"
done
