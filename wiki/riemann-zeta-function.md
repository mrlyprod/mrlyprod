---
title: The Riemann zeta function
lead: Add one over every whole number raised to the power s; the answer is zeta of s, it equals pi squared over six at s equal to 2, and it can be rewritten as a product over the primes.
prerequisites: prime-numbers
---

Pick a number `s` and add up `1` plus `1/2^s` plus `1/3^s` and on forever, one term for every whole number. The total is the zeta function, `zeta(s)`.

$$\zeta(s) = 1 + \frac{1}{2^s} + \frac{1}{3^s} + \frac{1}{4^s} + \cdots$$

The sum only settles if `s` is bigger than 1. At `s = 1` the terms are `1, 1/2, 1/3, 1/4` and their total grows without bound, slowly but forever. Push `s` a little above 1 and the terms shrink fast enough that the total stops somewhere.

The figure shows three of those totals being built. Each curve is the running sum for one value of `s`, drawn term by term for the first forty terms, and each closes on its own hairline. The top curve is `s = 2`, still visibly short of its line after forty terms because its terms only shrink like `1/n^2`. The middle is `s = 3` and the bottom is `s = 4`, and both are flat long before the right edge.

The top line is the famous one. `zeta(2) = 1 + 1/4 + 1/9 + 1/16 + ...` comes out to pi squared over six, which is 1.6449 and on. Euler found that, and it is still surprising: a sum of reciprocal squares, nothing round anywhere in it, and the answer carries pi. The same happens at `s = 4`, where the answer is pi to the fourth over 90.

The odd powers do not behave. `zeta(3)` is 1.2020569 and a bit, and nobody has ever written it in terms of pi or anything else familiar. It is known not to be a fraction, and that is about as much as is known.

Now the part that ties zeta to the primes. Take one prime `p` and form the sum `1 + 1/p^s + 1/p^(2s) + ...`, which is a geometric series and adds to `1/(1 - p^(-s))`. Do that for every prime and multiply all of those sums together.

$$\zeta(s) = \prod_{p \text{ prime}} \frac{1}{1 - p^{-s}}$$

Multiplying out that product means choosing one term from each bracket, which means choosing a power of each prime, which means building a whole number. Every whole number gets built, and because a number factors into primes in exactly one way, every whole number gets built exactly once. So the product over the primes and the sum over the numbers are the same thing. Unique factorisation, written as an equation.

That is the bridge, and it runs both ways. Anything you learn about the sum, which knows only about counting, becomes a statement about the primes. The first prize won this way was the fact that zeta blows up at `s = 1`, which forces the primes to be infinite in number and, pushed harder, says roughly how densely they sit.

To push harder you have to let `s` be a complex number, a point in the plane rather than on a line, and then extend the function past `s = 1` where the sum itself no longer works. That extension is unique, and it has zeros: points where `zeta(s)` is exactly 0.

Some of the zeros are dull and sit at the negative even numbers. The rest all lie in a vertical strip, and every one that has ever been found sits on a single line down the middle of that strip, the line where the real part of `s` is one half. That line is called the critical line. Whether every one of those zeros lies on it is the Riemann hypothesis, and it is unproved.

## In the tree

The zeta walk in [the critical line demo](/demos/zeta/) traces `zeta` along that line and shows the curve passing through the origin once for each zero. [The design zeta note](/research/zeta/) builds the same kind of sum over a design's own numbers instead of all of them, and asks where that function vanishes when the Euler product is no longer available. [The Mobius echo demo](/demos/echo/) hears the zeta zeros as frequencies in a counting function that never mentions them, and [the Mobius meter note](/research/mobius/) is where that measurement is kept honest. The value `zeta(2)` is what turns a count of coprime points into pi in [pi out of the stack](/research/pi/), and [the Farey stack note](/research/farey/) states the old equivalence between how evenly fractions spread and where those zeros lie.
