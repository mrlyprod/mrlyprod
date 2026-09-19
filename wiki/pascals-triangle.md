---
title: Pascal's triangle
lead: A triangle of numbers where every entry is the sum of the two directly above it, and colouring the odd ones draws a fractal.
prerequisites: parity
---

Write a single 1. Under it write 1 1. Under that, write a 1 at each end and fill the middle by adding the two numbers above it, which gives 1 2 1. Carry on for ever: every entry is the sum of its two neighbours in the row above, and the ends are always 1. The first six rows read 1, then 1 1, then 1 2 1, then 1 3 3 1, then 1 4 6 4 1, then 1 5 10 10 5 1.

The entries count choices. The entry in row `n` at place `k`, counting both from zero, is the number of ways to pick `k` things out of `n` when the order of the picking does not matter. Row 4 is 1 4 6 4 1, so there is 1 way to pick nothing from four things, 4 ways to pick one, 6 ways to pick two, 4 ways to pick three and 1 way to pick all four. These numbers are called the binomial coefficients, because they are also the numbers that appear when you multiply out `(x + y)^n`.

The addition rule is that counting argument in disguise. To choose `k` things from `n`, either you take the last item, and then you need `k - 1` more from the `n - 1` that remain, or you leave it, and you need all `k` from those `n - 1`. Two cases, no overlap, so the entry is the sum of the two above it.

Each row adds up to a power of two. Row 4 adds to 16, row 5 to 32. Every subset of `n` things has some size, so counting the subsets by size and adding gives the total number of subsets, which is `2^n`.

The figure has 64 rows, row 0 at the top and row 63 at the bottom. Every odd entry is a solid blue disc and every even entry is a faint dot. The odd entries do not scatter. They draw a triangle with a triangular hole in the middle, each of the three solid corners holding a smaller copy of the same shape, down to the single discs. That is the Sierpinski triangle, and nothing in the drawing was designed: it is the parities of the numbers and nothing else.

Counting the lit discs is easy once you look at the row numbers in binary. Row `n` holds exactly `2^d` odd entries, where `d` is the count of 1s in the binary form of `n`. Row 7 is 111 in binary, three ones, so all `8` of its entries are odd, and indeed row 7 reads 1 7 21 35 35 21 7 1. Row 8 is 1000, one 1, so only 2 of its 9 entries are odd. Add that over rows 0 to 63, whose row numbers use six binary digits, and the figure holds 729 lit discs, which is 3 multiplied by itself six times.

Kummer's rule says which entry is odd, and it is a rule about carrying. The entry in row `n` at place `k` is odd exactly when adding `k` and `n - k` in binary needs no carry at any digit. Take row 4 and place 2: that is `2 + 2`, which in binary is `10 + 10`, and the two 1s collide, so there is a carry and the entry 6 is even. Take row 5 and place 2: that is `2 + 3`, in binary `10 + 11`, and again the 1s collide, so 10 is even. Take row 5 and place 1: `1 + 4` is `001 + 100`, no digit is used twice, no carry, and the entry 5 is odd.

The same rule said another way: the entry is odd exactly when every 1 in the binary form of `k` sits where `n` also has a 1. So the odd places of row `n` are the subsets of the 1s of `n`, and there are `2^d` of them, which is the count above.

That rule is why the picture repeats itself. Look at the top 32 rows and the bottom 32 rows. Adding 32 to a row number turns on one more binary digit, so the bottom half carries two separate copies of the top half, one on the left and one on the right, and the middle stays dark because a place in the middle would need a binary digit the row number does not have. Double the depth again and the same thing happens again. Three copies in place of one, at half the size, for ever.

## In the tree

The odd entries are a parity rule on a grid, and choosing cells by parity is move one of the tree, set out in [the core](/research/core/). The shape they draw is one of the tree's own designs, the gasket, grown by [the Kronecker product](/wiki/kronecker-product/), and [the universe demo](/demos/universe/) has it in the gallery beside every other rule of the plane. [The sponge demo](/demos/sponge/) grows a rule of this kind level by level, and [the Sierpinski carpet](/wiki/sierpinski-carpet/) is the same construction on a three by three block instead of a two by two one. The bit being read is the one [parity](/wiki/parity/) sets out.
