---
title: Euler's totient
lead: Phi of n counts how many of the numbers 1 to n share nothing with n; it sinks for numbers with many small factors and hits its ceiling at every prime.
prerequisites: greatest-common-divisor
---

Write out the numbers from 1 to `n` and keep only the ones coprime to `n`, the ones whose [greatest common divisor](/wiki/greatest-common-divisor/) with `n` is 1. The count of survivors is Euler's totient, written `phi(n)`.

Try `n = 12`. Strike the even numbers and the multiples of 3, and 1, 5, 7 and 11 are left. So `phi(12) = 4`. Try `n = 10` and 1, 3, 7 and 9 survive, so `phi(10) = 4` as well.

The value at 1 is 1, since the only number in the list is 1 itself and 1 shares nothing with anything. It is a convention that pays for itself everywhere else.

A [prime](/wiki/prime-numbers/) is the easy case. If `p` is prime, nothing below it shares a factor with it, so every one of `1` to `p - 1` survives and `phi(p) = p - 1`. That is the ceiling: no `n` above 1 can do better, because `n` itself always fails the test.

A prime power is nearly as easy. To count what survives `9`, strike only the multiples of 3, which are 3, 6 and 9, leaving 6. In general `phi(p^k) = p^k - p^(k-1)`, one in every `p` struck out and the rest kept. So `phi(8) = 8 - 4 = 4` and `phi(27) = 27 - 9 = 18`.

Two coprime numbers multiply cleanly. If `a` and `b` share nothing then `phi(a b) = phi(a) phi(b)`. Check it on 12, which is 4 times 3: `phi(4) = 2` and `phi(3) = 2`, and 2 times 2 is 4, which is what the strike-out above gave. Check it on 15, which is 3 times 5: `phi(3) = 2` and `phi(5) = 4`, and 2 times 4 is 8.

The coprime condition is not decoration. 8 is 4 times 2, but `phi(4) phi(2) = 2 times 1 = 2` while `phi(8) = 4`. The rule fails because 4 and 2 overlap, and the prime power rule is what you use instead.

Those two rules together compute anything. Split `n` into prime powers, take the totient of each, multiply. For 60, which is 4 times 3 times 5, that is 2 times 2 times 4, so `phi(60) = 16`. The same sum written as one formula is `n` multiplied by `(1 - 1/p)` for each distinct prime `p` dividing `n`.

The figure draws `phi(n)` as a bar for every `n` from 1 to 60, with a hairline running along the ceiling `n - 1`. The yellow bars are the ones that touch the hairline. They touch it exactly at the primes, since `phi(n) = n - 1` says every smaller number is coprime to `n`, which says `n` has no factor to share.

The blue bars fall away from the line in a pattern you can read. The deepest dips are the numbers made of many small primes: 30 and 60 each keep only a little over a quarter of their range, because they lose half to 2, then a third of what is left to 3, then a fifth of that to 5. Numbers that are a prime times a prime sit close under the line, and the whole picture fans out into bands rather than scattering.

## In the tree

The totient is the counter behind the [Farey sequence](/wiki/farey-sequence/): the fractions a new denominator `n` adds are exactly the `a/n` in lowest terms, so the row grows by `phi(n)` and the prime rows grow the most. [The Farey stack note](/research/farey/) turns that into a reading of primality off the stack, and [pi out of the stack](/research/pi/) counts the coprime points of the whole grid as a running sum of totients before inverting the density into pi.
