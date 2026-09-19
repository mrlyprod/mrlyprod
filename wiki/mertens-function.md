---
title: The Mertens function
lead: Mark every whole number plus one, minus one or nothing, then keep a running total. How far that total is allowed to wander is the Riemann hypothesis.
prerequisites: mobius-function
---

Every whole number gets one of three marks. Factor it into primes. If any prime appears twice, the mark is 0. Otherwise count the distinct primes: an even count marks plus one, an odd count marks minus one. One has no prime factors at all, an even count of none, so it marks plus one. This mark is the Mobius value, written `mu(n)`.

Run through the first few. `mu(1)` is 1. `mu(2)` is minus one, one prime. `mu(3)` is minus one. `mu(4)` is 0, because two appears twice. `mu(5)` is minus one. `mu(6)` is plus one, two primes. `mu(12)` is 0 again. Roughly six numbers in ten escape a repeated prime and get a live mark, and the other four are dead.

The Mertens function is the running total of those marks: `M(n) = mu(1) + mu(2) + ... + mu(n)`. The first values are 1, 0, minus one, minus one, minus two, minus one. It is a walk. At each step you go up, down, or stand still, and where you are is the balance so far between the even-count numbers and the odd-count ones.

The figure walks it out to a thousand. The blue line is `M(n)`. The two faint curves are plus and minus the square root of n, opening outwards like a trumpet. Inside that trumpet the walk wanders: it climbs to 7 at n equal to 586, it falls to minus 12 at n equal to 665, and it finishes at 2. The trumpet at the right edge is 31.6 wide either way, so the walk is nowhere near touching it.

The square root is the right yardstick for a reason you can feel. If the marks were coin flips, plus one or minus one at random, then after n flips you would expect to be about the square root of n away from where you started. That is how random walks behave. So the picture asks one question: are the Mobius marks as well balanced as coin flips, or do they conspire and drift?

Nothing in the picture says they are balanced. The walk staying inside the trumpet up to a thousand is one sample of one range. The walk sits exactly at zero 92 times below a thousand, which looks like good behaviour, and that is exactly what an unproved pattern looks like from close up.

The precise statement is short and unreachable. If `M(n)` stays below any fixed multiple of `n^(1/2 + e)` for every small `e` you pick, then the Riemann hypothesis is true, and if the Riemann hypothesis is true then it does. The two are the same statement wearing different clothes. Nobody has either one.

There was a stronger guess, that `M(n)` stays strictly inside the trumpet, below the square root of n exactly, for every n. That guess is false. It has been proved that the walk leaves the trumpet somewhere, and no one has produced the place where it does. The proof gives a fact about the far distance and hands you no number to check, which is the reverse of the Goldbach situation and just as unsatisfying.

Why anyone cares about these marks: the Mobius value is what inverts counting. Sum `mu(d)` over all the divisors d of a number and the answer is zero for every number except one, where it is one. That single line is the sieve written as arithmetic, and it is why a sum of Mobius values is a measure of how much the primes cancel against each other.

## In the tree

[The Mobius page](/research/mobius/) measures exactly this cancellation, not on the whole numbers but on digit designs, with the classical Mertens function as its control. [The zeta page](/research/zeta/) holds the other face, the zeros, and says why no route runs between the two on a design. [The Farey stack note](/research/farey/) weights the stack by `mu` and turns the picture into a Mertens meter, and [the echo demo](/demos/echo/) reads the swings of that meter against the zeta zeros. The plain sum against the square root sits with seven other formulas on [the famous formulas hub](/wiki/famous-formulas/). The marks being summed are [the Mobius function](/wiki/mobius-function/), and the hypothesis the sum is equivalent to belongs to [the Riemann zeta function](/wiki/riemann-zeta-function/).
