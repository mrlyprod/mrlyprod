---
title: Parity
lead: Every whole number is even or odd, and that single bit, read on each coordinate at once, is the whole of what a design uses to choose its cells.
prerequisites:
---

A whole number is even when it divides by two and odd when it does not. That is its parity, and there are only ever two answers. Zero is even, one is odd, and from there the answers alternate for ever.

Parity is easy to work with because it survives arithmetic. Even plus even is even, odd plus odd is even, even plus odd is odd. You never need the numbers themselves, only their two answers, and that is what makes parity a rule a machine can apply to a grid of any size.

A point on a line has one coordinate, so it has one parity. A cell in a grid has two, a row number and a column number, so it carries two parities at once, and there are four ways that can come out: both even, row even and column odd, row odd and column even, both odd.

The figure is a six by six grid with every cell painted by that pair. Four colours appear, nine cells of each, and they repeat every second row and every second column, so the whole picture is one two by two tile stamped nine times. Wherever you stand in the grid, you are in exactly one of the four classes, and moving one step in any direction moves you to another.

In three dimensions the same reading gives three parities and eight combinations. Write even as 0 and odd as 1 and those eight combinations are the eight corners of a cube: `(0,0,0)`, `(0,0,1)`, `(0,1,0)` and so on up to `(1,1,1)`. Every cell of a three-dimensional grid, however far from the origin, belongs to exactly one corner.

That is the move the tree is built on. A design is a choice of which corners to keep. Keep a corner and every cell of that class is filled; drop it and every cell of that class is void. In two dimensions there are four corners, so `2^4 = 16` designs. In three there are eight corners, so `2^8 = 256`. The list is finite and it is short enough to write down.

The Sierpinski carpet is one such choice. Lay a three by three block down, number the rows and columns 0, 1, 2, and throw away the one cell whose row and column are both the middle number. Eight cells survive of nine, and that rule, repeated, is the carpet.

The eight answers are also a number. Read the corners in binary order and write 1 for kept and 0 for dropped, and the byte you get is what the tree calls the design's `code`. The carpet is code 7 in the plane, the Menger sponge is code 23 in the cube, and the code is the whole of the design's name.

Past base 2 the reading widens. Parity is a coordinate's last binary digit, and the general rule reads a whole digit instead of a bit, one digit per axis, with `base` digits to choose from. At `base` 3 each coordinate has three residues rather than two, so a plane design chooses among nine cells rather than four. The idea does not change: the rule looks at one digit of each coordinate and nothing else.

## In the tree

Choosing corners of the parity cube is move one of the tree, written up in [the core](/research/core/), and [the automata](/research/automata/) reads the same eight corners as a rule on a cell and its two neighbours. [The universe demo](/demos/universe/) is the gallery of every choice in dimensions 1 to 4, [the sponge demo](/demos/sponge/) lets you pick corners of a cube and grow them, and [the wolfram demo](/demos/wolfram/) shows the eight corner bits of a rule as a stamp. Move two, stamping the chosen cells into themselves, is [the Kronecker product](/wiki/kronecker-product/).
