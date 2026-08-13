import json
import os
import shutil
import subprocess
import sys
import mrlyweb as mp
from PIL import Image
from config import ROOT

LOGOS = os.path.join(ROOT, "files", "logos")
WORK = os.path.join(ROOT, "data", "logos")
WORD = "MRLYPROD"
PAD = 1
BANNER = (16, 9)
LOGO = 200
GRID = 32
TEXT = 50
CELL = 10
BLACK = (0, 0, 0)
WHITE = (255, 255, 255)
HANDLE = None

# DOORS

def handle():
    global HANDLE
    if HANDLE is None:
        HANDLE = mp.boot("full")
    return HANDLE

def mark():
    mp.call(handle(), {"verb": "two.set", "args": {"key": "level", "value": 1}})
    return ["".join(str(bit) for bit in row) for row in mp.read(handle(), "two/cells/ids")]

def wordmark():
    book = mp.read(handle(), "fonts/glyphs")
    letters = [book[char] for char in WORD]
    height = len(letters[0]) + 2 * PAD
    width = sum(len(letter[0]) for letter in letters) + len(letters) - 1 + 2 * PAD
    grid = [["0"] * width for _ in range(height)]
    col = PAD
    for letter in letters:
        for y, row in enumerate(letter):
            for x, bit in enumerate(row):
                if bit == "1":
                    grid[PAD + y][col + x] = "1"
        col += len(letter[0]) + 1
    return ["".join(row) for row in grid]

def palette():
    return mp.read(handle(), "colors/palette")

def frames():
    out = subprocess.run(
        ["cargo", "run", "-q", "-p", "mrlyfont", "--example", "cycle"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=True,
    )
    return json.loads(out.stdout)

# GRIDS

def tile(rows, across, down):
    wide = ["".join(row * across) for row in rows]
    return [row for _ in range(down) for row in wide]

def spread(lit, rows, cols):
    grid = [["0"] * cols for _ in range(rows)]
    for index in lit:
        grid[index // cols][index % cols] = "1"
    return ["".join(row) for row in grid]

# RENDER

def svg(rows, scale):
    wide = len(rows[0]) * scale
    tall = len(rows) * scale
    lines = [f'<svg width="{wide}" height="{tall}" xmlns="http://www.w3.org/2000/svg">']
    for y, row in enumerate(rows):
        for x, bit in enumerate(row):
            fill = "#000000" if bit == "1" else "#ffffff"
            lines.append(
                f'<rect x="{x * scale}" y="{y * scale}" width="{scale}" height="{scale}" fill="{fill}" stroke="none"/>'
            )
    lines.append("</svg>")
    return "\n".join(lines) + "\n"

def image(rows, scale):
    im = Image.new("RGB", (len(rows[0]), len(rows)))
    for y, row in enumerate(rows):
        for x, bit in enumerate(row):
            im.putpixel((x, y), BLACK if bit == "1" else WHITE)
    return im.resize((im.width * scale, im.height * scale), Image.NEAREST)

def home():
    os.makedirs(LOGOS, exist_ok=True)

def save_text(name, text):
    path = os.path.join(LOGOS, name)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)
    print(f"Saved: {path}")

def save_image(im, name, **kwargs):
    path = os.path.join(LOGOS, name)
    im.save(path, **kwargs)
    print(f"Saved: {path}")

# STILLS

def stills():
    home()
    seed = mark()
    save_text("mrlylogo.svg", svg(seed, LOGO))
    save_image(image(seed, LOGO), "mrlylogo.png")
    banner = tile(seed, BANNER[0], BANNER[1])
    save_text("mrlygrid.svg", svg(banner, GRID))
    save_image(image(banner, GRID), "mrlygrid.png")
    word = wordmark()
    save_text("mrlyprod.svg", svg(word, TEXT))
    save_image(image(word, TEXT), "mrlyprod.png")

# MOTION

def sheet(anim):
    if os.path.exists(WORK):
        shutil.rmtree(WORK)
    os.makedirs(WORK)
    paths = []
    for index, lit in enumerate(anim["frames"]):
        path = os.path.join(WORK, f"frame_{index:04d}.png")
        image(spread(lit, anim["rows"], anim["cols"]), CELL).save(path)
        paths.append(path)
    print(f"Drew: {len(paths)} frames at {anim['cols'] * CELL}x{anim['rows'] * CELL}")
    return paths

def gif(paths, fps):
    stack = [Image.open(path) for path in paths]
    save_image(
        stack[0],
        "mrlyprod.gif",
        save_all=True,
        append_images=stack[1:],
        duration=round(1000 / fps),
        loop=0,
    )

def mp4(fps):
    path = os.path.join(LOGOS, "mrlyprod.mp4")
    subprocess.run(
        [
            "ffmpeg", "-y", "-loglevel", "error",
            "-framerate", str(fps),
            "-i", os.path.join(WORK, "frame_%04d.png"),
            "-c:v", "libx264", "-pix_fmt", "yuv420p", "-r", str(fps),
            path,
        ],
        check=True,
    )
    print(f"Saved: {path}")

def motion():
    home()
    anim = frames()
    paths = sheet(anim)
    gif(paths, anim["fps"])
    mp4(anim["fps"])
    shutil.rmtree(WORK)

# COLORS

def colors():
    lines = ["MRLYCOLORS", ""]
    for color in palette():
        code = color["hex"]
        red, green, blue = (int(code[at : at + 2], 16) for at in (1, 3, 5))
        lines.append(f"{color['name']:<7} {code} {red:>3} {green:>3} {blue:>3}")
    return "\n".join(lines) + "\n"

def marks():
    home()
    save_text("mrlycolors.txt", colors())

# TERMINAL

def main():
    stills()
    motion()
    marks()

def terminal():
    match sys.argv[1:]:
        case ["stills"]: stills()
        case ["motion"]: motion()
        case ["colors"]: marks()
        case _: main()

if __name__ == "__main__":
    terminal()
