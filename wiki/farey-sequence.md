---
title: The Farey sequence
lead: Every fraction between zero and one whose bottom number is at most Q, written in order; each new Q slips a few new fractions between the old ones and never moves one.
prerequisites:
---

Pick a whole number Q and write down every fraction between 0 and 1 whose denominator is Q or less, each in lowest terms, in increasing order. That list is the Farey sequence of order Q. For Q equal to 5 it reads 0/1, 1/5, 1/4, 1/3, 2/5, 1/2, 3/5, 2/3, 3/4, 4/5, 1/1.

The figure grows the sequence one order at a time, a row per Q. The dots already on a row stay put on every row below it, and each new row only adds dots, in yellow, between the old ones. Order 1 holds 0/1 and 1/1. Order 2 adds 1/2, order 3 adds 1/3 and 2/3, order 4 adds 1/4 and 3/4, and 2/4 is not new because it is 1/2 already.

How many does each order add? A fraction a/Q is new exactly when a and Q share no divisor bigger than 1, so the count of new fractions at order Q is the count of numbers from 1 to Q that share nothing with Q. For a prime Q that is every number below it, Q minus 1, which is why the prime rows in the figure carry the most yellow.

Two neighbours in the list are always as close as fractions of their size can be. Take 2/5 and 1/2, side by side at order 5: the cross products are 2 times 2 and 5 times 1, and they differ by exactly 1. That holds for every pair of neighbours at every order, and it is the rule that decides what comes between them.

Between two neighbours a/b and c/d the first fraction to appear, at some later order, is the mediant, the fraction (a + c)/(b + d) made by adding tops and adding bottoms. Between 1/3 and 1/2 the mediant is 2/5, and it arrives at order 5, exactly the row where the figure shows it. The whole sequence can be built by mediants alone, starting from 0/1 and 1/1.

![The Farey stack: a line at every k/n for every scale n up to Q, laid over each other, so a reduced fraction lights up once for each scale that draws it.](demos/farey/stack)

The stack above draws the same fractions another way. Lay a ruler with n equal divisions on the unit line for every n up to Q, and count how many rulers put a mark at each point: a fraction a/b is marked by every n that b divides, so the tallest bars are the simplest fractions and [the primes](/wiki/prime-numbers/) stand out as the scales that mark the most new points.

## In the tree

The grid of every design at every scale, laid over itself, lights up at the Farey fractions, and the [Farey stack note](/research/farey/) reads that [moire](/wiki/moire/) as a diagram of the fractions and of how evenly they spread. The circles standing on the same fractions are in [the Apollonian demo](/demos/apollonian/), and counting the lit points of the grid is how [pi comes out of the grid](/research/pi/).
