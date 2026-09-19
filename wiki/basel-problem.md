---
title: The Basel problem
lead: Add up the reciprocals of the squares and the total stops at pi squared over six, a circle appearing in a question that never mentioned one.
prerequisites:
---

The figure has two panels. The top one lays twelve squares in a row, of sides 1, 1/2, 1/3 and on down to 1/12, standing on one line, so the area of each square is one term of the sum. The bottom one draws the running total after each of the first forty terms as a bar, against the dashed line the totals are climbing towards.

The question is as plain as a question gets: what is `1 + 1/4 + 1/9 + 1/16 + 1/25` and so on forever, the sum of one over each square? It was posed in Basel, it stood unanswered for decades while good mathematicians chipped at it, and Euler settled it.

That the total is finite is easy. For `k` above 1 the term `1/k^2` is smaller than `1/((k - 1)k)`, which is the difference `1/(k - 1) - 1/k`. Those differences cancel in pairs and add up to 1, so the whole sum is under 2. Knowing it is finite is one thing; knowing the number is another.

The approach is slow. What remains after `n` terms is close to `1/n`, so forty terms give 1.6202 and are still 0.0247 short, and a thousand terms are still about 0.001 short. The bars in the lower panel flatten early and then creep.

The answer is `pi^2/6`, which is 1.6449340668. A sum of the reciprocals of the squares, built from nothing but whole numbers, turns out to be the square of pi over six.

The top panel is the same statement as an area. Each term is the area of a square of side `1/k`: a unit square, then a square of a quarter its area, then a ninth, then a sixteenth. Lay every one of them down and the paint you need is `pi^2/6` unit squares, a little under five thirds.

Euler's first argument was a raid. A polynomial can be rebuilt from the places where it is zero, and he treated the sine wave as an endless polynomial whose zeros sit at every whole multiple of pi. Matching one coefficient on each side gave the sum at once. The step was daring rather than sound, proper proofs came later, and the answer was right.

The number has a second life as a probability. Turn `pi^2/6` upside down and you get `6/pi^2`, about 0.6079, which is the chance that two whole numbers picked at random share no factor above 1. So the same constant that measures a pile of squares also measures how often a fraction is already in lowest terms.

## In the tree

That reading is how [the pi note](/research/pi/) gets pi out of a grid: lay the whole numbers out as points, count the ones visible from the corner, and the share of them is `6/pi^2`, so counting points hands pi back. The fractions those [visible points](/wiki/visible-lattice-points/) stand on are [the Farey sequence](/wiki/farey-sequence/), and [the famous formulas](/wiki/famous-formulas/) measure how slowly this sum pays beside seven other rules.
