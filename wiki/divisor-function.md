---
title: The divisor function
lead: d(n) counts the ways to split n into a product, 2 at every prime and 12 at 60; it never grows as fast as any power of n, and on average it is log n.
prerequisites: prime-numbers
---

Write down every number that divides `n`. For 12 the list is 1, 2, 3, 4, 6, 12, six of them, so `d(12) = 6`. For a [prime](/wiki/prime-numbers/) it is 1 and the prime itself, so `d(p) = 2`, the smallest value any `n > 1` can have. The count is the divisor function `d(n)`. Its sibling `sigma(n)` adds the divisors instead of counting them: `sigma(12) = 28`.

The figure is another way to count. A pair `(a, b)` with `ab = n` is a divisor `a` of `n` with its partner `b`, so `d(n)` is the number of lattice points on the hyperbola `ab = n`. The dots are every lattice point on or under the hyperbola `ab = 36`, 140 of them, and 140 is `d(1) + d(2) + ... + d(36)`. Nearly all of them hug the axes: a product lands under the curve only when one factor is small.

Both read off the prime factorisation. A prime power `p^a` has divisors `1, p, ..., p^a`, so `d(p^a) = a + 1` and `sigma(p^a) = (p^(a+1) - 1) / (p - 1)`. Two numbers with no common factor multiply cleanly, `d(m n) = d(m) d(n)` and the same for `sigma`, since a divisor of `m n` is a divisor of `m` times a divisor of `n`. So for `n = p_1^a_1 ... p_r^a_r`,

$$d(n) = (a_1 + 1)(a_2 + 1) \cdots (a_r + 1).$$

At `60 = 2^2 3 5` that is `3 * 2 * 2 = 12`.

The function is jumpy: 2 at every prime, 12 at 60, 24 at 360, 64 at 7560, records set by numbers built from many small primes. The divisor bound says the jumps never reach a power: `d(n) <= C n^eps` for every `eps > 0`, and more sharply `d(n) <= n^(O(1 / log log n))`, as a post of Tao states. Wigert found the exact exponent: the `lim sup` of `log d(n) log log n / log n` is `log 2`, so `d(n)` climbs no faster than `n^((log 2 + o(1)) / log log n)` and infinitely often as fast.

On average it is tame. Dirichlet proved that

$$\sum_{n \le x} d(n) = x \log x + (2 \gamma - 1) x + O(\sqrt{x}),$$

with `gamma` the [Euler-Mascheroni constant](/wiki/euler-mascheroni-constant/), so the average of `d(n)` over `n <= x` is about `log x`. The proof is the figure. Column `a` holds `floor(x / a)` dots; the points with `a <= sqrt x` are one arm plus the square, those with `b <= sqrt x` the other arm plus the same square, so the total is `2 (floor(x / 1) + ... + floor(x / sqrt x)) - floor(sqrt x)^2`, which at `x = 36` is `2 (36 + 18 + 12 + 9 + 7 + 6) - 36 = 140`. The reciprocals `1 + 1/2 + ... + 1 / sqrt x` add to `log sqrt x + gamma + O(1 / sqrt x)` and each floor loses less than 1, so the main terms fall out. This is the hyperbola method; at `x = 36` it gives `134.6` against 140.

The true size of the error is Dirichlet's divisor problem, still open: Huxley proved `O(x^(131/416 + eps))`, `131/416 = 0.3149...`, Hardy showed the exponent cannot go below `1/4`, and `1/4` is the conjectured truth. The sources are the Wikipedia articles on the divisor function and the divisor summatory function; Huxley's paper is in Proceedings of the London Mathematical Society 87.

## In the tree

[The integers note](/research/integers/) reads the cell count of the sponge rule at odd side `2n + 1` as `d(x^n)`, the divisor count of a power of one fixed `x`.
