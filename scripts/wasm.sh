#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/wasm.sh" "$@"
cd "$HERE/.."

font=$(mktemp)
manifest=$(mktemp)
trap 'rm -f "$font" "$manifest"' EXIT

rm -rf "$PWD/sites/net/pkg"
wasm-pack build crates/mrlydemo --target web --release --out-dir "$PWD/sites/net/pkg"
cargo run -q -p mrlyfont --example book > "$font"
install -m 644 "$font" sites/kit/ui/font.json

pkg=sites/net/pkg
bucket="${MRLYPROD_BUCKET:-mrlyprod}"
region="${AWS_DEFAULT_REGION:-us-east-2}"
while IFS= read -r file; do
  printf '%s  %s\n' "$(shasum -a 256 "$pkg/$file" | awk '{print $1}')" "$file" >> "$manifest"
done < <(cd "$pkg" && find . -type f | sed 's|^\./||' | LC_ALL=C sort)
hash=$(shasum -a 256 < "$manifest" | awk '{print $1}')
files=$(wc -l < "$manifest" | tr -d ' ')
printf '%s\n' "$hash" > sites/net/pkg.lock

prefix="s3://$bucket/pkg/$hash/"
if [[ -n "$(aws s3 ls "$prefix" --region "$region" 2>/dev/null | head -1)" ]]; then
  state="already present"
else
  aws s3 sync "$pkg" "$prefix" --region "$region" --only-show-errors --exclude "*.wasm" --exclude "*.js"
  aws s3 sync "$pkg" "$prefix" --region "$region" --only-show-errors --exclude "*" --include "*.wasm" --content-type application/wasm
  aws s3 sync "$pkg" "$prefix" --region "$region" --only-show-errors --exclude "*" --include "*.js" --content-type text/javascript
  state="uploaded"
fi
echo "pkg $hash $files files $state"

# PRUNE

if [ "${PRUNE:-1}" = "0" ]; then
  exit 0
fi
lock=$(git show HEAD:sites/net/pkg.lock 2>/dev/null | tr -d '[:space:]' || true)
[ -n "$lock" ] || { echo "no HEAD pkg.lock; skipping prune"; exit 0; }
pruned=0
while IFS= read -r old; do
  [ -n "$old" ] || continue
  [ "$old" != "$hash" ] || continue
  [ "$old" != "$lock" ] || continue
  aws s3 rm "s3://$bucket/pkg/$old/" --recursive --region "$region" --only-show-errors
  pruned=$((pruned + 1))
done < <(aws s3 ls "s3://$bucket/pkg/" --region "$region" | awk '$1 == "PRE" {print $2}' | tr -d '/')
echo "pruned $pruned"
