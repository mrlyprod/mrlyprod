---
title: The Sierpinski carpet
lead: Cut a square into nine, throw the middle one away, then do the same to the eight that are left, and keep going for ever.
prerequisites: kronecker-product
---

Start with a solid square. Divide it into a three by three grid of nine equal squares and remove the middle one. Eight squares are left, arranged in a ring around a square hole.

Now do the same to each of the eight. Each is cut into nine and loses its middle, so eight rings of eight sit where eight solid squares were. The picture is nine cells wide and holds 64 filled cells, with one hole of three by three in the middle and eight holes of one cell each around it.

Repeat. At level `n` the picture is `3^n` cells wide and `8^n` of them are filled. Level 1 is 8 of 9, level 2 is 64 of 81, level 3 is 512 of 729 and level 4, which the figure draws, is 4096 filled cells on a side of 81.

The step is one move, not a list of instructions. Take the eight-around-a-hole picture and stamp a copy of it into each of its own filled cells: that is [the Kronecker product](/wiki/kronecker-product/) of the picture with itself, and doing it `n` times is the carpet at level `n`. The counts follow with no extra work, because eight filled cells each carrying eight filled cells is 64.

The area falls away. Each round keeps eight ninths of what it started with, so after `n` rounds the shape covers `(8/9)^n` of the original square. That number shrinks towards zero, and it never stops shrinking, so the carpet itself has no area at all. The figure looks solid only because it stops at level 4.

The holes are countable too. Level 1 opens one hole, level 2 opens eight more, level 3 opens sixty-four more, and level `n` has `(8^n - 1)/7` holes in all: 1, then 9, then 73, then 585. Each round's holes are a third of the width of the round before.

Nothing ever gets cut off. At every level the filled cells form a single connected piece, because the eight cells of the rule touch each other edge to edge all the way round the ring, and stamping a connected picture into a connected picture leaves it connected. The carpet has no area and is still all one thing.

There is a short way to say which points survive. Write the two coordinates of a point in base 3. A point is thrown away at some round exactly when a 1 appears in the same place of both expansions, so the carpet is the set of points whose two base-3 expansions never carry a 1 in the same position. That is a rule on one digit of each coordinate, which is the tree's way of naming it: keep eight of the nine cells and drop the middle, `code` 495 read at `base` 3.

A shape that has no area but is not a curve does not fit the usual count of dimensions, and the honest answer is not a whole number. Counting boxes gives `log 8 / log 3`, about 1.89, which is [the fractal dimension](/wiki/fractal-dimension/).

## In the tree

The carpet is one code among the sixteen plane designs of [the core](/research/core/), where it fills 8 of 9 cells at side 3 and `8^level` at every level after. [Structure against noise](/research/connectivity/) races it against a random set of exactly the same cell count and finds the carpet in one piece where the random set is in hundreds. [The tour demo](/demos/tour/) grows it level by level and reads its perimeter off as an integer sequence. Its cube is [the Menger sponge](/wiki/menger-sponge/).
