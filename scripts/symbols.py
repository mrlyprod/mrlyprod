import html
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

from fontTools.subset import Options, Subsetter
from fontTools.ttLib import TTFont

ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / "site"
FONTS = SITE / "kit" / "ui" / "fonts"
MASTER = ROOT / "files" / "fonts" / "symbols.ttf"
SHIPPED = FONTS / "symbols.woff2"
CSS = FONTS / "fonts.css"
CONFIG = SITE / "site.json"
DIST = Path(os.environ.get("MRLY_DIST") or SITE / "dist")
SKIP = {"git", "raw"}
FAMILY = "Noto Sans Symbols 2"
DROP = re.compile(r"<(script|style)\b[^>]*>.*?</\1>", re.S | re.I)
TAG = re.compile(r"<[^>]*>", re.S)

def pages(dist):
    out = []
    for here, dirs, files in os.walk(dist):
        if Path(here) == dist:
            dirs[:] = [d for d in dirs if d not in SKIP]
        out += [Path(here) / name for name in files if name.endswith(".html")]
    return sorted(out)

def used(dist):
    seen = set()
    found = pages(dist)
    for file in found:
        text = html.unescape(TAG.sub(" ", DROP.sub(" ", file.read_text(encoding="utf8", errors="replace"))))
        seen |= {ord(ch) for ch in set(text)}
    return seen, len(found)

def source():
    here = FONTS / "symbols.ttf"
    if here.exists():
        MASTER.parent.mkdir(parents=True, exist_ok=True)
        if subprocess.run(["git", "mv", str(here), str(MASTER)], cwd=ROOT, capture_output=True).returncode:
            shutil.move(str(here), str(MASTER))
        print(f"symbols: master kept at {MASTER.relative_to(ROOT)}")
    if MASTER.exists():
        return MASTER
    if SHIPPED.exists():
        print("symbols: no master ttf, narrowing the shipped woff2")
        return SHIPPED
    raise SystemExit("symbols: nothing to subset")

def ranges(points):
    out = []
    for cp in points:
        if out and cp == out[-1][1] + 1:
            out[-1][1] = cp
        else:
            out.append([cp, cp])
    return out

def spell(spans):
    return ", ".join(f"U+{a:04x}" if a == b else f"U+{a:04x}-{b:04x}" for a, b in spans)

def face(text, ranged):
    block = re.compile(r'@font-face \{\n  font-family: "' + FAMILY + r'";.*?\n\}', re.S)
    found = block.search(text)
    if not found:
        raise SystemExit(f"symbols: no @font-face for {FAMILY} in {CSS}")
    body = re.sub(
        r"  src: url\([^)]*\) format\([^)]*\);\n(?:  unicode-range: [^\n]*\n)?",
        f'  src: url(symbols.woff2) format("woff2");\n  unicode-range: {ranged};\n',
        found.group(0),
    )
    return text[: found.start()] + body + text[found.end() :]

def listed(text):
    return text.replace('"fonts/symbols.ttf"', '"fonts/symbols.woff2"')

def main():
    if not DIST.exists():
        raise SystemExit(f"symbols: no dist at {DIST}; build the site first")
    seen, count = used(DIST)
    file = source()
    font = TTFont(file, recalcTimestamp=False)
    keep = sorted(cp for cp in seen & set(font.getBestCmap()) if cp > 0x7F)
    if not keep:
        raise SystemExit("symbols: no page needs a symbol glyph")
    options = Options()
    options.layout_features = ["*"]
    options.name_IDs = ["*"]
    options.hinting = False
    options.flavor = "woff2"
    cut = Subsetter(options=options)
    cut.populate(unicodes=keep)
    cut.subset(font)
    font.flavor = "woff2"
    font.save(SHIPPED)
    font.close()
    spans = ranges(keep)
    ranged = spell(spans)
    CSS.write_text(face(CSS.read_text(encoding="utf8"), ranged), encoding="utf8")
    CONFIG.write_text(listed(CONFIG.read_text(encoding="utf8")), encoding="utf8")
    was = file.stat().st_size
    now = SHIPPED.stat().st_size
    print(f"symbols: {count} pages, {len(keep)} glyphs, {len(spans)} ranges, {was:,} -> {now:,} bytes")
    print(f"symbols: unicode-range: {ranged}")
    return 0

if __name__ == "__main__":
    sys.exit(main())
