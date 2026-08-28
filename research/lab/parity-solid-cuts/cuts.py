import sys
import time
from fractions import Fraction
from math import comb

import numpy as np

W1 = [(1, 0, 0), (0, 1, 0), (0, 0, 1)]
W2 = [(0, 1, 1), (1, 0, 1), (1, 1, 0)]
PERMS = [(0, 1, 2), (0, 2, 1), (1, 0, 2), (1, 2, 0), (2, 0, 1), (2, 1, 0)]
CODES = [23, 63, 105, 111, 126, 127]
NEIGHBOURS = [63, 105, 111, 126, 127]

def flag(ok):
    return "OK" if ok else "FAIL"

def corners(code):
    return [(x, y, z) for x in (0, 1) for y in (0, 1) for z in (0, 1)
            if code >> (4 * x + 2 * y + z) & 1]

def canonical(code):
    cells = corners(code)
    best = None
    for perm in PERMS:
        for flip in range(8):
            img = 0
            for q in cells:
                r = [q[perm[i]] ^ (flip >> i & 1) for i in range(3)]
                img |= 1 << (4 * r[0] + 2 * r[1] + r[2])
            best = img if best is None else min(best, img)
    return best

def by_digits(code, level):
    cells = corners(code)
    pts = {(0, 0, 0)}
    for _ in range(level):
        pts = {(2 * x + a, 2 * y + b, 2 * z + c) for x, y, z in pts for a, b, c in cells}
    return pts

def by_scan(code, level):
    n = 1 << level
    grid = np.indices((n, n, n)).reshape(3, -1).T
    ok = np.ones(len(grid), dtype=bool)
    allowed = np.zeros(8, dtype=bool)
    for x, y, z in corners(code):
        allowed[4 * x + 2 * y + z] = True
    for k in range(level):
        bits = (grid >> k) & 1
        ok &= allowed[4 * bits[:, 0] + 2 * bits[:, 1] + bits[:, 2]]
    return {tuple(int(v) for v in p) for p in grid[ok]}

def profile(pts):
    out = {}
    for x, y, z in pts:
        out[x + y + z] = out.get(x + y + z, 0) + 1
    return dict(sorted(out.items()))

def scheduled(level, offset):
    pts = {(0, 0, 0)}
    for k in reversed(range(level)):
        cells = W2 if offset >> k & 1 else W1
        pts = {(2 * x + a, 2 * y + b, 2 * z + c) for x, y, z in pts for a, b, c in cells}
    return pts

def true_slice(pts, total):
    return {p for p in pts if sum(p) == total}

def trinomial_layer(n):
    return {(x, y, n - x - y) for x in range(n + 1) for y in range(n - x + 1)
            if x & y == 0 and y & (n - x - y) == 0 and x & (n - x - y) == 0}

def trinomial_layer_parity(n):
    return {(x, y, n - x - y) for x in range(n + 1) for y in range(n - x + 1)
            if comb(n, x) * comb(n - x, y) % 2 == 1}

def wt(n):
    return bin(n).count("1")

def distinct3(a, b, c):
    return len({a, b, c})

def det3(m):
    return (m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]))

def inverse3(m):
    d = det3(m)
    inv = [[Fraction(0)] * 3 for _ in range(3)]
    for i in range(3):
        for j in range(3):
            rows = [r for r in range(3) if r != i]
            cols = [c for c in range(3) if c != j]
            minor = (m[rows[0]][cols[0]] * m[rows[1]][cols[1]]
                     - m[rows[0]][cols[1]] * m[rows[1]][cols[0]])
            inv[j][i] = (-1) ** (i + j) * minor / d
    return inv, d

def octahedron_check():
    half = Fraction(1, 2)
    points = [[Fraction(v) - half for v in p] for p in corners(126)]
    basis = [[half, -half, -half], [-half, half, -half], [-half, -half, half]]
    m = [[basis[j][i] for j in range(3)] for i in range(3)]
    t, d = inverse3(m)
    img = sorted(tuple(sum(t[i][j] * v[j] for j in range(3)) for i in range(3)) for v in points)
    axes = sorted(tuple(Fraction(sign if i == axis else 0) for i in range(3))
                  for axis in range(3) for sign in (1, -1))
    colsum = [m[0][j] + m[1][j] + m[2][j] for j in range(3)]
    return d, img == axes, colsum

def check_canonical():
    print("CANONICAL CODES")
    ok = True
    for code in CODES:
        found = canonical(code)
        ok &= found == code
        print(f"  code {code:3} canonical {found:3}  {flag(found == code)}")
    return ok

def check_a(top):
    print("CLAIM A  support and constant count for code 126")
    ok = True
    for level in range(1, top + 1):
        pts = by_digits(126, level)
        prof = profile(pts)
        keys = list(prof)
        want = list(range((1 << level) - 1, (1 << (level + 1)) - 1))
        count = 3 ** level
        good = keys == want and all(v == count for v in prof.values())
        ok &= good
        agree = "-"
        if level <= 6:
            agree = "same" if by_scan(126, level) == pts else "DIFFER"
            ok &= agree == "same"
        print(f"  L={level} support [{keys[0]},{keys[-1]}]  every slice {count}  "
              f"digits vs scan {agree}  {flag(good)}")
    return ok

def check_a_recursion(top):
    print("CLAIM A  again, by a height recursion that builds no point set")
    ok = True
    for level in range(1, top + 1):
        cnt = {0: 1}
        for k in range(level):
            nxt = {}
            for s, c in cnt.items():
                for d in (1, 2):
                    nxt[s + (d << k)] = nxt.get(s + (d << k), 0) + 3 * c
            cnt = nxt
        keys = sorted(cnt)
        want = list(range((1 << level) - 1, (1 << (level + 1)) - 1))
        count = 3 ** level
        good = keys == want and set(cnt.values()) == {count}
        ok &= good
        print(f"  L={level:2} support [{keys[0]},{keys[-1]}]  {len(keys)} heights  "
              f"every slice {count}  {flag(good)}")
    return ok

def check_b(top):
    print("CLAIM B  binary orientation schedule, every offset t")
    ok = True
    total = 0
    for level in range(1, top + 1):
        pts = by_digits(126, level)
        count = 3 ** level
        good = True
        for t in range(1 << level):
            gasket = scheduled(level, t)
            if true_slice(pts, (1 << level) - 1 + t) != gasket or len(gasket) != count:
                good = False
        ok &= good
        total += 1 << level
        print(f"  L={level}  all {1 << level} slices equal their scheduled gasket  {flag(good)}")
    print(f"  slices checked as sets {total}")
    return ok

def six_pieces(level):
    m = 1 << (level - 1)
    heavy = scheduled(level - 1, m - 1)
    light = scheduled(level - 1, 0)
    pieces = [{(m * e[0] + x, m * e[1] + y, m * e[2] + z) for x, y, z in heavy} for e in W1]
    pieces += [{(m * d[0] + x, m * d[1] + y, m * d[2] + z) for x, y, z in light} for d in W2]
    return pieces

def check_c(top):
    print("CLAIM C  the six gaskets of the central union")
    ok = True
    sizes = []
    for level in range(2, top + 1):
        m = 1 << (level - 1)
        a = scheduled(level, m - 1)
        b = scheduled(level, m)
        pieces = six_pieces(level)
        union = set().union(*pieces)
        piece_size = 3 ** (level - 1)
        good = (union == a | b and sum(map(len, pieces)) == len(union)
                and all(len(p) == piece_size for p in pieces) and len(union) == 2 * 3 ** level)
        sectors = {}
        spokes = 0
        for p in union:
            if distinct3(*p) < 3:
                spokes += 1
            else:
                key = tuple(sorted(range(3), key=lambda i: p[i]))
                sectors[key] = sectors.get(key, 0) + 1
        even = sorted(sectors.values()) == [piece_size - 1] * 6 and spokes == 6
        s3 = all({(p[i], p[j], p[k]) for p in union} == union for i, j, k in PERMS)
        high = (1 << level) - 1
        comp = {(high - x, high - y, high - z) for x, y, z in union} == union
        swap = {(high - x, high - y, high - z) for x, y, z in a} == b
        spine = {p for p in union if distinct3(*p) < 3}
        wanted = {(x, y, z) for x in (m - 1, m) for y in (m - 1, m) for z in (m - 1, m)
                  if distinct3(x, y, z) == 2 and x + y + z in (high + m - 1, high + m)}
        sym = s3 and comp and swap and spine == wanted
        ok &= good and even and sym
        sizes.append(len(union))
        print(f"  L={level}  |union| {len(union)} = 2*3^L  six disjoint gaskets of {piece_size}  "
              f"{flag(good)}")
        print(f"        by coordinate order: six sectors of {piece_size - 1} plus {spokes} "
              f"central points  {flag(even)}")
        print(f"        S3 and complement invariant, complement swaps the heights, "
              f"centre is the 6 perms of (m,m,m+1) and (m,m+1,m+1)  {flag(sym)}")
    print(f"  union sizes L=2..{top}: {', '.join(map(str, sizes))}  group order 12")
    return ok

def check_pascal(top):
    print("CLAIM D  the flat slices are odd trinomial layers")
    ok = True
    for level in range(1, top + 1):
        layer = (1 << level) - 1
        kummer = trinomial_layer(layer)
        good = scheduled(level, 0) == kummer
        ok &= good
        by_parity = "-"
        if level <= 5:
            by_parity = "same" if kummer == trinomial_layer_parity(layer) else "DIFFER"
            ok &= by_parity == "same"
        print(f"  L={level}  weight-one slice = odd trinomials of layer {layer}, "
              f"{3 ** level} points  Kummer vs integer parity {by_parity}  {flag(good)}")
    print("CLAIM E  code 23 slice counts are 3^wt(s), A048883")
    for level in range(1, top + 1):
        prof = profile(by_digits(23, level))
        good = all(prof.get(s, 0) == 3 ** wt(s) for s in range(1 << level))
        ok &= good
        shown = ", ".join(str(prof.get(s, 0)) for s in range(min(1 << level, 8)))
        print(f"  L={level}  [{shown}]  {flag(good)}")
    return ok

def check_neighbours(top):
    print("NEIGHBOURING DESIGNS  slice profiles")
    for code in NEIGHBOURS:
        for level in range(1, top + 1):
            prof = profile(by_digits(code, level))
            keys = list(prof)
            flat = all(v == 3 ** level for v in prof.values())
            print(f"  code {code:3} L={level}  support [{keys[0]},{keys[-1]}]  "
                  f"nonzero {len(keys)} of {keys[-1] - keys[0] + 1}  min {min(prof.values())}  "
                  f"max {max(prof.values())}  flat {flat}")
    print("RETRACTED FORM  4(L+5)*3^(L-1) against code 127")
    for level in range(1, 7):
        prof = profile(by_digits(127, level))
        total = sum(prof.values())
        print(f"  L={level}  claimed {4 * (level + 5) * 3 ** (level - 1)}   "
              f"actual max {max(prof.values())}  min {min(prof.values())}  "
              f"total {total} = 7^L {total == 7 ** level}")

def check_octahedron():
    print("OCTAHEDRON FLAKE  exact conjugation")
    d, matched, colsum = octahedron_check()
    threefold = colsum[0] == colsum[1] == colsum[2]
    print(f"  det of the centred basis {d}")
    print(f"  six centred offsets map to the six octahedron axes  {flag(matched)}")
    print(f"  x+y+z pulls back to [{', '.join(map(str, colsum))}], a threefold axis  "
          f"{flag(threefold)}")
    return matched and threefold

def main():
    top = int(sys.argv[1]) if len(sys.argv) > 1 else 8
    start = time.time()
    ok = check_canonical()
    print()
    ok &= check_a(top)
    print()
    ok &= check_a_recursion(14)
    print()
    ok &= check_b(min(top, 6))
    print()
    ok &= check_c(min(top, 8))
    print()
    ok &= check_pascal(min(top, 6))
    print()
    check_neighbours(min(top, 4))
    print()
    ok &= check_octahedron()
    print()
    print(f"ALL CHECKS {flag(ok)}  top level {top}  {time.time() - start:.1f} s")

if __name__ == "__main__":
    main()
