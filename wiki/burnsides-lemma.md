---
title: Burnside's lemma
lead: Counting arrangements that turning and flipping should not tell apart, by averaging how many each symmetry leaves untouched.
prerequisites: parity
---

Colour each of the four cells of a two by two square either blue or grey. There are two choices per cell and four cells, so there are 16 arrangements. Now decide that two arrangements are the same thing if one can be turned or flipped into the other, the way a tile laid on a floor is the same tile whichever way round you set it down. How many genuinely different tiles are there?

Counting by hand is possible here and the answer is 6, but the honest problem is that dividing 16 by 8, the number of symmetries, gives 2, which is wrong. Dividing fails because some arrangements are left unchanged by some of the symmetries, so they are not counted eight times over. Burnside's lemma is the repair.

First, list the symmetries of the square. There are four turns: by nothing, by a quarter, by a half and by three quarters. There are four mirrors: across the vertical middle, across the horizontal middle, and across each of the two diagonals. Eight in all, and every way of picking the square up and putting it back down is one of them.

Now ask, for each symmetry, how many of the 16 arrangements it leaves looking exactly as they were. Doing nothing leaves all 16. A quarter turn sends each cell to the next one round, so an arrangement survives only if all four cells match, which allows 2. The half turn swaps the cells in two opposite pairs, so each pair must match and the two pairs are free, which allows 4. The three-quarter turn is like the quarter turn, 2. Each of the two middle mirrors swaps two pairs, so 4 each. Each of the two diagonal mirrors holds two cells still and swaps the other two, leaving three free choices, so 8 each.

Add those up: `16 + 2 + 4 + 2 + 4 + 4 + 8 + 8 = 48`. Divide by the eight symmetries and you get 6. That is Burnside's lemma: the number of genuinely different arrangements is the average, taken over the symmetries, of how many arrangements each one leaves untouched.

The figure lays out all 16 arrangements in six rows, one row per class, blue for a filled cell and grey for an empty one. The top row is the single all-grey tile. The second row is the four tiles with exactly one blue cell, which are all the same tile turned. The third row is the four with two blue cells side by side, an edge of the square. The fourth row is the two with the blue cells on a diagonal, and there are only two of them because a diagonal pair maps to itself under the half turn. The fifth row is the four with three blue cells, and the last row is the single all-blue tile. Count the tiles in the six rows: `1 + 4 + 4 + 2 + 4 + 1 = 16`, every arrangement once.

The fourth row is the reason plain division fails. Its two tiles have a genuine symmetry of their own, so the eight symmetries of the square only produce two distinct copies of each, not eight. The lemma handles that automatically, because an arrangement with extra symmetry shows up extra times in the fixed counts, which is exactly the compensation needed.

Why the averaging works is a counting trick worth seeing. Make a list of every pair consisting of a symmetry and an arrangement that symmetry leaves untouched. Counting the list one symmetry at a time gives the 48 above. Counting it one class at a time gives 8 for every class, always the group size, because a class of size `m` has `8/m` symmetries fixing each of its members. So the list has 8 entries per class, and 48 divided by 8 is the number of classes.

Nothing about the argument is special to a two by two square. Necklaces of coloured beads counted up to rotation, the ways of painting the faces of a cube, patterns on a wallpaper strip: whenever the question is how many arrangements there are once some group of motions is declared harmless, the same average answers it, and the fixed counts are usually easy because a symmetry that shuffles cells in cycles forces every cycle to be one colour.

## In the tree

The tree's own count is this count. A design is a choice of which corners of a cube to fill, two designs that differ only by turning or reflecting the cube are the same design, and [the core](/research/core/) states the census that follows. [A design is a Boolean function](/research/bijection/) carries the classification that census is taken in, and [the universe demo](/demos/universe/) draws every class in each dimension and base with the Burnside counts beside them. The arrangements being classified are choices made on parities, which is [parity](/wiki/parity/).
