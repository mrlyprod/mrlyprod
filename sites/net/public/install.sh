#!/bin/sh
set -eu

cdn="https://cdn.mrly.net"
dir="$HOME/.local/bin"

case "$(uname -s)-$(uname -m)" in
    Darwin-arm64) target="aarch64-apple-darwin" ;;
    Darwin-x86_64) target="x86_64-apple-darwin" ;;
    Linux-x86_64) target="x86_64-unknown-linux-musl" ;;
    Linux-aarch64 | Linux-arm64) target="aarch64-unknown-linux-musl" ;;
    *) echo "mrly: no build for $(uname -s) $(uname -m)" >&2; exit 1 ;;
esac

version=$(curl -fsSL "$cdn/cli/latest")
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
curl -fsSL "$cdn/cli/$version/mrly-$target.tar.gz" | tar -xzf - -C "$tmp"
mkdir -p "$dir"
mv -f "$tmp/mrly" "$dir/mrly"
echo "mrly $version -> $dir/mrly"
case ":$PATH:" in
    *:"$dir":*) ;;
    *) echo 'add to your PATH: export PATH="$HOME/.local/bin:$PATH"' ;;
esac
