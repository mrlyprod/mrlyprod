---
title: The graph Laplacian
lead: Degree minus adjacency, one table per network; its eigenvalues are the tones the network can ring at, and the staircase they make says how it is knit together.
prerequisites: graphs
---

Take a graph and build two tables. The first is the adjacency table: a 1 where two dots are joined and a 0 where they are not. The second is the degree table: the count of lines at each dot down the diagonal and zeros everywhere else. Subtract the first from the second and you have the graph Laplacian.

Written out, the rule is short. The diagonal entry for a dot is its degree. The entry for a pair of joined dots is `-1`. Everything else is 0. Every row therefore adds up to zero, because the degree on the diagonal is exactly the number of minus ones in that row.

What the table does is compare a dot with its neighbours. Give every dot a number, apply the Laplacian, and the answer at each dot is its own value times its degree minus the sum of its neighbours' values. It is zero when a dot agrees with the average of its neighbours and large when it disagrees, so the Laplacian measures roughness.

Now ask for the eigenvalues: the numbers `lambda` for which some assignment of values to the dots comes back multiplied by `lambda` and otherwise unchanged. A graph with `v` dots has `v` of them, counted with repeats, and none of them is negative. The smallest is always 0, belonging to the assignment that gives every dot the same value, because that one is perfectly smooth and the Laplacian flattens it to nothing.

Those eigenvalues are the tones. A drum skin has shapes it can vibrate in, each with its own frequency; a network has the same, with the Laplacian in place of the drum's smoothness, and the eigenvalue playing the part of the frequency squared. Small eigenvalues are slow, smooth, long-wavelength modes and large eigenvalues are jagged ones where neighbours disagree everywhere.

The figure sorts the eigenvalues from small to large and draws them as a staircase, one tread per eigenvalue, for two networks of 32 dots each. The blue staircase is a path: 32 dots in a row, each joined to the next, two loose ends. The orange staircase is a cycle: the same 32 dots with the two ends joined up.

Both have closed forms, and they are almost the same formula. The path's eigenvalues are `2 - 2 cos(pi k / 32)` for `k` from 0 to 31, and the cycle's are `2 - 2 cos(2 pi k / 32)` for the same `k`. Both climb from 0 towards 4, which is why the two staircases shadow each other so closely.

The difference is in the repeats. On the cycle, `k` and `32 - k` give the same number, so all but two of its tones come in matched pairs, 17 distinct values across 32 eigenvalues. On the path every value is different, all 32 of them. Joining the two loose ends changes almost nothing about the pitch of the network and everything about how many ways it can ring at that pitch, and that is the cycle's symmetry made audible.

The shape of the staircase is the real reading. A gentle start with the tones packed close together means many slow modes, which means the network is loosely knit and slow to mix. A staircase that jumps away from zero at once means there is no slow mode at all, and a network that mixes fast. How many eigenvalues are 0 is a count of pieces: one zero for each component, so a network in one piece has exactly one.

Take a fractal instead of a path or a cycle and the staircase goes strange. Tones repeat in great blocks, the same eigenvalue arriving with a multiplicity that grows with the level, and the low end of the staircase follows a power law whose exponent is a dimension of its own. The shape of the shape is written in the list of its tones.

## In the tree

[The spectra demo](/demos/spectra/) diagonalises the Laplacian of a design's graph in the browser, draws this staircase, and fits the slope of its low end. [The complexity note](/research/complexity/) is where those repeats become laws, counted level by level, and [the walk dimension note](/research/walks/) reads the same low end as the speed of a wanderer. [The modes demo](/demos/modes/) shows the other spectrum of the same object, the design laid on a torus, where the eigenvalue field turns out to be a picture of the tile.
