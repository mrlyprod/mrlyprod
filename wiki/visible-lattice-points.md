---
title: Visible lattice points
lead: Stand at a corner of the grid and some points hide behind nearer ones; the ones you can see are the pairs sharing no divisor, and counting them hands back pi.
prerequisites:
---

The figure is a window on the grid, a hundred points across and a hundred up, with the corner you stand at in the lower left. Every point you can see from that corner is drawn as a full blue cell. Every point hidden behind a nearer one is left empty apart from a small square at its centre, and the smaller that square, the more thoroughly the point is hidden. Of the ten thousand points in the window, 6087 are full.

Put the corner at (0, 0) and pick the point (a, b), meaning a steps right and b steps up. Draw the straight line from the corner to it. The point is visible when no other grid point sits on that line between the two, and hidden when one does, because from the corner the two lie in the same direction and the nearer one stands in front.

Two whole numbers share a divisor when some whole number bigger than 1 divides both of them with nothing left over. 6 and 9 share the divisor 3. 6 and 35 share nothing, since 6 is built from 2 and 3 while 35 is built from 5 and 7. The rule of the picture is that (a, b) is visible exactly when a and b share no divisor.

One direction is easy to see. If some g bigger than 1 divides both a and b, then (a/g, b/g) is a grid point as well, it sits on the same line out of the corner, and it is g times nearer, so it hides (a, b). The other direction is the same sentence backwards: a grid point on the line between the corner and (a, b) is some fraction of the way along, the same fraction of both steps, and the bottom of that fraction divides a and b alike.

So every hidden point is a scaled copy of a visible one. Take any point, let g be the largest number dividing both its steps, and the point is exactly g times the visible point (a/g, b/g). That is what the figure shades: a hidden point owned by the scale g gets a square of side one over g, so the deeper a point is buried, the smaller its mark.

Read that backwards and the whole grid is the visible set stamped down over and over. Lay the visible points on the grid at scale 1, then again at scale 2, then at scale 3, and so on for every whole scale. The scale-2 copy lands on the points whose two steps are both even, the scale-3 copy on the points whose steps are both divisible by three, and between them the copies cover every point once and never twice.

Now count. A window n by n holds `n^2` points, and the share of them that are visible settles down as the window grows. It does not settle on a round number. It closes on `6/pi^2`, which is 0.6079 and a little more, so about three points in every five are visible however far you push the window out.

The pi in that constant comes from the primes. For a prime p, one pair of steps in `p^2` has both steps divisible by p, so the share of pairs p does not spoil is `1 - 1/p^2`. A pair is visible when no prime at all spoils it, which multiplies those shares over every prime, and that product is 1 divided by `1 + 1/4 + 1/9 + 1/16 + ...`, the sum of one over every square. That sum is the famous one worth `pi^2/6`, so the share of visible points is its reciprocal, `6/pi^2`.

Turn the constant round and the counting becomes a measurement. If the visible share of a window is d, then

$$\pi = \sqrt{6/d}$$

Count the full cells of the figure, divide 6087 by 10000, and the formula gives 3.1396, which is pi to three figures from nothing but dots on a grid. It is an honest way to the constant and a slow one; how slowly it closes, and what happens when the grid is three dimensional instead of two, is the pi note's business.

![The window of the grid: the points visible from the corner lit, the hidden ones shaded by the scale that hides them.](demos/pi/window)

The slider grows the window from eight points across to three hundred. The line under the picture counts the visible points, the points in the window and the share between them, and that share runs high in the smallest windows and settles towards 0.608 as you push the slider right. Turning the shading off flattens every hidden point to one tone, leaving only the two kinds of point.

## In the tree

Counting these points is how [pi comes out of the grid](/research/pi/). The same count restricted to the cells of one design, where every design gets its own constant, is [the coprimality spine](/research/coprime/). Written as fractions rather than as points, the visible set is [the Farey sequence](/wiki/farey-sequence/), and the stack of scales above is read there and in [the Farey note](/research/farey/).
