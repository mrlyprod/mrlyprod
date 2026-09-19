---
title: Fractal dimension
lead: Cover a shape with boxes, shrink the boxes, and watch how fast the count grows; the growth rate is the dimension, and for a carpet it is not a whole number.
prerequisites: sierpinski-carpet
---

Dimension is usually a word, one for a line, two for a square, three for a cube. It can also be a measurement, and the measurement is counting.

Take a line segment and cut it in half. It takes 2 half-length copies of itself to rebuild it. Take a square and halve both sides: it takes 4 of the small squares. Take a cube and halve all three sides: 8 of the small cubes. The counts are `2^1`, `2^2` and `2^3`, and the exponent is the familiar dimension.

Nothing depends on halving. Cut everything into thirds instead and the counts are 3, 9 and 27, which are `3^1`, `3^2` and `3^3`. The exponent is the same each time, so the rule is general: if shrinking by a factor `s` takes `N` copies, the dimension is `log N / log s`.

The figure counts boxes three times over. Each panel lays the same three by three lattice over a shape and lights the boxes the shape actually touches. The filled square lights all 9. The carpet lights 8, because the middle box is empty at every level. The diagonal line lights 3, the boxes it runs through corner to corner.

Read the three counts back through the rule. Nine boxes at scale one third gives `log 9 / log 3 = 2`, which is the square. Three boxes gives `log 3 / log 3 = 1`, which is the line. Eight boxes gives `log 8 / log 3`, and that is 1.892789, which is no whole number at all.

The carpet's count is not an accident of the scale chosen. Shrink the boxes by 3 and 8 are needed, by 9 and 64 are needed, by `3^n` and `8^n` are needed, for ever, because each round of the construction replaces one filled cell by eight. The ratio never settles on a whole number, so the answer stays `log 8 / log 3`.

That the answer lies between 1 and 2 matches what the shape looks like. The carpet has no area, so it cannot be two-dimensional, and it is far too tangled to be drawn as a curve, so it is not one-dimensional either. A number between the two is the honest report.

The Menger sponge is the same sum with different counts. Shrink by 3 and 20 of the 27 small cubes are needed, so its dimension is `log 20 / log 3`, about 2.73. It has no volume, so it is not solid, and it has far too much surface to be a sheet, so a number between 2 and 3 is again the answer.

Write it once for everything the tree builds. If a rule keeps `fill` cells out of a block `base` cells wide on each axis, then at level `n` there are `fill^n` cells of side `base^-n`, so the count of boxes at scale `base^-n` is `fill^n` and the dimension is

$$\frac{\log(\mathrm{fill})}{\log(\mathrm{base})}.$$

The carpet is `fill` 8 at `base` 3, the sponge is `fill` 20 at `base` 3, and a solid square is `fill` 9 at `base` 3, which returns the ordinary answer of 2. Box counting never contradicts the usual dimensions; it just keeps working where the usual ones run out.

## In the tree

The dimension is not a parameter anywhere in the tree, it is read off the fill count, and [the core](/research/core/) derives it from the one line that says fills multiply with the level. [Complex dimensions](/research/dimensions/) promotes that single number to the real part of a whole family of them, which is what makes the box count wobble rather than settle. [The walk dimension](/research/walks/) measures a second and different one, how far a random walker gets on the same shape, and pairs the two. [The sponge demo](/demos/sponge/) prints the dimension of any cube rule beside its fills and voids.
