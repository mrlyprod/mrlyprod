import os
import shutil
import subprocess
import sys
import tempfile
import time
from collections import Counter, defaultdict
from concurrent.futures import ProcessPoolExecutor
from itertools import groupby
from math import comb, isqrt

from sympy import Poly, discriminant, resultant, symbols

T, N = symbols("t n")
WORKERS = min(8, os.cpu_count() or 1)
CHUNKS = 256
DIMS = (2, 3, 4, 5, 6)
VERBS = {}

# BOX

def sig_bounds(dim, cap=None):
    return [min(comb(dim, j), cap if cap else comb(dim, j)) + 1 for j in range(1, dim + 1)]

def box_size(bounds):
    out = 1
    for b in bounds:
        out *= b
    return out

def unrank(index, bounds):
    out = [1]
    for b in bounds:
        out.append(index % b)
        index //= b
    return tuple(out)

def multiplicity(dim, sig):
    out = 1
    for j, s in enumerate(sig):
        out *= comb(comb(dim, j), s)
    return out

def weight(sig):
    return list(sig)

# POLYNOMIALS

def polymul(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, u in enumerate(a):
        for j, v in enumerate(b):
            out[i + j] += u * v
    return out

def polypow(a, k):
    out = [1]
    for _ in range(k):
        out = polymul(out, a)
    return out

def trim(a):
    while len(a) > 1 and a[-1] == 0:
        a.pop()
    return a

def poly_of(coeffs, var=T):
    return Poly(list(reversed(coeffs)), var, domain="ZZ")

def factors(coeffs):
    body = trim(list(coeffs))
    if body == [0]:
        return 0, []
    lead, parts = poly_of(body).factor_list()
    out = []
    for g, mult in parts:
        out.append((tuple(int(c) for c in reversed(g.all_coeffs())), mult))
    return int(lead), sorted(out)

def star(g):
    d = len(g) - 1
    out = [0] * (d + 1)
    for j, c in enumerate(g):
        if c:
            term = polymul([0] * j + [c], polypow([1, 1], d - j))
            for i, v in enumerate(term):
                out[i] += v
    return out

def fill_coeffs(dim, sig):
    out = [0] * (dim + 1)
    for j, s in enumerate(sig):
        if s:
            term = polymul([0] * j + [s], polypow([1, 1], dim - j))
            for i, v in enumerate(term):
                out[i] += v
    return out

def corners(dim, code):
    return [tuple((c >> i) & 1 for i in range(dim)) for c in range(1 << dim) if (code >> c) & 1]

def signature_of(dim, code):
    sig = [0] * (dim + 1)
    for c in corners(dim, code):
        sig[sum(c)] += 1
    return tuple(sig)

def text(coeffs, var="n"):
    body = []
    for i, c in reversed(list(enumerate(coeffs))):
        if not c:
            continue
        head = "" if c == 1 and i else str(c)
        body.append(head + ("" if i == 0 else var if i == 1 else "{}^{}".format(var, i)))
    return " + ".join(body) if body else "0"

# NORM

def norm_report(dim):
    seen = {}
    bad = []
    lifted = 0
    linear = Counter()
    stray = 0
    for code in range(1 << (1 << dim)):
        sig = signature_of(dim, code)
        if sig not in seen:
            body = trim(list(sig))
            fill = fill_coeffs(dim, sig)
            m = len(body) - 1
            lead, parts = factors(sig)
            build = polypow([1, 1], dim - m)
            for g, mult in parts:
                for _ in range(mult):
                    build = polymul(build, star(g))
            build = [lead * c for c in build]
            res = Poly(resultant(poly_of(body, T).as_expr(), N - T * (N + 1), T), N, domain="ZZ")
            res = polymul([int(c) for c in reversed(res.all_coeffs())], polypow([1, 1], dim - m))
            units = Counter(g[1] + 1 for g, mult in parts for _ in range(mult) if len(g) == 2 and g[0] == 1)
            wild = sum(1 for g, mult in parts if len(g) == 2 and g[0] != 1)
            seen[sig] = (build == fill, res == fill, m < dim, units, wild)
        row = seen[sig]
        if not (row[0] and row[1]):
            bad.append((code, sig))
        lifted += row[2]
        if sig[0] == 1:
            linear += row[3]
            stray += row[4]
    return len(seen), bad, lifted, linear, stray

def disc_pairs(dim):
    ok = 0
    off = 0
    for index in range(box_size(sig_bounds(dim))):
        sig = unrank(index, sig_bounds(dim))
        for g, mult in factors(weight(sig))[1]:
            if len(g) >= 3:
                a = int(discriminant(poly_of(list(g), T)))
                b = int(discriminant(poly_of(star(list(g)), N)))
                ok += a == b
                off += a != b
    return ok, off

def verb_norm():
    print("NORM FORM")
    print("  P(n) = (n+1)^(D-deg W) cont(W) prod over irreducible factors g of W of g*(n), g*(n) = (n+1)^deg g g(n/(n+1))")
    print("  cont(W) is the content, which is 1 on the origin-filled box; the constant is never the leading coefficient")
    print("  the resultant form is P(n) = (n+1)^(D-deg W) Res_t(W(t), n - t(n+1))")
    for dim in (2, 3, 4):
        sigs, bad, lifted, linear, stray = norm_report(dim)
        print("  D = {}: {} designs, {} signatures, factored form exact on {}, resultant form exact on {}".format(
            dim, 1 << (1 << dim), sigs, (1 << (1 << dim)) - len(bad), (1 << (1 << dim)) - len(bad)))
        print("    designs needing the (n+1)^(D-deg W) lift, s_D = 0: {} of {}".format(lifted, 1 << (1 << dim)))
        print("    origin-filled designs: linear factors are (a n + 1) with a counted {}".format(dict(sorted(linear.items()))))
        print("    origin-filled linear factors not of the form (a n + 1): {}".format(stray))
        ok, off = disc_pairs(dim)
        print("    disc(g) = disc(g*) over the {} origin-filled signatures, {} factor slots of degree at least 2 with g(1) != 0, mismatches {}".format(
            box_size(sig_bounds(dim)), ok, off))
    sig = (0, 2, 1)
    lead, parts = factors(sig)
    print("  origin empty breaks the law: signature {} has W = {} and fill {} = {}".format(
        sig, text(list(sig), "t"), text(fill_coeffs(2, sig)),
        " ".join("({})".format(text(star(list(g)))) for g, mult in parts for _ in range(mult))))

VERBS["norm"] = verb_norm

# SWEEP

def label_of(top, signs):
    if top == 1:
        return "Q"
    if top == 2:
        return "quadratic " + ("mixed" if len(signs) > 1 else ("imaginary" if -1 in signs else "real"))
    return "degree {}".format(top)

def sweep_chunk(job):
    dim, cap, lo, hi, want = job
    bounds = sig_bounds(dim, cap)
    sigs = Counter()
    codes = Counter()
    degrees = Counter()
    keep = defaultdict(set)
    for index in range(lo, hi):
        sig = unrank(index, bounds)
        parts = factors(sig)[1]
        top = 1
        signs = set()
        for g, mult in parts:
            d = len(g) - 1
            if d < 2:
                continue
            top = max(top, d)
            degrees[d] += mult
            if d == 2:
                signs.add(-1 if g[1] * g[1] - 4 * g[2] * g[0] < 0 else 1)
            if want and 2 <= d <= 5:
                keep[d].add(g)
        tag = label_of(top, signs)
        sigs[tag] += 1
        codes[tag] += multiplicity(dim, sig)
    return sigs, codes, degrees, {d: s for d, s in keep.items()}

def sweep(dim, cap=None, want=False):
    total = box_size(sig_bounds(dim, cap))
    edges = [total * i // CHUNKS for i in range(CHUNKS + 1)]
    jobs = [(dim, cap, edges[i], edges[i + 1], want) for i in range(CHUNKS) if edges[i] < edges[i + 1]]
    sigs, codes, degrees = Counter(), Counter(), Counter()
    keep = defaultdict(set)
    if len(jobs) == 1:
        results = [sweep_chunk(jobs[0])]
    else:
        with ProcessPoolExecutor(max_workers=WORKERS) as pool:
            results = list(pool.map(sweep_chunk, jobs))
    for a, b, c, d in results:
        sigs += a
        codes += b
        degrees += c
        for key, value in d.items():
            keep[key] |= value
    return total, sigs, codes, degrees, keep

def order(tag):
    return ("Q", "quadratic imaginary", "quadratic real", "quadratic mixed").index(tag) if tag.startswith(("Q", "quad")) else 4 + int(tag.split()[-1])

def verb_ladder():
    print("THE LADDER BY DEGREE")
    print("  box: s_0 = 1, 0 <= s_j <= C(D, j); a signature carries C(D, j) choose s_j oriented designs")
    for dim in DIMS:
        total, sigs, codes, degrees, _ = sweep(dim)
        print("  D = {}: {} signatures, {} oriented designs".format(dim, total, sum(codes.values())))
        for tag in sorted(sigs, key=order):
            print("    {:20s} signatures {:8d}  designs {}".format(tag, sigs[tag], codes[tag]))
        print("    irreducible factor slots by degree: {}".format(dict(sorted(degrees.items()))))

VERBS["ladder"] = verb_ladder

# TABLES

TABLES = {
    (3, (1, 1)): [23, 31, 44, 59, 76, 83, 87, 104, 107, 108, 116, 135, 139, 140, 152, 172, 175, 199, 200, 204, 211, 212, 216, 231, 239, 243, 244, 247, 255, 268, 283, 300, 307, 324, 327, 331, 335, 339, 351, 356, 364, 367, 379, 411, 419, 424, 431, 436, 439, 440, 451, 459, 460, 472, 484, 491, 492, 499, 503, 515, 516, 519, 524, 527, 543, 547, 563, 567, 588, 620, 628, 643, 648, 652, 655, 671, 675, 676, 679, 680, 687, 695, 696, 707, 716, 728, 731, 743, 744, 748, 751, 755, 756, 759, 771, 780, 804, 808, 812, 815],
    (3, (3, 0)): [49, 81, 148, 169, 229, 257, 316, 321, 361, 404, 469, 473, 564, 568, 621, 697, 733, 756, 761, 785, 788, 837, 892, 940, 961, 985, 993, 1016, 1076, 1101, 1129, 1229, 1257, 1300, 1304, 1345, 1369, 1373, 1384, 1396, 1425, 1436, 1489, 1492, 1509, 1524, 1556, 1573, 1593, 1620, 1708, 1765, 1772, 1825, 1849, 1901, 1929, 1937, 1940, 1944, 1957, 2021, 2024, 2057, 2089, 2101, 2177, 2213, 2228, 2233, 2241, 2292, 2296, 2300, 2349, 2429, 2505, 2557, 2589, 2597, 2636, 2673, 2677, 2700, 2708, 2713, 2777, 2804, 2808, 2836, 2857, 2917, 2920, 2941, 2981, 2993, 3021, 3028, 3124, 3132],
    (4, (0, 2)): [117, 125, 144, 189, 225, 229, 256, 257, 272, 320, 333, 392, 400, 432, 441, 512, 513, 549, 576, 576, 592, 605, 656, 657, 697, 761, 784, 788, 832, 837, 873, 892, 981, 985, 1008, 1008, 1016, 1025, 1040, 1040, 1076, 1088, 1088, 1089, 1129, 1161, 1168, 1197, 1197, 1225, 1229, 1257, 1264, 1280, 1372, 1384, 1396, 1413, 1421, 1424, 1436, 1489, 1492, 1509, 1521, 1525, 1552, 1556, 1568, 1593, 1600, 1616, 1629, 1728, 1737, 1765, 1805, 1808, 1809, 1813, 1825, 1856, 1872, 1929, 1936, 1937, 1940, 1953, 1953, 2021, 2048, 2048, 2057, 2061, 2089, 2112, 2112, 2133, 2156, 2156],
    (4, (2, 1)): [275, 283, 331, 400, 448, 475, 491, 507, 563, 643, 688, 731, 751, 775, 848, 976, 1024, 1099, 1107, 1156, 1192, 1255, 1323, 1328, 1371, 1375, 1399, 1423, 1424, 1456, 1472, 1472, 1475, 1588, 1600, 1728, 1732, 1775, 1791, 1792, 1823, 1856, 1879, 1927, 1931, 1963, 1968, 1975, 1984, 1984, 2000, 2048, 2051, 2068, 2092, 2096, 2116, 2151, 2183, 2191, 2219, 2243, 2284, 2312, 2319, 2327, 2375, 2412, 2443, 2475, 2480, 2488, 2563, 2608, 2619, 2687, 2696, 2704, 2736, 2763, 2764, 2767, 2787, 2816, 2824, 2843, 2859, 2911, 2943, 3008, 3052, 3119, 3163, 3175, 3188, 3216, 3223, 3267, 3271, 3275],
    (4, (4, 0)): [725, 1125, 1600, 1957, 2000, 2048, 2225, 2304, 2525, 2624, 2777, 3600, 3981, 4205, 4225, 4352, 4400, 4525, 4752, 4913],
    (5, (1, 2)): [1609, 1649, 1777, 2209, 2297, 2617, 2665, 2869, 3017, 3089, 3233, 3369, 3857, 3889, 4169, 4261, 4409, 4417, 4429, 4432, 4477, 4549, 4597, 4757, 4817, 4897, 5025, 5164, 5437, 5501, 5584, 5653, 5753, 5864, 5913, 6241, 6449, 6581, 6757, 6793, 7096, 7177, 7265, 7333, 7373, 7376, 7672, 7684, 7717, 7909, 8073, 8105, 8249, 8329, 8357, 8529, 8705, 8752, 8945, 8968, 9065, 9137, 9412, 9437, 9489, 9552, 9584, 9664, 9701, 9808, 9829, 10229, 10277, 10329, 10381, 10449, 10492, 10532, 10589, 10609, 10729, 10825, 10832, 10933, 11317, 11332, 11469, 11693, 11809, 11876, 11993, 12184, 12205, 12349, 12389, 12440, 12481, 12517, 12533, 12752],
    (5, (3, 1)): [4511, 4903, 5519, 5783, 7031, 7367, 7463, 8519, 8647, 9439, 9759, 10407, 11119, 11243, 11551, 12447, 13219, 13523, 13799, 13883],
    (5, (5, 0)): [14641, 24217, 36497, 38569, 65657, 70601, 81509, 81589, 89417, 101833, 106069, 117688, 122821, 124817, 126032, 135076, 138136, 138917, 144209, 147109],
}

QUADRATIC_REACH = 200

def squarefree(m):
    m = abs(m)
    k = 2
    while k * k <= m:
        if m % (k * k) == 0:
            return False
        k += 1
    return True

def fundamental(d):
    if d % 4 == 1:
        return squarefree(d)
    if d % 4 == 0:
        m = d // 4
        return m % 4 in (2, 3) and squarefree(m)
    return False

def sign_of(sig):
    return (-1) ** sig[1]

def table(degree, sig):
    if degree == 2:
        return [d for d in range(2, QUADRATIC_REACH) if fundamental(sign_of(sig) * d)]
    return TABLES.get((degree, sig))

# FIELDS

def square(value):
    if value <= 0:
        return 0
    root = isqrt(value)
    return root if root * root == value else 0

def gp_text(coeffs):
    return "+".join("{}*x^{}".format(c, i) for i, c in enumerate(coeffs) if c)

def gp_run(lines, per):
    if not lines:
        return []
    handle, name = tempfile.mkstemp(suffix=".gp")
    with os.fdopen(handle, "w") as fh:
        fh.write("\n".join(lines) + "\nquit\n")
    done = subprocess.run(["gp", "-q", "-s", "500000000", name], capture_output=True, text=True)
    os.unlink(name)
    values = [int(v) for v in done.stdout.split()]
    if len(values) != per * len(lines):
        raise RuntimeError("gp returned {} values for {} lines".format(len(values), len(lines)))
    return [values[i:i + per] for i in range(0, len(values), per)]

def guarded(disc, raw):
    if not disc or raw % disc or disc % 4 not in (0, 1):
        return 0
    return square(raw // disc)

def pari_fields(keys):
    lines = ["P={};print(nfdisc(P));print(polsturm(P));print(poldisc(P))".format(gp_text(list(reversed(g))))
             for g in keys]
    out = {}
    for g, (disc, r1, raw) in zip(keys, gp_run(lines, 3)):
        d = len(g) - 1
        index = guarded(disc, raw)
        out[g] = (d, (r1, (d - r1) // 2), disc if index else None, index, raw, "value" if index else "guard")
    return out

def pari_classes(carriers, need):
    bodies = [gp_text(list(reversed(g))) for g in carriers]
    pairs = [(i, j) for i in range(len(bodies)) for j in range(i)]
    rows = gp_run(["print(nfisisom({},{})!=0)".format(bodies[i], bodies[j]) for i, j in pairs], 1)
    same = dict(zip(pairs, [row[0] for row in rows]))
    reps = []
    for i in range(len(bodies)):
        if all(not same[(i, j)] for j in reps):
            reps.append(i)
        if len(reps) >= need:
            break
    return len(reps)

def field_data(keys):
    keys = sorted(keys)
    if not shutil.which("gp"):
        raise SystemExit("verb fields needs PARI, and gp is not on PATH")
    out = {}
    step = 20000
    for start in range(0, len(keys), step):
        out.update(pari_fields(keys[start:start + step]))
    return out

def candidates(delta):
    out = set()
    f = 1
    while f * f <= abs(delta):
        if delta % (f * f) == 0 and (delta // (f * f)) % 4 in (0, 1):
            out.add(delta // (f * f))
        f += 1
    return out

CACHE = {}

def collect():
    if CACHE:
        return CACHE["data"], CACHE["reach"]
    per = {}
    for dim in DIMS:
        per[dim] = sweep(dim, want=True)[4]
    keys = set()
    for dim in DIMS:
        for d in per[dim]:
            keys |= per[dim][d]
    data = field_data(keys)
    reach = {}
    for dim in DIMS:
        classes = defaultdict(set)
        open_slots = defaultdict(list)
        members = defaultdict(list)
        for d in per[dim]:
            for g in per[dim][d]:
                degree, sig, disc, index, raw, mode = data[g]
                if disc is None:
                    open_slots[(degree, sig)].append(g)
                else:
                    classes[(degree, sig)].add(disc)
                    members[(degree, sig)].append(g)
        reach[dim] = (classes, open_slots, members)
    CACHE["data"], CACHE["reach"] = data, reach
    return data, reach

def risky(key, value, open_slots, data):
    return any(value in {abs(v) for v in candidates(data[g][4])} for g in open_slots.get(key, ()))

def run_and_gap(reached, listed, key=None, open_slots=None, data=None):
    run = []
    gap = None
    owed = []
    for value in listed:
        if value in reached:
            run.append(value)
            continue
        if data is not None and risky(key, value, open_slots, data):
            owed.append(value)
        gap = value
        break
    return run, gap, owed

def distinct_fields(carriers, need, cap=24):
    if not carriers:
        return 0, False
    return pari_classes(carriers[:cap], need), len(carriers) > cap

def verb_fields():
    print("FIELD DISCRIMINANTS BY DEGREE AND SIGNATURE")
    print("  every distinct irreducible factor of W of degree 2..5 over the origin-filled box at D = 2..6")
    print("  field discriminant by PARI nfdisc on the reversed monic model x^d g(1/x), signature by polsturm, guarded against poldisc")
    print("  a value is accepted only if it is 0 or 1 mod 4 and divides the polynomial discriminant with a square quotient, the quotient being the square of the index")
    print("  the polynomial discriminant is used only for that guard, for the index and for the candidate bound on an unresolved factor")
    print("  tables of smallest field discriminants: degree 2 generated here, degrees 3, 4, 5 read from the LMFDB")
    data, reach = collect()
    print("  {} distinct irreducible factors of degree 2..5".format(len(data)))
    indexes = Counter(v[3] for v in data.values() if v[2] is not None and v[3])
    print("  index [O_K : Z[1/theta]]: {}".format(dict(sorted(indexes.items())[:8])))
    modes = Counter(v[5] for v in data.values())
    bad = sum(n for m, n in modes.items() if m != "value")
    print("  nfdisc values failing the guard: {} of {}".format(bad, len(data)))
    for dim in DIMS:
        classes, open_slots, members = reach[dim]
        print("  D = {}, box {}".format(dim, sig_bounds(dim)))
        for key in sorted(set(classes) | set(open_slots)):
            degree, sig = key
            listed = table(degree, sig)
            reached = {abs(v) for v in classes.get(key, ())}
            run, gap, owed = run_and_gap(reached, listed or [], key, open_slots, data)
            print("    degree {} signature {}: {} distinct field discriminants, {} unresolved".format(
                degree, sig, len(classes.get(key, ())), len(open_slots.get(key, ()))))
            if listed:
                print("      run {}".format(" ".join(str(v) for v in run) if run else "empty"))
                print("      first gap {}{}".format(
                    gap if gap is not None else "none inside the table",
                    "" if not owed else " (owed: an unresolved factor could carry {}, no probe)".format(owed)))
        layer_report(dim, classes)

VERBS["fields"] = verb_fields

# LAYER

def fundamental_part(d):
    f = 1
    best = d
    while f * f <= abs(d):
        if d % (f * f) == 0 and fundamental(d // (f * f)):
            best = d // (f * f)
        f += 1
    return best

def pure_layer(dim):
    out = defaultdict(set)
    for b in range(dim + 1):
        for c in range(1, comb(dim, 2) + 1):
            d = b * b - 4 * c
            root = int(abs(d) ** 0.5)
            if d >= 0 and root * root == d:
                continue
            out["imaginary" if d < 0 else "real"].add(fundamental_part(d))
    return out

def layer_report(dim, classes):
    reach = 4 * comb(dim, 2)
    pure = pure_layer(dim)
    told = {"imaginary": {d for d in classes.get((2, (0, 1)), set())},
            "real": {d for d in classes.get((2, (2, 0)), set())}}
    window = {d for d in range(-reach, 0) if fundamental(d)}
    print("    reach 4 C(D, 2) = {}, fundamental discriminants inside it {}".format(reach, len(window)))
    for kind in ("imaginary", "real"):
        extra = sorted(told[kind] - pure[kind])
        short = sorted(pure[kind] - told[kind])
        print("      {}: pure layer {} fields, census {} fields, census only {}, pure only {}".format(
            kind, len(pure[kind]), len(told[kind]), extra if extra else "none", short if short else "none"))
    print("      imaginary pure layer equals the window: {}".format(pure["imaginary"] == window))

# SWAP

def splits(coeffs):
    return all(len(g) <= 2 for g, mult in factors(coeffs)[1])

def swap_chunk(job):
    dim, lo, hi = job
    bounds = sig_bounds(dim)
    codes = Counter()
    sigs = Counter()
    for index in range(lo, hi):
        sig = unrank(index, bounds)
        void = [comb(dim, j) - sig[j] for j in range(1, dim + 1)]
        key = (splits(list(sig)), splits(void))
        codes[key] += multiplicity(dim, sig)
        sigs[key] += 1
    return codes, sigs

def verb_swap():
    print("THE FILL AND VOID TABLE")
    print("  P(n) is the fill, V(n) = (2n+1)^D - P(n) the void, V(0) = 0 so V = n Q(n)")
    print("  P splits means P is a product of linear factors over Q, V splits means Q is")
    print("  the empty void, W = (1+t)^D, is counted as splitting")
    for dim in (3, 4):
        total = box_size(sig_bounds(dim))
        edges = [total * i // CHUNKS for i in range(CHUNKS + 1)]
        jobs = [(dim, edges[i], edges[i + 1]) for i in range(CHUNKS) if edges[i] < edges[i + 1]]
        codes, sigs = Counter(), Counter()
        with ProcessPoolExecutor(max_workers=WORKERS) as pool:
            for a, b in pool.map(swap_chunk, jobs):
                codes += a
                sigs += b
        designs = sum(codes.values())
        print("  D = {}: {} oriented designs with the origin filled, {} signatures".format(dim, designs, total))
        for key in ((True, True), (True, False), (False, True), (False, False)):
            tag = "P{} V{}".format("+" if key[0] else "-", "+" if key[1] else "-")
            print("    {} designs {:6d} share {:.4f} signatures {}".format(
                tag, codes[key], codes[key] / designs, sigs[key]))
        full = fill_coeffs(dim, [comb(dim, j) for j in range(dim + 1)])
        solid = polypow([1, 2], dim)
        print("    fill of the full box equals (2n+1)^D: {}".format(full == solid))

VERBS["swap"] = verb_swap

# HUNTER

def merged(degree):
    rows = []
    limit = None
    sigs = sorted({k[1] for k in TABLES if k[0] == degree}) if degree > 2 else [(0, 1), (2, 0)]
    for sig in sigs:
        listed = table(degree, sig)
        seen = Counter()
        for value in listed:
            seen[value] += 1
            rows.append((value, sig, seen[value]))
        limit = listed[-1] if limit is None else min(limit, listed[-1])
    return sorted(rows), limit

def witnessed(key, value, copy, dim, reach, data, fields):
    classes, open_slots, members = reach[dim]
    reached = {abs(v) for v in classes.get(key, ())}
    if value not in reached:
        return False, value if risky(key, value, open_slots, data) else None
    if copy == 1 or not fields:
        return True, None
    carriers = [g for g in members.get(key, ()) if abs(data[g][2]) == value]
    found, capped = distinct_fields(carriers, copy)
    return found >= copy, None

def verb_hunter():
    print("THE HUNTER BOUND")
    print("  B is the largest bound with every field of degree d and abs discriminant at most B reached by the box")
    print("  the merge is signature aware: one discriminant in two signatures is two fields, and a discriminant listed twice in one signature is two fields resolved by field isomorphism")
    print("  the merged table is valid only below the smallest per-signature table maximum, printed per degree")
    print("  the box is s_0 = 1, 0 <= s_j <= C(D, j); its height is max_j C(D, j)")
    data, reach = collect()
    for degree in (2, 3, 4, 5):
        rows, limit = merged(degree)
        print("  degree {}: {} table entries, merge valid below {}".format(degree, len(rows), limit))
        for dim in (3, 4, 5, 6):
            marks = {}
            for fields in (True, False):
                bound = 0
                miss = None
                owed = None
                for value, group in groupby(rows, key=lambda row: row[0]):
                    if value > limit or miss:
                        break
                    for value, sig, copy in group:
                        ok, short = witnessed((degree, sig), value, copy, dim, reach, data, fields)
                        if not ok:
                            owed = short
                            miss = (value, sig, copy)
                            break
                    if not miss:
                        bound = value
                marks[fields] = (bound, miss, owed)
            field_bound, miss, owed = marks[True]
            disc_bound = marks[False][0]
            tail = "none inside the table" if miss is None else "{} at signature {}{}".format(
                miss[0], miss[1], " listed twice" if miss[2] > 1 else "")
            print("    D = {} box {} height {}: B by field {}, by discriminant {}, first miss {}{}".format(
                dim, sig_bounds(dim), max(comb(dim, j) for j in range(dim + 1)),
                field_bound or "none", disc_bound or "none", tail,
                "" if owed is None else " (owed: an unresolved factor could carry {}, no probe)".format(owed)))

VERBS["hunter"] = verb_hunter

# MAIN

def main():
    asked = sys.argv[1:] or ["all"]
    if asked == ["all"]:
        asked = ["norm", "ladder", "fields", "swap", "hunter"]
    started = time.time()
    for name in asked:
        VERBS[name]()
    print()
    print("wall time {:.1f} s on {} workers".format(time.time() - started, WORKERS))

if __name__ == "__main__":
    main()
