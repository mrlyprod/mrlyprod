from fractions import Fraction
from itertools import product

import numpy as np

from fills import canonical
from mesh import FAMILIES, corners, edge_owners, fill_triangles, flag, fractal_rule, slice_mesh

WINDOWS = (0.01, 0.02, 0.05, 0.10, 0.20)


def tile(code, n):
    parity = np.arange(n) % 2
    out = np.zeros((n, n, n), dtype=np.uint8)
    for a, b, c in corners(code):
        out[np.ix_(parity == a, parity == b, parity == c)] = 1
    return out


def level_array(base, level):
    out = base
    for _ in range(level - 1):
        out = np.kron(out, base)
    return out


def contacts(arr, axis):
    lo = [slice(None)] * arr.ndim
    hi = [slice(None)] * arr.ndim
    lo[axis], hi[axis] = slice(None, -1), slice(1, None)
    return int(np.sum(arr[tuple(lo)] & arr[tuple(hi)]))


def face_census(arr):
    cells = int(arr.sum())
    hidden = 2 * sum(contacts(arr, a) for a in range(3))
    return cells, 6 * cells - hidden, hidden


def face(arr, axis, top):
    idx = [slice(None)] * 3
    idx[axis] = -1 if top else 0
    return arr[tuple(idx)]


def lambda2_formula(name, n):
    half = n // 2
    return {"carpet": n * n - half * half, "net": half * half, "tree": (n - half) ** 2}[name]


def fit_matrix(rows):
    (va, ha), (vb, hb), (vc, hc) = [(Fraction(v), Fraction(h)) for _, v, h in rows[:3]]
    det = va * hb - ha * vb
    if det == 0:
        return None
    top = ((vb * hb - ha * vc) / det, (va * vc - vb * vb) / det)
    bot = ((hb * hb - ha * hc) / det, (va * hc - vb * hb) / det)
    return [top, bot]


def apply(m, state):
    return (m[0][0] * state[0] + m[0][1] * state[1], m[1][0] * state[0] + m[1][1] * state[1])


def family_block(name, code, n, lmax):
    base = tile(code, n)
    fc = int(base.sum())
    adj = [contacts(base, a) for a in range(3)]
    prof = [int(face(base, a, False).sum()) for a in range(3)]
    live = sorted({prof[a] for a in range(3) if adj[a] > 0})
    l2 = live[-1] if live else None
    w = sum(adj)
    rows = []
    for level in range(1, lmax + 1):
        arr = level_array(base, level)
        cells, vis, hid = face_census(arr)
        assert cells == fc**level
        for a in range(3):
            lo, hi = face(arr, a, False), face(arr, a, True)
            assert np.array_equal(lo, hi) and int(lo.sum()) == prof[a] ** level
        rows.append((level, vis, hid))
    steps = all(fc * v - 2 * w * (l2 or 0) ** lv == rows[i + 1][1]
                for i, (lv, v, h) in enumerate(rows[:-1]))
    print(f"  {name:9} n={n} fc={fc:<4} adj={adj} profiles={prof} W={w:<3} l2={l2}"
          f"  one live profile {flag(len(live) <= 1)}  recurrence {flag(steps)}")
    print("            visible", ", ".join(str(v) for _, v, _ in rows),
          " hidden", ", ".join(str(h) for _, _, h in rows))
    if l2 is not None and l2 != fc:
        b = Fraction(2 * w, fc - l2)
        exact = all((6 - b) * fc**lv + b * l2**lv == v for lv, v, h in rows)
        formula = lambda2_formula(name, n)
        print(f"            V(i) = {6 - b}*{fc}^i + {b}*{l2}^i  {flag(exact)}"
              f"  lambda2 formula {formula} {flag(l2 == formula)}")
    if len(rows) < 3:
        return
    m = fit_matrix(rows)
    if m is None:
        print("            state vectors parallel, no 2x2 matrix determined")
        return
    tr, det = m[0][0] + m[1][1], m[0][0] * m[1][1] - m[0][1] * m[1][0]
    pred = all(apply(m, rows[i - 1][1:]) == rows[i][1:] for i in range(3, len(rows)))
    more = f"  predicts level 4 {flag(pred)}" if len(rows) > 3 else ""
    print(f"            fitted M = [[{m[0][0]}, {m[0][1]}], [{m[1][0]}, {m[1][1]}]]"
          f"  trace {tr} = {fc} + {l2}  det {det} = {fc} * {l2}"
          f"  {flag(tr == fc + l2 and det == fc * l2)}{more}")


def recurrence():
    print("FACE-COUNT RECURRENCE, BRUTE FORCE AT n = 3 TO LEVEL 4, n = 5 TO 3, n = 7 TO 2")
    for n, lmax in ((3, 4), (5, 3), (7, 2)):
        for name, code in FAMILIES:
            family_block(name, code, n, lmax)


def square_census(k):
    n = 2 * k - 1
    cells = [(i, j) for i in range(n) for j in range(n) if i % 2 == j % 2]
    filled = set(cells)
    touched = set()
    perimeter = 0
    for i, j in cells:
        touched |= {("h", i, j), ("h", i + 1, j), ("v", i, j), ("v", i, j + 1)}
        steps = ((1, 0), (-1, 0), (0, 1), (0, -1))
        perimeter += sum((i + di, j + dj) not in filled for di, dj in steps)
    return len(cells), len(touched), perimeter


def exposure():
    print("NO HIDDEN FACES: THE ANTIPODAL DESIGN, k = 1..12")
    grid = [face_census(tile(129, 2 * k - 1)) for k in range(1, 13)]
    good = all(c == 2 * k**3 - 3 * k**2 + 3 * k - 1 and h == 0 and v == 6 * c
               for k, (c, v, h) in enumerate(grid, 1))
    print("  cells = 2k^3 - 3k^2 + 3k - 1, hidden 0, surface = 6 cells:", flag(good))
    print("  cells k = 1..6:", ", ".join(str(c) for c, _, _ in grid[:6]))
    good = all(e == p == 4 * c for c, e, p in map(square_census, range(1, 13)))
    print("  2D: edges touched = perimeter = 4 cells at k = 1..12:", flag(good))
    levels = [face_census(level_array(tile(129, 3), lv)) for lv in range(1, 5)]
    good = all(hid == 0 and vis == 6 * 9**lv for lv, (c, vis, hid) in enumerate(levels, 1))
    print("  levels 1..4 surface:", ", ".join(str(vis) for _, vis, _ in levels), "= 6 * 9^L:",
          flag(good))
    print("SWEEP OF ALL 256 DESIGNS AT n = 3, 5, 7")
    isolated = []
    for code in range(256):
        cs = corners(code)
        iso = all(sum(a != b for a, b in zip(p, q)) != 1 for p in cs for q in cs)
        if iso:
            isolated.append(code)
        for n in (3, 5, 7):
            assert iso == (face_census(tile(code, n))[2] == 0)
    classes = sorted({canonical(c) for c in isolated})
    whole = sum(1 for c in range(256) if canonical(c) in classes) == len(isolated)
    print("  surface = 6 cells exactly when corners are pairwise at Hamming distance >= 2: ok")
    print(f"  {len(isolated)} designs, whole classes {flag(whole)}:",
          ", ".join(f"mrly_{c:03d}" for c in classes))


def corner_touch(arr):
    n = arr.shape[0]
    out = np.zeros(tuple(s + 1 for s in arr.shape), dtype=bool)
    for d in product((0, 1), repeat=arr.ndim):
        out[tuple(slice(o, o + n) for o in d)] |= arr.astype(bool)
    return int(out.sum())


def square_tile(k):
    n = 2 * k - 1
    parity = np.arange(n) % 2
    out = np.ones((n, n), dtype=np.uint8)
    out[np.ix_(parity == 0, parity == 0)] = 0
    return out


def grid_corners():
    print("GRID CORNERS TOUCHED AT ODD SIDE n = 2k - 1, k = 1..20")
    for name, code in FAMILIES:
        got = [corner_touch(tile(code, 2 * k - 1)) for k in range(1, 21)]
        if all(v == 8 * k**3 for k, v in enumerate(got, 1)):
            print(f"  {name:10} touches every grid corner, 8k^3: ok")
            continue
        law = all(v == (2 * k - 2) ** 3 + 6 * (2 * k - 2) ** 2 == 8 * k**3 - 24 * k + 16
                  == 8 * (k - 1) ** 2 * (k + 2) and 8 * k**3 - v == 24 * k - 16
                  for k, v in enumerate(got, 1))
        print(f"  {name:10} m^3 + 6m^2 = 8k^3 - 24k + 16 = 8(k-1)^2(k+2), m = 2k-2,"
              f" short by 24k - 16: {flag(law)}")
    good = all(corner_touch(square_tile(k)) == 4 * k * k - 4 for k in range(1, 25))
    print("  2D at least one odd coordinate, k = 1..24: 4k^2 - 4:", flag(good))
    even = [code for code in range(256) if code & 1]
    good = all((corner_touch(tile(code, 2 * k - 1)) == 8 * k**3) == (code in set(even))
               for code in range(256) for k in range(1, 4))
    print(f"  all 256 designs at k = 1..3: exactly the {len(even)} rules holding the all-even"
          f" corner touch every corner: {flag(good)}")


def giant(tris):
    links = [tuple(o) for o in edge_owners(tris).values() if len(o) == 2]
    parent = list(range(len(tris)))

    def find(a):
        while parent[a] != a:
            parent[a] = parent[parent[a]]
            a = parent[a]
        return a

    for a, b in links:
        ra, rb = find(a), find(b)
        if ra != rb:
            parent[ra] = rb
    roots = [find(a) for a in range(len(tris))]
    big = max(set(roots), key=roots.count)
    keep = {a: i for i, a in enumerate(a for a in range(len(tris)) if roots[a] == big)}
    sub = [(keep[a], keep[b]) for a, b in links if a in keep and b in keep]
    return len(set(roots)), len(keep), sub


def spectrum(m, links):
    adj = np.zeros((m, m))
    for a, b in links:
        adj[a, b] = adj[b, a] = 1.0
    inv = 1.0 / np.sqrt(adj.sum(axis=1))
    return np.linalg.eigvalsh(np.eye(m) - inv[:, None] * adj * inv[None, :])


def exponent(eigs, frac):
    vals = np.sort(np.maximum(eigs, 0.0))
    xs, ys = [], []
    i = 0
    while i < len(vals):
        j = i
        while j + 1 < len(vals) and vals[j + 1] == vals[i]:
            j += 1
        if vals[j] > 1e-12:
            xs.append(vals[j])
            ys.append((j + 1) / len(vals))
        i = j + 1
    top = max(int(frac * len(vals)), 3)
    return 2 * float(np.polyfit(np.log(xs[:top]), np.log(ys[:top]), 1)[0])


def spectral():
    print("SPECTRAL EXPONENT, NORMALISED LAPLACIAN IDOS ON THE GIANT PIECE")
    print("  graph            nodes  comps  giant   d_s at windows 1%, 2%, 5%, 10%, 20%")
    jobs = [(f"carpet n=3 L{lv}", 3**lv, fractal_rule(23, 3, lv)) for lv in (1, 2, 3)]
    jobs += [(f"solid n={n}", n, lambda c: True) for n in (9, 19)]
    jobs.append(("carpet n=5 L2", 25, fractal_rule(23, 5, 2)))
    for label, side, rule in jobs:
        tris = fill_triangles(slice_mesh(side), rule)
        comps, m, sub = giant(tris)
        eigs = spectrum(m, sub)
        assert np.sum(eigs < 1e-10) == 1
        ds = "  ".join(f"{exponent(eigs, f):.2f}" for f in WINDOWS)
        print(f"  {label:16} {len(tris):6} {comps:6} {m:6}   {ds}")


if __name__ == "__main__":
    recurrence()
    exposure()
    grid_corners()
    spectral()
