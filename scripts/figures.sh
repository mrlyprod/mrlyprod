#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
mkdir -p data files/figures
lock=data/cargo.lock
until mkdir "$lock" 2>/dev/null; do sleep 2; done
trap 'rmdir "$lock"' EXIT

case "${1:-}" in
  check) cargo check -q -p mrlyfig --examples --tests; exit 0 ;;
  test) cargo test -q -p mrlyfig; exit 0 ;;
esac

if [[ $# -gt 0 ]]; then
  names=("$@")
else
  names=()
  while IFS= read -r name; do names+=("$name"); done < <(grep -A1 '^\[\[example\]\]' crates/mrlyfig/Cargo.toml | sed -n 's/^name = "\(.*\)"/\1/p')
fi

for name in "${names[@]}"; do
  for theme in dark light; do
    MRLYFIG_THEME="$theme" cargo run -q --profile fig -p mrlyfig --example "$name"
  done
done
dark=$(ls files/figures/*-dark.png 2>/dev/null | wc -l | tr -d ' ')
light=$(ls files/figures/*-light.png 2>/dev/null | wc -l | tr -d ' ')
echo "$dark dark and $light light figures in files/figures"
