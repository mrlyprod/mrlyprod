---
title: The Menger sponge
lead: The carpet's solid cousin: cut a cube into 27, drill out the middle and the middle of each face, then do the same to the 20 cubes that are left.
prerequisites: sierpinski-carpet
---

Take a solid cube and cut it into 27 small cubes, three along each edge, like a puzzle cube. Remove the one in the very middle, where nothing shows from outside, and remove the middle cube of each of the six faces. Seven cubes are gone and 20 are left.

What that leaves is a cube with a square tunnel drilled through it in each of the three directions, all three meeting in the hollow middle. The 20 survivors are the eight corner cubes and the twelve edge cubes.

Now do the same to each of the 20. Each is cut into 27 and loses its own seven, so the second round leaves `20 x 20 = 400` cubes on a side of nine. At level `n` the count is `20^n` cubes on a side of `3^n`: 20, then 400, then 8000. The figure draws level 3, all 8000 of its cubes standing on a side of 27.

The tree builds it with one move. Write the level-1 rule as a small block of 27 cells, 20 filled and 7 void, and stamp a copy of that block into every one of its own filled cells. Repeat, and each repetition is a level. The counts multiply because 20 filled cells each carrying 20 filled cells is 400.

The rule reads one digit of each coordinate. Number the three slots along each axis 0, 1 and 2, and a small cube is thrown away exactly when two or three of its digits are the middle one. Nothing else is looked at. That is the same kind of rule as the carpet's, one axis longer, and it is why the sponge and the carpet sit side by side in the tree rather than in separate stories.

Volume drains away. Each round keeps 20 of 27, so after `n` rounds the solid occupies `(20/27)^n` of the cube it started in. That falls to zero, so the finished sponge has no volume. Its surface does the opposite: every round drills new tunnels and new walls, and the surface area grows past any bound you name. The shape is all skin.

Look straight at a face and you see the carpet. The cells of the sponge that touch one outer face are exactly the cells the carpet keeps, because on that face one coordinate is pinned and the rule on the other two is the carpet's rule. So each of the six faces of the figure is a Sierpinski carpet at the same level, and the shadow the sponge casts along an axis is a carpet too.

Like the carpet it stays in one piece at every level, because the 20 cubes of the rule are joined face to face all the way round and stamping a joined-up block into a joined-up block keeps it joined. A shape of no volume whose surface never stops growing is still one connected object you could walk across.

Counting boxes gives the sponge a dimension of `log 20 / log 3`, about 2.73, more than a surface and less than a solid, which is [the fractal dimension](/wiki/fractal-dimension/) read on three axes instead of two.

## In the tree

The sponge is the design `bang dim 3, code 23` of [the core](/research/core/), filling 20 of 27 cells at side 3 and `20^level` at every level after. [Structure against noise](/research/connectivity/) races it against a random set of matched size at `27^3` and `81^3` and finds it in one piece both times. [The sponge demo](/demos/sponge/) grows it and any other cube rule level by level, [the universe demo](/demos/universe/) shows it among all 22 distinct cube designs, and [the tour demo](/demos/tour/) reads its exposed faces off as an integer sequence. Its square cousin is [the Sierpinski carpet](/wiki/sierpinski-carpet/).
