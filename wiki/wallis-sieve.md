---
title: The Wallis sieve
lead: Cut a square into nine and drop the middle, cut each survivor into twenty-five and drop the middle, and keep going; the area left is a quarter of pi.
prerequisites: wallis-product, kronecker-product
---

The figure is the sieve after three rounds, drawn on a square of 105 cells a side. One big hole sits in the middle, eight middling holes sit around it, and 192 small holes are scattered through what is left. That is 201 holes in three sizes, one size per round, and everything not painted is the body that survives.

The rule is one line. Round `k` cuts every surviving square into `(2k + 1)^2` equal squares and drops the centre one. Round one cuts into nine and drops one, leaving eight. Round two cuts each of those eight into twenty-five and drops each centre. Round three cuts into forty-nine and drops each centre again.

The sides multiply: 3, then 3 times 5 is 15, then 3 times 5 times 7 is 105. The surviving cells multiply too: 8, then 8 times 24 is 192, then 192 times 48 is 9216. The holes are one per surviving square of the round before, so there is 1 hole, then 8, then 192, and the biggest is 35 cells wide, the next 7 and the smallest 1.

What matters is the area. Round `k` keeps `(2k + 1)^2 - 1` squares out of `(2k + 1)^2`, which is a share of `1 - 1/(2k + 1)^2`. Three rounds keep 8/9 times 24/25 times 48/49, which is 0.8359 of the original square. Carry on forever and the area is the product of `1 - 1/(2k + 1)^2` over every `k`, and that product is `pi/4`, or 0.785398.

Compare that with [the Sierpinski carpet](/wiki/sierpinski-carpet/), which uses the same cut every round: nine squares, drop the centre, again and again. Its area is 8/9 of 8/9 of 8/9 without end, and that runs to nothing. The sieve escapes because its cuts get gentler fast: it drops a ninth, then a twenty-fifth, then a forty-ninth, and the shares stop falling before they reach zero. The sieve buys area by growing its letters.

Letter is the right word. Round `k` has a tile of its own, a square of side `2k + 1` with its centre cell removed, and the sieve is those tiles multiplied together by [the Kronecker product](/wiki/kronecker-product/): stamp the side-5 tile into every filled cell of the side-3 tile, then stamp the side-7 tile into every filled cell of that. Sides multiply and filled cells multiply, which is where 105 and 9216 came from.

![The plane sieve at one, two and three rounds, with the side it reaches, the cells it keeps and the share of the square that survives](demos/wallis/sieve)

Drag the level in the panel above. At level 1 you see the single hole in a 3 by 3 square and an area of 0.888889. At level 2 the side is 15 and the area has fallen to 0.853333. At level 3 the side is 105, the cells are 9216 of 11025, and the area is 0.835918, on its way down to `pi/4` and not to zero.

The order and the choice of letters are the whole story. Use the same letter twice and the area starts dying again; use letters that grow, and the losses add up to something finite. That is why the product over the odd numbers, which [the Wallis product](/wiki/wallis-product/) splits off from a telescoping product, is exactly the area on the screen.

## In the tree

[The pi note](/research/pi/) uses this sieve as its counterexample: a fixed design keeps the same share every level, so its area is rational at every level and its limit is 0 or 1, while the sieve changes its share every level and so can land on `pi/4`. Reading a design as a word of letters, one per scale, is the move [the words demo](/demos/words/) draws, and [the Wallis product](/wiki/wallis-product/) is the arithmetic behind the area.
