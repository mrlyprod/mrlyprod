import math
from fractions import Fraction
from itertools import permutations, product

from mesh import FAMILIES, ch, corners, edge_owners, fill_triangles, flag
from mesh import fractal_rule, parity_rule, slice_mesh

GROUP = [(p, f) for p in permutations(range(3)) for f in product((0, 1), repeat=3)]


def padd(a, b):
    n = max(len(a), len(b))
    return [(a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0) for i in range(n)]


def pmul(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, u in enumerate(a):
        for j, v in enumerate(b):
            out[i + j] += u * v
    return out


def peval(a, k):
    return sum(c * k**p for p, c in enumerate(a))


def pstr(a):
    parts = []
    for p in range(len(a) - 1, -1, -1):
        c = a[p]
        if c == 0:
            continue
        body = "" if p == 0 else ("k" if p == 1 else f"k^{p}")
        head = "" if (abs(c) == 1 and p > 0) else str(abs(c))
        parts.append(("-" if c < 0 else "+", head + body))
    first = ("-" if parts[0][0] == "-" else "") + parts[0][1]
    return first + "".join(f" {s} {b}" for s, b in parts[1:])


def fill_poly(code):
    total = [0]
    for c in corners(code):
        term = [1]
        for bit in c:
            term = pmul(term, [-1, 1] if bit else [0, 1])
        total = padd(total, term)
    while len(total) > 1 and total[-1] == 0:
        total.pop()
    return total


def direct_cells(code, k):
    rule = parity_rule(code)
    return sum(1 for c in product(range(2 * k - 1), repeat=3) if rule(c))


def image(code, g):
    perm, flip = g
    out = 0
    for c in corners(code):
        r = [c[perm[a]] ^ flip[a] for a in range(3)]
        out |= 1 << (4 * r[0] + 2 * r[1] + r[2])
    return out


def orbit(code):
    return sorted({image(code, g) for g in GROUP})


def canonical(code):
    return orbit(code)[0]


def polynomials():
    print("FILL POLYNOMIALS FROM THE CORNER SETS")
    print(f"  carpet 23:  {pstr(fill_poly(23))}")
    print(f"  net 232:    {pstr(fill_poly(232))}")
    print(f"  mrly_024:   {pstr(fill_poly(24))}")
    cubes = all(peval(fill_poly(129), k) == k**3 + (k - 1) ** 3 for k in range(1, 20))
    print(f"  mrly_129:   {pstr(fill_poly(129))}   k^3 + (k-1)^3:", flag(cubes))
    named = ((23, "carpet"), (232, "net"), (3, "tree"), (129, "antipodal"), (105, "checkerboard"))
    for code, name in named:
        print(f"  {name:12} code {code:3} class mrly_{canonical(code):03d}")
    polys = sorted({tuple(fill_poly(c)) for c in orbit(23)}, key=lambda p: -p[2])
    print(f"  orbit of 23: {len(orbit(23))} members, {len(polys)} distinct polynomials")
    for p in polys:
        print("   ", pstr(list(p)))
    seq = ", ".join(str(peval(fill_poly(232), k)) for k in range(1, 6))
    form = all(peval(fill_poly(232), k) == (k - 1) ** 2 * (4 * k - 1) for k in range(1, 30))
    print(f"  complement fills k = 1..5: {seq}  (k-1)^2 (4k-1): {flag(form)}")
    bad = []
    for k in range(1, 10):
        for c in orbit(23):
            a, b = direct_cells(c, k), direct_cells(255 - c, k)
            if a != peval(fill_poly(c), k) or a + b != (2 * k - 1) ** 3:
                bad.append((k, c))
    print("  cell-by-cell counts k = 1..9, each pair summing to (2k-1)^3:", flag(not bad))
    mesh = slice_mesh(3)
    fills = ", ".join(f"{n} {len(fill_triangles(mesh, parity_rule(c)))}" for n, c in FAMILIES)
    print("  slice fills at n = 3:", fills)


def section_area(t):
    clip = lambda x: x if x > 0 else Fraction(0)
    return 4 * (t * t - 3 * clip(t - 1) ** 2 + 3 * clip(t - 2) ** 2 - clip(t - 3) ** 2)


def partition():
    print("CARPET + NET PARTITION THE HEXAGON, ODD n = 1..31")
    pairs = []
    for n in range(1, 32, 2):
        mesh = slice_mesh(n)
        carpet = set(fill_triangles(mesh, parity_rule(23)))
        net = set(fill_triangles(mesh, parity_rule(232)))
        whole = {t for t, _ in mesh}
        assert not carpet & net and carpet | net == whole and len(whole) == 6 * n * n
        pairs.append((n, len(carpet), len(net)))
    print("  disjoint and covering triangle by triangle at every n: ok")
    shown = (3, 5, 7, 9, 11, 31)
    print("  ", "; ".join(f"n={n}: {c} + {v} = {c + v}" for n, c, v in pairs if n in shown))
    print("  layers at n = 5, carpet/net cut cells per diagonal layer:")
    layers = {}
    for t, c in slice_mesh(5):
        layers.setdefault(sum(c), set()).add(c)
    rule = parity_rule(23)
    for s in sorted(layers):
        a = sum(1 for c in layers[s] if rule(c))
        print(f"    s = {s}: {a}/{len(layers[s]) - a}")
    print("SECTION AREA ROUTE, n = 1..16, IN SMALL TRIANGLES")
    print("    n    total   6n^2  carpet     net")
    rule_c, rule_n = parity_rule(23), parity_rule(232)
    for n in range(1, 17):
        tot = car = net = Fraction(0)
        for c in product(range(n), repeat=3):
            t = Fraction(3 * n, 2) - sum(c)
            if 0 < t < 3:
                area = section_area(t)
                tot += area
                car += area if rule_c(c) else 0
                net += area if rule_n(c) else 0
        good = tot == 6 * n * n and car + net == tot and (n % 2 or car == net)
        print(f"  {n:3} {str(tot):>8} {6 * n * n:6} {str(car):>7} {str(net):>7}  {flag(good)}")


class Forest:
    def __init__(self, n):
        self.p = list(range(n))

    def find(self, a):
        while self.p[a] != a:
            self.p[a] = self.p[self.p[a]]
            a = self.p[a]
        return a

    def join(self, a, b):
        ra, rb = self.find(a), self.find(b)
        if ra != rb:
            self.p[ra] = rb

    def roots(self):
        return [self.find(a) for a in range(len(self.p))]


def adjacency_forest(tris):
    forest = Forest(len(tris))
    for o in edge_owners(tris).values():
        if len(o) == 2:
            forest.join(o[0], o[1])
    return forest


def topology(tris):
    if not tris:
        return 0, 0, 0, 0
    owners = edge_owners(tris)
    verts = {p for t in tris for p in t}
    index = {p: i for i, p in enumerate(verts)}
    by_point = Forest(len(verts))
    for t in tris:
        by_point.join(index[t[0]], index[t[1]])
        by_point.join(index[t[0]], index[t[2]])
    chi = len(verts) - len(owners) + len(tris)
    b0 = len(set(by_point.roots()))
    return len(set(adjacency_forest(tris).roots())), b0, chi, b0 - chi


def holes_by_complement(mesh, fill):
    tris = [t for t, _ in mesh]
    void = {i for i, t in enumerate(tris) if t not in fill}
    outside = len(tris)
    forest = Forest(outside + 1)
    for o in edge_owners(tris).values():
        if len(o) == 1 and o[0] in void:
            forest.join(o[0], outside)
        elif len(o) == 2 and o[0] in void and o[1] in void:
            forest.join(o[0], o[1])
    return len({forest.find(i) for i in void} - {forest.find(outside)})


def claim(k):
    return (ch((k + 1) // 2), 0) if k % 2 else (1, ch(k // 2))


def components():
    print("CARPET SLICE COMPONENTS AND HOLES, k = 1..14, EACH COUNTED TWO WAYS")
    print("   k   n  fills  comp_edge  comp_point  chi  holes  holes_compl  claim")
    comps, holes = [], []
    for k in range(1, 15):
        mesh = slice_mesh(2 * k - 1)
        fill = fill_triangles(mesh, parity_rule(23))
        ce, cp, chi, h = topology(fill)
        hc = holes_by_complement(mesh, set(fill))
        good = ce == cp == claim(k)[0] and h == hc == claim(k)[1]
        comps.append(cp)
        holes.append(h)
        print(f"{k:4} {2 * k - 1:3} {len(fill):6} {ce:10} {cp:11} {chi:4} {h:6} {hc:12}"
              f"  {claim(k)}  {flag(good)}")
    print("  components:", ", ".join(map(str, comps)))
    print("  holes:     ", ", ".join(map(str, holes)))
    print("OTHER FAMILIES, HOLES AT k = 1..10")
    for name, code in FAMILIES[1:]:
        hs = []
        for k in range(1, 11):
            mesh = slice_mesh(2 * k - 1)
            fill = fill_triangles(mesh, parity_rule(code))
            h = topology(fill)[3]
            assert h == holes_by_complement(mesh, set(fill))
            hs.append(h)
        print(f"  {name:10} {', '.join(map(str, hs))}")


def percolation_row(name, base, level, code):
    fill = fill_triangles(slice_mesh(base**level), fractal_rule(code, base, level))
    roots = adjacency_forest(fill).roots()
    largest = max(roots.count(r) for r in set(roots)) if roots else 0
    print(f"  {name:10} {base:4} {level:5} {base**level:5} {len(fill):7} {len(set(roots)):11}"
          f" {largest:8}")
    return len(fill)


def percolation():
    print("SLICE CENSUS AND PERCOLATION AT BASE 3, LEVELS 1..4, AND BASE 5, LEVELS 1..2")
    print("  family     base  level  side   fills  components  largest")
    series = [6]
    for level in range(1, 5):
        for name, code in FAMILIES:
            fills = percolation_row(name, 3, level, code)
            if name == "carpet":
                series.append(fills)
    for level in (1, 2):
        for name, code in FAMILIES[:2]:
            percolation_row(name, 5, level, code)
    print("  carpet census at base 3, levels 0..4:", ", ".join(map(str, series)))
    rec = all(series[i] == 9 * series[i - 1] - 12 * series[i - 2] for i in range(2, 5))
    dim = math.log((9 + math.sqrt(33)) / 2) / math.log(3)
    print(f"  recurrence a = 9a' - 12a'': {flag(rec)}  log((9 + sqrt 33)/2)/log 3 = {dim:.4f}")


if __name__ == "__main__":
    polynomials()
    partition()
    components()
    percolation()
