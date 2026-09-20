import os
import sys
from concurrent.futures import ProcessPoolExecutor

from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
FIGURES = os.path.join(HERE, "..", "files", "figures")
WIDTH = 1024
QUALITY = 90
METHOD = 6

def press(png):
    webp = png[:-4] + ".webp"
    fresh = os.path.exists(webp) and os.path.getmtime(webp) >= os.path.getmtime(png)
    if not fresh:
        image = Image.open(png)
        if image.width > WIDTH:
            image = image.resize((WIDTH, round(image.height * WIDTH / image.width)), Image.LANCZOS)
        image.save(webp, "WEBP", quality=QUALITY, method=METHOD)
    return os.path.getsize(png), os.path.getsize(webp), fresh, os.path.basename(webp)

def main():
    home = os.path.normpath(FIGURES)
    names = sorted(name for name in os.listdir(home) if name.endswith(".png"))
    if not names:
        print("no figures in files/figures")
        return 0
    with ProcessPoolExecutor() as pool:
        rows = list(pool.map(press, [os.path.join(home, name) for name in names]))
    png = sum(row[0] for row in rows)
    webp = sum(row[1] for row in rows)
    kept = sum(1 for row in rows if row[2])
    big = max(rows, key=lambda row: row[1])
    mb = lambda n: f"{n / 1048576:.1f} MB"
    print(f"{len(rows)} webp ({len(rows) - kept} pressed) {mb(webp)} from {mb(png)} of png, {webp / png:.0%}")
    print(f"largest {big[3]} {big[1] / 1024:.0f} KB")
    return 0

if __name__ == "__main__":
    sys.exit(main())
