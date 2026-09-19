---
title: The Leibniz series
lead: Add one, take away a third, add a fifth, and the running total swings over and under a quarter of pi, closing on it very slowly.
prerequisites:
---

The figure is thirty dots, one for each running total. The blue dots sit above the dashed line and the orange dots below it, and the thin thread joining them in order shows the swing. The dashed line is a quarter of pi.

The rule is short. Start at 1, take away 1/3, add 1/5, take away 1/7, add 1/9, and carry on: the denominators are the odd numbers in order and the signs alternate.

Every term is smaller than the term before it and points the other way, so each step carries the total across the line rather than up to it. The odd steps land above the limit and the even steps land below it, which is the alternation the figure draws.

That overshooting is useful. The limit is trapped between any two neighbouring totals, so a total is never wrong by more than the next term, `1/(2n + 1)`. After thirty terms the total is 0.7771 and the error is under 1/61, about 0.016, against a quarter of pi at 0.7853982.

It is the slowest of the classical closers. Halving the error means doubling the number of terms, so a millionth of accuracy asks for about half a million terms. Nobody computes pi this way; the series is here because it is so plain.

There is a free trick in the trapping. Since the limit lies between two neighbouring totals, their midpoint is a much better guess than either: the twenty-ninth and thirtieth totals average to 0.78554, off by 0.00014, about sixty times closer than the thirtieth total on its own.

The series comes from the rule that turns a tangent back into an angle, read at tangent 1. The angle whose tangent is 1 is 45 degrees, and 45 degrees measured in the way that makes a half turn equal pi is `pi/4`, which is why a sum of odd reciprocals knows about a circle at all.

[The Wallis product](/wiki/wallis-product/) is the same kind of statement in the same family: correct, convergent and slow. The difference is in the approach. The product climbs from below and never crosses its limit, while this series crosses at every single step.

## In the tree

[The pi note](/research/pi/) counts pi a different way, out of the density of the visible points of a grid, and puts that count in the same family as this series and the product: correct, convergent, not fast. The neighbouring pages here are [the Wallis product](/wiki/wallis-product/) and [the Basel problem](/wiki/basel-problem/), and [the famous formulas](/wiki/famous-formulas/) put all of them on one board of speeds.
