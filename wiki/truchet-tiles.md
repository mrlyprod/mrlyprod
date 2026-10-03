---
title: Truchet tiles
lead: A square with two quarter circles on it, laid on a grid in one of two turns, joins up into closed loops and long strands that never branch and never cross.
prerequisites: parity
---

Take a square tile and mark the midpoint of each edge. Draw a quarter circle round one corner, from the midpoint of one edge at that corner to the midpoint of the other, and a second quarter circle round the opposite corner. That is the tile. Turn it a quarter turn and the arcs sit round the other two corners; turn it once more and it is back where it started, so the tile has only two ways to lie.

Fill a grid with these tiles, choosing the turn of each one by a coin or by a rule. Every tile has an arc ending at each of its four edge midpoints, so wherever an arc ends, the tile across that edge has an arc ending at the same point. Both arcs meet the shared edge at a right angle, so they join without a kink. Each midpoint inside the board is the end of exactly two arcs, one from each side, so the arcs chain into smooth curves that never branch and never cross.

A curve can end up in one of two ways. It can come back to where it started, a closed loop, or it can run into the border of the board at both ends, an open strand. The border fixes how many strands there are. An `n` by `n` board has `4n` midpoints on its border, each is the end of exactly one strand, and each strand has two ends, so there are always `2n` strands whatever the coins say. The loops are what the coins decide.

The figure is a 16 by 16 board, 256 tiles and 512 arcs, every turn drawn from a fixed seed. Its 32 open strands, in blue, end on the 64 midpoints of the border, and its 19 closed loops are yellow. The smallest loop is a circle of four arcs round a single corner, which appears wherever the four tiles that share that corner all put an arc round it.

The curves cut the board into regions, and the regions can always be coloured with two colours so that the two sides of every curve differ: grey and ground in the figure. Give every corner of the grid its coordinates and call it even or odd by the [parity](/wiki/parity/) of their sum. The two arcs of a tile go round two opposite corners, which have the same parity, and each cuts its corner off from the band in the middle of the tile, which holds the other two corners, of the other parity. Across a tile edge the pieces near a corner join up into one region. So every region holds corners of one parity only; colour it by that parity, and every curve has an even region on one side and an odd one on the other.

A rule in place of the coins gives an ordered pattern. Lay every tile the same way and each curve is a wave running diagonally from border to border: all strands and no loops. Alternate the two turns like a chessboard and every tile puts its arcs round the even corners, so each even corner inside the board is ringed by a small circle and the board is a lattice of rings. Any picture drawn in two colours on a grid can choose the turns, one turn for each colour.

The arcs are the later form of the tile. The first Truchet tile is a square cut along its diagonal into two triangles, one dark and one light. It has no symmetry to spare, so it has four turns, one for each corner the dark triangle can fill. [Truchet 1704](https://archive.org/details/histoiredelacad04laca) counts the ways two of these tiles can be set side by side, 64 in all, giving 32 different figures, and builds plates of patterns from those pairs. [Smith 1987](https://doi.org/10.2307/1578535) translates that memoir and frees its idea from the symmetry rules, and the tile of two quarter circles, which turns a pattern of triangles into a pattern of curves, is the form credited to Smith.

## In the tree

A Truchet board is one bit a cell, one turn or the other, and a plane design of [the core](/research/core/) is one bit a cell too, filled or empty, so every design can be laid out as a Truchet board with its filled cells turned one way and its empty cells the other. [Loops in arcs](/research/arcs/) draws designs that way and counts the closed loops they make level by level, and [the arcs demo](/demos/arcs/) lays any design out as a board to count its loops. [Wang tiles](/wiki/wang-tiles/) are the opposite case: there each edge carries a colour and only tiles that agree may touch, while a Truchet tile fits beside any other in either turn.
