import itertools
import math
import sys
import time
from collections import defaultdict
from fractions import Fraction
from functools import lru_cache
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "level-tiles"))
from level_tiles import cyclo, divisors, every_level, factor, normalize, omega, phi, polydiv, zero_set

# ORDERS AND HADAMARD TRIPLES

def orders(E, b):
    m0 = min(E)
    out = set()
    for m in divisors(b)[1:]:
        red = [0] * m
        for e in E:
            red[(e - m0) % m] += 1
        if not polydiv(red + [0], cyclo(m))[1]:
            out.add(m)
    return out

def cliques(allowed, b, k, cap=64, step=1):
    cand = [d for d in range(step, b, step) if b // math.gcd(b, d) in allowed]
    ok = lambda x, y: b // math.gcd(b, (x - y) % b) in allowed
    found = []
    def rec(R, P):
        if len(found) >= cap:
            return
        if len(R) == k:
            found.append(list(R))
            return
        for i, v in enumerate(P):
            if len(R) + len(P) - i < k:
                return
            rec(R + [v], [w for w in P[i + 1:] if ok(v, w)])
    rec([0], cand)
    return found

def hadamard(F, b):
    got = cliques(orders(F, b), b, len(F), cap=1)
    return got[0] if got else None

def is_hadamard(blocks, L, b):
    if len({x % b for x in L}) != len(L) or len(L) != math.prod(len(B) for B in blocks):
        return False
    os = [orders(B, b) for B in blocks]
    for x, y in itertools.combinations(L, 2):
        m = b // math.gcd(b, (x - y) % b)
        if not any(m in o for o in os):
            return False
    return True

def sumset(parts):
    out = [0]
    for P in parts:
        out = [x + y for x in out for y in P]
    return out

# STAGE FORM

def digits(f, B, J):
    return [(f // B ** j) % B for j in range(J)]

def stage_form(F, b):
    fb = dict(factor(b))
    P = [p for p, _ in factor(len(F))]
    if any(p not in fb for p in P):
        return None
    bP = math.prod(p ** fb[p] for p in P)
    b2 = b // bP
    J = 1
    while bP ** J <= max(F):
        J += 1
    K = J - 1
    dg = {f: digits(f, bP, J) for f in F}
    stages = []
    for j in range(J):
        groups = defaultdict(set)
        for f in F:
            groups[tuple(dg[f][:j])].add(dg[f][j] * b2 ** (K - j))
        stages.append(sorted({tuple(sorted(s)) for s in groups.values()}))
    options = []
    for sets in stages:
        k = len(sets[0])
        if any(len(s) != k for s in sets):
            return None
        if k == 1:
            options.append([[0]])
            continue
        allowed = set.intersection(*[orders(s, b) for s in sets])
        Ls = cliques(allowed, b, k, step=b2)
        if not Ls:
            return None
        options.append(Ls)
    for Ls in itertools.product(*options):
        if all(is_hadamard(list(ch), sumset([Ls[j] for j in rng]), b) for m in range(J) for rng in (range(m + 1), range(m, J)) for ch in itertools.product(*[stages[j] for j in rng])):
            return [list(x) for x in Ls]
    return None

# FOURIER

def hat_mu(F, b, xi):
    F = np.asarray(F, dtype=float)
    out = np.ones_like(xi, dtype=complex)
    s = np.max(np.abs(xi)) + 1.0
    j = 1
    while s * max(F.max(), 1.0) / b ** (j - 1) > 1e-13:
        out *= np.exp(-2j * np.pi * np.outer(xi / b ** j, F)).mean(axis=1)
        j += 1
    return out

def q_values(F, b, spec, xi):
    spec = np.asarray([float(x) for x in spec])
    tot = np.zeros(len(xi))
    for ch in np.array_split(spec, max(1, len(spec) // 4096)):
        X = (xi[:, None] + ch[None, :]).ravel()
        tot += (np.abs(hat_mu(F, b, X)) ** 2).reshape(len(xi), len(ch)).sum(axis=1)
    return tot

def orthogonal(F, b, spec):
    Z = set(zero_set(F))
    for x, y in itertools.combinations(spec, 2):
        d = Fraction(x) - Fraction(y)
        bad = True
        for j in range(1, 80):
            t = d / b ** j
            if t.denominator > 1 and t.denominator in Z:
                bad = False
                break
            if t.denominator > 10 ** 6:
                break
        if bad:
            return False
    return True

def lam(b, L, n):
    out = [0]
    for i in range(n):
        out = [x + b ** i * l for x in out for l in L]
    return out

def exponent_candidate(F, b, n):
    k = len(F)
    (p, r), = factor(k)
    e = dict(factor(b))[p]
    bp = b // p ** e
    Z = zero_set(F)
    cs = sorted(a for a in range(1, 64) if p ** a in Z)
    q = {c: (c - 1) // e for c in cs}
    rho = {c: c - e * q[c] for c in cs}
    J = max(q.values())
    L0 = [Fraction(0)]
    for c in cs:
        for j in range(1, q[c] + 1):
            L0 = [x + Fraction(s * bp ** J * p ** (j * e), p ** c) for x in L0 for s in range(p)]
    L = sumset([[s * b // p ** rho[c] for s in range(p)] for c in cs])
    return [x + bp ** J * g for x in L0 for g in lam(b, L, n)], cs, L0, L

def q_report(F, b, spec_of_n, ns, xi):
    out = []
    for n in ns:
        out.append(1 - q_values(F, b, spec_of_n(n), xi).min())
    return out

# VERBS

def census(bfull=16, bmax=30, cap=130_000):
    t0 = time.time()
    cells = defaultdict(lambda: [0, 0, 0, 0])
    gaps = []
    def visit(b, k, F):
        Z = zero_set(F)
        if not every_level(Z, k, b)[0]:
            return
        c = cells[(b, k)]
        c[0] += 1
        G = normalize(F)
        if hadamard(G, b) is not None:
            c[1] += 1
        elif stage_form(F, b) is not None or stage_form(G, b) is not None:
            c[2] += 1
        else:
            c[3] += 1
            gaps.append((b, F))
    for b in range(2, bfull + 1):
        for k in range(2, b + 1):
            if b % k == 0:
                for rest in itertools.combinations(range(1, b), k - 1):
                    visit(b, k, (0,) + rest)
    skipped = []
    for b in range(bfull + 1, bmax + 1):
        for k in range(2, b):
            if b % k or (omega(k) < 2 and k not in (4, 8, 9)):
                continue
            if math.comb(b - 1, k - 1) > cap:
                skipped.append((b, k, math.comb(b - 1, k - 1)))
                continue
            for rest in itertools.combinations(range(1, b), k - 1):
                visit(b, k, (0,) + rest)
    tot = [sum(c[i] for c in cells.values()) for i in range(4)]
    low = [sum(c[i] for (b, k), c in cells.items() if b <= bfull) for i in range(4)]
    print("every-level tiling digit sets: bases 2..%d all sizes dividing b; bases to %d sizes 4, 8, 9 and two or more primes, cells up to %d sets" % (bfull, bmax, cap))
    print("bases 2..%d: %d, Hadamard after the gcd %d, stage form %d, neither %d" % (bfull, *low))
    print("total %d: Hadamard after the gcd %d, stage form %d, neither %d" % tuple(tot))
    for (b, k), c in sorted(cells.items()):
        if c[2] or c[3] or (omega(k) >= 2 and b > 12):
            print("  base %d size %d: every level %d, Hadamard %d, stage %d, neither %d" % (b, k, *c))
    pp = sum(c[2] for (b, k), c in cells.items() if omega(k) == 1)
    print("stage form at prime-power size: %d; at two or more primes: %d" % (pp, tot[2] - pp))
    for b, F in gaps[:10]:
        print("  neither:", b, F)
    for s in skipped:
        print("  skipped base %d size %d (%d sets)" % s)
    print("time %.1f s" % (time.time() - t0))

def ahl_shape(F, b):
    v = min((f & -f).bit_length() - 1 for f in F if f)
    G = [f >> v for f in F]
    ev = sorted(f for f in G if f % 2 == 0)
    od = sorted(f for f in G if f % 2)
    if len(ev) != 2 or len(od) != 2:
        return False
    t1 = ((ev[1] - ev[0]) & -(ev[1] - ev[0])).bit_length() - 1
    t2 = ((od[1] - od[0]) & -(od[1] - od[0])).bit_length() - 1
    beta = (b & -b).bit_length() - 1
    return t1 == t2 and beta >= 1 and t1 % beta != 0

def four(bmax=36):
    t0 = time.time()
    tot = agree = ev = 0
    for b in range(4, bmax + 1):
        for rest in itertools.combinations(range(1, b), 3):
            F = (0,) + rest
            ours = every_level(zero_set(F), 4, b)[0]
            tot += 1
            ev += ours
            agree += ours == ahl_shape(F, b)
    print("four-digit sets {0,a,b,c} in {0..b-1}, bases 4..%d: %d sets, %d tile at every level" % (bmax, tot, ev))
    print("every level tiles  ==  the An-He-Lai Theorem 1.7 condition: %d of %d" % (agree, tot))
    print("time %.1f s" % (time.time() - t0))

def spectra():
    t0 = time.time()
    rng = np.random.default_rng(1)
    xi = np.concatenate([[0.0, 1.0, 0.5, 1 / 3], rng.uniform(-1, 1, 28)])
    rows = []
    rows.append(("{0,2} base 4, Lambda(4,{0,1})", (0, 2), 4, lambda n: lam(4, [0, 1], n), range(4, 13, 4)))
    rows.append(("{0,2} base 4, Lambda(4,{0,3}) control", (0, 2), 4, lambda n: lam(4, [0, 3], n), range(4, 13, 4)))
    rows.append(("{0,1,8,9} base 12, {0,3/4} + 3 Lambda(12,{0,3,6,9})", (0, 1, 8, 9), 12, lambda n: [x + 3 * g for x in (0, Fraction(3, 4)) for g in lam(12, [0, 3, 6, 9], n)], (2, 4, 6)))
    rows.append(("{0,1,8,9} base 12, Lambda(12,{0,3,6,9}) control", (0, 1, 8, 9), 12, lambda n: lam(12, [0, 3, 6, 9], n), (2, 4, 6)))
    rows.append(("{0,1,12,13,24,25} base 30, {0,5/6,5/3} + 5 Lambda(30,{0,5,..,25})", (0, 1, 12, 13, 24, 25), 30, lambda n: [x + 5 * g for x in (0, Fraction(5, 6), Fraction(5, 3)) for g in lam(30, [0, 5, 10, 15, 20, 25], n)], (2, 3, 5)))
    for F, b in [((0, 1, 8, 25), 28)]:
        _, cs, L0, L = exponent_candidate(F, b, 1)
        rows.append(("%s base %d, exponents %s, %s + b' Lambda(%d,%s)" % (F, b, cs, [str(x) for x in L0], b, L), F, b, (lambda F, b: lambda n: exponent_candidate(F, b, n)[0])(F, b), (2, 3, 4) if len(F) == 8 else (2, 4, 6)))
    for name, F, b, spec, ns in rows:
        o = orthogonal(F, b, spec(2))
        gaps = q_report(F, b, spec, ns, xi)
        print("%s: orthogonal %s; 1 - min Q over %d points at n = %s: %s" % (name, o, len(xi), list(ns), ", ".join("%.2e" % g for g in gaps)))
    print("time %.1f s" % (time.time() - t0))

def tree_sets(p, cs, b):
    J = 1
    while p ** J < b or J < max(cs):
        J += 1
    classes = [[0]]
    for j in range(1, J + 1):
        nxt = []
        for cl in classes:
            if j in cs:
                nxt.append([r + s * p ** (j - 1) for r in cl for s in range(p)])
            else:
                opts = [[r + s * p ** (j - 1) for s in range(p)] for r in cl]
                opts[0] = [cl[0]]
                for pick in itertools.product(*opts):
                    nxt.append(list(pick))
        classes = nxt
    for cl in classes:
        if max(cl) < b:
            yield tuple(sorted(cl))

def hunt():
    t0 = time.time()
    rows = []
    for b in (12, 20, 24, 28, 36, 40, 44, 48):
        e = dict(factor(b))[2]
        for r in range(2, e + 1):
            for cs in itertools.combinations(range(1, 8), r):
                if len({c % e for c in cs}) < r or max(cs) <= e or min(cs) > 1:
                    continue
                for F in tree_sets(2, set(cs), b):
                    if math.gcd(*F) == 1:
                        assert every_level(zero_set(F), len(F), b)[0]
                        rows.append((b, len(F), F, stage_form(F, b) is not None))
    for b in (36, 45, 72):
        e = dict(factor(b))[3]
        for cs in itertools.combinations(range(1, 6), 2):
            if len({c % e for c in cs}) < 2 or max(cs) <= e or min(cs) > 1:
                continue
            for F in tree_sets(3, set(cs), b):
                if math.gcd(*F) == 1:
                    assert every_level(zero_set(F), len(F), b)[0]
                    rows.append((b, len(F), F, stage_form(F, b) is not None))
    cnt = defaultdict(lambda: [0, 0])
    for b, k, F, ok in rows:
        cnt[(b, k)][0] += 1
        cnt[(b, k)][1] += ok
    print("tree digit sets with gcd 1 and an exponent above e_p: %d, every one tiling at every level; in stage form: %d" % (len(rows), sum(r[3] for r in rows)))
    for (b, k), (n, ok) in sorted(cnt.items()):
        print("  base %d size %d: %d sets, stage form %d" % (b, k, n, ok))
    for b, k, F, ok in rows:
        if not ok:
            print("  not in stage form: base %d %s" % (b, F))
    print("time %.1f s" % (time.time() - t0))

def pq_shape(F, b):
    k = len(F)
    (p1, _), (p2, _) = factor(k)
    fb = dict(factor(b))
    Z = zero_set(F)
    for p, q in ((p1, p2), (p2, p1)):
        c = [a for a in range(1, 40) if p ** a in Z]
        d = [a for a in range(1, 40) if q ** a in Z]
        if len(c) != 1 or d != [1]:
            continue
        c = c[0]
        M = p ** (c - 1) * q
        groups = defaultdict(list)
        for f in F:
            groups[f % M].append(f)
        if len(groups) != q or any(len(g) != p for g in groups.values()):
            continue
        X = sorted(groups)
        K = {x: [(f - x) // M for f in groups[x]] for x in X}
        if len({x % q for x in X}) != q or any(len({t % p for t in K[x]}) != p for x in X):
            continue
        e, f2 = fb[p], fb[q]
        if c <= e:
            return "window"
        a = -(-c // e) - 1
        Q = q ** (f2 * a)
        if any(len({t % Q for t in K[x]}) != 1 for x in X):
            return None
        s = (b // (p ** e * q ** f2)) ** a
        A = [s * (x + M * K[x][0]) for x in X]
        B = [[p ** (c - 1 - a * e) * q * ((t - K[x][0]) // Q) for t in K[x]] for x in X]
        if sorted(Ai + b ** a * Bt for Ai, Bj in zip(A, B) for Bt in Bj) != sorted(s * f for f in F):
            return None
        L1 = [b // q * u for u in range(q)]
        L2 = [b // p ** (c - a * e) * v for v in range(p)]
        ok = is_hadamard([A], L1, b) and all(is_hadamard([Bj], L2, b) and is_hadamard([A, Bj], sumset([L1, L2]), b) for Bj in B)
        return ("product", p, c, a) if ok else None
    return None

def pq(cells=((30, 30), (36, 36), (42, 42), (48, 48), (66, 36)), k=6, chunk=400000):
    t0 = time.time()
    (p, _), (q, _) = factor(k)
    for b, top in cells:
        t1 = time.time()
        ords = sorted(m for m in range(2, 4 * b * b) if {x for x, _ in factor(m)} <= {p, q} and phi(m) <= b - 1)
        pp = [m for m in ords if len(factor(m)) == 1]
        it = itertools.combinations(range(1, top), k - 1)
        surv = []
        while True:
            flat = np.fromiter(itertools.chain.from_iterable(itertools.islice(it, chunk)), dtype=np.int32)
            if flat.size == 0:
                break
            A = np.concatenate([np.zeros((flat.size // (k - 1), 1), dtype=np.int32), flat.reshape(-1, k - 1)], axis=1)
            Z = {m: np.abs(np.exp(2j * np.pi * np.arange(b) / m)[A].sum(1)) < 1e-6 for m in ords}
            cp = sum(Z[m].astype(int) for m in pp if m % p == 0)
            cq = sum(Z[m].astype(int) for m in pp if m % q == 0)
            surv.extend(tuple(int(x) for x in row) for row in A[(cp == 1) & (cq == 1)])
        res = defaultdict(int)
        for F in surv:
            if not every_level(zero_set(F), k, b)[0]:
                continue
            res["every"] += 1
            G = normalize(F)
            if hadamard(G, b) is not None:
                res["hadamard"] += 1
            else:
                res["outside"] += 1
                sh = pq_shape(G, b)
                res["product form" if sh and sh[0] == "product" else "unexplained"] += 1
        print("size %d base %d digits below %d: %d sets pass T1, every level %d, Hadamard after the gcd %d, outside %d, of them in the product form of the proof %d, unexplained %d (%.1f s)" % (k, b, top, len(surv), res["every"], res["hadamard"], res["outside"], res["product form"], res["unexplained"], time.time() - t1), flush=True)
    print("time %.1f s" % (time.time() - t0))

# CONVERSE

def rad(n):
    return math.prod(p for p, _ in factor(n))

def vq(n, q):
    v = 0
    while n % q == 0:
        n //= q
        v += 1
    return v

@lru_cache(maxsize=None)
def reducer(m):
    c = list(cyclo(m))
    deg = len(c) - 1
    rows = []
    for i in range(m):
        rem = polydiv([0] * i + [1], c)[1] if i >= deg else [0] * i + [1]
        rows.append(rem + [0] * (deg - len(rem)))
    return np.array(rows, dtype=np.int64)

def orders_b(A, b):
    rb = rad(b)
    ms = [m for m in range(2, 4 * b * b) if rb % rad(m) == 0 and phi(m) <= b - 1]
    rows = np.arange(A.shape[0])[:, None]
    out = [set() for _ in range(A.shape[0])]
    for m in ms:
        cnt = np.zeros((A.shape[0], m), dtype=np.int64)
        np.add.at(cnt, (rows, A % m), 1)
        hit = ~(cnt @ reducer(m)).any(axis=1)
        for i in np.nonzero(hit)[0]:
            out[i].add(m)
    return out

def level(m, b):
    return max(-(-vq(m, q) // e) for q, e in factor(b))

def signature(m, b):
    j = level(m, b)
    return tuple(("d", j * e - vq(m, q)) if m % q == 0 and j * e - vq(m, q) < e else "s" for q, e in factor(b))

def res_signature(d, b):
    return tuple(("d", vq(d % q ** e, q)) if d % q ** e else "s" for q, e in factor(b))

def residue_clique(Z, b, k):
    sigs = {signature(m, b) for m in Z}
    ok = [res_signature(d, b) in sigs for d in range(b)]
    best = [1]
    def rec(size, P):
        best[0] = max(best[0], size)
        if best[0] >= k:
            return True
        for i, v in enumerate(P):
            if size + len(P) - i <= best[0]:
                return False
            if rec(size + 1, [w for w in P[i + 1:] if ok[(w - v) % b]]):
                return True
        return False
    if sigs:
        rec(1, [d for d in range(1, b) if ok[d]])
    return best[0]

def integer_clique(Z, b, k):
    J = max(level(m, b) for m in Z)
    B = b ** J
    ok = lambda d: d % b != 0 and any(b ** j // math.gcd(d % B, b ** j) in Z for j in range(1, J + 1))
    for R in itertools.combinations([d for d in range(1, B) if ok(d)], k - 1):
        if len({0, *(r % b for r in R)}) == k and all(ok(y - x) for x, y in itertools.combinations(R, 2)):
            return (0,) + R
    return None

def covering(Z, b):
    fb = dict(factor(b))
    best = math.inf
    for r in range(len(fb) + 1):
        for Q in itertools.combinations(sorted(fb), r):
            if all(any(m % q == 0 for q in Q) for m in Z):
                best = min(best, math.prod(q ** len({vq(m, q) % fb[q] for m in Z if m % q == 0}) for q in Q))
    return best

def orders_smooth(F, b):
    ps = [q for q, _ in factor(b)]
    ms = [1]
    for q in ps:
        ms = [x * q ** i for x in ms for i in range(40) if x * q ** i <= 8 * max(F) + 8]
    out = set()
    for m in sorted(set(ms)):
        if m > 1 and phi(m) <= max(F):
            red = [0] * m
            for f in F:
                red[f % m] += 1
            if not polydiv(red + [0], cyclo(m))[1]:
                out.add(m)
    return out

def reaches(d, Z, b):
    return [(j, b ** j // math.gcd(d, b ** j)) for j in range(1, 8) if b ** j // math.gcd(d, b ** j) in Z]

def t1_condition(Z, b, k):
    fb = dict(factor(b))
    ex = defaultdict(list)
    for m in Z:
        f = factor(m)
        if len(f) == 1:
            ex[f[0][0]].append(f[0][1])
    if math.prod(p ** len(cs) for p, cs in ex.items()) != k:
        return False
    return all(len({c % fb[p] for c in cs}) == len(cs) for p, cs in ex.items())

def covering_cells(cells=((6, 30), (6, 42), (8, 24)), chunk=200000):
    for k, b in cells:
        t1 = time.time()
        tally = defaultdict(int)
        it = (F for F in ((0,) + r for r in itertools.combinations(range(1, b), k - 1)) if math.gcd(*F) == 1)
        while True:
            block = list(itertools.islice(it, chunk))
            if not block:
                break
            for Z in orders_b(np.array(block, dtype=np.int64), b):
                t = t1_condition(Z, b, k)
                c = covering(Z, b)
                tally["pass" if t else "fail"] += 1
                tally["pass below"] += t and c < k
                tally["fail at least"] += (not t) and c >= k
        print("size %d base %d, gcd 1: pass the T1 condition %d, of them with covering bound below %d: %d; fail %d, of them with covering bound at least %d: %d (%.1f s)" % (k, b, tally["pass"], k, tally["pass below"], tally["fail"], k, tally["fail at least"], time.time() - t1), flush=True)

def converse(cells=((5, 42), (7, 26), (11, 20), (13, 20)), chunk=200000):
    t0 = time.time()
    print("{0,1,3,5,6}: cyclotomic factors of the mask %s" % sorted(zero_set((0, 1, 3, 5, 6))))
    Z4 = orders_b(np.array([[0, 1, 9, 10]]), 12)[0]
    print("{0,1,9,10} base 12: orders %s, four integers incongruent mod 12 with orthogonal differences %s, covering bound %d" % (sorted(Z4), integer_clique(Z4, 12, 4), covering(Z4, 12)))
    F, b, Y = (0, 1, 251, 375, 501), 510, (0, 17, 1734, 1751, 176868)
    Z = orders_smooth(F, b)
    ok = len({y % b for y in Y}) == 5 and all(reaches(y - x, Z, b) for x, y in itertools.combinations(Y, 2))
    print("%s base %d: residues mod 5 %s, orders with primes in b %s, the five integers %s pairwise incongruent mod b with orthogonal differences %s, levels reached %s, covering bound %d" % (F, b, sorted(f % 5 for f in F), sorted(Z), Y, ok, sorted({j for x, y in itertools.combinations(Y, 2) for j, _ in reaches(y - x, Z, b)}), covering(Z, b)))
    t1 = time.time()
    tally = defaultdict(int)
    for b in range(4, 37):
        block = [(0,) + r for r in itertools.combinations(range(1, b), 3)]
        block = [F for F in block if math.gcd(*F) == 1]
        for F, Z in zip(block, orders_b(np.array(block, dtype=np.int64), b)):
            tiles = ahl_shape(F, b)
            c = covering(Z, b)
            tally["tile" if tiles else "not"] += 1
            tally["tile, bound below 4"] += tiles and c < 4
            tally["not, bound below 4"] += (not tiles) and c < 4
    print("size 4 bases 4..36, gcd 1: every level tiles %d, of them with covering bound below 4: %d; not %d, of them with covering bound below 4: %d (%.1f s)" % (tally["tile"], tally["tile, bound below 4"], tally["not"], tally["not, bound below 4"], time.time() - t1), flush=True)
    for p, bmax in cells:
        t1 = time.time()
        tally = defaultdict(int)
        for b in range(p, bmax + 1):
            it = (F for F in ((0,) + r for r in itertools.combinations(range(1, b), p - 1)) if math.gcd(*F) == 1)
            while True:
                block = list(itertools.islice(it, chunk))
                if not block:
                    break
                for Z in orders_b(np.array(block, dtype=np.int64), b):
                    tiles = p in Z
                    w = residue_clique(Z, b, p)
                    bad = sum(1 for m in Z if len(factor(m)) > 1)
                    tally["sets"] += 1
                    tally["tile" if tiles else "not"] += 1
                    tally["agree"] += tiles == (w >= p)
                    if not tiles:
                        tally["w%d" % w] += 1
                        tally["none" if bad == 0 else "one" if bad == 1 else "more"] += 1
                        tally["cover"] += covering(Z, b) < p
                    else:
                        tally["tile cover"] += covering(Z, b) < p
        ws = ", ".join("%d: %d" % (int(key[1:]), tally[key]) for key in sorted(k for k in tally if k[0] == "w"))
        print("size %d bases %d..%d, gcd 1: %d sets, every level tiles %d, not %d; every level tiles == residue bound >= %d on %d; non-tiling residue bounds {%s}; non-prime-power orders with primes in b: none %d, one %d, more %d; covering bound below %d: non-tiling %d, tiling %d (%.1f s)" % (p, p, bmax, tally["sets"], tally["tile"], tally["not"], p, tally["agree"], ws, tally["none"], tally["one"], tally["more"], p, tally["cover"], tally["tile cover"], time.time() - t1), flush=True)
    covering_cells()
    print("time %.1f s" % (time.time() - t0))

VERBS = {"census": census, "four": four, "spectra": spectra, "hunt": hunt, "pq": pq, "converse": converse}

if __name__ == "__main__":
    for v in sys.argv[1:] or list(VERBS):
        print("== %s" % v)
        VERBS[v]()
