import json
import os
import subprocess
import sys
from PIL import Image
from config import ROOT

PAINTS = os.path.join(ROOT, "data", "paints")
CELL = 10

# DOORS

def rows():
    out = subprocess.run(
        ["cargo", "run", "-q", "-p", "mrlymath", "--example", "paints"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=True,
    )
    return json.loads(out.stdout)

# RENDER

def rgba(code):
    code = code.lstrip("#")
    if len(code) == 6:
        code += "ff"
    return tuple(int(code[at : at + 2], 16) for at in (0, 2, 4, 6))

def image(picture, scale):
    palette = [rgba(code) for code in picture["palette"]]
    im = Image.new("RGBA", (picture["width"], picture["height"]))
    for y, row in enumerate(picture["rows"]):
        for x, index in enumerate(row):
            im.putpixel((x, y), palette[index])
    return im.resize((im.width * scale, im.height * scale), Image.NEAREST)

def home():
    os.makedirs(PAINTS, exist_ok=True)

def save_image(im, name):
    path = os.path.join(PAINTS, name)
    im.save(path)
    print(f"Saved: {path}")

# TILES

def paints(scale):
    home()
    book = rows()
    for paint in book["paints"]:
        save_image(image(paint["image"], scale), f"{paint['edition']}.png")
    print(f"Drew: {len(book['paints'])} editions under seed {book['seed']}")

# TERMINAL

def terminal():
    match sys.argv[1:]:
        case [scale]: paints(int(scale))
        case _: paints(CELL)

if __name__ == "__main__":
    terminal()
