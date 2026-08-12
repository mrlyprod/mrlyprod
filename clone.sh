#!/bin/sh
set -eu

dir="${1:-mrlyprod}"

mkdir -p "$dir"
curl -fsSL https://cdn.mrly.net/mrlyprod.tar.gz | tar -xzf - --strip-components=1 -C "$dir"
echo "$dir"
