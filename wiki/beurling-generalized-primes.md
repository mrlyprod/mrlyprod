---
title: Beurling generalised primes
lead: Pick any climbing list of real numbers and call them primes; their products are the integers. Beurling's theorem says how straight the integer count must run for the prime number theorem to hold, and the exponent is sharp.
prerequisites: prime-numbers, prime-counting-function
---

The ordinary [primes](/wiki/prime-numbers/) 2, 3, 5, 7, ... build every whole number as a product, one way each. Beurling turned that round in 1937. Take any list of real numbers `1 < p_1 <= p_2 <= p_3 <= ...` climbing to infinity and call them the primes. The integers of the system are then all the finite products `p_1^a_1 p_2^a_2 ...`, each counted as many times as it can be formed, with the empty product 1 among them. Nothing says the primes must be whole numbers, or distinct, or that a product must land on a whole number either.

Two counts describe the system: `pi_P(x)`, the number of its primes up to `x`, and `N_P(x)`, the number of its integers up to `x`. For the true primes these are [the prime counting function](/wiki/prime-counting-function/) and the plain count `N(x) = floor(x)`. The system also has a zeta function, `zeta_P(s) = prod (1 - p_i^(-s))^(-1)`, which multiplies out to the sum of `n^(-s)` over its integers exactly as [Riemann's](/wiki/riemann-zeta-function/) does.

The figure takes a toy system: the primes that leave remainder 1 on division by 4, that is 5, 13, 17, 29, 37, 41, ... Its integers are their products, 1, 5, 13, 17, 25, 29, 37, 41, 53, 61, 65, ... The upper panel is `N_P(x)` in yellow against `N(x) = x` in blue, up to `x = 100000`. The toy integers thin out: 9623 of them lie below a hundred thousand, about a tenth of the whole numbers there, and the fraction keeps sinking as `x` grows. The lower panel is `pi_P(x)` in yellow against `pi(x)` in blue: 4783 against 9592, half, since the primes split evenly between remainder 1 and remainder 3.

Beurling's theorem is the reason to build such systems at all. If the integer count runs close to a straight line,

$$N_P(x) = A x + O\left(\frac{x}{(\log x)^{\gamma}}\right)$$

for some `A > 0` and some `gamma > 3/2`, then the primes of the system obey the prime number theorem: `pi_P(x) ~ x / log x`. Whatever real numbers the primes are, a straight enough integer count forces the primes to be as common as the true ones.

The toy in the figure fails the hypothesis, since its integer count is far from any line, and its primes come in at half the rate the theorem would give. Drop the single prime 2 instead and everything holds: the integers are the odd numbers, `N_P(x) = x / 2 + O(1)`, the hypothesis is met with room to spare, and the primes are the true primes less one, so `pi_P(x) ~ x / log x` as promised.

The exponent `3/2` is not an artefact of the proof. Diamond built in 1970 a system whose integer count satisfies the hypothesis at `gamma = 3/2` exactly and whose primes still disobey the prime number theorem, so the theorem is sharp. The statements here follow the Wikipedia article on the Beurling zeta function; Beurling's paper is in Acta Mathematica 68 and Diamond's, whose title says that Beurling's theorem is sharp, is in the Illinois Journal of Mathematics 14.

## In the tree

[The zeta note](/research/zeta/) builds a Beurling system on the primes that lie inside a digit design and counts its integers.
