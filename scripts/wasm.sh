#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
mkdir -p data
lock=data/cargo.lock
until mkdir "$lock" 2>/dev/null; do sleep 2; done
trap 'rmdir "$lock"' EXIT
wasm-pack build crates/mrlyweb --target web --release --out-dir "$PWD/sites/net/pkg"
cargo run -q -p mrlyfont --example book > pkgs/js/mrlyjs/ui/font.json

pkg=sites/net/pkg
bucket="${MRLYPROD_BUCKET:-mrlyprod}"
region="${AWS_DEFAULT_REGION:-us-east-2}"
manifest=$(mktemp)
trap 'rmdir "$lock"; rm -f "$manifest"' EXIT
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
