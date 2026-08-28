from fractions import Fraction
from itertools import combinations, product

AXES = (0, 1, 2)
CORNERS = tuple(product((0, 1), repeat=3))
FAMILIES = (("carpet", 23), ("net", 232), ("tree", 3), ("antipodal", 129))
CLOSED = {
    "F": lambda k: 24 * k * k - 24 * k + 6,
    "B": lambda k: 12 * k - 6,
    "E": lambda k: 36 * k * k - 30 * k + 6,
    "I": lambda k: 36 * k * k - 42 * k + 12,
    "V": lambda k: 12 * k * k - 6 * k + 1,
}


def corners(code):
    return tuple(c for c in CORNERS if code >> (4 * c[0] + 2 * c[1] + c[2]) & 1)


def dist2(u, v):
    return sum((a - b) ** 2 for a, b in zip(u, v))


def cut_points(i, j, l, d):
    base = (2 * i, 2 * j, 2 * l)
    pts = []
    for a in AXES:
        o = [b for b in AXES if b != a]
        for b1 in (0, 2):
            for b2 in (0, 2):
                if d - b1 - b2 == 1:
                    p = list(base)
                    p[a] += 1
                    p[o[0]] += b1
                    p[o[1]] += b2
                    pts.append(tuple(p))
    return pts


def cell_pieces(i, j, l, d):
    pts = cut_points(i, j, l, d)
    if len(pts) == 3:
        return [tuple(sorted(pts))]
    assert len(pts) == 6, pts
    mid = (2 * i + 1, 2 * j + 1, 2 * l + 1)
    out = [tuple(sorted((mid, p, q))) for p, q in combinations(pts, 2) if dist2(p, q) == 2]
    assert len(out) == 6
    return out


def slice_mesh(n):
    sigma = (3 * n - 1) // 2
    seen = set()
    out = []
    for i in range(n):
        for j in range(n):
            for s in (sigma - 2, sigma - 1, sigma):
                l = s - i - j
                if 0 <= l < n:
                    for t in cell_pieces(i, j, l, 3 * n - 2 * s):
                        assert t not in seen, t
                        seen.add(t)
                        out.append((t, (i, j, l)))
    return out


def parity_rule(code):
    keep = set(corners(code))
    return lambda c: (c[0] % 2, c[1] % 2, c[2] % 2) in keep


def fractal_rule(code, base, level):
    keep = set(corners(code))

    def filled(c):
        x, y, z = c
        for _ in range(level):
            if (x % base % 2, y % base % 2, z % base % 2) not in keep:
                return False
            x, y, z = x // base, y // base, z // base
        return True

    return filled


def fill_triangles(mesh, rule):
    return [t for t, c in mesh if rule(c)]


def edge_owners(tris):
    owners = {}
    for idx, t in enumerate(tris):
        for e in combinations(t, 2):
            owners.setdefault(e, []).append(idx)
    assert all(len(o) <= 2 for o in owners.values())
    return owners


def counts(tris):
    owners = edge_owners(tris)
    verts = {p for t in tris for p in t}
    boundary = sum(1 for o in owners.values() if len(o) == 1)
    adjacency = sum(1 for o in owners.values() if len(o) == 2)
    return len(verts), len(owners), len(tris), boundary, adjacency


def row(k):
    v, e, f, b, a = counts([t for t, _ in slice_mesh(2 * k - 1)])
    return {"V": v, "E": e, "F": f, "B": b, "I": a, "chi": v - e + f}


def interpolate(ks, ys):
    deg = len(ks) - 1
    coeffs = [Fraction(0)] * (deg + 1)
    for i, (ki, yi) in enumerate(zip(ks, ys)):
        basis = [Fraction(1)]
        den = Fraction(1)
        for j, kj in enumerate(ks):
            if j == i:
                continue
            basis = [Fraction(0)] + basis
            for p in range(len(basis) - 1):
                basis[p] -= kj * basis[p + 1]
            den *= ki - kj
        for p, c in enumerate(basis):
            coeffs[p] += c * yi / den
    return coeffs


def poly_str(coeffs):
    parts = []
    for p in range(len(coeffs) - 1, -1, -1):
        c = coeffs[p]
        if c == 0:
            continue
        body = "" if p == 0 else ("k" if p == 1 else f"k^{p}")
        head = "" if (abs(c) == 1 and p > 0) else str(abs(c))
        parts.append(("-" if c < 0 else "+", head + body))
    if not parts:
        return "0"
    first = ("-" if parts[0][0] == "-" else "") + parts[0][1]
    return first + "".join(f" {s} {b}" for s, b in parts[1:])


def evaluate(coeffs, k):
    return sum(c * Fraction(k) ** p for p, c in enumerate(coeffs))


def ch(m):
    return 3 * m * m - 3 * m + 1


def is_prime(m):
    return m >= 2 and all(m % d for d in range(2, int(m**0.5) + 1))


def factor(m):
    out = []
    d = 2
    while d * d <= m:
        while m % d == 0:
            out.append(d)
            m //= d
        d += 1
    if m > 1:
        out.append(m)
    return out


def norm_witness(p):
    for a in range(int(p**0.5) + 1):
        for b in range(a, int(p**0.5) + 1):
            if a * a + a * b + b * b == p:
                return a, b
    return None


def flag(good):
    return "ok" if good else "MISMATCH"


def census():
    print("SLICE CENSUS, n = 2k - 1")
    print("   k   n     V     E     F     B     I  chi")
    rows = {}
    for k in range(1, 11):
        r = rows[k] = row(k)
        good = all(CLOSED[key](k) == r[key] for key in CLOSED)
        good = good and r["chi"] == 1 and r["B"] * (2 * k - 1) == r["F"]
        print(f"{k:4} {2 * k - 1:3} {r['V']:5} {r['E']:5} {r['F']:5} {r['B']:5}"
              f" {r['I']:5} {r['chi']:4}  {flag(good)}")
    print("closed forms at n = 3:", ", ".join(f"{key} = {CLOSED[key](2)}" for key in CLOSED))
    print("BLIND FIT THROUGH k = 1..3, CHECKED ON k = 4..10 AND FRESH BUILDS")
    fits = {}
    for key in CLOSED:
        ks = [1, 2, 3]
        fits[key] = interpolate(ks, [rows[k][key] for k in ks])
        bad = [k for k in rows if evaluate(fits[key], k) != rows[k][key]]
        print(f"  {key}  {poly_str(fits[key]):24} {flag(not bad)}")
    for k in (12, 16, 20):
        r = row(k)
        good = all(evaluate(fits[key], k) == r[key] == CLOSED[key](k) for key in CLOSED)
        print(f"  k = {k} n = {2 * k - 1}: triangles {r['F']} edges {r['E']}"
              f" vertices {r['V']} boundary {r['B']} chi {r['chi']}"
              f"  {flag(good and r['chi'] == 1)}")


def lemma():
    print("LEMMA: ADJACENCIES = E' - B' ON THE SUB-MESH, FIVE FAMILIES, LEVELS 1..4")
    print("  level side family     fills     E'     B'  E'-B'    adj  full E-B")
    for level in range(1, 5):
        side = 3**level
        mesh = slice_mesh(side)
        full = counts([t for t, _ in mesh])
        for name, code in (("solid", 255),) + FAMILIES:
            tris = fill_triangles(mesh, fractal_rule(code, 3, level))
            v, e, f, b, a = counts(tris)
            print(f"  {level:5} {side:4} {name:9} {f:8} {e:6} {b:6} {e - b:6} {a:6}"
                  f" {full[1] - full[3]:9}  {flag(a == e - b)}")


def vertices():
    print("VERTEX COUNT 12k^2 - 6k + 1 = CH(2k), CH(m) = 3m^2 - 3m + 1")
    ks = range(-40, 41)
    good = all(CLOSED["V"](k) == ch(2 * k) and CLOSED["V"](k) % 3 == 1 for k in ks)
    print("  identity and residue 1 mod 3 over k = -40..40:", flag(good))
    print("  CH(1..5) =", ", ".join(str(ch(m)) for m in range(1, 6)))
    good = all(ch(m) == m * m + m * (m - 1) + (m - 1) ** 2 for m in range(1, 81))
    print("  CH(m) = m^2 + m(m-1) + (m-1)^2 over m = 1..80:", flag(good))
    primes, composites = [], []
    for k in range(1, 41):
        v = CLOSED["V"](k)
        (primes if is_prime(v) else composites).append((k, v))
    print("  primes k = 1..20:", ", ".join(str(v) for k, v in primes if k <= 20))
    print("  at k =", ", ".join(str(k) for k, v in primes if k <= 20))
    print("  composites k = 1..20:", ", ".join(str(v) for k, v in composites if k <= 20))
    print("  primes k = 21..40 add:", ", ".join(str(v) for k, v in primes if k > 20))
    print(f"  all {len(primes)} primes are 1 mod 3:", flag(all(v % 3 == 1 for _, v in primes)))
    for k, v in primes:
        a, b = norm_witness(v)
        assert a * a + a * b + b * b == v
    a, b = norm_witness(4219)
    print(f"  norm witness 4219 = {a}^2 + {a}*{b} + {b}^2, every prime has one")
    print("  composites factored:", "; ".join(
        f"{v} = {'*'.join(map(str, factor(v)))}" for k, v in composites if k <= 20))


if __name__ == "__main__":
    census()
    lemma()
    vertices()
