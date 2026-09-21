#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
[ -n "${CARGO_LOCK:-}" ] || exec "$HERE/cargo.sh" "$HERE/figures.sh" "$@"
cd "$HERE/.."

mkdir -p files/figures

case "${1:-}" in
  check) cargo check -q -p figures --all-targets; exit 0 ;;
  test) cargo test -q -p figures --lib; exit 0 ;;
esac

bench=0
if [[ "${1:-}" == bench ]]; then bench=1; shift; fi

if [[ $# -gt 0 ]]; then
  names=("$@")
  full=0
else
  names=()
  while IFS= read -r name; do names+=("$name"); done < <(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "figures") | .targets[] | select(.kind[] == "bin") | .name')
  full=1
fi

BENCH_TMP="$(mktemp "${TMPDIR:-/tmp}/figures-bench.XXXXXX")"
export BENCH_TMP
trap 'rm -f "$BENCH_TMP"' EXIT

if [[ $full -eq 1 ]]; then
  cargo build -q --profile fig -p figures --bins
else
  cargo build -q --profile fig -p figures "${names[@]/#/--bin=}"
fi
for name in "${names[@]}"; do
  for theme in dark light; do
    echo "$theme $name"
  done
done | xargs -P "$(sysctl -n hw.ncpu)" -n 2 sh -c '
s=$(perl -MTime::HiRes=time -e "print time")
c=0
FIGURES_THEME="$0" perl -e "alarm 5; exec @ARGV; exit 127" "target/fig/$1" || c=$?
e=$(perl -MTime::HiRes=time -e "print time")
perl -e "printf qq{%.2f\t%s\t%s\t%s\n}, \$ARGV[1] - \$ARGV[0], \$ARGV[2], \$ARGV[3], \$ARGV[4]" "$s" "$e" "$1" "$0" "$c" >> "$BENCH_TMP"
' || true

table="$(sort -rn "$BENCH_TMP" | awk -F'\t' '{m = ($4 == 142) ? "  TIMEOUT" : ($4 == 0 ? "" : "  FAIL"); printf "%8.2f  %s  %s%s\n", $1, $2, $3, m}')"
echo "$table"

if [[ $full -eq 1 ]]; then
  out="$HERE/../../data/mrlyprod/figures"
  mkdir -p "$out"
  echo "$table" > "$out/bench.txt"
fi

uv run --project "$HERE/.." python "$HERE/webp.py"

dark=$(ls files/figures/*-dark.png 2>/dev/null | wc -l | tr -d ' ')
light=$(ls files/figures/*-light.png 2>/dev/null | wc -l | tr -d ' ')
echo "$dark dark and $light light figures in files/figures"

broke="$(awk -F'\t' '$4 != 0 && $4 != 142 {print "failed: " $2 " " $3}' "$BENCH_TMP")"
slow=$(awk -F'\t' '$4 == 142 || $1 > 5 {n++} END {print n + 0}' "$BENCH_TMP")
[[ -z "$broke" ]] || echo "$broke"
if [[ -n "$broke" ]] || { [[ $bench -eq 0 ]] && [[ $slow -gt 0 ]]; }; then
  [[ $slow -eq 0 ]] || echo "$slow runs over 5 s"
  exit 1
fi
