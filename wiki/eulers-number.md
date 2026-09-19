---
title: Euler's number
lead: Split a year's interest into more and more payments and the yearly growth climbs, slows and stops at 2.718281828.
prerequisites:
---

The figure is sixty points, one for each number of payments from one to sixty, and the height of a point is what a pound grows to in the year. The curve rises steeply and then flattens against the dashed line, which it never meets.

Start with one pound at an interest rate of 100 percent a year. Paid once at the end of the year, the pound becomes 2. Paid as half twice, the pound becomes 1.5 times 1.5, which is 2.25, because the second half year earns interest on the first half's interest as well.

Keep splitting. Four payments of a quarter give `1.25^4 = 2.4414`. Twelve monthly payments give 2.6130. Three hundred and sixty-five daily payments give 2.71457. With `n` payments the rule is `(1 + 1/n)^n`.

Two things pull against each other. Each extra split lets interest start earning sooner, which raises the total, but the pieces being split are smaller, so what the split adds is smaller too. The second effect wins in the end and the total settles.

Where it settles is `e`, 2.718281828459. At sixty payments the pound is at 2.6960, short by 0.0223. The shortfall is close to `e/(2n)`, so doubling the payments only halves it: the picture is another slow closer.

There is a fast recipe for the same number. Add the reciprocals of the factorials: `1 + 1 + 1/2 + 1/6 + 1/24 + 1/120` and on. Eleven of those terms pin `e` to seven decimal places, while a million payments a year pin it to five. The two recipes give the same number and cost wildly different amounts of work.

The reason `e` is everywhere is that it is what continuous growth costs. Anything that grows by the same proportion in equal stretches of time, money at interest, a population, a signal fading, is a power of `e` once the stretches are made small. Paid continuously for a year at 100 percent, a pound becomes exactly `e` pounds.

Like pi, `e` is irrational: its decimals never fall into a repeating block, so no fraction of whole numbers is equal to it, and the string 1828 appearing twice at the start is a coincidence and nothing more.

## In the tree

[The pi note](/research/pi/) keeps a list of elementary systems that close correctly and slowly, and `(1 + 1/n)^n` closing on `e` is one of them, beside [the Wallis product](/wiki/wallis-product/) and [the Leibniz series](/wiki/leibniz-series/) closing on pi; [the famous formulas](/wiki/famous-formulas/) draw the whole family's speeds on one board.
