---
title: Rajchman measures
lead: Spread a unit of mass round a circle and listen to it at higher and higher frequencies; if the sound dies away the measure is Rajchman, and the Cantor dust is the famous one that never goes quiet.
prerequisites:
---

Take a circle of circumference one and spread one unit of mass along it any way you like: evenly, in a smooth hump, in a few heavy dots, or as dust on a Cantor set. The spread is a measure, written `mu`. To listen to it at frequency `n`, average the wave `e^(-2 pi i n x)` against it; the number that comes out is the Fourier coefficient `hat mu(n)`. At `n = 0` it is the total mass, 1. The question is what happens as `n` climbs.

Mass spread evenly answers with silence: every coefficient past `n = 0` is exactly zero, because the wave averages out. Mass with any density at all answers with a fade: `hat mu(n) -> 0` as `abs(n) -> infinity`. That is the Riemann-Lebesgue lemma. A measure whose coefficients tend to zero is called a Rajchman measure, so every measure with a density is one.

A heavy dot is the opposite. Mass 1 at one point `a` gives `hat mu(n) = e^(-2 pi i n a)`, of size 1 at every `n`. A measure carried on countably many dots keeps ringing for ever, so it is never Rajchman.

Between the two lies the interesting case: no dots, no density. Cut the middle third out of `[0, 1]`, then the middle third out of each piece left, and so on for ever; at stage `k` there are `2^k` pieces of length `3^-k`, and the Cantor measure gives each of them mass `2^-k`. No point carries mass, and the dust has length zero, so there is no density either. Does it fade?

The figure answers. The upper panel is `abs(hat mu(n))` for `n` from 1 to 243, one bar per `n`. The comb never settles: the yellow bars at `n = 1, 3, 9, 27, 81, 243` all stand at exactly the same height, `0.371`. The lower panel is the same plot for a smooth bump one tenth of the circle wide, and its comb falls to nothing by `n = 40`.

The six equal bars come from one line. The Cantor set is two copies of itself shrunk by 3, one at each end, so its measure obeys `hat mu(3 t) = (1 + e^(-4 pi i t)) hat mu(t) / 2` for every real `t`. When `t` is a whole number the bracket is 2, and

$$\hat\mu(3n) = \hat\mu(n).$$

So `hat mu(3^k) = hat mu(1)` for every `k`, and `hat mu(1)` is not zero, so a spike of height `0.371` stands at every power of three. The coefficients never tend to zero and the Cantor measure is not Rajchman. The whole comb obeys the rule: the bar at `3n` copies the bar at `n`, so every third bar replays the first 81 stretched out by three.

The name is Rajchman's because in 1922 he proved that every measure whose coefficients fade gives the Cantor set mass zero (the history is told in Lyons 1995, a survey of seventy years of Rajchman measures), a step in the question of which sets a trigonometric series may ignore and still be pinned down by its sum, and his papers of 1928 and 1929 began the study of the class itself. The class holds everything with a density, nothing with a dot, and for dust the answer is decided case by case: when the shrinking maps of a self-similar set have two ratios whose logarithms are incommensurable, every self-similar measure on it is Rajchman (Li and Sahlsten, arXiv:1902.00426), and when all the maps share one ratio, as the two Cantor maps do, the question turns on whether the reciprocal of that ratio is a Pisot number, as 3 is.

## In the tree

[The weights note](/research/weights/) asks this question of the measure a weighted digit design carries.
