import sys
import time
from fractions import Fraction
from math import gcd, pi

import numpy as np

BASE3 = (3, frozenset({0, 1}), "base 3, digits {0,1}")
BASE10 = (10, frozenset(range(9)), "base 10, digit 9 missing")

TWO_PI_I = 2j * pi

# THE DESIGN


def holds(base, digits, n):
    if n == 0:
        return True
    while n > 0:
        if n % base not in digits:
            return False
        n //= base
    return True


def flags(base, digits, q):
    keep = np.zeros(q + 1, dtype=bool)
    for n in range(q + 1):
        keep[n] = holds(base, digits, n)
    return keep


def mobius_sieve(n):
    mu = np.ones(n + 1, dtype=np.int64)
    primes = np.ones(n + 1, dtype=bool)
    primes[:2] = False
    for p in range(2, n + 1):
        if not primes[p]:
            continue
        primes[p * p :: p] = False
        mu[p::p] *= -1
        mu[p * p :: p * p] = 0
    mu[0] = 0
    return mu


def totients(n):
    phi = np.arange(n + 1, dtype=np.int64)
    for p in range(2, n + 1):
        if phi[p] == p:
            phi[p::p] -= phi[p::p] // p
    return phi


# THE DILATED MERTENS SUMS


def mertens_dilated(keep, mu, q, d):
    c = np.arange(1, q // d + 1, dtype=np.int64)
    return int(mu[c][keep[d * c]].sum())


def mertens_design(keep, mu, q):
    return mertens_dilated(keep, mu, q, 1)


# THE LITERAL FOURIER SIDE, DENOMINATOR SET


def literal_denominator(keep, q, freqs):
    out = {m: 0j for m in freqs}
    for b in range(1, q + 1):
        if not keep[b]:
            continue
        a = np.arange(1, b + 1, dtype=np.int64)
        a = a[np.gcd(a, b) == 1]
        for m in freqs:
            r = (m * a) % b
            out[m] += complex(np.exp(TWO_PI_I * (r / b)).sum())
    return out


def card_denominator(keep, phi, q):
    return int(phi[1 : q + 1][keep[1 : q + 1]].sum())


def ladder_check(keep, mu, q):
    total = 0j
    worst = 0.0
    at = 0
    bad = 0
    for b in range(1, q + 1):
        if not keep[b]:
            continue
        a = np.arange(1, b + 1, dtype=np.int64)
        a = a[np.gcd(a, b) == 1]
        v = complex(np.exp(TWO_PI_I * (a % b) / b).sum())
        total += v
        e = abs(v - int(mu[b]))
        if e > worst:
            worst, at = e, b
        if round(v.real) != int(mu[b]) or abs(v.imag) > 0.5:
            bad += 1
    return total, worst, at, bad


# THE STRICT SET


def design_list(keep, q):
    return np.nonzero(keep[1 : q + 1])[0] + 1


def strict_literal(keep, q):
    sf = design_list(keep, q)
    total = 0j
    card = 0
    for i, b in enumerate(sf):
        b = int(b)
        a = sf[: i + 1]
        a = a[np.gcd(a, b) == 1]
        card += a.size
        total += complex(np.exp(TWO_PI_I * (a % b) / b).sum())
    return total, card


def strict_ramanujan_literal(keep, b):
    a = np.arange(1, b + 1, dtype=np.int64)
    a = a[keep[a] & (np.gcd(a, b) == 1)]
    return complex(np.exp(TWO_PI_I * (a % b) / b).sum())


def strict_ramanujan_divisor(keep, mu, b):
    total = 0j
    for d in range(1, b + 1):
        if b % d or mu[d] == 0:
            continue
        n = b // d
        ap = np.arange(1, n + 1, dtype=np.int64)
        ap = ap[keep[d * ap]]
        total += int(mu[d]) * complex(np.exp(TWO_PI_I * (ap % n) / n).sum())
    return total


def strict_weights(keep, mu, phi, q):
    sf = design_list(keep, q)
    plain = 0
    counted = 0
    ratio = 0.0
    for i, b in enumerate(sf):
        b = int(b)
        a = sf[: i + 1]
        pf = int((np.gcd(a, b) == 1).sum())
        plain += int(mu[b])
        counted += int(mu[b]) * pf
        ratio += int(mu[b]) * pf / int(phi[b])
    return plain, counted, ratio


# THE GCD KERNEL


def kernel_sum(keep, mu, q):
    ms = {}
    for d in range(1, q + 1):
        v = mertens_dilated(keep, mu, q, d)
        if v:
            ms[d] = v
    total = Fraction(0)
    ks = sorted(ms)
    for d in ks:
        for e in ks:
            total += Fraction(gcd(d, e) ** 2 * ms[d] * ms[e], d * e)
    return total, ms


def fourier_side(keep, q, cut):
    nodes = []
    for b in range(1, q + 1):
        if not keep[b]:
            continue
        for a in range(1, b + 1):
            if gcd(a, b) == 1:
                nodes.append((a, b))
    m = len(nodes)
    num = np.array([a for a, _ in nodes], dtype=np.int64)
    den = np.array([b for _, b in nodes], dtype=np.int64)
    total = 0.0
    step = max(1, 2_000_000 // max(m, 1))
    k = 1
    while k <= cut:
        hi = min(cut, k + step - 1)
        ks = np.arange(k, hi + 1, dtype=np.int64)
        r = (ks[:, None] * num[None, :]) % den[None, :]
        s = np.exp(TWO_PI_I * (r / den[None, :])).sum(axis=1)
        total += float((np.abs(s) ** 2 / ks.astype(float) ** 2).sum())
        k = hi + 1
    return 2.0 * total, m


# THE BACKWARD SCAN


def kernel_float(keep, mu, w, q):
    ms = np.zeros(q + 1)
    for d in range(1, q + 1):
        ms[d] = mertens_dilated(keep, mu, q, d)
    v = ms[1 : q + 1]
    return float(v @ w[:q, :q] @ v)


def gcd_weights(qmax):
    d = np.arange(1, qmax + 1, dtype=np.int64)
    g = np.gcd.outer(d, d).astype(np.float64)
    return g * g / np.outer(d.astype(np.float64), d.astype(np.float64))


def scan_backward(design, qmax):
    keep = keep_of(design, qmax)
    mu = mobius_sieve(qmax)
    w = gcd_weights(qmax)
    best = 0.0
    at = 0
    tail = 0.0
    tail_at = 0
    seen = 0
    bad = 0
    for q in range(1, qmax + 1):
        if not keep[q]:
            continue
        seen += 1
        g = kernel_float(keep, mu, w, q)
        mf = mertens_design(keep, mu, q)
        ident = g * pi * pi / 3.0
        r = 2.0 * mf * mf / ident
        if r > 1.0 + 1e-9:
            bad += 1
        if r > best:
            best, at = r, q
        if q >= SCAN_FLOOR and r > tail:
            tail, tail_at = r, q
    return seen, bad, best, at, tail, tail_at


# THE CONTROL


def farey_delta_square(keep, q):
    nodes = []
    for b in range(1, q + 1):
        if not keep[b]:
            continue
        for a in range(1, b + 1):
            if gcd(a, b) == 1:
                nodes.append(Fraction(a, b))
    nodes.sort()
    m = len(nodes)
    s2 = sum((r - Fraction(j + 1, m)) ** 2 for j, r in enumerate(nodes))
    return m, s2


def reflection_closed(keep, q):
    for b in range(1, q + 1):
        if not keep[b]:
            continue
        for a in range(1, b):
            if gcd(a, b) == 1 and not (keep[a] == keep[b - a]):
                return False
    return True


# THE VERBS

FREQS = [1, 2, 3, 4, 5, 6, 12]

DEN_SMALL = [(BASE3, 2187), (BASE10, 1000), ((0, None, "full set, control"), 300)]

DEN_LADDER = [
    (BASE3, [243, 2187, 19683, 100000]),
    (BASE10, [100, 1000, 10000]),
    ((0, None, "full set, control"), [100, 1000, 10000]),
]


def keep_of(design, q):
    base, digits, _ = design
    if digits is None:
        k = np.ones(q + 1, dtype=bool)
        return k
    return flags(base, digits, q)


def verb_denominator():
    print("DENOMINATOR SET, THE FREQUENCY-m IDENTITY")
    for design, q in DEN_SMALL:
        keep = keep_of(design, q)
        mu = mobius_sieve(q)
        lit = literal_denominator(keep, q, FREQS)
        print(f"\n{design[2]}, Q = {q}")
        print("    m | sum_(d|m) d M_F(Q/d; d) |    Re literal |    Im literal |       err")
        for m in FREQS:
            exact = sum(d * mertens_dilated(keep, mu, q, d) for d in range(1, m + 1) if m % d == 0)
            v = lit[m]
            err = abs(v - exact)
            print(f"{m:5d} | {exact:23d} | {v.real:13.9f} | {v.imag:13.9f} | {err:.3e}")

    print("\nDENOMINATOR SET, FREQUENCY 1 AGAINST THE DESIGN MERTENS SUM")
    print(
        "   set                        |      Q |  M_F(Q) |    Re literal |    Im literal |  agg err |"
        " per-b err |  at b | bad |    s"
    )
    for design, ladder in DEN_LADDER:
        for q in ladder:
            t0 = time.time()
            keep = keep_of(design, q)
            mu = mobius_sieve(q)
            exact = mertens_design(keep, mu, q)
            v, worst, at, bad = ladder_check(keep, mu, q)
            dt = time.time() - t0
            print(
                f"   {design[2]:26s} | {q:6d} | {exact:7d} | {v.real:13.9f} | "
                f"{v.imag:13.9f} | {abs(v - exact):.2e} | {worst:9.2e} | {at:5d} | {bad:3d} | {dt:4.1f}"
            )


SCAN_FLOOR = 100

SCAN = [(BASE3, 2187), (BASE10, 400), ((0, None, "full set, control"), 400)]

ID_CASES = [(BASE3, 81, 200000), (BASE3, 243, 200000), (BASE10, 40, 200000), ((0, None, "full set, control"), 40, 200000)]


def verb_identity():
    print("\nTHE DIGIT-RESTRICTED FRANEL IDENTITY, GCD-KERNEL SIDE AGAINST THE FOURIER SIDE")
    print("   set                        |    Q |    m |      G_F(Q) | (pi^2/3) G_F |  truncated |      gap |  tail bound")
    for design, q, cut in ID_CASES:
        keep = keep_of(design, q)
        mu = mobius_sieve(q)
        g, _ = kernel_sum(keep, mu, q)
        ident = float(g) * pi * pi / 3.0
        lhs, m = fourier_side(keep, q, cut)
        tail = 2.0 * m * m / cut
        print(
            f"   {design[2]:26s} | {q:4d} | {m:4d} | {float(g):11.6f} | {ident:12.6f} | "
            f"{lhs:10.6f} | {abs(ident - lhs):.2e} | {tail:.4f}"
        )

    print("\n   THE RANK FORM, G_F(Q) - 1 = 12 m sum delta^2, exact rationals")
    print("   set                        |    Q |    m |   sum delta^2 |  G_F - 1 == 12 m sum delta^2")
    for design, q in [(BASE3, 81), (BASE3, 243), (BASE10, 40), ((0, None, "full set, control"), 40)]:
        keep = keep_of(design, q)
        mu = mobius_sieve(q)
        g, _ = kernel_sum(keep, mu, q)
        m, s2 = farey_delta_square(keep, q)
        print(f"   {design[2]:26s} | {q:4d} | {m:4d} | {float(s2):13.10f} | {g - 1 == 12 * m * s2}")

    print("\n   the backward inequality 2 M_F(Q)^2 <= (pi^2/3) G_F(Q), sampled")
    print("   set                        |      Q |  M_F(Q) |   (pi^2/3) G_F |  ratio")
    for design, ladder in [(BASE3, [243, 2187]), (BASE10, [100, 1000]), ((0, None, "full set, control"), [100, 1000])]:
        for q in ladder:
            keep = keep_of(design, q)
            mu = mobius_sieve(q)
            g, _ = kernel_sum(keep, mu, q)
            mf = mertens_design(keep, mu, q)
            ident = float(g) * pi * pi / 3.0
            print(f"   {design[2]:26s} | {q:6d} | {mf:7d} | {ident:14.6f} | {2.0 * mf * mf / ident:.6f}")

    print("\n   the same inequality asserted at EVERY integer Q up to the bound")
    print("   both sides step only at Q in S_F, so scanning S_F covers every integer Q")
    print("   set                        |   Qmax | jumps | violations |  max ratio |  at Q |  max over Q >= 100 |  at Q")
    for design, qmax in SCAN:
        seen, bad, best, at, tail, tail_at = scan_backward(design, qmax)
        assert bad == 0
        print(
            f"   {design[2]:26s} | {qmax:6d} | {seen:5d} | {bad:10d} | {best:10.6f} | {at:5d} | "
            f"{tail:18.6f} | {tail_at:5d}"
        )


STRICT_LADDER = [(BASE3, [243, 2187, 19683, 177147]), (BASE10, [100, 1000, 10000])]


def verb_strict():
    print("\nTHE STRICT SET, THE DIVISOR-SUM IDENTITY")
    print("   set                        |    Q | denominators |  max abs diff")
    for design, q in [(BASE3, 729), (BASE10, 200)]:
        keep = keep_of(design, q)
        mu = mobius_sieve(q)
        worst = 0.0
        n = 0
        for b in range(1, q + 1):
            if not keep[b]:
                continue
            n += 1
            worst = max(worst, abs(strict_ramanujan_literal(keep, b) - strict_ramanujan_divisor(keep, mu, b)))
        print(f"   {design[2]:26s} | {q:4d} | {n:12d} | {worst:.3e}")

    print("\nTHE STRICT SET, THE FIRST Q WITH A NONZERO IMAGINARY PART")
    for design, q in [(BASE3, 100), (BASE10, 100)]:
        keep = keep_of(design, q)
        run = 0j
        hit = None
        for b in range(1, q + 1):
            if not keep[b]:
                continue
            run += strict_ramanujan_literal(keep, b)
            if hit is None and abs(run.imag) > 1e-9:
                hit = (b, run)
        b, run = hit
        print(f"   {design[2]:26s}: Q = {b}, sum = {run.real:.9f} + {run.imag:.9f} i")

    print("\nTHE STRICT SET, FREQUENCY 1 AGAINST EVERY MERTENS-TYPE CANDIDATE")
    print(
        "   set                        |      Q |     card |     Re T |     Im T |     abs T | abs T/card |"
        "  M_F(Q) |  sum mu phi_F |  sum mu phi_F/phi"
    )
    for design, ladder in STRICT_LADDER:
        for q in ladder:
            keep = keep_of(design, q)
            mu = mobius_sieve(q)
            phi = totients(q)
            t, card = strict_literal(keep, q)
            plain, counted, ratio = strict_weights(keep, mu, phi, q)
            print(
                f"   {design[2]:26s} | {q:6d} | {card:8d} | {t.real:8.3f} | {t.imag:8.3f} | "
                f"{abs(t):9.3f} | {abs(t) / card:10.6f} | {plain:7d} | {counted:13d} | {ratio:17.6f}"
            )


def main():
    verbs = {"denominator": verb_denominator, "identity": verb_identity, "strict": verb_strict}
    want = sys.argv[1:] or list(verbs)
    t0 = time.time()
    for v in want:
        verbs[v]()
    print(f"\ntotal runtime {time.time() - t0:.1f} s")


if __name__ == "__main__":
    main()
