---
title: The Ulam spiral
lead: Wind the whole numbers outwards on a square spiral, light up the primes, and they fall on diagonal streaks instead of scattering evenly.
prerequisites: prime-numbers
---

Put 1 on a square grid. Put 2 to its right, 3 above that, then 4 and 5 to the left, then 6 and 7 below, then 8, 9 and 10 to the right again, and carry on turning left whenever the cell ahead of you is already taken. The numbers wind outwards in a square spiral that fills the whole grid, one number to a cell, with no gaps and no choices to make.

Now light up every cell whose number is prime and leave the rest dark. That is all the construction there is. It was done on a scrap of paper by a bored mathematician during a talk, and the surprise is that the lit cells are not evenly scattered.

The figure is the spiral out to 101 cells on a side, so it holds every number from 1 to 10201, with the 1252 primes among them lit in blue. The eye finds diagonal streaks at once, some long and dense, others thin, with dark stretches between them. Nothing has been chosen or fitted. The only inputs are the winding and the primes.

The streaks are real, and they have an ordinary explanation. Walk along any straight diagonal of the spiral and write down the numbers you pass. They do not go up by a fixed step. They go up by a step that itself grows by a fixed amount each time, and a sequence like that is exactly what you get from a quadratic, a formula of the shape `4 k^2 + b k + c`. Every straight line in the picture is one such formula, and the picture is a hundred of them side by side.

Some quadratics produce far more primes than others, and the reason is small divisors. A formula whose values are always even can only ever hit the prime 2, so its line is dark. A formula whose values avoid being multiples of 2, 3 and 5 has fewer ways to be composite than a random number of the same size, so it hits primes more often than its neighbours do, and its line is bright. The bright diagonals are the formulas that dodge the small primes, and the dark ones are the formulas that cannot.

The most famous of these is Euler's `k^2 + k + 41`, which is prime for every `k` from 0 to 39, forty values in a row with no exception. It fails at `k = 40`, where the value is `41 times 41`, and it must fail somewhere for the simple reason that at `k = 41` every term has a factor of 41. Still, over a long range it carries a startling share of primes, and on a spiral centred at the right number it draws one unbroken bright line.

Alternate diagonals of the spiral are even numbers, so half of the diagonal directions are dark by construction, and that alone makes the remaining ones stand out. But the effect is stronger than that: among the odd diagonals some are plainly richer than others, and that difference is the one the small divisors explain.

None of this says the primes are orderly. Change the centre, change the shape, wind the numbers on a hexagonal grid instead of a square one, and different lines light up, because the quadratic behind each line has changed. What the picture shows is one honest fact seen very clearly: primes are not uniform across the quadratics, and some polynomials are far better prime factories than others. Why the best ones are as good as they are is still open.

## In the tree

[The Ulam spiral demo](/demos/ulam/) winds the numbers on squares or on hexagons with the primes lit, and lets you pick out a single line and read the quadratic behind it. [The primes demo](/demos/primes/) reads the same numbers the other way, as a sieve and as a count, and [the snail demo](/demos/snail/) gives every cell of the winding a design tile whose side is a power of the base, so the spiral widens by that factor at each new digit. The lit cells are the subject of [prime numbers](/wiki/prime-numbers/), and how many of them there are up to a given point is [the prime counting function](/wiki/prime-counting-function/).
