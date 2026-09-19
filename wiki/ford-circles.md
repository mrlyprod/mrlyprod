---
title: Ford circles
lead: Above every fraction sits a circle resting on the number line, the simpler the fraction the bigger the circle, and no two of them ever overlap.
prerequisites: farey-sequence
---

Draw a horizontal line and mark the fractions on it. Over the fraction `a/b`, written in lowest terms, place a circle that touches the line at that exact point and has radius `1/(2 b^2)`. That is the Ford circle of `a/b`, and there is one for every fraction, for ever.

The bottom number does all the work. Over `0/1` and over `1/1` the radius is `1/2`, so those circles are as tall as the gap between the two fractions is wide. Over `1/2` the radius is `1/8`, over `1/3` it is `1/18`, over `1/7` it is `1/98`. Each step up in the denominator shrinks the circle by the square, so the simple fractions are the big circles and the awkward ones are specks.

The figure is the unit interval with every circle of denominator 8 or less drawn as an outline, tinted from blue for the small denominators to yellow for the large, with a dot on the line at the point each one touches. The two halves at the far left and far right are the circles over 0 and 1. The big one in the middle stands over `1/2`, the two beside it over `1/3` and `2/3`, and so on down to the specks over the sevenths and eighths.

No two of these circles ever cross. Two of them touch, at a single point, exactly when their fractions are neighbours in the Farey sense, meaning that `a/b` and `c/d` satisfy `a d - b c` equal to plus or minus 1. Any other pair sits strictly apart.

That is a short calculation and worth seeing once. The horizontal gap between the two touching points is `(a d - b c)/(b d)`, and the vertical gap between the two centres is the difference of the radii. Square both, add them, and compare with the square of the sum of the radii. Everything cancels except one term, and the two agree exactly when `(a d - b c)^2` is 1. So the whole arrangement of touching is decided by that one cross product, which is precisely the rule that governs neighbours in [the Farey sequence](/wiki/farey-sequence/).

Two touching circles and the line between them leave a small curved triangle with three corners. The largest circle that fits in that gap is again a Ford circle, and it stands over the mediant of the two fractions, the one you get by adding the tops and adding the bottoms. Between `1/3` and `1/2` the gap is filled by the circle over `2/5`. So the picture builds itself: start with the circles over 0 and 1, fill every gap with a mediant, and you have written down every fraction and every circle.

That gap-filling move is the beginning of something bigger. Three circles that all touch each other leave two curved triangles, and each of them holds exactly one circle touching all three. Draw both, and now there are more triples, each with its own gaps. Keep going and the circles multiply without end, their areas eating up almost all the room, and what is left over is a dust called an Apollonian gasket. The Ford circles are one slice of one such packing, the slice that sits on a straight line, with the line itself playing the part of a circle of infinite radius.

The whole picture also repeats itself. Shift everything one unit to the right and it lands back on itself, because the circle over `a/b` goes to the circle over `(a + b)/b`, which has the same denominator and so the same radius. Look instead at what happens near a single fraction and you find the same arrangement again at a smaller scale, which is the geometric face of the fact that a fraction with a big denominator has little room around it.

## In the tree

[The Apollonian gasket note](/research/apollonian/) identifies the circles of one integer packing that rest on the line as exactly these, so the packing's shadow on the line is the Farey stack, and [the Farey stack note](/research/farey/) reads that same stack as a moire of rulers. [The Apollonian demo](/demos/apollonian/) grows the packing and lays the stack underneath it. The fractions the circles stand on are the ones [the Farey sequence](/wiki/farey-sequence/) lists, and lowest terms is [the greatest common divisor](/wiki/greatest-common-divisor/) doing its job.
