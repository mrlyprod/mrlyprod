---
title: Moire
lead: Lay two rulings over each other and they beat; the points both of them mark are the ones their scales agree on, and stacking many scales lights the points most scales share.
prerequisites: farey-sequence, greatest-common-divisor
---

Rule the unit square into 5 equal columns and 5 equal rows. Now rule the same square into 7 columns and 7 rows, in another colour, and lay one over the other. That is the figure, and the pattern it makes is a moire.

Neither ruling is interesting on its own. Together they are, because their lines fall out of step. The blue line at `1/5 = 0.2` and its nearest orange neighbour at `1/7 = 0.1429` sit two thirty fifths apart; the blue line at `2/5 = 0.4` and the orange one at `3/7 = 0.4286` are only one thirty fifth apart. The gaps swell and shrink across the square, and that swelling is the beat.

The grey dots in the figure mark every place where a line of one ruling crosses a line of the other. There are 92 of them, and their spacing is visibly uneven: tight where the two rulings are in step, loose where they are not. Nothing is moving and nothing is curved, and still the eye sees bands.

The yellow discs are the four points where the two rulings actually agree, one at each corner. Those are the only places where a line of the 5-ruling and a line of the 7-ruling land on exactly the same coordinate.

Why only the corners? Scale `n` puts a line at every `k/n`. A point at the fraction `a/b`, written in lowest terms, gets a line from scale `n` exactly when `b` divides `n`. So a point marked by both scale 5 and scale 7 has a bottom number dividing both, which means it divides [their greatest common divisor](/wiki/greatest-common-divisor/), and `5` and `7` share only 1. A bottom number of 1 leaves only 0 and 1 themselves.

Change the pair and the picture changes completely. Scales 4 and 6 share a divisor of 2, so they agree at 0, at `1/2` and at 1, and the beat is short and coarse. Scales 5 and 10 agree at every line the coarser one draws, and there is no beat at all, just one ruling sitting inside the other. Coprime scales are the ones that agree least and therefore beat longest.

Now stack instead of pairing. Draw the ruling for every scale `n` from 1 up to some limit, all faintly, one on top of another, and count how many lines land on each point. A point `a/b` collects one line from every multiple of `b` in the range, so its brightness is the count of those multiples. Small bottom numbers are bright, large ones are faint, and the brightness falls off like one over `b`.

That stack lights exactly the fractions in lowest terms, ordered by how simple they are: `1/2` brightest after the ends, then `1/3` and `2/3`, then the quarters, and so on. It is [the Farey sequence](/wiki/farey-sequence/) drawn as light. The picture is not a decoration of the fractions; it is the fractions.

One more reading follows for free. The new points a scale `n` contributes, the ones no smaller scale had already lit, are the fractions `a/n` with `a` sharing no factor with `n`. A prime scale shares a factor with nothing below it, so it contributes the most new points of any scale near it, and primality is visible in a stack of rulings as a sudden burst of new lines.

## In the tree

[The moire demo](/demos/moire/) stacks one design's grid at scale 1, 3, 5 and on, and the interference is the fine grids landing on the coarse. [The Farey note](/research/farey/) reads that moire as a diagram of the fractions and asks how evenly its lit points spread, while [the algebra of the stack](/research/stack/) asks what happens when the layers are weighted, restricted or spun. [The tourbillon demo](/demos/tourbillon/) turns every layer by its own angle so the shared grid breaks and only the centre survives, and [the hexagon note](/research/hexagon/) runs the same interference on stacked diagonal slices.
