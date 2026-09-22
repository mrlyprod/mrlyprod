#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

font=$(mktemp)
trap 'rm -f "$font"' EXIT

units=$(cat bridge/units.txt)

rm -rf "$PWD/pkgs/mrlyjs/pkg"
for unit in $units; do
  crate="pkgs/mrlyjs/units/$unit"
  out="$PWD/pkgs/mrlyjs/pkg/$unit"
  wasm-pack build "$crate" --target web --release --out-dir "$out"
  rm -f "$out/.gitignore" "$out/package.json" "$out/README.md" "$out/LICENSE"
done

rm -rf "$PWD/site/pkg"
wasm-pack build site/demos/logic --target web --release --out-dir "$PWD/site/pkg"
cargo run -q -p mrlyrs --example book > "$font"
install -m 644 "$font" site/kit/font/font.json

echo "unit bytes"
for unit in $units; do
  printf '%s %s\n' "$unit" "$(wc -c < "pkgs/mrlyjs/pkg/$unit/mrlyjs_${unit}_bg.wasm" | tr -d ' ')"
done
printf 'demos %s\n' "$(wc -c < site/pkg/demos_bg.wasm | tr -d ' ')"
