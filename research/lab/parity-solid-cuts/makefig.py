import sys
from pathlib import Path

import numpy as np
from PIL import Image

from cuts import scheduled, six_pieces

LEVEL = 7
WIDTH = 1100
HEIGHT = 980
SURF = (0xFC, 0xFC, 0xFB)
PALETTE = [(0x2A, 0x78, 0xD6), (0x2C, 0x96, 0x86), (0x7A, 0x57, 0xC8),
           (0xD0, 0x60, 0x2C), (0xBF, 0x94, 0x2A), (0xB0, 0x41, 0x4A)]

def pieces():
    return [sorted(p) for p in six_pieces(LEVEL)]

def decomposition_holds(parts):
    m = 1 << (LEVEL - 1)
    union = set().union(*map(set, parts))
    both = scheduled(LEVEL, m - 1) | scheduled(LEVEL, m)
    shadow = {(x - y, x + y - 2 * z) for x, y, z in union}
    return (union == both and sum(map(len, parts)) == 2 * 3 ** LEVEL
            and all(len(p) == 3 ** (LEVEL - 1) for p in parts) and len(shadow) == len(union))

def disc(img, px, py, r, colour):
    x0, x1 = int(px - r - 2.0), int(px + r + 2.0)
    y0, y1 = int(py - r - 2.0), int(py + r + 2.0)
    xs = np.arange(max(x0, 0), min(x1 + 1, WIDTH))
    ys = np.arange(max(y0, 0), min(y1 + 1, HEIGHT))
    gx, gy = np.meshgrid(xs, ys)
    d = np.hypot(gx + 0.5 - px, gy + 0.5 - py)
    alpha = np.clip(r + 0.5 - d, 0.0, 1.0)
    window = img[ys[0]:ys[-1] + 1, xs[0]:xs[-1] + 1].astype(np.float64)
    blended = window + (np.array(colour, dtype=np.float64) - window) * alpha[:, :, None]
    hit = alpha > 0.0
    window[hit] = np.rint(blended[hit])
    img[ys[0]:ys[-1] + 1, xs[0]:xs[-1] + 1] = window.astype(np.uint8)

def render():
    parts = pieces()
    assert decomposition_holds(parts)
    r2, r6 = 2.0 ** 0.5, 6.0 ** 0.5
    flat = [(((x - y) / r2, (x + y - 2 * z) / r6), i)
            for i, part in enumerate(parts) for x, y, z in part]
    us = [q[0][0] for q in flat]
    vs = [q[0][1] for q in flat]
    min_x, max_x, min_y, max_y = min(us), max(us), min(vs), max(vs)
    cx, cy = (min_x + max_x) / 2.0, (min_y + max_y) / 2.0
    scale = min((WIDTH - 60) / (max_x - min_x), (HEIGHT - 60) / (max_y - min_y))
    radius = 0.40 * scale * (2.0 / 3.0) ** 0.5
    img = np.empty((HEIGHT, WIDTH, 3), dtype=np.uint8)
    img[:, :] = SURF
    for (u, v), i in flat:
        disc(img, WIDTH / 2.0 + scale * (u - cx), HEIGHT / 2.0 - scale * (v - cy), radius, PALETTE[i])
    return img, len(flat)

def main():
    path = Path(sys.argv[1] if len(sys.argv) > 1 else "research/figures/cuts-fig.png")
    img, points = render()
    path.parent.mkdir(parents=True, exist_ok=True)
    Image.fromarray(img, "RGB").save(path, compress_level=9)
    print(f"wrote {path.name}: {WIDTH}x{HEIGHT}, level {LEVEL}, {points} points "
          f"in six gaskets of {3 ** (LEVEL - 1)}")

if __name__ == "__main__":
    main()
