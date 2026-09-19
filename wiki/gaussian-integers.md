---
title: The Gaussian integers
lead: Numbers of the form a plus b times i, drawn as a square lattice, where each point has a size and some ordinary primes break in two while others stay whole.
prerequisites: prime-numbers
---

Let `i` be the number whose square is `-1`. A Gaussian integer is anything of the form `a + b i` with `a` and `b` ordinary whole numbers. Draw `a` across and `b` up and they are the corners of a square grid, one point per cell, stretching out in all four directions.

Adding two of them adds the coordinates, so the sum of two grid points is a grid point. Multiplying works out too, because `i^2` folds back to `-1`: `(2 + i)(2 - i) = 4 - 2i + 2i - i^2 = 5`. So the grid is closed under both operations, and it is a world you can do arithmetic in.

Every point gets a size, called its norm: the norm of `a + b i` is `a^2 + b^2`. That is the square of the distance from the origin, so it is always a whole number and never negative. The one fact that matters about it is that it multiplies: the norm of a product is the product of the norms. Check the line above, `5` times `5` is `25`, and `25` is the norm of `5`.

Four points have norm 1: `1`, `-1`, `i` and `-i`. These are the units, the Gaussian version of plus and minus one, and multiplying by one of them turns the picture a quarter turn. They are why every pattern in this world has four-fold symmetry.

A point is prime here when it cannot be written as a product of two points of norm bigger than 1. The norm rule makes this checkable: to split a point of norm `N`, you need two points whose norms multiply to `N`, so a point whose norm is an ordinary prime cannot split at all.

Now the interesting part. An ordinary prime, sitting on the horizontal axis, may or may not survive the move into this larger world. Three small cases tell the whole story.

Five splits. `5 = (2 + i)(2 - i)`, two points of norm 5 each, and neither of them can be broken further. So 5 is no longer prime once you allow `i`.

Two splits too, but oddly. `2 = (1 + i)(1 - i)`, and `1 - i` is just `1 + i` turned by a unit, so 2 is essentially a square: the same prime used twice. It is the only ordinary prime that behaves this way, and it is why the figure has a special point at `1 + i` and its three reflections.

Three stays whole. To break 3 you would need a point of norm 3, which means whole numbers with `a^2 + b^2 = 3`. The squares available are 0, 1 and 4, and no two of them add to 3, so there is no such point and 3 stays prime. A prime that survives like this is called inert.

The rule behind the three cases is old and exact. An odd prime splits when it is one more than a multiple of four, and stays whole when it is three more. So 5, 13, 17 and 29 break up, and 3, 7, 11 and 19 do not. Another way to say the same thing is that a prime is the sum of two squares exactly when it is of the form `4k + 1`, which is Fermat's theorem on two squares.

The figure marks every Gaussian prime inside a window reaching 20 steps in each direction. Blue is a point whose norm is an ordinary prime, which is a piece of a split prime, and there are a great many of them, arranged in a four-armed snowflake. Orange is an inert prime, and there are just sixteen: `3`, `7`, `11` and `19`, each with its minus and each with its `i` copy, so eight on the horizontal axis and eight on the vertical. The orange points appear nowhere else, because an inert prime keeps sitting on an axis no matter how you turn it.

## In the tree

[The primes in the plane demo](/demos/gaussian/) draws this window at any reach and switches the lattice from square to hexagonal, where the same three fates appear on a different grid. That switch is the subject of [what base 3 hides](/research/bases/): base 2 puts a design on the square lattice, which is this one, base 3 puts it on the hexagonal lattice, and which lattice a design lives on decides which constant it hides.
