from fractions import Fraction
from math import comb, log
import mpmath
import numpy as np
import sympy as sp

DIGITS = ((0, 0), (1, 0), (0, 1))
LOG3 = log(3)
KAPPA = 3 - log(5) / LOG3
WALL = 2 / (3 + log(5) / LOG3)

def log3(x):
    return log(x) / LOG3

def beta(kappa):
    return kappa / (2 * kappa + 2 - KAPPA)

def delta_counts(K):
    types = [(b, c, comb(K, b) * comb(K - b, c)) for b in range(K + 1) for c in range(K + 1 - b)]
    out = {}
    for b, c, m in types:
        for B, C, M in types:
            out[(b - B, c - C)] = out.get((b - B, c - C), 0) + m * M
    return out

def carry_matrix(K):
    radius = (K - 1) // 2
    states = [(i, j) for i in range(-radius, radius + 1) for j in range(-radius, radius + 1)]
    index = {s: i for i, s in enumerate(states)}
    M = [[0] * len(states) for _ in states]
    for s in states:
        for (d1, d2), w in delta_counts(K).items():
            z1, z2 = s[0] + d1, s[1] + d2
            if z1 % 3 == 0 and z2 % 3 == 0:
                M[index[s]][index[(z1 // 3, z2 // 3)]] += w
    return M, index[(0, 0)]

def energies(K, levels):
    M, zero = carry_matrix(K)
    v = [0] * len(M)
    v[zero] = 1
    out = [1]
    for _ in range(levels):
        v = [sum(v[i] * M[i][j] for i in range(len(v))) for j in range(len(v))]
        out.append(v[zero])
    return out

def gasket(a):
    pts = [(0, 0)]
    for l in range(a):
        pts = [(x + dx * 3**l, y + dy * 3**l) for x, y in pts for dx, dy in DIGITS]
    return pts

def energy_direct(K, a):
    pts = gasket(a)
    top = K * (3**a - 1) + 1
    sums = np.zeros((top, top), dtype=np.int64)
    sums[0, 0] = 1
    for _ in range(K):
        fresh = np.zeros_like(sums)
        for x, y in pts:
            fresh[x:, y:] += sums[: top - x, : top - y]
        sums = fresh
    return sum(int(v) * int(v) for v in sums.flat if v)

print("kappa = 3 - log_3 5 = %.6f, beta_0^(4) = %.6f, wall 2/(3 + log_3 5) = %.12f" % (KAPPA, beta(KAPPA), WALL))

print("energies E_2K(G_a), carry matrix exact integers, a = 0..6")
rows = {}
for K in (2, 3, 4, 5):
    rows[K] = energies(K, 6)
    print("2K = %d:" % (2 * K), rows[K])
print("E_4 = 15^a for a <= 6:", all(rows[2][a] == 15**a for a in range(7)))
for K in (2, 3, 4, 5):
    top = 4 if K < 5 else 3
    agree = all(energy_direct(K, a) == rows[K][a] for a in range(1, top + 1))
    print("2K = %d direct convolution agrees at a = 1..%d: %s" % (2 * K, top, agree))

print("characteristic polynomials and Perron roots")
x = sp.symbols("x")
mpmath.mp.dps = 30
perron = {}
for K in (3, 4, 5):
    M, _ = carry_matrix(K)
    poly = sp.Matrix(M).charpoly(x).as_expr()
    print("2K = %d states %d factors: %s" % (2 * K, len(M), sp.factor(poly)))
    perron[K] = max(mpmath.mpf(str(r)) for f, _ in sp.factor_list(poly)[1] for r in sp.Poly(f, x).nroots(n=25) if r.is_real)
lam6 = 57 + 6 * mpmath.sqrt(46)
lam8 = 456 + 3 * mpmath.sqrt(11017)
lam10 = perron[5]
print("lambda_6 = 57 + 6 sqrt 46 = %s, matches largest factor root %s" % (mpmath.nstr(lam6, 12), abs(lam6 - perron[3]) < 1e-15))
print("lambda_8 = 456 + 3 sqrt 11017 = %s, matches largest factor root %s" % (mpmath.nstr(lam8, 12), abs(lam8 - perron[4]) < 1e-15))
print("lambda_10 largest root of the quartic factor = %s" % mpmath.nstr(lam10, 13))

print("ladder rungs: moments, kappa_2K, beta_0^(2K)")
kappas = {}
for K, lam in ((2, 15), (3, lam6), (4, lam8), (5, lam10)):
    kappas[K] = 2 * K - mpmath.log(lam) / mpmath.log(3)
    print("2K = %2d kappa %s beta %s" % (2 * K, mpmath.nstr(kappas[K], 13), mpmath.nstr(beta(kappas[K]), 12)))
print("tenth rung above eighth by %s" % mpmath.nstr(beta(kappas[5]) - beta(kappas[4]), 9))
print("Holder block exponents 2K = 6, 8, 10:", ", ".join(str(Fraction(2, 1) / (1 - Fraction(1, 2 * K))) for K in (3, 4, 5)))

print("rows above the tenth, floating Perron roots of exact matrices")
for K in range(6, 11):
    M, _ = carry_matrix(K)
    lam = max(np.linalg.eigvals(np.array(M, dtype=float)).real)
    kappa = 2 * K - log3(lam)
    print("2K = %2d states %3d lambda %.6f beta %.9f wall - beta %.2e" % (2 * K, len(M), lam, beta(kappa), WALL - beta(kappa)))

def primes(lo, hi):
    return list(sp.primerange(lo, hi + 1))

def fourier(p, n):
    t = np.arange(p)
    e = np.exp(2j * np.pi * t / p)
    W = np.abs(1 + e[:, None] + e[None, :]) / 3
    F = np.ones((p, p))
    out = []
    for l in range(n):
        I = (t * pow(3, l, p)) % p
        F = F * W[np.ix_(I, I)]
        out.append(F.copy())
    return out

print("moment identities on (Z/p)^2, primes 5..199")
checks, worst = 0, 0.0
for p in primes(5, 199):
    F = fourier(p, 6)
    for a in range(1, 6):
        targets = []
        if 3**a <= p:
            targets.append((2, p * p * 3.0 ** (-a)))
        if 2 * 3**a <= p:
            targets.append((4, p * p * (5 / 27) ** a))
        for K in (3, 4, 5):
            if K * (3**a - 1) < p and a < len(rows[K]):
                targets.append((2 * K, p * p * rows[K][a] / 3 ** (2 * K * a)))
        for power, target in targets:
            worst = max(worst, abs((F[a - 1] ** power).sum() - target) / target)
            checks += 1
print("identities checked %d, worst relative error %.1e" % (checks, worst))

def half(p):
    a = 0
    while 2 * 3 ** (a + 1) <= p:
        a += 1
    return a

def digits(p, K):
    a = 0
    while K * (3 ** (a + 1) - 1) < p:
        a += 1
    return a

def master(p, n):
    a4 = half(p)
    a6, a8 = digits(p, 3), digits(p, 4)
    lam6f, lam8f = float(lam6) * (1 + 1e-12), float(lam8) * (1 + 1e-12)
    bounds = []
    if 3 ** (n // 2) <= p:
        bounds.append(p * p * 3.0 ** (-(n // 2)))
    c = min(a4, n // 2)
    b = min(c, n - 2 * c)
    bounds.append(p * p * 3.0 ** (-c / 2) * (5 / 27) ** ((c + b) / 4))
    b6 = min(a6, max(0, n - 2 * c))
    bounds.append(p * p * 3.0 ** (-(4 + KAPPA) * c / 6) * (lam6f / 729) ** (b6 / 6))
    b8 = min(a8, max(0, n - 2 * c))
    bounds.append(p * p * 3.0 ** (-(6 + KAPPA) * c / 8) * (lam8f / 6561) ** (b8 / 8))
    if a6 > 0 and n >= 6 * a6:
        bounds.append(p * p * (lam6f / 729) ** a6)
    if a8 > 0 and n >= 8 * a8:
        bounds.append(p * p * (lam8f / 6561) ** a8)
    return min(bounds)

print("master bound against exact L_n(p), primes 5..199, n = 2..24")
violations, worst, where = 0, 0.0, None
for p in primes(5, 199):
    for n, F in enumerate(fourier(p, 24), start=1):
        if n < 2:
            continue
        ratio = F.sum() / master(p, n)
        if ratio > 1 + 1e-9:
            violations += 1
        if ratio > worst:
            worst, where = ratio, (p, n)
print("violations %d, worst ratio L/bound %.4f at (p, n) = %s" % (violations, worst, where))

def regime(p, n):
    a = half(p)
    if 2 * a > n:
        return None
    return "I" if n >= 4 * a else "II" if n >= 3 * a else "III"

def tail_bound(p, n):
    a = half(p)
    r = regime(p, n)
    if r == "I":
        return p * p * (5 / 27) ** a
    if r == "II":
        return p * p * (5 / 81) ** (a / 2)
    return p * p * 3.0 ** (-a / 2) * (5 / 27) ** ((n - a) / 4)

print("dyadic tail: exponents (kappa - 1)/8 = %.4f, (2 + kappa)/4 = %.4f" % ((KAPPA - 1) / 8, (2 + KAPPA) / 4))
print("tail constants 16/(kappa - 1) = %.2f, 10 (1 + kappa)/(kappa - 1) 2^((1 - kappa)/2) = %.2f, 8/(2 + kappa) = %.2f" % (16 / (KAPPA - 1), 10 * (1 + KAPPA) / (KAPPA - 1) * 2 ** ((1 - KAPPA) / 2), 8 / (2 + KAPPA)))
uncovered = [(p, n) for n in (8, 10, 12, 14, 16, 20, 24) for p in primes(5, 199) if p <= 3 ** (beta(KAPPA) * n) and regime(p, n) is None]
print("regime trichotomy total below 3^(beta_0 n) for n in 8..24: uncovered %d" % len(uncovered))
fails, cases = 0, 0
for n in (8, 10, 12):
    for p in primes(5, 199):
        if regime(p, n) is None:
            continue
        cases += 1
        R = fourier(p, n)[-1].sum() - 1
        if R > tail_bound(p, n) * (1 + 1e-9):
            fails += 1
print("per-prime tail bounds at n = 8, 10, 12: %d cases, %d failures" % (cases, fails))
