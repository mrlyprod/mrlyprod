---
title: The spirograph
lead: A small wheel rolls inside a big ring carrying a pen, and the fraction made by the two radii decides how many petals the pen draws and when it comes home.
prerequisites: greatest-common-divisor
---

Take a ring of radius `R` with teeth on the inside, and a wheel of radius `r` with teeth on the outside. Drop the wheel in, press a pen through a hole in it, and push the wheel round the inside of the ring without letting it slip. The pen draws a curve. The only three numbers that matter are the ring, the wheel, and how far the pen sits from the wheel's centre, which we will call the reach and measure in wheel radii, so a reach of 1 puts the pen on the rim and a reach of 0 puts it dead centre.

A pen at the centre just draws a circle, of radius `R - r`, because the wheel's centre goes round and round at that distance. Everything interesting comes from the pen being off centre: while the centre travels its circle, the wheel is also spinning, and the pen feels both motions at once.

The spinning is fixed by the rolling. No slipping means the arc the wheel covers on the ring equals the arc that passes under it on its own rim, so the wheel turns through `R/r` full turns for every one trip the centre makes round the ring. With a big ring and a small wheel that is a lot of spin per lap, which is why the curve wanders in and out so many times.

The figure is the case `R = 7` and `r = 3` with the pen at reach 0.8, drawn from 4000 samples as one unbroken line, with the ring itself as a faint circle around it. The curve has seven arms. It runs out near the ring, turns, comes back in, and repeats, and after three trips round the ring it arrives exactly where it started and closes.

Those two counts are the fraction `R/r` in lowest terms. Write it as `a/b` with no common factor: the curve closes after `b` trips round the ring, and it has `a`-fold symmetry, so it shows `a` arms. Here `7/3` is already in lowest terms, so three laps and seven arms. If the ring and wheel share a factor, cancel it first: `R = 6` and `r = 3` is `2/1`, so one lap and two arms, which is just a flattened oval.

This is why a spirograph set gives such different curves from wheels of similar size. A wheel of 30 teeth in a ring of 96 is `16/5` after cancelling, so five laps and sixteen arms. A wheel of 32 teeth in the same ring is `3/1`, so one lap and three arms, done almost before it starts. Two teeth of difference, an entirely different picture, and the reason is nothing but the common factor.

The reach changes the shape of the arms without changing either count. At reach 1 the pen is on the rim, and at the moment a point of the rim touches the ring it is instantaneously still, so the curve comes to a sharp point there: the arms end in cusps. Below 1 the pen never reaches the ring and the arms end in smooth blunt tips, as in the figure. Above 1 the pen sticks out past the rim on an arm of its own, it overshoots at each turn, and the arms end in little loops that cross themselves.

Rolling the wheel around the outside of the ring instead gives the other family. The centre then travels a circle of radius `R + r`, the wheel spins the other way relative to the ring, and the arms point outwards like the petals of a flower rather than inwards like a star. The counting rule is the same one: cancel the fraction, and the numerator counts the arms while the denominator counts the laps.

## In the tree

[The spirograph note](/research/spirograph/) makes a design the wheel: it seats a pen at the centre of every cell at once, so one design draws a whole family of these curves in one roll, and it counts how many of those curves are genuinely different rather than the same curve drawn twice. [The spirograph demo](/demos/spirograph/) rolls any design on a circle, on a straight line or around a polygon and prints those counts as it draws. Reducing `R/r` to lowest terms is [the greatest common divisor](/wiki/greatest-common-divisor/) at work, and the fractions that give the longest curves before closing are the ones [the Farey sequence](/wiki/farey-sequence/) orders.
