import json
import os
from datetime import datetime
import mrlypy as mp
from fontTools.fontBuilder import FontBuilder
from fontTools.misc.timeTools import timestampSinceEpoch
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib import TTFont, newTable
from fontTools.ttLib.removeOverlaps import removeOverlaps
from config import ROOT

NAME = "MrlyFont"
STYLE = "Regular"
PIXEL = 125
UPEM = 1000
OUT = os.path.join(ROOT, "data")

def trim(rows):
    width = len(rows[0])
    cols = [c for c in range(width) if any(row[c] == "1" for row in rows)]
    if not cols:
        return ["0" for _ in rows]
    lo, hi = cols[0], cols[-1]
    return [row[lo : hi + 1] for row in rows]

def outline(rows, baseline):
    pen = TTGlyphPen(None)
    height = len(rows)
    for y, row in enumerate(rows):
        bottom = (height - y - 1) * PIXEL + baseline
        for x, pixel in enumerate(row):
            if pixel != "1":
                continue
            left = x * PIXEL
            pen.moveTo((left, bottom))
            pen.lineTo((left, bottom + PIXEL))
            pen.lineTo((left + PIXEL, bottom + PIXEL))
            pen.lineTo((left + PIXEL, bottom))
            pen.closePath()
    return pen.glyph()

def notdef():
    pen = TTGlyphPen(None)
    pen.moveTo((PIXEL // 2, PIXEL // 2))
    pen.lineTo((PIXEL * 2.5, PIXEL // 2))
    pen.lineTo((PIXEL * 2.5, PIXEL * 2.5))
    pen.lineTo((PIXEL // 2, PIXEL * 2.5))
    pen.closePath()
    return pen.glyph()

def build(ttf_path):
    glyphs = mp.font.glyphs()
    descenders = set(mp.font.descenders())
    order = [".notdef"] + sorted(glyphs)
    shapes = {".notdef": notdef()}
    metrics = {".notdef": (PIXEL * 3, PIXEL // 2)}
    cmap = {}
    for char, rows in glyphs.items():
        trimmed = trim(rows)
        baseline = -PIXEL if char in descenders else 0
        shapes[char] = outline(trimmed, baseline)
        width = (len(trimmed[0]) + 1) * PIXEL
        metrics[char] = (width, 0)
        cmap[ord(char)] = char
    codes = [ord(c) for c in glyphs]
    fb = FontBuilder(UPEM, isTTF=True)
    fb.setupGlyf(shapes)
    fb.font.setGlyphOrder(order)
    fb.setupHorizontalMetrics(metrics)
    fb.setupCharacterMap(cmap)
    fb.setupNameTable({
        "familyName": NAME,
        "styleName": STYLE,
        "uniqueFontIdentifier": f"{NAME}-{STYLE}",
        "fullName": f"{NAME} {STYLE}",
        "psName": f"{NAME}-{STYLE}",
        "version": "1.0",
    })
    fb.setupHorizontalHeader(ascent=int(UPEM * 0.8), descent=int(UPEM * -0.2))
    fb.setupOS2(
        sTypoAscender=int(UPEM * 0.8),
        sTypoDescender=int(UPEM * -0.2),
        usWinAscent=int(UPEM * 0.8),
        usWinDescent=int(UPEM * 0.2),
        usWeightClass=400,
        usWidthClass=5,
        fsType=0,
        achVendID="PXFT",
        fsSelection=64,
        fsFirstCharIndex=min(codes),
        fsLastCharIndex=min(max(codes), 0xFFFF),
        sxHeight=int(UPEM * 0.5),
        sCapHeight=int(UPEM * 0.7),
        ulCodePageRange1=1,
        ulCodePageRange2=0,
    )
    fb.setupPost(
        italicAngle=0.0,
        underlinePosition=-100,
        underlineThickness=50,
        isFixedPitch=True,
        minMemType42=0,
        maxMemType42=0,
        minMemType1=0,
        maxMemType1=0,
    )
    fb.font["post"].formatType = 3.0
    fb.setupMaxp()
    gasp = newTable("gasp")
    gasp.gaspRange = {0xFFFF: 0x000F}
    fb.font["gasp"] = gasp
    fb.font["head"].created = timestampSinceEpoch(datetime.now().timestamp())
    removeOverlaps(fb.font)
    fb.font.save(ttf_path)
    print(f"Saved: {ttf_path}")

def web(ttf_path):
    font = TTFont(ttf_path)
    for flavor in ("woff", "woff2"):
        path = os.path.join(OUT, f"{NAME}.{flavor}")
        font.flavor = flavor
        font.save(path)
        print(f"Saved: {path}")

def manifest():
    path = os.path.join(OUT, f"{NAME}.json")
    with open(path, "w", encoding="utf-8") as f:
        f.write(mp.font.json())
        f.write("\n")
    print(f"Saved: {path}")

def create():
    os.makedirs(OUT, exist_ok=True)
    ttf = os.path.join(OUT, f"{NAME}.ttf")
    build(ttf)
    web(ttf)
    manifest()

def main():
    create()

if __name__ == "__main__":
    main()
