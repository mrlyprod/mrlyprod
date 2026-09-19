---
title: Prime numbers
lead: A prime is a number whose stones make only one rectangle; every other number is built from primes, and the supply never runs out.
prerequisites:
---

A whole number is prime when exactly two numbers divide it, itself and 1. Lay `n` stones on a table and try to arrange them in a rectangle. Twelve stones make 2 by 6 and 3 by 4, so 12 is not prime. Thirteen stones make only the single row 1 by 13, so 13 is prime.

The number 1 is left out on purpose. It has one divisor, not two, and admitting it would break the one rule that makes primes worth having.

That rule is that every number above 1 is a product of primes in exactly one way, apart from the order you write the factors in. 60 is 2 times 2 times 3 times 5 and nothing else will do. The primes are the parts, and every other number is an assembly of them.

The figure is the oldest way of finding them, the sieve of Eratosthenes, run on the first hundred numbers laid out as ten rows of ten, 1 at the top left and 100 at the bottom right. Keep 2 and strike out every later multiple of 2. Move to the next number still standing, 3, keep it, strike out its multiples. Then 5, then 7. Whatever is still standing at the end is prime.

Every struck cell in the figure is tinted by the first prime that struck it, so the colours read as a history of the sieve. The grey cells are the even numbers, taken by 2. The next tint is the odd multiples of 3, the next the multiples of 5 that survived 2 and 3, and the last is the three cells 49, 77 and 91, which no prime below 7 could reach. The 25 cells still lit are the primes below 100.

The sieve finishes after 7 because 11 times 11 is 121, already off the board. Any composite number up to 100 must have a factor at or below 10, so four primes clear the whole hundred. To sieve up to a million you would need only the primes up to a thousand.

Primes thin out as you go. There are 25 below 100, 168 below 1000 and 1229 below 10000, so the share falls from about one in four to one in six to one in eight. The honest summary is that a number near `n` is prime about one time in `ln n`, which fades slowly and never reaches zero.

The gaps between them can be made as long as you like. Take the product of all the numbers from 1 to 100, call it `P`, and look at `P + 2, P + 3` up to `P + 100`. Each one is divisible by the number you added, so that is a run of 99 numbers in a row with no prime in it, and the same trick gives a run of any length.

Even so, the primes never stop. Suppose you had a complete list of them. Multiply the whole list together and add 1. The new number leaves remainder 1 when divided by any prime on the list, so none of them divides it, so either it is prime itself or it has a prime factor the list missed. The list was not complete after all. Euclid wrote that down and nobody has needed to improve on it.

Those two facts sit together and neither one softens the other. Primes get sparse, they leave arbitrarily long empty stretches, and they still go on forever.

## In the tree

The [primes demo](/demos/primes/) runs this sieve on a larger board and reads the same numbers three more ways: as stones in a rectangle, as [a running count](/wiki/prime-counting-function/) against `x / ln x`, and as a stack of grids whose layers fall out of step everywhere except at the primes. The [Ulam spiral demo](/demos/ulam/) winds the whole numbers outward from the middle with the primes lit, where every straight line reads a quadratic and some of those lines are strangely prime-rich. How much a new scale adds to the [Farey sequence](/wiki/farey-sequence/) is a test of primality, which the [Farey stack note](/research/farey/) reads straight off the picture, and [the coprimality spine](/research/coprime/) asks the harder version of the question, which numbers of a design are prime at all.
