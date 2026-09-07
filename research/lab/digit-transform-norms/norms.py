import itertools
import math
import sys
import time
from fractions import Fraction

import numpy as np
from mpmath import iv, mp, mpf, mpc, quad, expjpi, cos as mpcos, log as mplog

# DESIGNS

def cube_minus(q, D, drop):
    return (q, D, [v for v in itertools.product(range(q), repeat=D) if v != drop])

def base_missing(b, a0):
    return (b, 1, [(a,) for a in range(b) if a != a0])

GASKET = cube_minus(2, 2, (1, 1))
CARPET = cube_minus(3, 2, (1, 1))
GASKET4 = base_missing(4, 3)
CARPET9 = base_missing(9, 4)

ROSTER = [
    ("gasket 2D", GASKET),
    ("carpet 2D", CARPET),
    ("gasket base 4", GASKET4),
    ("carpet base 9", CARPET9),
    ("base 10 missing 0", base_missing(10, 0)),
    ("base 10 missing 1", base_missing(10, 1)),
    ("base 10 missing 5", base_missing(10, 5)),
]

# CHECKS

FAILS = []

def chk(value, want, tol):
    if abs(value - want) <= tol:
        return "ok"
    FAILS.append((value, want))
    return "MISMATCH"

def chk_le(value, cap, name):
    if value <= cap:
        return "ok"
    FAILS.append((name, value, cap))
    return "MISMATCH"

def close(t0):
    print(f"runtime {time.time() - t0:.1f}s")
    if FAILS:
        raise SystemExit(f"{len(FAILS)} rows off target: {FAILS}")
    print("every asserted row matches")

# FLOAT TRANSFORM

def mask(F, D):
    k = len(F)
    def f(t):
        acc = np.zeros(np.broadcast(*t).shape, dtype=complex)
        for v in F:
            acc = acc + np.exp(2j * np.pi * sum(v[i] * t[i] for i in range(D)))
        return np.abs(acc) / k
    return f, k

def lip_of(F):
    return (2 * math.pi / len(F)) * sum(sum(abs(c) for c in v) for v in F)

# ONE DIGIT, EXACT

def digit_sums():
    for name, (q, D, F) in ROSTER:
        k = len(F)
        assert k == q ** D - 1
        exact = Fraction(1) + Fraction(q ** D - 1, k)
        f, _ = mask(F, D)
        axes = np.meshgrid(*([np.arange(q)] * D), indexing="ij")
        num = float(f([a / q for a in axes]).sum())
        e = math.log(float(exact)) / math.log(q)
        print(f"{name}: q={q} D={D} k={k} one-digit l1 sum {exact} exact, scan {num:.12f} {chk(num, float(exact), 1e-12)}, log_q sum {e:.6f}, dim {math.log(k) / math.log(q):.6f}")

def sqrt_split(n):
    a, b = 1, n
    d = 2
    while d * d <= b:
        while b % (d * d) == 0:
            b //= d * d
            a *= d
        d += 1
    return a, b

def gasket_anchor():
    q, D, F = GASKET
    c4 = [1, 0, -1, 0]
    terms = {}
    for i in itertools.product(range(4), repeat=2):
        s = []
        for j in (0, 1):
            u = ((2 ** j) * i[0] % 4, (2 ** j) * i[1] % 4)
            s.append(3 + 2 * c4[u[0]] + 2 * c4[u[1]] + 2 * c4[(u[0] - u[1]) % 4])
        a, b = sqrt_split(s[0] * s[1])
        terms[b] = terms.get(b, 0) + a
    body = " + ".join(f"{a} sqrt {b}" if b > 1 else f"{a}" for b, a in sorted(terms.items()))
    val = sum(a * math.sqrt(b) for b, a in terms.items()) / 9
    g = Fraction(sum(terms.values()) if len(terms) == 1 else 0)
    f, k = mask(F, D)
    n = 4
    axes = [a.ravel() for a in np.meshgrid(*([np.arange(n)] * D), indexing="ij")]
    prod = np.ones(axes[0].shape)
    for j in range(2):
        prod *= f([((2 ** j) * a % n) / n for a in axes])
    scan = float(prod.sum())
    print(f"gasket 2D Sigma_2(0) = ({body})/9 = (8 + 2 sqrt 5)/3 exact = {val:.12f}, scan {scan:.12f} {chk(scan, val, 1e-12)}, threshold q^(ND/2) = 4, margin {val - 4:.6f}, exponent {math.log(val) / (2 * math.log(2)):.6f}")

def entropy_bound():
    mp.dps = 30
    m1 = 2 * quad(lambda u: mplog(2 * mpcos(mp.pi * u)), [0, mpf(1) / 3])
    q, D, F = GASKET
    a = float(m1) - math.log(3)
    print(f"gasket 2D entropy bound: int_(T^2) log|1 + e(t1) + e(t2)| = {float(m1):.12f}, int log|hat F| = {a:.12f}, Jensen gives alpha_1 >= D + that/log q = {2 + a / math.log(2):.6f}, threshold D/2 = 1.0")

# GRID SUMS

def grid_sum(q, D, F, L):
    f, k = mask(F, D)
    n = q ** L
    axes = np.meshgrid(*([np.arange(n)] * D), indexing="ij")
    prod = np.ones(axes[0].shape)
    for j in range(L):
        prod *= f([((q ** j) * a % n) / n for a in axes])
    return float(prod.sum())

GRID = [
    ("gasket 2D", GASKET, [1.0000, 1.0278, 1.0388, 1.0446, 1.0482, 1.0505, 1.0522, 1.0535, 1.0545, 1.0553]),
    ("carpet 2D", CARPET, [0.6309, 0.6931, 0.7174, 0.7301, 0.7378, 0.7429]),
    ("gasket base 4", GASKET4, [0.5000, 0.4903, 0.4876, 0.4862, 0.4853, 0.4848, 0.4844, 0.4841, 0.4838, 0.4836]),
    ("carpet base 9", CARPET9, [0.3155, 0.3298, 0.3344, 0.3367, 0.3380, 0.3389]),
    ("base 10 missing 0", base_missing(10, 0), [None, None, None, None, None, 0.3086]),
    ("base 10 missing 1", base_missing(10, 1), [None, None, None, None, None, 0.3328]),
    ("base 10 missing 5", base_missing(10, 5), [None, None, None, None, None, 0.3420]),
]

def grids(rows):
    for name, (q, D, F), want in rows:
        for L, w in enumerate(want, start=1):
            S = grid_sum(q, D, F, L)
            e = math.log(S) / (L * math.log(q))
            tag = "" if w is None else " " + chk(e, w, 1e-4)
            print(f"{name} L={L:2d}: sum {S:16.4f} exponent {e:.6f}{tag}")

# SANDWICH, FLOAT

def sandwich(name, q, D, F, N, m, want_lo, want_hi, chunk=2048):
    t0 = time.time()
    f, k = mask(F, D)
    lip = lip_of(F)
    n = q ** N
    axes = [a.ravel() for a in np.meshgrid(*([np.arange(n)] * D), indexing="ij")]
    h = 1.0 / (n * m)
    slack = lip * (q ** N - 1) / (q - 1) * (q ** (D * N)) * h / 2
    shifts = [a.ravel() * h for a in np.meshgrid(*([np.arange(m)] * D), indexing="ij")]
    vals = np.empty(len(shifts[0]))
    for s0 in range(0, len(shifts[0]), chunk):
        sh = [s[s0:s0 + chunk] for s in shifts]
        prod = np.ones((len(sh[0]), len(axes[0])))
        for j in range(N):
            prod *= f([((q ** j) * (a[None, :] / n + b[:, None])) % 1.0 for a, b in zip(axes, sh)])
        vals[s0:s0 + chunk] = prod.sum(axis=1)
    lo, hi = float(vals.min()) - slack, float(vals.max()) + slack
    el, eh = math.log(lo) / (N * math.log(q)), math.log(hi) / (N * math.log(q))
    thr = q ** (N * D / 2)
    tl = "" if want_lo is None else " " + chk(el, want_lo, 1e-4)
    th = "" if want_hi is None else " " + chk(eh, want_hi, 1e-4)
    print(f"{name} N={N} m={m}: scan [{vals.min():.6f}, {vals.max():.6f}] slack {slack:.6f} threshold {thr:.4f} -> alpha_1 > {el:.6f}{tl} and < {eh:.6f}{th}  ({time.time() - t0:.1f}s)")

# INTERVAL KERNEL

def dn(x):
    return np.nextafter(x, -np.inf)

def up(x):
    return np.nextafter(x, np.inf)

PI_UP = up(math.pi)
TABLES = {}

def costable(Q):
    if Q not in TABLES:
        iv.prec = 96
        lo = np.empty(Q)
        hi = np.empty(Q)
        two = 2 * iv.pi
        for j in range(Q // 2 + 1):
            c = iv.cos(two * j / Q)
            lo[j] = dn(float(c.a))
            hi[j] = up(float(c.b))
        for j in range(Q // 2 + 1, Q):
            lo[j], hi[j] = lo[Q - j], hi[Q - j]
        TABLES[Q] = (lo, hi)
    return TABLES[Q]

def folded(F):
    mult = {}
    for v in F:
        for w in F:
            d = tuple(a - b for a, b in zip(v, w))
            mult[d] = mult.get(d, 0) + 1
    zero = tuple(0 for _ in F[0])
    out, seen = [], set()
    for d, c in mult.items():
        if d == zero or d in seen:
            continue
        neg = tuple(-a for a in d)
        seen.add(d)
        seen.add(neg)
        out.append((c + mult.get(neg, 0), d))
    return out

def lip_up(F):
    s = sum(sum(abs(c) for c in v) for v in F)
    return up(up(2 * PI_UP * s) / len(F))

def hat_iv(kc, Q, dif, k, side):
    lo, hi = costable(Q)
    tab = hi if side else lo
    s = np.full(kc[0].shape, float(k))
    for c, d in dif:
        idx = kc[0] * d[0]
        for j in range(1, len(d)):
            idx = idx + kc[j] * d[j]
        t = tab[idx % Q]
        s = up(s + up(c * t)) if side else dn(s + dn(c * t))
    r = np.sqrt(np.maximum(s, 0.0))
    if side:
        return np.minimum(up(up(r) / k), 1.0)
    return dn(dn(r) / k)

def kernel_check(pts=400, Q=1944):
    mp.dps = 40
    rng = np.random.default_rng(20250906)
    bad, wide = 0, 0.0
    for name, (q, D, F) in [("gasket 2D", GASKET), ("carpet 2D", CARPET), ("carpet base 9", CARPET9)]:
        dif, k = folded(F), len(F)
        kc = [rng.integers(0, Q, pts) for _ in range(D)]
        lo = hat_iv(kc, Q, dif, k, False)
        hi = hat_iv(kc, Q, dif, k, True)
        for i in range(pts):
            acc = mpc(0)
            for v in F:
                acc += expjpi(2 * mpf(int(sum(int(v[c]) * int(kc[c][i]) for c in range(D)))) / Q)
            t = abs(acc) / k
            if not (mpf(float(lo[i])) <= t <= mpf(float(hi[i]))):
                bad += 1
            wide = max(wide, float(hi[i] - lo[i]))
    print(f"interval kernel against {mp.dps}-digit truth on {3 * pts} arguments mod {Q}: outside enclosure {bad} {chk(bad, 0, 0)}, widest enclosure {wide:.3e}")

# SANDWICH, CERTIFIED

def sandwich_iv(name, q, D, F, N, m, chunk=2048):
    t0 = time.time()
    Q = q ** N * m
    costable(Q)
    dif, k = folded(F), len(F)
    n = q ** N
    grid = [a.ravel() * m for a in np.meshgrid(*([np.arange(n)] * D), indexing="ij")]
    shifts = [a.ravel() for a in np.meshgrid(*([np.arange(m)] * D), indexing="ij")]
    slack = up(up(up(lip_up(F) * ((q ** N - 1) // (q - 1))) * q ** (D * N)) / (2 * q ** N * m))
    best_lo, best_hi = np.inf, 0.0
    for s0 in range(0, len(shifts[0]), chunk):
        sh = [s[s0:s0 + chunk] for s in shifts]
        pl = np.ones((len(sh[0]), len(grid[0])))
        ph = np.ones((len(sh[0]), len(grid[0])))
        for j in range(N):
            kc = [((q ** j) * (g[None, :] + b[:, None])) % Q for g, b in zip(grid, sh)]
            pl = dn(pl * hat_iv(kc, Q, dif, k, False))
            ph = up(ph * hat_iv(kc, Q, dif, k, True))
        al = np.zeros(len(sh[0]))
        ah = np.zeros(len(sh[0]))
        for c in range(pl.shape[1]):
            al = dn(al + pl[:, c])
            ah = up(ah + ph[:, c])
        best_lo = min(best_lo, float(al.min()))
        best_hi = max(best_hi, float(ah.max()))
    cl, ch = dn(best_lo - slack), up(best_hi + slack)
    thr = q ** (N * D / 2)
    el = math.floor(math.log(cl) / (N * math.log(q)) * 1e4) / 1e4
    eh = math.ceil(math.log(ch) / (N * math.log(q)) * 1e4) / 1e4
    assert cl > q ** (N * el) and ch < q ** (N * eh)
    print(f"{name} N={N} m={m} certified: min > {cl:.6f}, max < {ch:.6f}, slack {slack:.6f}, threshold q^(ND/2) = {thr:.4f} -> alpha_1 > {el:.4f} and < {eh:.4f}  [{'KILL' if cl > thr else 'no kill'}]  ({time.time() - t0:.1f}s)")
    return el, eh

# WINDOW STATES

def window_state(q, D, nd):
    S = q ** D
    dig = np.array(list(itertools.product(range(q), repeat=D)))
    W = S ** nd
    idx = np.arange(W)
    bi = []
    for c in range(D):
        acc = np.zeros(W, dtype=np.int64)
        for i in range(nd):
            acc = acc + dig[:, c][(idx // S ** i) % S] * (q ** (nd - 1 - i))
        bi.append(acc)
    return S, W, idx, bi

def window_G(q, D, F, nd, m):
    f, k = mask(F, D)
    S, W, idx, bi = window_state(q, D, nd)
    box = 2.0 / q ** nd
    h = box / m
    sub = [a.ravel() for a in np.meshgrid(*([np.arange(m) * h + h / 2] * D), indexing="ij")]
    G = np.empty(W)
    B = max(1, 2_000_000 // len(sub[0]))
    for s0 in range(0, W, B):
        t = [(b[s0:s0 + B, None] / q ** nd + s[None, :]) % 1.0 for b, s in zip(bi, sub)]
        G[s0:s0 + B] = f(t).max(axis=1)
    return np.minimum(G + lip_of(F) * h / 2, 1.0), S, W

def window_G_iv(q, D, F, nd, m):
    Q = q ** nd * m
    costable(Q)
    dif, k = folded(F), len(F)
    S, W, idx, bi = window_state(q, D, nd)
    sub = [a.ravel() for a in np.meshgrid(*([2 * np.arange(m) + 1] * D), indexing="ij")]
    G = np.empty(W)
    B = max(1, 2_000_000 // len(sub[0]))
    for s0 in range(0, W, B):
        kc = [(b[s0:s0 + B, None] * m + s[None, :]) % Q for b, s in zip(bi, sub)]
        G[s0:s0 + B] = hat_iv(kc, Q, dif, k, True).max(axis=1)
    return np.minimum(up(G + up(lip_up(F) / (q ** nd * m))), 1.0), S, W

def perron(G, S, nd, t=1.0, iters=500):
    idx = np.arange(len(G))
    succ = [(idx % S ** (nd - 1)) * S + s for s in range(S)]
    Gt = G if t == 1.0 else G ** t
    w = np.ones(len(G))
    lam = 0.0
    for it in range(iters):
        nw = sum(Gt[u] * w[u] for u in succ)
        nl = float(nw.max())
        w = nw / nl
        if abs(nl - lam) < 1e-13:
            lam = nl
            break
        lam = nl
    return lam, np.maximum(w, 1e-12), succ, Gt

def cw_bound(Gt, w, succ):
    acc = np.zeros(len(w))
    for u in succ:
        acc = up(acc + up(Gt[u] * w[u]))
    plain = np.zeros(len(w))
    for u in succ:
        plain = up(plain + Gt[u])
    return float(up(acc / w).max()), float(plain.max())

MAY1 = ("published lambda_(1,4) < 2.24190 at 5 digits and box 10^(-5)", 2.24190, 27 / 77)
MAYT = ("published lambda_(235/154,4) < 1.36854", 1.36854, 59 / 433)
KAR1 = ("published 0.3219 at 4 digits", None, 0.3219)
KART = ("published 0.14355 at 4 digits", None, 0.14355)

WINDOWS = [
    ("base 10 missing 5", base_missing(10, 5), [(4, 64, 2.245878, MAY1), (5, 64, 2.242123, MAY1)]),
    ("base 10 missing 0", base_missing(10, 0), [(4, 64, 2.045493, None), (5, 64, 2.041488, None)]),
    ("carpet 2D", CARPET, [(3, 24, 3.355167, None), (4, 24, 2.670258, None), (5, 24, 2.441254, None)]),
    ("gasket 2D", GASKET, [(4, 24, 2.627210, None), (6, 24, 2.219782, None), (7, 24, 2.153314, None)]),
    ("gasket base 4", GASKET4, [(4, 256, 1.988435, None), (6, 256, 1.952957, None), (8, 256, 1.950717, None)]),
    ("carpet base 9", CARPET9, [(4, 256, 2.134067, None)]),
    ("base 9 missing 0", base_missing(9, 0), [(4, 64, 2.033782, KAR1)]),
]

def windows(rows):
    for name, (q, D, F), rs in rows:
        for nd, m, want, ref in rs:
            t0 = time.time()
            G, S, W = window_G(q, D, F, nd, m)
            lam, w, succ, Gt = perron(G, S, nd)
            e = math.log(lam) / math.log(q)
            cal = "" if ref is None else f" calibration {ref[0]}, exponent {ref[2]:.6f}, gap {e - ref[2]:+.6f}"
            print(f"{name} windows {nd} digits {W} states sub-scan {m}^{D}: lambda {lam:.6f} {chk(lam, want, 5e-6)} exponent {e:.6f} threshold D/2 = {D / 2}{cal}  ({time.time() - t0:.1f}s)")

def certify_windows(rows):
    for name, (q, D, F), nd, m in rows:
        t0 = time.time()
        G, S, W = window_G_iv(q, D, F, nd, m)
        lam, w, succ, Gt = perron(G, S, nd)
        mu, plain = cw_bound(Gt, w, succ)
        e = math.ceil(math.log(mu) / math.log(q) * 1e4) / 1e4
        ep = math.ceil(math.log(plain) / math.log(q) * 1e4) / 1e4
        assert mu < q ** e and plain < q ** ep
        print(f"{name} windows {nd} digits {W} states sub-scan {m}^{D} certified: row sum {plain:.6f} -> alpha_1* < {ep:.4f}, weighted {mu:.6f} -> alpha_1* < {e:.4f}, threshold D/2 = {D / 2}  [{'PASS' if mu < q ** (D / 2) else 'no pass'}]  ({time.time() - t0:.1f}s)")

# MOMENTS

def crit(b):
    return (1 + math.log(b - 1) / math.log(b) / 2) / 5

MOMENTS = [
    ("base 10 missing 5", base_missing(10, 5), 4, 64, [(1.0, 0.3514, MAY1), (1.5, 0.1447, None), (235 / 154, 0.1370, MAYT), (1.6, 0.1170, None), (1.7, 0.0937, None)]),
    ("base 9 missing 0", base_missing(9, 0), 4, 64, [(1.5, 0.1446, KART), (235 / 154, 0.1380, None), (1.6, 0.1204, None), (1.7, 0.0995, None)]),
    ("carpet base 9", CARPET9, 4, 64, [(1.0, 0.3451, None), (1.5, 0.1544, None), (235 / 154, 0.1470, None), (1.6, 0.1275, None), (1.7, 0.1042, None)]),
    ("carpet base 9", CARPET9, 5, 64, [(1.0, 0.3437, None), (1.5, 0.1531, None), (235 / 154, 0.1457, None), (1.6, 0.1262, None), (1.7, 0.1031, None), (1.8, 0.0835, None)]),
    ("carpet base 9", CARPET9, 6, 64, [(1.0, 0.3435, None), (1.5, 0.1529, None)]),
    ("gasket base 4", GASKET4, 8, 256, [(1.0, 0.4820, None), (235 / 154, 0.3170, None)]),
]

def moments(rows):
    for name, (q, D, F), nd, m, ss in rows:
        t0 = time.time()
        G, S, W = window_G(q, D, F, nd, m)
        for s, want, ref in ss:
            lam, w, succ, Gt = perron(G, S, nd, s)
            g = math.log(lam) / math.log(q)
            rhs = crit(q) * (2 - s)
            cal = "" if ref is None else f" calibration {ref[0]}, exponent {ref[2]:.6f}, gap {g - ref[2]:+.6f}"
            print(f"{name} {nd} digits s={s:.6f}: lambda {lam:.6f} g(s) {g:.6f} {chk(g, want, 1e-4)} criterion {rhs:.6f} margin {rhs - g:+.6f} {'PASS' if g < rhs else 'FAIL'}{cal}")
        print(f"  ({time.time() - t0:.1f}s)")

# LEMMA A PRIME

def m_of(q, d):
    m, p = 1, q
    while 2 * p <= d:
        p *= q
        m += 1
    return m

def order(q, d):
    r, x = 1, q % d
    while x != 1:
        x = x * q % d
        r += 1
    return r

def lemma_a(name, q, D, F, dmax, want):
    f, k = mask(F, D)
    c = 1 - (2 / k) * (1 - math.cos(math.pi / (2 * q)))
    worst = []
    for d in range(3, dmax + 1):
        if math.gcd(d, q) != 1:
            continue
        r, md = order(q, d), m_of(q, d)
        axes = np.meshgrid(*([np.arange(d)] * D), indexing="ij")
        prod = np.ones(axes[0].shape)
        for j in range(r):
            prod *= f([((q ** j % d) * a % d) / d for a in axes])
        prod[(0,) * D] = 0.0
        rate = float(prod.max()) ** (1 / r)
        assert rate <= c ** (1 / md) + 1e-12
        worst.append((rate, d, r, md))
    worst.sort(reverse=True)
    print(f"{name}: c(q,k) = {c:.6f}, moduli 3..{dmax} coprime to q, every per-digit rate at most c^(1/m_d), zero violations")
    for rate, d, r, md in worst[:4]:
        w = want.get(d)
        tag = "" if w is None else " " + chk(rate, w, 5e-7)
        print(f"  d={d:4d} ord={r:3d} m_d={md:2d} rate {rate:.6f}{tag} Lemma A' rate c^(1/m_d) {c ** (1 / md):.6f} Lemma A rate c^(1/ord) {c ** (1 / r):.6f}")

# COROLLARIES

def counts_mod(q, D, F, d, n):
    c = np.zeros((d,) * D, dtype=np.int64)
    c[(0,) * D] = 1
    ax = tuple(range(D))
    for j in range(n):
        w = pow(q, j, d)
        nx = np.zeros((d,) * D, dtype=np.int64)
        for v in F:
            nx += np.roll(c, [(w * a) % d for a in v], axis=ax)
        c = nx
    return c

def corollary(name, q, D, F, dmax, nmax, cells):
    k = len(F)
    assert k ** nmax < 2 ** 62
    c = 1 - (2 / k) * (1 - math.cos(math.pi / (2 * q)))
    worst, rows = (0.0, None), []
    for d in range(3, dmax + 1):
        if math.gcd(d, q) != 1:
            continue
        r, md = order(q, d), m_of(q, d)
        for n in range(2, nmax + 1):
            C = counts_mod(q, D, F, d, n)
            dev = max(abs(Fraction(int(x), k ** n) - Fraction(1, d ** D)) for x in C.ravel())
            bp, ba = c ** (n // md), c ** (n // r)
            assert float(dev) <= bp
            ratio = float(dev) / bp
            if ratio > worst[0]:
                worst = (ratio, (d, n, r, md, float(dev), bp, ba))
            if (d, n) in cells:
                rows.append((d, n, r, md, float(dev), bp, ba, cells[(d, n)]))
    d, n, r, md, dev, bp, ba = worst[1]
    print(f"{name} equidistribution band: moduli 3..{dmax} coprime to q, levels 2..{nmax}, every exact deviation at most c^floor(n/m_d), zero violations")
    print(f"  worst ratio to the window bound {worst[0]:.6f} at d={d} n={n} (ord {r}, m_d {md}): deviation {math.ceil(dev * 1e7) / 1e7:.7f} against A' {math.ceil(bp * 1e6) / 1e6:.6f} and A {math.ceil(ba * 1e6) / 1e6:.6f}")
    for d, n, r, md, dev, bp, ba, want in rows:
        tag = "" if want is None else " " + chk(math.ceil(dev * 1e7) / 1e7, want, 5e-9)
        print(f"  d={d:3d} n={n:3d} ord={r:3d} m_d={md:2d} deviation {math.ceil(dev * 1e7) / 1e7:.7f}{tag} A' bound {math.ceil(bp * 1e6) / 1e6:.6f} A bound {math.ceil(ba * 1e6) / 1e6:.6f}")

def base_peel(name, q, D, F, ms, nmax):
    k = len(F)
    c = 1 - (2 / k) * (1 - math.cos(math.pi / (2 * q)))
    e = 1
    for p in range(2, q + 1):
        if q % p == 0 and all(p % r for r in range(2, p)):
            e *= p
    ke = sum(1 for v in F if all(a % e == 0 for a in v))
    worst = (0.0, None)
    for m in ms:
        assert math.gcd(m, q) == 1
        mm = m_of(q, m)
        for n in range(2, nmax + 1):
            T = int(counts_mod(q, D, F, e * m, n)[(0,) * D])
            assert T == int(counts_mod(q, D, F, m, n - 1)[(0,) * D])
            err = abs(Fraction(T, k ** n) - Fraction(ke, k) * Fraction(1, m ** D))
            bp = c ** ((n - 1) // mm)
            assert float(err) <= bp
            ratio = float(err) / bp
            if ratio > worst[0]:
                worst = (ratio, (m, n, mm, float(err), bp, c ** ((n - 1) // order(q, m))))
    m, n, mm, err, bp, ba = worst[1]
    print(f"{name} base peel band: rad(q) = {e}, k_e = {ke}, coprime parts {list(ms)}, levels 2..{nmax}, T_(e m)(n) = T_m(n-1) exact and every error at most c^floor((n-1)/m_m), zero violations")
    print(f"  worst ratio {worst[0]:.6f} at m={m} n={n} (m_m {mm}): error {math.ceil(err * 1e7) / 1e7:.7f} against A' {math.ceil(bp * 1e6) / 1e6:.6f} and A {math.ceil(ba * 1e6) / 1e6:.6f}")

# THE ORDER BAND

def hat_missing(b, a0, th):
    th = th - np.floor(th)
    s = np.sin(np.pi * th)
    safe = np.where(s == 0.0, 1.0, s)
    K = np.where(s == 0.0, float(b), np.sin(b * np.pi * th) / safe)
    ph = np.cos(2 * np.pi * th * (a0 - (b - 1) / 2))
    return np.sqrt(np.maximum(K * K - 2 * K * ph + 1.0, 0.0)) / (b - 1)

def hat_missing_check(b, a0, pts=2000):
    f, k = mask(base_missing(b, a0)[2], 1)
    rng = np.random.default_rng(20250906)
    th = rng.random(pts)
    w = float(np.abs(hat_missing(b, a0, th) - f([th])).max())
    print(f"base {b} missing {a0} closed form |sin(b pi th)/sin(pi th) - e((a0 - (b-1)/2) th)|/(b-1) against the character sum on {pts} arguments: worst gap {w:.3e} {chk(w, 0.0, 1e-11)}")

def order_band(b, a0, N, m, ss, chunk=500):
    lip = (2 * math.pi / (b - 1)) * sum(a for a in range(b) if a != a0)
    n = b ** N
    i = np.arange(n)
    h = 1.0 / (n * m)
    best = {s: np.inf for s in ss}
    for s0 in range(0, m, chunk):
        xg = (np.arange(s0, min(s0 + chunk, m)) + 0.5) * h
        P = np.ones((len(xg), n))
        for j in range(N):
            P *= np.maximum(hat_missing(b, a0, (b ** j) * (xg[:, None] + i[None, :] / n)) - (b ** j) * lip * h / 2, 0.0)
        lg = np.log(np.maximum(P, 1e-300))
        for s in ss:
            best[s] = min(best[s], float(np.exp(s * lg).sum(axis=1).min()))
    return best

def criterion_band(name, b, a0, N, m, want):
    t0 = time.time()
    ss = sorted(set([round(1.0 + 0.01 * i, 4) for i in range(30)] + [round(1.3 + 0.002 * i, 4) for i in range(151)] + [round(1.6 + 0.01 * i, 4) for i in range(21)] + [1.85, 1.9, 2.0]))
    best = order_band(b, a0, N, m, ss)
    par = (b / (b - 1)) ** N
    print(f"{name} shift sandwich at N={N}, m={m}: Parseval gives Sigma_N^(2)(x) = (b/(b-1))^N = {par:.6f} for every x, scan {best[2.0]:.6f}, slack {par - best[2.0]:.6f}, scan <= anchor {chk_le(best[2.0], par, 'parseval anchor')} ({time.time() - t0:.1f}s)")
    best[2.0] = par
    lo = {s: math.floor(math.log(best[s]) / (N * math.log(b)) * 1e6) / 1e6 for s in ss}
    rhs = {s: math.ceil(crit(b) * (2 - s) * 1e6) / 1e6 for s in ss}
    for s in ss:
        if s in want:
            print(f"  s={s:.4f} min Sigma {best[s]:.6f} -> m_s > {lo[s]:.6f} {chk(lo[s], want[s], 5e-7)} criterion {rhs[s]:.6f} margin {lo[s] - rhs[s]:+.6f}")
    for left in (1.5, 1.0):
        cur, cover = left, []
        while cur < 2.0:
            nxt = max((s for s in ss if s > cur and lo[s] > rhs[cur]), default=None)
            if nxt is None:
                break
            cover.append((cur, nxt))
            cur = nxt
        ok = bool(cover) and cover[-1][1] >= 2.0
        a, z = min(cover, key=lambda p: lo[p[1]] - rhs[p[0]])
        print(f"  monotone cover of [{left}, 2) in {len(cover)} intervals, Sigma_N^(s) falling in s and the criterion falling in s: {'COMPLETE' if ok else 'INCOMPLETE'} {chk(1.0 if ok else 0.0, 1.0, 0.0)}")
        print(f"    tightest cell s={a:.4f} to {z:.4f}: m_s > {lo[z]:.6f} against the criterion {rhs[a]:.6f} at the left end, margin {lo[z] - rhs[a]:+.6f}")
    print(f"  the criterion asks m_s < {crit(b):.6f} (2 - s) for some s in [3/2, 2) and no order in [1, 2) delivers it")

def order_rows(name, b, a0, N, m, ss, want, chunk):
    t0 = time.time()
    best = order_band(b, a0, N, m, ss, chunk)
    for s in ss:
        lo = math.floor(math.log(best[s]) / (N * math.log(b)) * 1e6) / 1e6
        rhs = math.ceil(crit(b) * (2 - s) * 1e6) / 1e6
        tag = "" if s not in want else " " + chk(lo, want[s], 5e-7)
        print(f"{name} shift sandwich at N={N}, m={m}, s={s:.6f}: min Sigma {best[s]:.6f} -> m_s > {lo:.6f}{tag} criterion {rhs:.6f} margin {lo - rhs:+.6f}  ({time.time() - t0:.1f}s)")

def two_dim_moments(rows, ss):
    for name, (q, D, F), nd, m, want in rows:
        G, S, W = window_G(q, D, F, nd, m)
        for s in ss:
            lam, w, succ, Gt = perron(G, S, nd, s)
            e = math.log(lam) / (D * math.log(q))
            tag = "" if s not in want else " " + chk(math.ceil(e * 1e6) / 1e6, want[s], 5e-8)
            print(f"{name} {nd} digit-vectors s={s:.6f}: lambda {lam:.6f} exponent in X = q^(D M) units {math.ceil(e * 1e6) / 1e6:.6f}{tag} criterion {math.ceil(crit(q ** D) * (2 - s) * 1e6) / 1e6:.6f}")

# THE LEAST BASE

TWIDDLE = {}

def imul(al, ah, bl, bh):
    p1, p2, p3, p4 = al * bl, al * bh, ah * bl, ah * bh
    return dn(np.minimum(np.minimum(p1, p2), np.minimum(p3, p4))), up(np.maximum(np.maximum(p1, p2), np.maximum(p3, p4)))

def isq(lo, hi):
    a, c = lo * lo, hi * hi
    return dn(np.where(lo >= 0.0, a, np.where(hi <= 0.0, c, 0.0))), up(np.maximum(a, c))

def twiddle(Q):
    if Q in TWIDDLE:
        return TWIDDLE[Q]
    B = math.isqrt(Q) + 1
    iv.prec = 96
    def tab(n, step):
        cl, ch, sl, sh = (np.empty(n) for _ in range(4))
        two = 2 * iv.pi
        for j in range(n):
            a = two * ((j * step) % Q) / Q
            c, s = iv.cos(a), iv.sin(a)
            cl[j], ch[j] = dn(float(c.a)), up(float(c.b))
            sl[j], sh[j] = dn(float(s.a)), up(float(s.b))
        return cl, ch, sl, sh
    TWIDDLE[Q] = (B, tab(B, 1), tab(Q // B + 2, B))
    return TWIDDLE[Q]

def cos_iv(TW, idx):
    B, small, big = TW
    i1 = idx // B
    i0 = idx - i1 * B
    pl, ph = imul(big[0][i1], big[1][i1], small[0][i0], small[1][i0])
    ql, qh = imul(big[2][i1], big[3][i1], small[2][i0], small[3][i0])
    return dn(pl - qh), up(ph - ql)

def hat_missing_iv(b, a0, j, P, Q, TW):
    jm = j % P
    s1l, s1h = cos_iv(TW, (2 * jm - P) % Q)
    sbl, sbh = cos_iv(TW, (2 * b * jm - P) % Q)
    phl, phh = cos_iv(TW, ((4 * a0 - 2 * b + 2) * jm) % Q)
    if s1l.min() <= 0.0:
        raise SystemExit("denominator enclosure touches zero")
    kl, kh = imul(sbl, sbh, dn(1.0 / s1h), up(1.0 / s1l))
    k2l, k2h = isq(kl, kh)
    ml, mh = imul(kl, kh, phl, phh)
    el = dn(dn(k2l - up(2.0 * mh)) + 1.0)
    eh = up(up(k2h - dn(2.0 * ml)) + 1.0)
    return dn(dn(np.sqrt(np.maximum(el, 0.0))) / (b - 1)), up(up(np.sqrt(np.maximum(eh, 0.0))) / (b - 1))

def base_windows(b, a0, nd, m, wide=1):
    W = b ** nd
    P = 2 * m * W
    Q = 4 * P
    TW = twiddle(Q)
    lip = up(up(2 * PI_UP * sum(a for a in range(b) if a != a0)) / (b - 1))
    slack = up(up(lip * wide) / (2 * W * m))
    off = (wide * (2 * np.arange(m) + 1)).astype(np.int64)
    Ghi, Glo = np.empty(W), np.empty(W)
    CH = max(1, 400_000 // m)
    for s0 in range(0, W, CH):
        w = np.arange(s0, min(s0 + CH, W), dtype=np.int64)
        j = 2 * m * w[:, None] + off[None, :]
        vl, vh = hat_missing_iv(b, a0, j, P, Q, TW)
        Ghi[s0:s0 + CH] = np.minimum(up(vh.max(axis=1) + slack), 1.0)
        Glo[s0:s0 + CH] = np.maximum(dn(vl.min(axis=1) - slack), 0.0)
    return Ghi, Glo, slack

def perron_red(G, b, nd, iters=6000, floor_it=300, streak_need=50, tol=1e-13):
    S = b ** (nd - 1)
    tgt = np.tile(np.arange(S), b)
    y = np.ones(S)
    lam = 0.0
    streak = 0
    for it in range(iters):
        z = (G * y[tgt]).reshape(S, b).sum(axis=1)
        nl = float(z.max())
        if nl <= 0.0:
            return 0.0, np.ones(S)
        y = z / nl
        streak = streak + 1 if abs(nl - lam) <= tol * nl else 0
        lam = nl
        if streak >= streak_need and it >= floor_it:
            break
    return lam, np.maximum(y, 1e-30)

def cw_red(G, y, b, nd, side):
    S = b ** (nd - 1)
    tgt = np.tile(np.arange(S), b)
    r = (up(G * y[tgt]) if side else np.maximum(dn(G * y[tgt]), 0.0)).reshape(S, b)
    acc = np.zeros(S)
    for c in range(b):
        acc = up(acc + r[:, c]) if side else np.maximum(dn(acc + r[:, c]), 0.0)
    return float(up(acc / y).max()) if side else float(dn(acc / y).min())

def alpha_band(b, a0, nd, m, wide=1):
    Ghi, Glo, slack = base_windows(b, a0, nd, m, wide)
    lh, yh = perron_red(Ghi, b, nd)
    ll, yl = perron_red(Glo, b, nd)
    mu_hi = cw_red(Ghi, yh, b, nd, True)
    mu_lo = cw_red(Glo, yl, b, nd, False)
    eh = math.ceil(math.log(mu_hi) / math.log(b) * 1e7) / 1e7
    el = None if mu_lo <= 0.0 else math.floor(math.log(mu_lo) / math.log(b) * 1e7) / 1e7
    if not (mu_hi < b ** eh and (el is None or mu_lo > b ** el)):
        raise SystemExit(f"rounding unsafe at base {b} missing {a0}")
    return el, eh, mu_lo, mu_hi, slack

def shift_check(b, a0, N, m):
    t0 = time.time()
    lip = (2 * math.pi / (b - 1)) * sum(a for a in range(b) if a != a0)
    n = b ** N
    i = np.arange(n)
    h = 1.0 / (n * m)
    lo, hi = np.inf, 0.0
    for s0 in range(0, m, 200):
        xg = (np.arange(s0, min(s0 + 200, m)) + 0.5) * h
        Pl = np.ones((len(xg), n))
        Ph = np.ones((len(xg), n))
        for j in range(N):
            v = hat_missing(b, a0, (b ** j) * (xg[:, None] + i[None, :] / n))
            Pl *= np.maximum(v - (b ** j) * lip * h / 2, 0.0)
            Ph *= np.minimum(v + (b ** j) * lip * h / 2, 1.0)
        lo = min(lo, float(Pl.sum(axis=1).min()))
        hi = max(hi, float(Ph.sum(axis=1).max()))
    el = math.floor(math.log(lo) / (N * math.log(b)) * 1e6) / 1e6
    eh = math.ceil(math.log(hi) / (N * math.log(b)) * 1e6) / 1e6
    print(f"base {b} missing {a0} shift sandwich N={N} m={m}, the grid machine not the window machine: min Sigma {lo:.6f} max Sigma {hi:.6f} -> alpha_1 in [{el:.6f}, {eh:.6f}]  ({time.time() - t0:.1f}s)")
    return el, eh

def base_cert(b, a0, nd, m, wide=1, tag=""):
    t0 = time.time()
    el, eh, mu_lo, mu_hi, slack = alpha_band(b, a0, nd, m, wide)
    lo = "no positive certificate at this window length" if el is None else f"{el:.7f}"
    verdict = "CLEARS 1/4" if eh < 0.25 else ("FAILS 1/4" if el is not None and el >= 0.25 else "undecided at 1/4")
    print(f"base {b} missing {a0}{tag} windows {nd} digits sub-scan {m} box {wide}/{b}^{nd} certified: lambda in [{mu_lo:.6f}, {mu_hi:.6f}] slack {slack:.3e} -> alpha_1 > {lo} and < {eh:.7f}  [{verdict}]  ({time.time() - t0:.1f}s)")
    return el, eh

def base_family(b, nd, m, side, want=None):
    t0 = time.time()
    rows = []
    for a0 in range((b + 1) // 2):
        el, eh, mu_lo, mu_hi, slack = alpha_band(b, a0, nd, m)
        rows.append((a0, el, eh))
    worst = max(rows, key=lambda r: r[2])
    best = min(rows, key=lambda r: r[2])
    band = " ".join(f"{a0}:[{-1.0 if el is None else el:.7f},{eh:.7f}]" for a0, el, eh in rows)
    if side:
        ok = all(eh < 0.25 for _, _, eh in rows)
        head = f"every one-missing-digit set in base {b} clears alpha_1 < 1/4" if ok else f"base {b} does not clear at every digit"
    else:
        ok = all(el is not None and el >= 0.25 for _, el, _ in rows)
        head = f"no one-missing-digit set in base {b} clears alpha_1 < 1/4" if ok else f"base {b} clears at some digit"
    print(f"{head}: {nd} digits sub-scan {m}, {len(rows)} digits up to the symmetry a0 <-> {b - 1} - a0, worst a0 = {worst[0]} at [{-1.0 if worst[1] is None else worst[1]:.7f}, {worst[2]:.7f}], best a0 = {best[0]} at [{-1.0 if best[1] is None else best[1]:.7f}, {best[2]:.7f}] {chk(1.0 if ok else 0.0, 1.0, 0.0)}  ({time.time() - t0:.1f}s)")
    print(f"  base {b} certified alpha_1 band by missing digit: {band}")
    return rows

def family_window(b, a0, nd, m):
    Ghi, Glo, slack = base_windows(b, a0, nd, m)
    lh, yh = perron_red(Ghi, b, nd)
    mu_hi = cw_red(Ghi, yh, b, nd, True)
    eh = math.ceil(math.log(mu_hi) / math.log(b) * 1e7) / 1e7
    if not mu_hi < b ** eh:
        raise SystemExit(f"rounding unsafe at base {b} missing {a0}")
    return eh, mu_hi

def family_close(lo, hi, m, nds):
    t0 = time.time()
    rows, bad = [], []
    for b in range(lo, hi + 1):
        for nd in nds:
            worst, arg = -1.0, -1
            for a0 in range((b + 1) // 2):
                eh, mu = family_window(b, a0, nd, m)
                if eh > worst:
                    worst, arg = eh, a0
            if worst < 0.25:
                break
        rows.append((b, nd, arg, worst))
        if worst >= 0.25:
            bad.append((b, arg, worst))
    ok = not bad
    head = f"every one-missing-digit set of every base {lo} <= q <= {hi} clears alpha_1 < 1/4" if ok else f"{len(bad)} base(s) in {lo} <= q <= {hi} do not clear at every digit"
    ceiling = max(rows, key=lambda r: r[3])
    print(f"{head}: {hi - lo + 1} bases, {sum((b + 1) // 2 for b in range(lo, hi + 1))} distinct sets, sub-scan {m}, shortest window in {nds} that clears; the band ceiling is q = {ceiling[0]} missing {ceiling[2]} at alpha_1 < {ceiling[3]:.7f} on {ceiling[1]} window digits {chk(1.0 if ok else 0.0, 1.0, 0.0)}  ({time.time() - t0:.1f}s)")
    for b, nd, arg, worst in rows:
        flag = "" if worst < 0.25 else "  FAILS 1/4"
        print(f"  q={b:3d} {(b + 1) // 2:2d} distinct sets {nd} window digits: worst a0 = {arg:2d} at alpha_1 < {worst:.7f}{flag}")
    return rows, bad

def base_ladder(bases, nd, m, a0s):
    t0 = time.time()
    out = []
    for b in bases:
        a0 = a0s(b)
        el, eh, mu_lo, mu_hi, slack = alpha_band(b, a0, nd, m)
        out.append((b, a0, el, eh))
    print(f"certified alpha_1 ladder at {nd} digits sub-scan {m}, one missing digit per base: " + " ".join(f"q={b} a0={a0} [{-1.0 if el is None else el:.7f},{eh:.7f}]" for b, a0, el, eh in out) + f"  ({time.time() - t0:.1f}s)")
    return out

# THE PAIR FAMILY

def pair_missing(b, a, c):
    return (b, 1, [(d,) for d in range(b) if d != a and d != c])

def hat_pair(b, a, c, th):
    th = th - np.floor(th)
    s = np.sin(np.pi * th)
    safe = np.where(s == 0.0, 1.0, s)
    K = np.where(s == 0.0, float(b), np.sin(b * np.pi * th) / safe)
    p = a - (b - 1) / 2
    r = c - (b - 1) / 2
    v = K * K + 2.0 + 2.0 * np.cos(2 * np.pi * (a - c) * th) - 2.0 * K * (np.cos(2 * np.pi * p * th) + np.cos(2 * np.pi * r * th))
    return np.sqrt(np.maximum(v, 0.0)) / (b - 2)

def hat_pair_check(b, a, c, pts=2000):
    f, k = mask(pair_missing(b, a, c)[2], 1)
    rng = np.random.default_rng(20250907)
    th = rng.random(pts)
    w = float(np.abs(hat_pair(b, a, c, th) - f([th])).max())
    print(f"base {b} missing {{{a},{c}}} closed form |K - e((a - (b-1)/2) th) - e((c - (b-1)/2) th)|/(b-2) against the character sum on {pts} arguments: worst gap {w:.3e} {chk(w, 0.0, 1e-11)}")

def pair_count(b):
    return ((b - 2) * (b - 3) // 2 + (b - 2) // 2) // 2 + b // 2

def pair_sets(b):
    out = [(0, c) for c in range(1, b // 2 + 1)]
    seen = set()
    for a in range(1, b - 1):
        for c in range(a + 1, b - 1):
            if (b - 1 - c, b - 1 - a) in seen:
                continue
            seen.add((a, c))
            out.append((a, c))
    return out

def hat_pair_iv(b, a, c, j, P, Q, TW):
    jm = j % P
    s1l, s1h = cos_iv(TW, (2 * jm - P) % Q)
    sbl, sbh = cos_iv(TW, (2 * b * jm - P) % Q)
    pal, pah = cos_iv(TW, ((4 * a - 2 * b + 2) * jm) % Q)
    pcl, pch = cos_iv(TW, ((4 * c - 2 * b + 2) * jm) % Q)
    dfl, dfh = cos_iv(TW, (4 * (a - c) * jm) % Q)
    if s1l.min() <= 0.0:
        raise SystemExit("denominator enclosure touches zero")
    kl, kh = imul(sbl, sbh, dn(1.0 / s1h), up(1.0 / s1l))
    k2l, k2h = isq(kl, kh)
    ml, mh = imul(kl, kh, dn(pal + pcl), up(pah + pch))
    el = dn(dn(dn(k2l + 2.0) + dn(2.0 * dfl)) - up(2.0 * mh))
    eh = up(up(up(k2h + 2.0) + up(2.0 * dfh)) - dn(2.0 * ml))
    return dn(dn(np.sqrt(np.maximum(el, 0.0))) / (b - 2)), up(up(np.sqrt(np.maximum(eh, 0.0))) / (b - 2))

def pair_windows(b, a, c, nd, m, wide=1):
    W = b ** nd
    P = 2 * m * W
    Q = 4 * P
    TW = twiddle(Q)
    lip = up(up(2 * PI_UP * (b * (b - 1) // 2 - a - c)) / (b - 2))
    slack = up(up(lip * wide) / (2 * W * m))
    off = (wide * (2 * np.arange(m) + 1)).astype(np.int64)
    Ghi, Glo = np.empty(W), np.empty(W)
    CH = max(1, 400_000 // m)
    for s0 in range(0, W, CH):
        w = np.arange(s0, min(s0 + CH, W), dtype=np.int64)
        j = 2 * m * w[:, None] + off[None, :]
        vl, vh = hat_pair_iv(b, a, c, j, P, Q, TW)
        Ghi[s0:s0 + CH] = np.minimum(up(vh.max(axis=1) + slack), 1.0)
        Glo[s0:s0 + CH] = np.maximum(dn(vl.min(axis=1) - slack), 0.0)
    return Ghi, Glo, slack

def pair_band(b, a, c, nd, m, wide=1, side=None):
    Ghi, Glo, slack = pair_windows(b, a, c, nd, m, wide)
    eh = el = None
    mu_hi = mu_lo = 0.0
    if side is None or side:
        lh, yh = perron_red(Ghi, b, nd)
        mu_hi = cw_red(Ghi, yh, b, nd, True)
        eh = math.ceil(math.log(mu_hi) / math.log(b) * 1e7) / 1e7
        if not mu_hi < b ** eh:
            raise SystemExit(f"rounding unsafe at base {b} missing {a},{c}")
    if side is None or not side:
        ll, yl = perron_red(Glo, b, nd)
        mu_lo = cw_red(Glo, yl, b, nd, False)
        el = None if mu_lo <= 0.0 else math.floor(math.log(mu_lo) / math.log(b) * 1e7) / 1e7
        if el is not None and not mu_lo > b ** el:
            raise SystemExit(f"rounding unsafe at base {b} missing {a},{c}")
    return el, eh, mu_lo, mu_hi, slack

def pair_cert(b, a, c, nd, m, wide=1, tag="", thr=0.25, name="1/4"):
    t0 = time.time()
    el, eh, mu_lo, mu_hi, slack = pair_band(b, a, c, nd, m, wide)
    lo = "no positive certificate at this window length" if el is None else f"{el:.7f}"
    verdict = f"CLEARS {name}" if eh < thr else (f"FAILS {name}" if el is not None and el >= thr else f"undecided at {name}")
    print(f"base {b} missing {{{a},{c}}}{tag} windows {nd} digits sub-scan {m} box {wide}/{b}^{nd} certified: lambda in [{mu_lo:.6f}, {mu_hi:.6f}] slack {slack:.3e} -> alpha_1 > {lo} and < {eh:.7f}  [{verdict}]  ({time.time() - t0:.1f}s)")
    return el, eh

def pair_census(bs):
    th = (np.arange(1001) + 0.5) / 1001
    for b in bs:
        fp = {}
        for a in range(b):
            for c in range(a + 1, b):
                fp.setdefault(tuple(np.round(hat_pair(b, a, c, th), 11)), []).append((a, c))
        sets = pair_sets(b)
        reps = {tuple(np.round(hat_pair(b, a, c, th), 11)) for (a, c) in sets}
        print(f"base {b}: {b * (b - 1) // 2} excluded pairs fall into {len(fp)} distinct transforms against (C(q-2,2) + floor((q-2)/2))/2 + floor(q/2) = {pair_count(b)} {chk(float(len(fp)), float(pair_count(b)), 0.0)}, the scan list holds {len(sets)} {chk(float(len(sets)), float(pair_count(b)), 0.0)} and meets every class {chk(float(len(reps)), float(len(fp)), 0.0)}; the edge class {{0,c}} is the one-missing-digit set of the {b - 1}-digit interval and collapses {{0,c}} with {{0,{b}-c}}")

def pair_family(b, nd, m, side, tag=""):
    t0 = time.time()
    rows = []
    for (a, c) in pair_sets(b):
        el, eh, mu_lo, mu_hi, slack = pair_band(b, a, c, nd, m, side=side)
        rows.append((a, c, el, eh))
    if side:
        ok = all(eh < 0.25 for _, _, _, eh in rows)
        key = max(rows, key=lambda r: r[3])
        head = f"every two-missing-digit set in base {b} clears alpha_1 < 1/4" if ok else f"base {b} does not clear at every pair"
        edge = f"worst pair {{{key[0]},{key[1]}}} at alpha_1 < {key[3]:.7f}"
    else:
        ok = all(el is not None and el >= 0.25 for _, _, el, _ in rows)
        key = min(rows, key=lambda r: 9.9 if r[2] is None else r[2])
        head = f"no two-missing-digit set in base {b} clears alpha_1 < 1/4" if ok else f"base {b} clears at some pair"
        edge = f"closest pair {{{key[0]},{key[1]}}} at alpha_1 > {-1.0 if key[2] is None else key[2]:.7f}"
    print(f"{head}{tag}: {nd} digits sub-scan {m}, {len(rows)} distinct sets, {edge} {chk(1.0 if ok else 0.0, 1.0, 0.0)}  ({time.time() - t0:.1f}s)")
    return rows

def pair_shortest(b, nds, m, side, thr=0.25, name="1/4"):
    t0 = time.time()
    todo = pair_sets(b)
    n0 = len(todo)
    done, wnd = [], {}
    for nd in nds:
        nxt = []
        for (a, c) in todo:
            el, eh, mu_lo, mu_hi, slack = pair_band(b, a, c, nd, m, side=side)
            v = eh if side else (-1.0 if el is None else el)
            if (v < thr) if side else (v >= thr):
                done.append((a, c, v))
                wnd[nd] = wnd.get(nd, 0) + 1
            else:
                nxt.append((a, c))
        todo = nxt
        if not todo:
            break
    ok = not todo
    key = (max(done, key=lambda r: r[2]) if side else min(done, key=lambda r: r[2])) if done else (-1, -1, -1.0)
    spread = " ".join(f"{nd}:{wnd[nd]}" for nd in sorted(wnd))
    verb = f"clears alpha_1 < {name} at every pair" if side else f"fails {name} at every pair"
    tail = ""
    if not ok:
        ups = []
        for (a, c) in todo:
            el, eh, mu_lo, mu_hi, slack = pair_band(b, a, c, nds[-1], m, side=not side)
            ups.append(f"{{{a},{c}}} S={a + c - b + 1} D={c - a} alpha_1 {'>' if side else '<'} {(-1.0 if el is None else el) if side else eh:.7f}")
        note = "not shown to clear" if side else "no positive lower certificate"
        tail = f"; {len(todo)} UNCLEAR at {nds[-1]} digits, {note}: " + ", ".join(ups)
    print(f"  q={b:3d} {n0:5d} distinct sets {verb}: window digits used {spread}, {'worst' if side else 'closest'} {{{key[0]},{key[1]}}} at alpha_1 {'<' if side else '>'} {key[2]:.7f}{tail} {chk(1.0 if ok else 0.0, 1.0, 0.0)}  ({time.time() - t0:.1f}s)", flush=True)
    return b, ok, key, wnd

def pair_first(b, nds, m, thr=0.25, name="1/4", want=None):
    t0 = time.time()
    best, arg, wnd = None, None, None
    for (a, c) in pair_sets(b):
        for nd in nds:
            el, eh, mu_lo, mu_hi, slack = pair_band(b, a, c, nd, m, side=True)
            if eh < thr:
                break
        if best is None or eh < best:
            best, arg, wnd = eh, (a, c), nd
    ok = best < thr
    pin = "" if want is None else " " + chk(1.0 if ok == want[0] else 0.0, 1.0, 0.0) + chk(best, want[1], 5e-7)
    print(f"  q={b:3d} {len(pair_sets(b)):5d} distinct sets, best pair {{{arg[0]},{arg[1]}}} at {wnd} window digits: alpha_1 < {best:.7f} [{'CLEARS' if ok else 'does not clear'} {name}]{pin}  ({time.time() - t0:.1f}s)", flush=True)
    return b, ok, arg, best

def pair_some(b, nds, m, thr=0.25, name="1/4"):
    t0 = time.time()
    best, arg, wnd = -1.0, None, None
    for nd in nds:
        for (a, c) in pair_sets(b):
            el, eh, mu_lo, mu_hi, slack = pair_band(b, a, c, nd, m, side=False)
            v = -1.0 if el is None else el
            if v > best:
                best, arg, wnd = v, (a, c), nd
            if v >= thr:
                break
        if best >= thr:
            break
    print(f"  q={b:3d} {len(pair_sets(b)):5d} distinct sets, witness pair {{{arg[0]},{arg[1]}}} at {wnd} window digits: alpha_1 > {best:.7f} [{'FAILS' if best >= thr else 'no witness at these windows'} {name}] {chk(1.0 if best >= thr else 0.0, 1.0, 0.0)}  ({time.time() - t0:.1f}s)", flush=True)
    return b, best >= thr, arg, best

def pair_ladder(bases, a, c, nd, m):
    t0 = time.time()
    out = []
    for b in bases:
        el, eh, mu_lo, mu_hi, slack = pair_band(b, a, c, nd, m)
        out.append((b, el, eh))
    print(f"certified alpha_1 ladder at {nd} digits sub-scan {m}, the interval class {{{a},{c}}} in each base: " + " ".join(f"q={b} [{-1.0 if el is None else el:.7f},{eh:.7f}]" for b, el, eh in out) + f"  ({time.time() - t0:.1f}s)")
    return out

# VERBS

def main():
    verb = sys.argv[1] if len(sys.argv) > 1 else "check"
    t0 = time.time()
    if verb == "check":
        digit_sums()
        gasket_anchor()
        entropy_bound()
        grids(GRID[:2])
        lemma_a("gasket 2D", *GASKET, 129, {127: 0.808166, 129: 0.809637})
        windows(WINDOWS[:1])
        kernel_check()
        sandwich_iv("gasket 2D", *GASKET, 2, 256)
        certify_windows([("carpet 2D", CARPET, 4, 24)])
        corollary("gasket 2D", *GASKET, 9, 10, {(5, 10): 0.0014741})
    elif verb == "grid":
        grids(GRID)
    elif verb == "sandwich":
        sandwich("gasket 2D", *GASKET, 2, 256, 1.0105, 1.1029)
        sandwich("gasket 2D", *GASKET, 3, 256, 1.0126, 1.1022)
        sandwich("gasket 2D", *GASKET, 4, 300, 1.0096, 1.1046)
        sandwich("carpet 2D", *CARPET, 2, 256, 0.5828, 0.9507)
        sandwich("carpet 2D", *CARPET, 3, 400, None, 0.9421)
        sandwich("gasket base 4", *GASKET4, 4, 20000, None, 0.4864)
        sandwich("carpet base 9", *CARPET9, 3, 20000, None, 0.3704)
    elif verb == "certify":
        gasket_anchor()
        entropy_bound()
        kernel_check()
        sandwich_iv("gasket 2D", *GASKET, 2, 256)
        sandwich_iv("gasket 2D", *GASKET, 3, 256)
        certify_windows([("carpet 2D", CARPET, 4, 24), ("carpet 2D", CARPET, 5, 24)])
    elif verb == "windows":
        windows(WINDOWS)
    elif verb == "moments":
        moments(MOMENTS)
    elif verb == "lemma":
        lemma_a("gasket 2D", *GASKET, 301, {257: 0.830915, 255: 0.830253, 129: 0.809637, 127: 0.808166})
    elif verb == "corollary":
        corollary("gasket 2D", *GASKET, 15, 12, {(3, 2): 0.2222223, (5, 10): 0.0014741})
        corollary("carpet 2D", *CARPET, 11, 8, {(4, 2): 0.0625, (5, 8): 0.0002842})
        base_peel("gasket 2D", *GASKET, [3, 5, 7], 11)
        base_peel("carpet 2D", *CARPET, [2, 5, 7], 7)
    elif verb == "criterion":
        hat_missing_check(9, 4)
        hat_missing_check(10, 5)
        criterion_band("carpet base 9", 9, 4, 4, 4000, {1.0: 0.334604, 1.46: 0.159988, 1.5: 0.148588, 1.6: 0.122780, 1.8: 0.081955, 2.0: 0.053605})
        order_rows("carpet base 9", 9, 4, 5, 3000, [1.5, 235 / 154], {1.5: 0.149397, 235 / 154: 0.142274}, 60)
        two_dim_moments([("carpet 2D", CARPET, 5, 24, {1.0: 0.406200, 1.5: 0.195631}), ("carpet base 9", CARPET9, 5, 64, {1.0: 0.343674, 1.5: 0.153069})], [1.0, 1.5, 235 / 154, 1.6, 1.7, 1.8])
    elif verb == "least":
        hat_missing_check(21, 0)
        hat_missing_check(34, 16)
        base_cert(10, 5, 4, 16, 2, " calibration against the study float row lambda 2.245878 and Maynard lambda_(1,4) < 2.24190")
        base_cert(10, 5, 5, 16, 2, " calibration against the study float row lambda 2.242123 and 27/77 = 0.3506494")
        base_cert(10, 5, 6, 16, 2, " calibration: 27/77 = 0.3506494 is a finite-window upper bound, not the exponent")
        base_cert(9, 0, 4, 16, 2, " calibration against the study float row lambda 2.033782 and Karwatowski 0.3219")
        base_cert(21, 0, 5, 8, 1, " THE LEAST BASE CARRYING ONE SUCH SET")
        base_family(20, 4, 8, False)
        base_family(34, 4, 8, True)
        base_cert(33, 15, 4, 8, 1, " THE WITNESS THAT 34 IS LEAST FOR THE WHOLE FAMILY")
        base_ladder([10, 14, 18, 20, 21, 22, 26, 30, 33, 34], 4, 8, lambda b: 0)
        base_ladder([10, 14, 18, 20, 21, 22, 26, 30, 33, 34], 4, 8, lambda b: b // 2)
        shift_check(21, 0, 4, 400)
        shift_check(34, 16, 3, 600)
        grids([("base 21 missing 0", base_missing(21, 0), [None] * 4)])
    elif verb == "six":
        base_cert(21, 0, 6, 8, 1, " THE LEAST BASE CARRYING ONE SUCH SET, THE HEADLINE WINDOW")
        base_cert(10, 5, 7, 16, 2, " calibration: the exponent is strictly under 27/77 = 0.3506494")
    elif verb == "pairs":
        hat_pair_check(21, 0, 1)
        hat_pair_check(32, 7, 19)
        pair_census([6, 9, 10, 12])
        pair_cert(32, 0, 1, 4, 8, 1, " THE LEAST BASE CARRYING ONE CERTIFIED SUCH SET")
        pair_cert(31, 0, 1, 4, 8, 1, " THE WITNESS THAT 32 IS LEAST FOR THE INTERVAL CLASS")
        pair_cert(20, 3, 11, 5, 8, 1, " THE WITNESS THAT 21 IS LEAST FOR THE WHOLE FAMILY AGAINST 1/3", 1 / 3, "1/3")
        pair_first(13, [3], 8, 1 / 3, "1/3", (True, 0.3318819))
        pair_first(12, [5], 8, 1 / 3, "1/3", (False, 0.3371162))
        pair_ladder([20, 24, 28, 30, 31, 32, 33, 36, 40], 0, 1, 4, 8)
    elif verb == "pairone":
        v = [int(x) for x in sys.argv[2:]]
        pair_cert(v[0], v[1], v[2], v[3], v[4], v[5] if len(v) > 5 else 1)
    elif verb == "pairfail":
        v = sys.argv[2:]
        thr, name = (1 / 3, "1/3") if len(v) > 2 and v[2] == "third" else (0.25, "1/4")
        for b in range(int(v[0]), int(v[1]) + 1):
            pair_shortest(b, [2, 3, 4, 5], 8, False, thr, name)
    elif verb == "pairclear":
        v = sys.argv[2:]
        thr, name = (1 / 3, "1/3") if len(v) > 2 and v[2] == "third" else (0.25, "1/4")
        for b in range(int(v[0]), int(v[1]) + 1):
            pair_shortest(b, [2, 3, 4], 8, True, thr, name)
    elif verb == "pairsome":
        v = sys.argv[2:]
        thr, name = (1 / 3, "1/3") if len(v) > 2 and v[2] == "third" else (0.25, "1/4")
        for b in range(int(v[0]), int(v[1]) + 1):
            pair_some(b, [3, 4, 5], 8, thr, name)
    elif verb == "pairfirst":
        v = sys.argv[2:]
        thr, name = (1 / 3, "1/3") if len(v) > 2 and v[2] == "third" else (0.25, "1/4")
        for b in range(int(v[0]), int(v[1]) + 1):
            pair_first(b, [2, 3, 4, 5], 8, thr, name)
    elif verb == "family":
        family_close(35, 125, 8, [2, 3, 4])
    else:
        raise SystemExit("verbs: check grid sandwich certify windows moments lemma corollary criterion least six family pairs pairfail pairclear")
    close(t0)

main()
