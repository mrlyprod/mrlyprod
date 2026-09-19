---
title: The prime counting function
lead: How many primes are there below a number? The answer is a staircase with one riser per prime, and two smooth curves that chase it without ever quite catching it.
prerequisites: prime-numbers
---

Write `pi(n)` for the count of primes at or below n. It is the plainest question you can ask about the primes and it has no formula. Up to ten there are four primes, 2, 3, 5 and 7, so `pi(10)` is 4. Up to a hundred there are 25. Up to two hundred there are 46. Up to a thousand there are 168.

The figure draws that count from 2 up to 200. The yellow line is a staircase: it is flat wherever a number is composite and it jumps by exactly one at a prime. Each faint vertical hairline stands on a prime, so the hairlines are the risers. There are 46 of them, and you can watch them crowd the left of the picture and thin out to the right.

Thinning is the whole shape. The first hundred numbers hold 25 primes, the second hundred hold 21, and by a thousand only 168 numbers out of the thousand are prime, about one in six. The primes never stop, but they get rarer, and they get rarer in a way that is smooth enough to guess at.

The first guess is n divided by `ln n`, the blue curve in the figure. The natural logarithm `ln n` is the number of times you have to multiply by 2.71828 to reach n, and the guess says that a number near n has roughly a one in `ln n` chance of being prime. At 200 the guess reads 37.7 against the true 46. At 1000 it reads 144.8 against 168. It is always low, and it is low by a widening amount, yet the ratio of the true count to the guess creeps towards 1.

The second guess is better and is built the same way. If the chance of being prime near x is one in `ln x`, then add that chance up over every x from the start to n, as a curve adds things up. The result is the logarithmic integral, written `li(n)`, and it is the area under the curve 1 over `ln x`. At 200 it reads 50.2 against 46. At 1000 it reads 177.6 against 168.

Compare the two guesses honestly. At 1000, n over `ln n` is short by 23 and `li` is over by 10, so the better curve is about twice as close, and the gap it leaves shrinks faster. The primes demo plots both against the staircase, and the picture there is the same picture: two curves, one below and one above, closing in slow motion.

That both ratios march to 1 is the prime number theorem, the central fact about how the primes thin out. It says the guesses are eventually right in proportion. It says nothing at all about the gap: the difference between the staircase and either curve can be, and is, large and jumpy forever.

One thing looks true in the figure and is not. Here and in every table anyone has printed, `li(n)` sits above the count. It is known that this cannot last, that the count overtakes the curve somewhere and then keeps swapping sides forever, and no one has produced a single number where it happens. It is the standard warning about reading a law off a table.

The staircase is also the slowest of the eight systems on the formulas page to pay. Five of them close on a constant at a steady, readable rate. The prime count against `li` closes too, but its gap is ragged at every scale, which is the visible face of the fact that nobody can say how ragged it is allowed to get.

## In the tree

[The primes demo](/demos/primes/) runs the sieve, splits numbers into rectangles, and draws this staircase against both guesses. [The Ulam spiral demo](/demos/ulam/) winds the whole numbers outward with the primes lit, so the same thinning reads as a texture. [The pi note](/research/pi/) counts pi out of the lattice rather than the primes, and this count is one of the eight on [the famous formulas hub](/wiki/famous-formulas/). What is being counted is [prime numbers](/wiki/prime-numbers/).
