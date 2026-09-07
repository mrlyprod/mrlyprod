import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "mrly-euler"))

import euler

# FAMILIES

def full(q):
    return (1 << q) - 1

def miss(q, d):
    return full(q) & ~(1 << d)

FAMILIES = [(3, 0b011, 14), (4, 0b0011, 11), (5, 0b00011, 9), (10, miss(10, 9), 6)]

CONTROLS = [(2, 0b11, 20), (3, 0b111, 13)]

THETAMAX = {(3, 0b011): 0.4869, (4, 0b0011): 0.4819, (5, 0b00011): 0.4633,
            (10, miss(10, 9)): 0.4871}

def digs(q, F):
    return [d for d in range(q) if (F >> d) & 1]

def alpha_of(q, F):
    from math import log
    return log(len(digs(q, F))) / log(q)

def strings(q, F, L):
    import numpy
    out = numpy.zeros(1, dtype=numpy.int64)
    d = numpy.array(digs(q, F), dtype=numpy.int64)
    for i in range(L):
        out = (out[:, None] + d[None, :] * (q ** i)).ravel()
    out.sort()
    return out

def indicator(q, F, L):
    import numpy
    v = numpy.zeros(q ** L, dtype=numpy.float64)
    v[strings(q, F, L)] = 1.0
    return v

# MOBIUS

def mu_upto(N):
    import numpy
    mu = numpy.ones(N + 1, dtype=numpy.int8)
    mu[0] = 0
    sieve = numpy.ones(N + 1, dtype=bool)
    sieve[:2] = False
    for p in range(2, int(N ** 0.5) + 1):
        if sieve[p]:
            sieve[p * p:: p] = False
    primes = numpy.nonzero(sieve)[0]
    for p in primes:
        mu[p:: p] = -mu[p:: p]
    for p in primes:
        if p * p > N:
            break
        mu[p * p:: p * p] = 0
    return mu

def mu_of(vals):
    import numpy
    rem = numpy.array(vals, dtype=numpy.int64)
    mu = numpy.ones(rem.shape, dtype=numpy.int8)
    top = int(rem.max())
    lim = int(top ** 0.5) + 1
    sieve = numpy.ones(lim + 1, dtype=bool)
    sieve[:2] = False
    for p in range(2, int(lim ** 0.5) + 1):
        if sieve[p]:
            sieve[p * p:: p] = False
    for p in numpy.nonzero(sieve)[0]:
        d = rem % p == 0
        if not d.any():
            continue
        rem[d] //= p
        mu[d] = -mu[d]
        again = d & (rem % p == 0)
        if again.any():
            mu[again] = 0
            while again.any():
                rem[again] //= p
                again = again & (rem % p == 0)
    mu[rem > 1] = -mu[rem > 1]
    return mu

VERBS = {}

# SPLIT

def levels(q, L):
    import numpy
    lev = numpy.full(q ** L, L, dtype=numpy.int16)
    for j in range(L - 1, -1, -1):
        lev[:: q ** (L - j)] = j
    return lev

def split_one(q, F, L):
    import numpy
    from math import log
    N = q ** L
    k = len(digs(q, F))
    a = alpha_of(q, F)
    ind = indicator(q, F, L)
    mu = mu_upto(N - 1).astype(numpy.float64)[:N]
    direct = float(numpy.dot(ind, mu))
    Gh = numpy.fft.fft(ind)
    del ind
    Sh = numpy.fft.fft(mu)
    del mu
    paired = float((numpy.conj(Gh) * Sh).sum().real) / N
    zero = float((numpy.conj(Gh[0]) * Sh[0]).real) / N
    e2S = float((numpy.abs(Sh) ** 2).sum()) / N
    del Sh
    absG = numpy.abs(Gh)
    del Gh
    e2G = float((absG ** 2).sum()) / N
    lev = levels(q, L)
    w = numpy.bincount(lev, weights=absG, minlength=L + 1)
    del absG, lev
    C = float(w.sum())
    lq = log(q)
    ell1 = log(C / N) / (L * lq)
    cand = [((log(w[j]) / lq - L) / L + min(0.5 + j / (2.0 * L), 0.75), j)
            for j in range(L + 1) if w[j] > 0]
    best, arg = max(cand)
    return dict(q=q, F=F, L=L, k=k, alpha=a, direct=direct, paired=paired,
                e2G=e2G, e2S=e2S, zero=zero, w=w, C=C, ell1=ell1,
                unif=ell1 + 0.75, best=best, arg=arg,
                top=float(w[L]) / C, hi=float(w[(L + 1) // 2:].sum()) / C)

def split():
    import numpy
    print("THE POSITION PAIRING ON THE GRID, EXACT")
    print("M_F(q^L) = q^(-L) sum_a G_L(a/q^L) S_L(a/q^L), both sides finite")
    print("q F        L  M_F(q^L)   pairing      int|G_L|^2 k^L      int|S_L|^2  a=0 term")
    rows = []
    for q, F, L in FAMILIES + CONTROLS:
        r = split_one(q, F, L)
        rows.append(r)
        print("%2d %-8s %2d %10.1f %12.4f %10.0f %8d %11.0f %9.5f"
              % (q, euler.show(F, q), L, r["direct"], r["paired"], r["e2G"],
                 r["k"] ** L, r["e2S"], r["zero"]))
    print()
    print("THE LEVEL PROFILE of the l^1 mass, C_L = sum_j k^(L-j) c_j")
    print("share of C_L at level j = L (primitive denominator q^L), at levels j >= L/2,")
    print("and the proved floor m/q for the top level")
    print("q F        L  C_L/q^L      top share  floor m/q  share j >= L/2")
    for r in rows:
        m = r["q"] - r["k"]
        print("%2d %-8s %2d %12.4f %10.6f %10.6f %14.6f"
              % (r["q"], euler.show(r["F"], r["q"]), r["L"], r["C"] / r["q"] ** r["L"],
                 r["top"], m / r["q"], r["hi"]))
    print()
    print("THE COST, exponents in log_q against the trivial alpha")
    print("cs = Cauchy-Schwarz (alpha+1)/2, unif = l^1 times the uniform GRH max x^(3/4),")
    print("den = the per-denominator split, GRH max x^(1/2) q^(j/2) at level j")
    print("mob = alpha times the thetamax of mobius.md, the measured exponent of M_F itself")
    print("q F        L  alpha    alpha/2  mob      cs       unif     den      C_L/w_j  q/m")
    for r in rows:
        m = r["q"] - r["k"]
        tm = THETAMAX.get((r["q"], r["F"]))
        print("%2d %-8s %2d %8.6f %8.6f %8s %8.6f %8.6f %8.6f %8.4f %8.4f"
              % (r["q"], euler.show(r["F"], r["q"]), r["L"], r["alpha"], r["alpha"] / 2,
                 ("%.6f" % (r["alpha"] * tm)) if tm else "-",
                 (r["alpha"] + 1) / 2, r["unif"], r["best"],
                 r["C"] / r["w"][r["arg"]], r["q"] / m if m else float("inf")))
    print()
    print("THE LADDER IN L at base 3 F = {0,1}: the per-denominator gain is a constant")
    print("unif - den = log_q(C_L/w_L)/L and C_L/w_L <= q/m = 1.5 at every L")
    print(" L  unif     den      unif-den  L(unif-den) C_L/w_L  top share")
    for L in range(6, 15):
        r = split_one(3, 0b011, L)
        d = r["unif"] - r["best"]
        print("%2d %8.6f %8.6f %9.6f %11.6f %8.4f %10.6f"
              % (L, r["unif"], r["best"], d, L * d, r["C"] / r["w"][r["arg"]], r["top"]))
    print()
    print("THE ONE-STEP CONSTANT read off the l^1 norms themselves")
    print("C_L = sum_(a mod q^L) |G_L(a/q^L)|, ratio C_L/C_(L-1) against the grid sup B_q(F)")
    print("q F        L  C_L            C_L/C_(L-1)     sup_t sum_r |g((t+r)/q)|  top share")
    for q, F, L in FAMILIES:
        cs = []
        for l in range(1, min(L, 9) + 1):
            r = split_one(q, F, l)
            cs.append(r["C"])
        d = digs(q, F)
        t = numpy.linspace(0.0, 1.0, 200001)[:-1]
        h = numpy.zeros_like(t)
        for rr in range(q):
            z = numpy.zeros_like(t, dtype=numpy.complex128)
            for dd in d:
                z += numpy.exp(2j * numpy.pi * dd * (t + rr) / q)
            h += numpy.abs(z)
        for l in range(len(cs) - 2, len(cs)):
            print("%2d %-8s %2d %14.6f %15.9f %25.9f %10.6f"
                  % (q, euler.show(F, q), l + 1, cs[l], cs[l] / cs[l - 1], h.max(),
                     split_one(q, F, l + 1)["top"]))
    print()
    print("THE PRINCIPAL FIBRE carries almost none of the design meter")
    print("the a = 0 term is q^(-L) k^L M(q^L), exponent alpha - 1/2 under RH,")
    print("against the conjectured alpha/2 for M_F itself: alpha - 1/2 < alpha/2 iff alpha < 1")
    print("q F        L  M_F(q^L)   a=0 term   share      alpha-1/2  alpha/2")
    for r in rows:
        sh = r["zero"] / r["direct"] if r["direct"] else float("nan")
        print("%2d %-8s %2d %10.1f %10.5f %10.6f %10.6f %8.6f"
              % (r["q"], euler.show(r["F"], r["q"]), r["L"], r["direct"], r["zero"],
                 sh, r["alpha"] - 0.5, r["alpha"] / 2))

VERBS["split"] = split

# GLUE

GLUE = [(3, 0b011, 18), (4, 0b0011, 14), (5, 0b00011, 12), (10, miss(10, 9), 7)]

GLUE_CONTROLS = [(2, 0b11, 20), (3, 0b111, 12)]

def coeffs(q, F, L):
    import numpy
    N = q ** L
    sf = strings(q, F, L)
    sf = sf[sf >= 1]
    mu = mu_of(sf).astype(numpy.int64)
    c = numpy.zeros(N + 1, dtype=numpy.int32)
    for e, m in zip(sf, mu):
        if m == 0:
            continue
        d = sf[: numpy.searchsorted(sf, N // e, "right")]
        c[d * e] += m
    return c

PHASES = [0.0, 0.25, 0.5, 0.75]

def partials(q, F, L):
    import numpy
    sf = strings(q, F, L)
    sf = sf[sf >= 1]
    mu = mu_of(sf).astype(numpy.int64)
    out = []
    for l in range(1, L):
        row = []
        for c in PHASES:
            x = int(float(q) ** (l + c))
            cut = numpy.searchsorted(sf, x, "right")
            a = numpy.searchsorted(sf, x // sf[:cut], "right")
            row.append((x, int((mu[:cut] * a).sum())))
        out.append(row)
    return out, sf, mu

def glue():
    import numpy
    from math import log
    print("THE COEFFICIENTS OF ZETA_F M_F, c_F(n) = sum_(d e = n, d, e in S_F) mu(e)")
    print("q F        L      n<=q^L    nonzero  first n>1  c_F(n)  max|c_F|  at n")
    for q, F, L in [(3, 0b011, 12), (4, 0b0011, 9), (5, 0b00011, 8), (10, miss(10, 9), 6)]:
        c = coeffs(q, F, L)
        nz = numpy.nonzero(c[2:])[0] + 2
        am = int(numpy.abs(c).argmax())
        print("%2d %-8s %2d %10d %10d %10d %7d %9d %5d"
              % (q, euler.show(F, q), L, q ** L, len(nz) + (1 if c[1] else 0),
                 int(nz[0]), int(c[nz[0]]), int(abs(c[am])), am))
    print()
    print("THE PARTIAL SUMS P(x) = sum_(n<=x) c_F(n) = sum_(e in S_F) mu(e) A_F(x/e)")
    print("the abscissa of D_F = zeta_F M_F - 1 is read off log|P|/log x against alpha")
    for q, F, L in GLUE + GLUE_CONTROLS:
        a = alpha_of(q, F)
        ps, sf, mu = partials(q, F, L)
        print("q = %d  F = %s  alpha = %.6f  alpha/2 = %.6f" % (q, euler.show(F, q), a, a / 2))
        print("  L   P(x) at log_q x = L, L+1/4, L+1/2, L+3/4      P(x)/x^alpha at the same four")
        for l in range(1, L):
            row = ps[l - 1]
            print("  %2d %10d %10d %10d %10d   %9.6f %9.6f %9.6f %9.6f"
                  % ((l,) + tuple(v for _, v in row)
                     + tuple(v / float(x) ** a for x, v in row)))
    print()
    print("THE LIMIT TEST: sigma_c(D_F) < alpha forces M_F(sigma) -> 0 as sigma -> alpha+")
    print("M_F(sigma) = sum_(n in S_F) mu(n) n^(-sigma), summed to n <= q^L")
    print("q F        L  sigma-alpha  M_F(sigma)   tail bound q^(-L alpha/2)")
    for q, F, L in GLUE:
        a = alpha_of(q, F)
        sf = strings(q, F, L)
        sf = sf[sf >= 1]
        mu = mu_of(sf).astype(numpy.float64)
        x = sf.astype(numpy.float64)
        for eps in [0.2, 0.1, 0.05, 0.02, 0.0]:
            v = float((mu * x ** (-(a + eps))).sum())
            print("%2d %-8s %2d %12.4f %12.6f %18.2e"
                  % (q, euler.show(F, q), L, eps, v, float(q) ** (-L * a / 2)))

VERBS["glue"] = glue

# INVERSE

SEEDS = {(3, 0b011): ("0.720788", "28.60568"), (10, miss(10, 9)): ("1.001589", "2.7392")}

def dirichlet_inverse(q, F, L):
    import numpy
    N = q ** L
    if F == full(q):
        sf = numpy.arange(2, N + 1, dtype=numpy.int64)
    else:
        sf = strings(q, F, L)
        sf = sf[sf >= 2]
        if (F >> 1) & 1 and (L == 0 or F & 1):
            sf = numpy.append(sf, numpy.int64(N))
    nu = numpy.zeros(N + 1, dtype=numpy.int32)
    nu[1] = 1
    lo = 1
    while lo <= N:
        hi = min(2 * lo, N + 1)
        for d in sf[: numpy.searchsorted(sf, hi - 1, "right")]:
            m0 = (lo + d - 1) // d
            m1 = (hi - 1) // d + 1
            if m1 <= m0:
                continue
            nu[m0 * d: (m1 - 1) * d + 1: d] -= nu[m0: m1]
        lo = hi
    return nu

def refine(q, F):
    from mpmath import mp
    sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                    "..", "design-zeta"))
    import design_zeta
    d = design_zeta.Design(q, tuple(digs(q, F)))
    re0, im0 = SEEDS[(q, F)]
    s = mp.mpc(re0, im0)
    for _ in range(8):
        v, _ = d.zeta(s)
        h = mp.mpf(10) ** -8
        v2, _ = d.zeta(s + h)
        s = s - v * h / (v2 - v)
    v, e = d.zeta(s)
    return s, v, e, d.alpha

def series_at(nu, q, L, sigmas):
    import numpy
    acc = [0.0] * len(sigmas)
    out = [[] for _ in sigmas]
    N = q ** L
    lo = 1
    marks = [q ** l for l in range(1, L + 1)]
    m = 0
    while lo <= N:
        hi = min(lo + (1 << 20), N + 1)
        if m < len(marks) and marks[m] < hi:
            hi = marks[m] + 1
        n = numpy.arange(lo, hi, dtype=numpy.float64)
        v = nu[lo:hi].astype(numpy.float64)
        for i, sg in enumerate(sigmas):
            acc[i] += float((v * n ** (-sg)).sum())
        if m < len(marks) and hi == marks[m] + 1:
            for i in range(len(sigmas)):
                out[i].append(acc[i])
            m += 1
        lo = hi
    return out

def abscissa(q, F, L):
    from mpmath import mp
    sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                    "..", "design-zeta"))
    import design_zeta
    d = design_zeta.Design(q, tuple(digs(q, F)))
    nu = dirichlet_inverse(q, F, L)
    a = float(d.alpha)
    r = float(refine(q, F)[0].real)
    sigmas = [round(r + 0.08, 4), round(r + 0.20, 4)]
    out = series_at(nu, q, L, sigmas)
    tgt = [float(1 / d.zeta(mp.mpf(str(sg)))[0]) for sg in sigmas]
    return sigmas, out, tgt, a, r

def ladder_stats(nu, q, L):
    import numpy
    N = q ** L
    marks = [q ** l for l in range(1, L + 1)]
    sig, mx = [], []
    run = 0
    best = 0
    lo, m = 1, 0
    while lo <= N:
        hi = min(lo + (1 << 20), N + 1)
        if m < len(marks) and marks[m] < hi:
            hi = marks[m] + 1
        c = numpy.cumsum(nu[lo:hi].astype(numpy.int64))
        c += run
        run = int(c[-1])
        numpy.abs(c, out=c)
        best = max(best, int(c.max()))
        if m < len(marks) and hi == marks[m] + 1:
            sig.append(run)
            mx.append(best)
            m += 1
        lo = hi
    return sig, mx

def inverse():
    import numpy
    from math import log
    print("THE DIRICHLET INVERSE OF THE DESIGN INDICATOR, nu_F = 1_(S_F)^(-1)")
    print("zeta_F(s) N_F(s) = 1 exactly, the identity that replaces zeta M = 1 on a design")
    print("control: the full digit set gives nu_F = mu term for term")
    for q in [2, 3]:
        nu = dirichlet_inverse(q, full(q), 17 if q == 2 else 11)
        mu = mu_upto(len(nu) - 1)
        print("  q = %d full, n <= %d, nu_F = mu at every n: %s"
              % (q, len(nu) - 1, bool((nu[1:] == mu[1:]).all())))
    print()
    print("THE ZERO THAT FORCES THE ABSCISSA, refined on the sibling engine lab/design-zeta")
    print("sigma_c(N_F) >= Re rho for every zero rho of zeta_F with Re rho > alpha")
    print("q F        zero rho                                   |zeta_F(rho)| bound     alpha")
    from mpmath import mp
    for q, F in [(3, 0b011), (10, miss(10, 9))]:
        r, v, e, a = refine(q, F)
        print("%2d %-8s %-42s %-9s %-9s %s"
              % (q, euler.show(F, q), mp.nstr(r, 16), mp.nstr(abs(v), 4),
                 mp.nstr(e, 4), mp.nstr(a, 10)))
    print()
    print("THE DESIGN MERTENS OF nu_F against the design's own mass A_F(x) and against")
    print("the rightmost zero of zeta_F, whose real part is a lower bound for the abscissa")
    for q, F, L in [(3, 0b011, 16), (4, 0b0011, 12), (5, 0b00011, 10), (10, miss(10, 9), 7)]:
        a = alpha_of(q, F)
        k = len(digs(q, F))
        nu = dirichlet_inverse(q, F, L)
        raw, mxs = ladder_stats(nu, q, L)
        rz = float(refine(q, F)[0].real) if (q, F) in SEEDS else None
        print("q = %d  F = %s  alpha = %.6f  rightmost censused zero Re = %s"
              % (q, euler.show(F, q), a, ("%.6f" % rz) if rz else "not censused"))
        print("  the level ratio of the running maximum against q^alpha = k = %d and"
              % k)
        print("  q^(Re rho) = %s"
              % ("%.6f" % (q ** rz) if rz else "not censused"))
        print("  L   sum nu_F(n)   max|sum|    level ratio  exponent   max/A_F(q^L)")
        prev = 0
        for l in range(1, L + 1):
            x = q ** l
            mx = mxs[l - 1]
            ex = log(mx) / log(x) if mx > 1 else 0.0
            print("  %2d %13d %12d %12s %10.6f %14.4f"
                  % (l, raw[l - 1], mx, ("%.4f" % (mx / prev)) if prev else "-", ex,
                     mx / float(k ** l)))
            prev = mx
        del nu

    print()
    print("THE ABSCISSA TEST: partial sums of N_F(sigma) = sum nu_F(n) n^(-sigma) against")
    print("1/zeta_F(sigma) from lab/design-zeta, at sigma above and below Re rho;")
    print("the transport theorem, not this table, places sigma_c(N_F) at or above Re rho")
    for q, F, L in [(3, 0b011, 16), (10, miss(10, 9), 7)]:
        sigmas, out, tgt, a, r = abscissa(q, F, L)
        print("q = %d  F = %s  alpha = %.7f  Re rho = %.7f" % (q, euler.show(F, q), a, r))
        print("  sigma    1/zeta_F     partial at q^(L-2)  q^(L-1)      q^L        error")
        for i, sg in enumerate(sigmas):
            p3, p2, p1 = out[i][-3], out[i][-2], out[i][-1]
            print("  %-8.4f %12.6f %14.6f %12.6f %12.6f %11.2e"
                  % (sg, tgt[i], p3, p2, p1, abs(p1 - tgt[i])))

VERBS["inverse"] = inverse

# BOX

BOXES = [(10, miss(10, 9), "1.00150", "1.00168", "2.73915", "2.73925", 1),
         (3, 0b011, "0.72074", "0.72084", "28.60563", "28.60573", 1),
         (10, miss(10, 9), "0.99900", "1.00050", "2.73810", "2.74030", 0)]

def box():
    from mpmath import mp
    sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                    "..", "design-zeta"))
    import design_zeta
    print("WINDING BOXES FOR THE ZEROS OF zeta_F, argument principle on lab/design-zeta")
    print("winding 1 on a rectangle certifies exactly one zero inside it, so Re rho is")
    print("pinned to the box edges; winding 0 certifies no zero there")
    for q, F, x0, x1, y0, y1, want in BOXES:
        d = design_zeta.Design(q, tuple(digs(q, F)))
        f = design_zeta.Cache(d)
        w, mx = design_zeta.box_phase(f, mp.mpf(x0), mp.mpf(x1), mp.mpf(y0), mp.mpf(y1), 16)
        lo = min(abs(v) for v in f.m.values())
        print("q = %d  F = %s  alpha = %s"
              % (q, euler.show(F, q), mp.nstr(d.alpha, 10)))
        print("  Re in [%s, %s]  Im in [%s, %s]" % (x0, x1, y0, y1))
        print("  winding %s  expected %d  max phase step %s  evaluations %d"
              % (mp.nstr(w, 8), want, mp.nstr(mx, 4), f.n))
        print("  min |zeta_F| on the contour %s  engine bound %s  ratio %s"
              % (mp.nstr(lo, 4), mp.nstr(f.emax, 4), mp.nstr(lo / f.emax, 4)))

VERBS["box"] = box

if __name__ == "__main__":
    VERBS[sys.argv[1] if len(sys.argv) > 1 else "split"]()
