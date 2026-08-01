BLACK = (0, 0, 0)

WHITE = (255, 255, 255)

RED = (255, 61, 64)

ORANGE = (255, 143, 44)

YELLOW = (255, 209, 0)

GREEN = (50, 204, 88)

MINT = (0, 209, 187)

TEAL = (0, 202, 216)

CYAN = (30, 201, 243)

BLUE = (0, 140, 255)

INDIGO = (103, 104, 250)

PURPLE = (211, 50, 233)

PINK = (255, 50, 90)

BROWN = (177, 132, 98)

GRAY = (142, 142, 147)

PRIMARIES = [BLACK, WHITE]

SECONDARIES = [RED, ORANGE, YELLOW, GREEN, MINT, TEAL, CYAN, BLUE, INDIGO, PURPLE, PINK, BROWN, GRAY]

def lerp(a, b, t):
    return tuple(round(a[i] + (b[i] - a[i]) * t) for i in range(3))

def gradient(colors, steps):
    if steps <= 1 or len(colors) < 2:
        return list(colors[:steps]) or [colors[0]]
    out = []
    segments = len(colors) - 1
    for i in range(steps):
        pos = i / (steps - 1) * segments
        lo = min(int(pos), segments - 1)
        out.append(lerp(colors[lo], colors[lo + 1], pos - lo))
    return out
