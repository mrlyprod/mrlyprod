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
    else:
        raise SystemExit("verbs: check grid sandwich certify windows moments lemma")
    close(t0)

main()
