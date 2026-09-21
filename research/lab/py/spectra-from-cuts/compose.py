import sys
import time
from fractions import Fraction
from itertools import product

import sympy as sp

BASES = list(range(3, 26, 2))


def digit_polynomial(b):
    p = [0] * (3 * (b - 1) + 1)
    for v in product(range(b), repeat=3):
        if sum(d % 2 for d in v) <= 1:
            p[sum(v)] += 1
    return p


def generating_polynomial(b):
    n = (b + 1) // 2
    e = [1 if k % 2 == 0 else 0 for k in range(b)]
    o = [1 if k % 2 == 1 else 0 for k in range(b)]
    ee = convolve(e, e)
    return [x + 3 * y for x, y in zip(convolve(ee, e), convolve(ee, o))]


def convolve(p, q):
    out = [0] * (len(p) + len(q) - 1)
    for i, x in enumerate(p):
        if x:
            for j, y in enumerate(q):
                out[i + j] += x * y
    return out


def coefficient(p, s):
    return p[s] if 0 <= s < len(p) else 0


def automaton(b, p):
    g = 3 * (b - 1) // 2
    return {(c, d): coefficient(p, c + g - b * d) for c in (-1, 0, 1) for d in (-1, 0, 1)}


def leak(b, p):
    g = 3 * (b - 1) // 2
    return sum(coefficient(p, c + g - b * d) for c in (-1, 0, 1) for d in (-2, 2))


def even_block(m):
    return [[m[0, 0], m[0, 1]], [m[1, 0] + m[-1, 0], m[1, 1] + m[-1, 1]]]


def power_column(m, levels):
    vec = {-1: 0, 0: 1, 1: 0}
    out = [(vec[0], vec[1] + vec[-1])]
    for _ in range(levels):
        vec = {c: sum(m[c, d] * vec[d] for d in (-1, 0, 1)) for c in (-1, 0, 1)}
        out.append((vec[0], vec[1] + vec[-1]))
    return out


def layer_census(b, p, level):
    dist = {0: 1}
    for k in range(level):
        shift = b**k
        nxt = {}
        for s0, v0 in dist.items():
            for s1, v1 in enumerate(p):
                if v1:
                    nxt[s0 + s1 * shift] = nxt.get(s0 + s1 * shift, 0) + v0 * v1
        dist = nxt
    h = 3 * (b**level - 1) // 2
    return dist.get(h, 0), dist.get(h - 1, 0) + dist.get(h + 1, 0)


def spectra_closed_form(b):
    if b % 4 == 3:
        return [
            [Fraction(3 * (b + 1) * (3 * b - 1), 16), Fraction((b + 1) * (b + 5), 32)],
            [Fraction(3 * (b + 1) ** 2, 8), Fraction(3 * (b + 1) ** 2, 16)],
        ]
    return [
        [Fraction(3 * b * b + 6 * b + 7, 16), Fraction(3 * (b - 1) * (b + 3), 32)],
        [Fraction(3 * (b - 1) * (3 * b + 5), 8), Fraction((b + 3) ** 2, 16)],
    ]


def verb_block():
    print("BLOCK: even carry block from the automaton against spectra's closed forms and a layer census")
    bad = []
    for b in BASES:
        p = digit_polynomial(b)
        if p != generating_polynomial(b):
            bad.append((b, "polynomial"))
        m = automaton(b, p)
        block = even_block(m)
        if leak(b, p):
            bad.append((b, "leak"))
        if [[Fraction(x) for x in row] for row in block] != spectra_closed_form(b):
            bad.append((b, "form"))
        counts = power_column(m, 8)
        for k in range(8):
            h, t = counts[k]
            if counts[k + 1] != (block[0][0] * h + block[0][1] * t, block[1][0] * h + block[1][1] * t):
                bad.append((b, "step", k))
        top = {3: 6, 5: 5, 7: 4, 9: 4}.get(b, 3)
        for k in range(top + 1):
            if layer_census(b, p, k) != counts[k]:
                bad.append((b, "census", k))
        print("b=%d g=%d block=%s hexagons=%s" % (b, 3 * (b - 1) // 2, block, [h for h, _ in counts[:5]]))
    print("odd bases 3..25: %d bases, mismatches=%s" % (len(BASES), bad))


def binomial2(x):
    return x * (x - 1) / 2


def extraction(kind, n, idx):
    if kind == "A":
        terms = [(1, 0), (-3, n), (3, 2 * n), (-1, 3 * n)]
    else:
        terms = [(1, 0), (-2, n), (1, 2 * n), (-1, n - 1), (2, 2 * n - 1), (-1, 3 * n - 1)]
    return [(sign, idx - shift) for sign, shift in terms]


def settle(expr, n):
    poly = sp.Poly(sp.expand(expr), n)
    alpha, beta = poly.coeff_monomial(n), poly.coeff_monomial(1)
    if alpha > 0:
        return True, sp.ceiling(-beta / alpha)
    if alpha < 0:
        return False, sp.floor(-beta / alpha) + 1
    return beta >= 0, 0


def closed_coefficient(parity, n, m, s_of_n):
    nn = 2 * m + parity
    s = s_of_n.subs(n, nn)
    even = sp.Poly(sp.expand(s), m).coeff_monomial(m) % 2 == 0 and int(s.subs(m, 0)) % 2 == 0
    idx = (s / 2) if even else ((s - 1) / 2)
    kind = "A" if even else "B"
    total, threshold = 0, 2
    for sign, arg in extraction(kind, nn, idx):
        active, since = settle(arg, m)
        threshold = max(threshold, 2 * since + parity)
        if active:
            total += sign * binomial2(arg + 2)
    if not even:
        total *= 3
    return sp.expand(total), threshold


ENTRIES = {
    "P[g]": lambda n: 3 * (n - 1),
    "P[g+1]": lambda n: 3 * (n - 1) + 1,
    "P[g+b]": lambda n: 3 * (n - 1) + 2 * n - 1,
    "P[g+b-1]": lambda n: 3 * (n - 1) + 2 * n - 2,
    "P[g+b+1]": lambda n: 3 * (n - 1) + 2 * n,
}


def derive_forms():
    n, m, b = sp.symbols("n m b")
    forms = {}
    for parity in (0, 1):
        for name, s_of_n in ENTRIES.items():
            poly_m, threshold = closed_coefficient(parity, n, m, s_of_n(n))
            poly_b = sp.factor(sp.expand(poly_m.subs(m, ((b + 1) / 2 - parity) / 2)))
            forms[(parity, name)] = (poly_b, threshold)
    return forms


def evaluate(expr, b_value):
    b = sp.Symbol("b")
    return sp.Rational(expr.subs(b, b_value))


def verb_forms():
    print("FORMS: the five coefficients as polynomials in b, by coefficient extraction on E^2 (E + 3 O)")
    forms = derive_forms()
    b = sp.Symbol("b")
    for parity, klass in ((0, "b = 3 mod 4"), (1, "b = 1 mod 4")):
        for name in ENTRIES:
            poly_b, threshold = forms[(parity, name)]
            print("%s: %s = %s, extraction stable from b >= %d" % (klass, name, poly_b, 2 * threshold - 1))
    bad = []
    for b_value in range(3, 102, 2):
        p = generating_polynomial(b_value)
        parity = 0 if b_value % 4 == 3 else 1
        for name, s_of_n in ENTRIES.items():
            s = s_of_n((b_value + 1) // 2)
            if evaluate(forms[(parity, name)][0], b_value) != coefficient(p, s):
                bad.append((b_value, name))
    print("closed forms against the digit polynomial at odd b = 3..101: mismatches=%s, every extraction threshold is at most b = %d" % (bad, max(2 * t - 1 for _, t in forms.values())))
    block = {}
    for parity in (0, 1):
        a = forms[(parity, "P[g]")][0]
        c = forms[(parity, "P[g+b]")][0]
        d = 2 * forms[(parity, "P[g+1]")][0]
        e = forms[(parity, "P[g+b-1]")][0] + forms[(parity, "P[g+b+1]")][0]
        block[parity] = [[sp.factor(a), sp.factor(c)], [sp.factor(d), sp.factor(e)]]
    spectra = {
        0: [[3 * (b + 1) * (3 * b - 1) / 16, (b + 1) * (b + 5) / 32], [3 * (b + 1) ** 2 / 8, 3 * (b + 1) ** 2 / 16]],
        1: [[(3 * b * b + 6 * b + 7) / 16, 3 * (b - 1) * (b + 3) / 32], [3 * (b - 1) * (3 * b + 5) / 8, (b + 3) ** 2 / 16]],
    }
    for parity, klass in ((0, "b = 3 mod 4"), (1, "b = 1 mod 4")):
        same = all(sp.expand(block[parity][i][j] - spectra[parity][i][j]) == 0 for i in range(2) for j in range(2))
        tr = sp.factor(block[parity][0][0] + block[parity][1][1])
        det = sp.factor(block[parity][0][0] * block[parity][1][1] - block[parity][0][1] * block[parity][1][0])
        print("%s: block %s equals spectra's closed form: %s, trace %s, det %s" % (klass, block[parity], same, tr, det))
    return block


def positive_from(poly, n, start):
    p = sp.Poly(sp.expand(poly), n)
    roots = p.intervals()
    top = max((hi for (lo, hi), _ in roots), default=sp.Integer(-10**9))
    value = p.eval(start)
    return top < start and value > 0, top, value


def verb_split():
    print("SPLIT: the sign of rho_b - fill/b as a polynomial inequality in n = (b + 1)/2")
    b, n = sp.symbols("b n")
    block = verb_forms()
    fill = n**2 * (4 * n - 3)
    q = fill / (2 * n - 1)
    for parity, klass, start, want in ((0, "b = 3 mod 4", 2, "above"), (1, "b = 1 mod 4", 3, "below")):
        blk = [[e.subs(b, 2 * n - 1) for e in row] for row in block[parity]]
        tr = sp.expand(blk[0][0] + blk[1][1])
        det = sp.expand(blk[0][0] * blk[1][1] - blk[0][1] * blk[1][0])
        disc = sp.expand(tr**2 - 4 * det)
        u = sp.expand(sp.cancel((tr - 2 * q) * (2 * n - 1)))
        chi = sp.expand(sp.cancel((q**2 - tr * q + det) * (2 * n - 1) ** 2))
        ok_disc, top_disc, _ = positive_from(disc, n, start)
        ok_u, top_u, _ = positive_from(-u, n, start)
        sign = -1 if want == "above" else 1
        ok_chi, top_chi, val_chi = positive_from(sign * chi, n, start)
        print("%s: trace %s, det %s, n >= %d" % (klass, sp.factor(tr), sp.factor(det), start))
        print("  disc > 0: %s (largest root bound %s)" % (ok_disc, top_disc))
        print("  (2n-1)(tr - 2q) = %s < 0: %s (largest root bound %s)" % (sp.factor(u), ok_u, top_u))
        print("  (2n-1)^2 chi(q) = %s, needs sign %+d: %s (largest root bound %s, value at n=%d is %s)" % (sp.factor(chi), sign, ok_chi, top_chi, start, sign * val_chi))
        print("  hence rho_b %s fill/b for every b = %s: %s" % (">" if want == "above" else "<", klass[4:], ok_disc and ok_u and ok_chi))


def verb_classes():
    print("CLASSES: residue-class sums of P_b are the row sums of the automaton, and P_b(w) = -2/(1 + w)^3 at w^b = 1, w != 1")
    w = sp.Symbol("w")
    bad = []
    for b in BASES:
        p = generating_polynomial(b)
        poly = sum(c * w**s for s, c in enumerate(p))
        rem = sp.rem(sp.expand(poly * (1 + w) ** 3 + 2), sum(w**k for k in range(b)), w)
        if rem != 0:
            bad.append((b, "root"))
        g = 3 * (b - 1) // 2
        m = automaton(b, p)
        for c in (-1, 0, 1):
            row = sum(m[c, d] for d in (-1, 0, 1))
            klass = sum(v for s, v in enumerate(p) if (s - g - c) % b == 0)
            if row != klass:
                bad.append((b, "row", c))
    print("odd b = 3..25: mismatches=%s" % bad)
    b = sp.Symbol("b")
    forms = derive_forms()
    for parity, klass in ((0, "b = 3 mod 4"), (1, "b = 1 mod 4")):
        f = {name: forms[(parity, name)][0] for name in ENTRIES}
        mid = sp.factor(f["P[g]"] + 2 * f["P[g+b]"])
        side = sp.factor(f["P[g+1]"] + f["P[g+b-1]"] + f["P[g+b+1]"])
        wrong = []
        for b_value in range(3 + parity * 2, 102, 4):
            p = generating_polynomial(b_value)
            g = 3 * (b_value - 1) // 2
            sums = [sum(v for s, v in enumerate(p) if (s - g - c) % b_value == 0) for c in (0, 1, -1)]
            if sums != [evaluate(mid, b_value), evaluate(side, b_value), evaluate(side, b_value)]:
                wrong.append(b_value)
        print("%s: class of g sums to %s, classes of g +- 1 sum to %s each, mismatches to b = 101: %s" % (klass, mid, side, wrong))


VERBS = {"block": verb_block, "forms": verb_forms, "split": verb_split, "classes": verb_classes}


def main():
    names = sys.argv[1:] or ["block", "split", "classes"]
    for name in names:
        start = time.time()
        VERBS[name]()
        print("verb %s took %.1fs" % (name, time.time() - start))
        print("")


main()
