#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

font=$(mktemp)
trap 'rm -f "$font"' EXIT

units=""
if [ -f bridge/units.txt ]; then
  units=$(cat bridge/units.txt)
else
  for manifest in pkgs/mrlyjs/units/*/Cargo.toml; do
    [ -f "$manifest" ] || continue
    units="$units $(basename "$(dirname "$manifest")")"
  done
fi

rm -rf "$PWD/pkgs/mrlyjs/pkg"
for unit in $units; do
  crate="pkgs/mrlyjs/units/$unit"
  [ -f "$crate/Cargo.toml" ] || continue
  out="$PWD/pkgs/mrlyjs/pkg/$unit"
  wasm-pack build "$crate" --target web --release --out-dir "$out"
  rm -f "$out/.gitignore" "$out/package.json" "$out/README.md" "$out/LICENSE"
done

rm -rf "$PWD/site/pkg"
wasm-pack build site/demos/logic --target web --release --out-dir "$PWD/site/pkg"
cargo run -q -p mrlyrs --example book > "$font"
install -m 644 "$font" site/kit/font/font.json
