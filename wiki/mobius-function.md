---
title: The Mobius function
lead: Mu of n is 0 when a square divides n, and otherwise plus or minus one depending on whether n has an even or an odd number of prime factors.
prerequisites: prime-numbers
---

The Mobius function takes only three values and the rule for choosing between them is short. Factor `n` into primes. If any prime appears twice, `mu(n) = 0`. Otherwise count the distinct primes: an even count gives `+1` and an odd count gives `-1`.

Work a few. `mu(1) = 1`, since 1 has no prime factors at all and zero is an even count. `mu(2) = -1` and `mu(3) = -1`, one prime each. `mu(6) = 1`, two primes. `mu(30) = -1`, three primes. `mu(4) = 0` and `mu(12) = 0`, because 4 divides both.

Here are the first thirty, the row of values under the row of numbers.

```
n   1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
mu  1 -1 -1  0 -1  1 -1  0  0  1 -1  0 -1  1  1

n  16 17 18 19 20 21 22 23 24 25 26 27 28 29 30
mu  0 -1  0 -1  0  1  1 -1  0  0  1  0  0 -1 -1
```

The zeros are the numbers a square divides, and they are common: 4, 8, 9, 12, 16, 18, 20, 24, 25, 27 and 28 in the first thirty. The numbers that survive are called squarefree, and among those the signs alternate by how many primes they carry.

The figure lays out the first hundred as ten rows of ten. Yellow is `+1`, blue is `-1`, grey is `0`. There are 31 yellow, 30 blue and 39 grey. So about two fifths of the board is grey, and the signs on the rest are split almost evenly.

The reason for so odd-looking a definition is that `mu` is a subtraction machine. Add `mu(d)` over every divisor `d` of `n` and the total is 1 when `n = 1` and 0 for every other `n`. For `n = 6` the divisors are 1, 2, 3 and 6, the values are `1, -1, -1, 1`, and they cancel to 0.

That cancellation is what makes it useful. It is inclusion and exclusion written as a number: if you count something by the multiples of 2, then the multiples of 3, you have double-counted the multiples of 6, and the signs of `mu` are exactly the bookkeeping that puts each thing back once. Any count that is easy over multiples can be turned into the count you actually wanted by weighting with `mu`.

Now add the values up as you go, `mu(1) + mu(2)` and onwards. That running sum wanders. It is `-1` after 10 terms, `1` after 100 terms and `2` after 1000 terms, and in between it climbs and falls and crosses zero over and over, never settling and never running away.

How far it is allowed to wander is one of the famous open questions in mathematics. It is known that the sum keeps returning near zero rather than drifting, and it is known that the sum grows slower than `n` itself, but the exact size of its swings has never been pinned down.

## In the tree

The running sum of `mu` over a restricted set of numbers is the object [the Mobius meter note](/research/mobius/) measures, where the question is how big the swings get when you only allow numbers whose digits come from a fixed list. [The design zeta note](/research/zeta/) reads the same meter from the other side, and [the Mobius echo demo](/demos/echo/) listens to it: the swings turn out to carry frequencies borrowed from the zeta zeros. Weighting by `mu` instead of by 1 also changes what [the Farey stack](/research/farey/) draws, and it is `mu` that inverts the divisor count in [pi out of the stack](/research/pi/).
