# The zeros, at home

Anthropic published a striking result this year: a research version of
their Claude model, given two long sessions and free rein of a coding
harness, improved a classical bound in analytic number theory - the
proportion of the Riemann zeta function's nontrivial zeros known to sit
on the critical line rose from 41.6% to 67.2% in its hands. The work
combined a 2024-line of research on zero-density with an older
quadratic-forms idea, was checked by two staff mathematicians, reviewed
favourably by outside experts, and accompanied by a machine-checkable
Lean proof. The write-up is at
[anthropic.com/research/riemann-zeta](https://www.anthropic.com/research/riemann-zeta).

This post is not that. Nothing here advances the Riemann Hypothesis by
a millimetre, and it would be a disservice to pretend otherwise. But
the news was a good excuse to do something this project has never quite
done: stop orbiting the zeta function and actually go look at the
zeros, with our own tools, at our own scale.

## What this project already owned

The honest inventory, all of it re-run in public before:

- The values at the integers. The stacked coprime lattice puts
  `1/zeta(d)` on the table as a density you can count: the exact Mobius
  tally agrees with `1/zeta(d)` to a few parts in ten thousand at
  d = 2, 3, 4. The same function the Hypothesis is about, evaluated
  where everything is long understood.
- A clean negative. The Hilbert-Polya dream says the zeros are the
  spectrum of some operator, and the measured fact about the real zeros
  is that their spacings repel like GUE random-matrix eigenvalues. Our
  fractal Laplacians were tested against exactly that and refused: the
  self-similar construction manufactures degenerate modes, the spectra
  cluster (P(s < 0.5) around 0.45-0.57 where GOE sits at 0.17), and the
  thread was closed as the wrong kind of spectrum.
- The string dichotomy. The lattice lab studies complex dimensions of
  digit fractals - poles marching up vertical lines in the complex
  plane, the fractal-string picture where "lives on a line" is the
  natural state of affairs rather than the million-dollar question.

Values, not zeros; spectra, but the wrong statistics; lines, but the
easy ones. That was the whole estate.

## What we did with the excuse

One new lab script, pure python, no imports beyond the standard
library. Euler-Maclaurin summation for zeta anywhere reasonable, seven
Bernoulli terms deep; the Riemann-Siegel theta to rotate the critical
line real, so a zero of zeta becomes a sign change of a real function
Z(t); a scan and bisection to pin the crossings.

Controls first, house rule. The pipeline must reproduce the integers it
will never be asked about again: `zeta(2)` against `pi^2/6` and
`zeta(3)` against Apery's constant, both to 1e-10. Then the line:

- The first 100 zeros between t = 10 and t = 237, matching the
  published first ten to six decimals.
- The count against the smooth Riemann-von Mangoldt formula: 100 found,
  100.06 predicted. Nothing missing in the window - which is the small,
  classical, numerical sense in which the Hypothesis is "verified"
  here: every zero in reach sits where it should.
- The spacings, unfolded to unit density and set against the two
  reference statistics. KS distance 0.068 to the GUE surmise, 0.349 to
  Poisson. Small gaps at 6.1% where GUE predicts 10.6% and Poisson
  39.3%. Ninety-nine spacings is a modest sample, but it is not
  ambiguous: the zeros repel.

Which lands the nicest sentence of the day: the real zeros do exactly
what our fractals would not. The clustering that closed the
Hilbert-Polya thread for mrly geometry is the mirror image of the
repulsion the genuine article shows on the very first hundred zeros,
measured on a laptop in a third of a second.

## The catch, kept

The first run had a wrong constant: B2 = 6 where the Bernoulli number
is 1/6. One digit-flip of a fraction moved every zero by about a tenth
and invented six phantom crossings - and the spacing statistics,
sampled from those wrong zeros, still looked pleasantly GUE. The
integer controls are what failed loudly and pointed at the tail
correction. The lesson is old but earns its keep: beautiful statistics
survive broken inputs; exact values do not.

## The honest scoreboard

Did anything here move the Hypothesis? No. The Anthropic result lives
in a different weight class: it is analytic number theory, new
inequalities about how much of the zero set provably behaves, produced
and then formally verified. What a small lab can do with that news is
what this one did: rebuild the window, verify the classical facts
inside it, and know precisely where its own objects stand relative to
the real thing - values at the integers, strings on their easy lines,
spectra with the opposite statistics, and now the first hundred zeros
sitting where Riemann said they would.

The `-1` in this page's name is a promise to come back: more zeros,
the pair correlation against Montgomery's conjecture, and the string
dichotomy pushed until it says something sharp. When it does, the
numbers will be in a lab folder first, same as these.
