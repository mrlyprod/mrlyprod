---
title: The random walk
lead: A walker on a graph steps to a neighbour picked at random; after n steps on a grid it is about the square root of n away, and on a fractal it is slower.
prerequisites: graphs, fractal-dimension
---

Put a walker on a dot. At every tick it looks at the dots joined to the one it is standing on, picks one with no preference at all, and steps there. Repeat. That is the whole rule, and there is nothing else to it: no memory, no aim, no speed.

The figure runs it once. The lattice is 64 cells by 64, the walker starts at the middle yellow dot, and the blue trace is its path over 1024 steps. The orange dot is where it finished. The dim circle is drawn round the start with radius 32 cells, which is the square root of 1024.

The end of the walk sits on that circle, and this is the point of the picture. After `n` steps the walker is not `n` away, which is what a walker with a purpose would manage, and it is not still at home either. It is about `sqrt(n)` away. A thousand steps buy about thirty two cells of distance; a million steps would buy a thousand.

The reason is that squared distances add and distances do not. Suppose the walker is at distance `d` from home and takes one more step. Half the time the step is roughly outward and half the time roughly inward, so the average change in `d` itself is about nothing. But the average of `d` squared goes up by exactly one every step, since each step has length one and its direction is uncorrelated with where the walker already is. Start at zero, add one per step, and after `n` steps the average of `d` squared is `n`, so the typical `d` is `sqrt(n)`.

Look at the trace and you can see where the time went. The path is not a tour of the lattice; it is a dense scribble in two or three knots with thin threads between them. A random walker spends most of its steps revisiting cells it has already worn out, and only occasionally makes a run in one direction. The knots are the wasted time, and there is no way to avoid them.

That `sqrt(n)` is the honest shape of any walk on any full grid, in any number of dimensions. It is usually written as `n^(1/2)`, or turned upside down: the time to travel a distance `r` is about `r^2`. Doubling the distance costs four times the wait. This is diffusion, the same law that spreads ink through still water.

Fractals break it. Give the walker a shape with holes at every scale, the [carpet](/wiki/sierpinski-carpet/) or the gasket, and it now has to work around a hole of every size on the way out. It still wanders, but the detours are built into the shape and never end, so getting a distance `r` from home costs more than `r^2` steps.

The exponent is the measurement. Write the time to reach distance `r` as `r` to some power `d_w`, so that distance after `n` steps is about `n^(1/d_w)`. That number `d_w` is the walk dimension. On any full grid it is exactly 2. On a fractal it is bigger than 2, and the bigger it is the slower the shape is to cross.

It is a second and independent number from [the fractal dimension](/wiki/fractal-dimension/), which counts how fast mass piles up with scale. Mass tells you how much of the shape there is; the walk dimension tells you how well connected it is, how much of the mass is on the way to somewhere. Two shapes can weigh exactly the same at every scale and still take different times to cross.

The two together say how the shape rings. The density of low tones of [the graph Laplacian](/wiki/graph-laplacian/) is set by twice the mass dimension divided by the walk dimension, a ratio called the spectral dimension, so how a shape sounds is how much of it there is divided by how hard it is to get around it.

## In the tree

[The race demo](/demos/race/) puts walkers on two base-3 designs with the same mass and the same fractal dimension at once, and they spread at different speeds. [The walk dimension note](/research/walks/) is the census behind that race, measuring `d_w` two ways on designs matched by fill, and [the complexity note](/research/complexity/) reads the same number off the low end of the Laplacian spectrum instead of off a stopwatch.
