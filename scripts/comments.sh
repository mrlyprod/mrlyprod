#!/bin/sh
set -u

root=$HOME/Developer
owners="mrlyprod carlomitchener"

scan='
FNR == 1 {
  n = split(FILENAME, parts, ".")
  ext = parts[n]
  slash = (ext ~ /^(rs|ts|tsx|js|jsx|mjs)$/)
  block = 0; quote = ""; word = ""; dash = 0; pep = 0
}
{
  line = $0
  sub(/\r$/, "", line)
  t = line
  sub(/^[ \t]+/, "", t)
  body = t
  sub(/^[\/#*!]+[ \t]*/, "", body)
  directive = (body ~ /^(@ts-|<reference|biome-ignore|eslint-|prettier-ignore|@jsx|@vitest|noqa|type:|pyright:|ruff:|fmt:|-\*-|shellcheck)/)
  if (slash) {
    if (block) {
      at = index(line, "*/")
      if (at) { block = 0; if (substr(line, at + 2) ~ /^[ \t]*$/) hits[FILENAME]++ }
      else hits[FILENAME]++
      next
    }
    if (directive) next
    if (substr(t, 1, 2) == "//") { hits[FILENAME]++; next }
    if (substr(t, 1, 2) == "/*") {
      at = index(substr(t, 3), "*/")
      if (!at) { block = 1; hits[FILENAME]++ }
      else if (substr(t, at + 4) ~ /^[ \t]*$/) hits[FILENAME]++
    }
    next
  }
  if (ext == "py") {
    if (pep) { if (t == "# ///") pep = 0; next }
    if (quote != "") { if (index(line, quote)) quote = ""; next }
    if (t == "# /// script") { pep = 1; next }
    if (substr(t, 1, 1) == "#" && !(FNR == 1 && substr(t, 1, 2) == "#!") && !directive) { hits[FILENAME]++; next }
    for (q = 1; q <= 2; q++) {
      d = (q == 1) ? "\"\"\"" : "\047\047\047"
      rest = line; count = 0
      while ((at = index(rest, d)) > 0) { count++; rest = substr(rest, at + 3) }
      if (count % 2 == 1) { quote = d; break }
    }
    next
  }
  if (ext == "sh") {
    if (word != "") { if ((dash ? t : line) == word) word = ""; next }
    if (substr(t, 1, 1) == "#" && !(FNR == 1 && substr(t, 1, 2) == "#!") && !directive) { hits[FILENAME]++; next }
    if (match(line, /<<-?[ \t]*[\047"\\]?[A-Za-z_][A-Za-z0-9_]*/) && substr(line, RSTART, 3) != "<<<") {
      spec = substr(line, RSTART + 2, RLENGTH - 2)
      dash = (substr(spec, 1, 1) == "-")
      gsub(/^-|[ \t\047"\\]/, "", spec)
      word = spec
    }
  }
}
END { for (f in hits) printf "%5d  %s\n", hits[f], f }
'

check() {
  cd "$1" || return
  files=$(git ls-files -- '*.rs' '*.ts' '*.tsx' '*.js' '*.jsx' '*.mjs' '*.py' '*.sh' |
    grep -Ev '(^|/)(demos?|examples)/|\.d\.ts$|\.gen\.')
  [ -n "$files" ] || return
  found=$(printf '%s\n' "$files" | while IFS= read -r f; do [ -f "$f" ] && printf '%s\0' "$f"; done |
    xargs -0 awk "$scan" | sort -rn)
  [ -n "$found" ] || return
  echo "${1#"$root/"}"
  printf '%s\n' "$found"
  lines=$((lines + $(printf '%s\n' "$found" | awk '{ s += $1 } END { print s }')))
  count=$((count + $(printf '%s\n' "$found" | wc -l)))
}

lines=0
count=0
here=$PWD
if top=$(git -C "$here" rev-parse --show-toplevel 2>/dev/null); then
  case "$top/" in
    "$root/mrlyprod/"* | "$root/carlomitchener/"*) check "$top" ;;
    *) echo "the comment rule covers $owners only"; exit 0 ;;
  esac
else
  for owner in $owners; do
    for dir in "$root/$owner"/*/; do
      [ -d "$dir.git" ] && check "${dir%/}"
    done
  done
fi
if [ "$count" -eq 0 ]; then
  echo "no comments"
  exit 0
fi
echo "$lines comment lines in $count files"
exit 1
