---
title: The Euler-Mascheroni constant
lead: Add 1 and a half and a third and on to one over n, take away the logarithm of n, and the difference settles on 0.5772157, a number nobody has yet placed.
prerequisites:
---

Add the reciprocals of the whole numbers in order: 1, then 1 and a half, then 1 and a half and a third. That running total is the harmonic sum, and it has no ceiling. Add enough terms and it passes any number you name, but it does so unbearably slowly: it takes ten terms to pass 2.9, a hundred to pass 5.1, and a thousand to pass 7.4.

The thing it climbs like is the logarithm. Write `ln n` for the natural logarithm of n, the curve that rises by the same amount each time n is multiplied by the same factor. The harmonic sum of n terms and `ln n` grow at the same rate forever, and the interesting question is what separates them.

The figure answers it. The yellow staircase is the harmonic sum, one tread per term, standing at 1 after one term and just under 4 after thirty. The blue curve under it is `ln n`. The shaded strip between the two is the whole story: it is wide at the left, and by the right hand edge it has stopped narrowing.

Read the strip's width off the numbers. After one term it is exactly 1. After ten it is 0.626383. After thirty, the right edge of the figure, it is 0.593790. After a hundred it is 0.582207, and after a thousand it is 0.577716. Those numbers are falling towards something, and that something is the Euler-Mascheroni constant, written with the Greek letter gamma and worth 0.5772157 to seven places.

Why is there a strip at all? Every tread of the staircase adds one over n. Over the same stretch the curve rises by a little less than one over n, because the curve is already flattening while the tread is still using the old value. The surplus is a thin sliver, and the slivers shrink fast enough that all of them together come to a finite amount. Gamma is the total area of every sliver, counted to infinity.

The closing is slow and completely regular. At n terms the strip is wider than gamma by about one over twice n. At thirty terms that predicts a surplus near one sixtieth, or 0.0167, and the true surplus is 0.0166. At a thousand terms it predicts one two-thousandth, and delivers it. So to pin gamma down to one more decimal place by this route you need ten times as many terms, which is why nobody computes it this way.

Gamma turns up wherever a sum is traded for a curve. The commonest place is the logarithmic integral, the smooth curve that guesses how many primes lie below a number, and gamma sits in its series as a plain additive term. It is in that sense a conversion constant between counting and measuring.

What is not known about gamma is embarrassing. It has been computed to many billions of digits. No one has shown that it is not a fraction. Pi and e were both settled as irrational long ago, and gamma, the third constant of the same size and the same age, still has not been.

## In the tree

The harmonic sum closing on gamma is one of the eight on [the famous formulas hub](/wiki/famous-formulas/), beside four other constant chasers and three systems that never settle. The same demo is the one [the pi note](/research/pi/) points at when it puts its own slow estimator in that family. Where gamma does its real work here is inside the logarithmic integral of [the prime counting function](/wiki/prime-counting-function/).
