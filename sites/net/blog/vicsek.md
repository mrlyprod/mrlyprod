# The verification catch

A short story about a result that was right, a program that was wrong, and
why the second reader exists.

## The draft

The Vicsek fractal is the plus sign that keeps its promise: start with one
cell, replace it with the plus pentomino, replace every cell of that with the
plus pentomino, forever. At level `n` it is a polyomino of `5^n` cells on a
`3^n x 3^n` grid, and its perimeter - the unit edges separating a filled cell
from an empty one or from the exterior - came out to

```
4, 12, 52, 252, 1252, 6252, 31252, ...
```

with the closed form `a(n) = 2*5^n + 2`. The proof is a gluing argument, and
it is a nice one. The level-`n+1` figure is five disjoint copies of the
level-`n` figure placed in a plus. Only four pairs of copies touch - the
centre against each arm - and each touching pair shares exactly one unit
edge, because the border line of a copy holds exactly one filled cell, forced
to the centre by the base-3 digits. Every shared edge was counted once in
each copy, so gluing removes exactly 8: `a(n+1) = 5*a(n) - 8`, and solving
from `a(0) = 4` gives the closed form. Proved - and the deficit was measured
at 8 on every level the grid audit reaches, just to be sure.

The draft did everything a careful draft should. Two independent generators
sharing no code - an explicit grid build and a digit automaton - agreeing on
201 terms. Three recurrences and a generating function checked against all of
them. The OEIS searched, live and against a local dump: no hits for the
sequence or its shifts. A new sequence, proved and verified, ready to submit.

## The catch

Then the verification pass, whose job is to re-run everything with fresh eyes
and no shared code, found two things.

First, the draft's own demonstration snippet - the little program block that
shows a reader how to generate the terms - was off by one. It seeded the
Kronecker chain with the 3 x 3 plus tile, which is already the level-1
figure, and then applied `n` more steps. Run as printed it emits
`4, 52, 252, 1252, ...`: the level-`n+1` perimeter for every `n >= 1`, with
`a(1) = 12` never appearing at all. Every claimed number in the draft was
right; the code shown to justify them, run as shown, produced different
ones. The fix is one seed - start from the 1 x 1 all-true matrix - and both
the catch and the fix are recorded in the sequence's run log.

Second, and better: the terms were hiding a known face. Divide by 2 and the
sequence reads `2, 6, 26, 126, 626, ...`, which is `5^n + 1` exactly - OEIS
A034474. The identity is one line, `2*5^n + 2 = 2*(5^n + 1)`, and it had been
sitting inside the closed form the whole time. The doubled sequence really is
absent from the OEIS, so the novelty checks were correct and this is a
crossref, not a duplicate - but it changes the story. What the draft framed
as a new sequence is a known sequence wearing a factor of two, and the honest
line is `a(n) = 2*A034474(n)`, now recorded in the sequence ledger.

## What verification is for

Notice what the catches were not. Not one of the 201 terms was wrong. The
closed form, the recurrences, the generating function, the proof: all
correct. A checker that only recomputes numbers would have passed the draft
untouched.

What verification caught was the relationship between the numbers and their
story - code that claimed to generate data it did not generate, and novelty
that sat one factor of 2 from the known. Both catches made the result more
true, not less: the snippet now runs, and the crossref now points somewhere.
Neither could have been found by trusting the draft's own checks, however
honestly they were run. That is the whole argument for the second reader.
Not that the first one is careless, but that some mistakes are only visible
from outside the story. The perimeter still equals `2*5^n + 2`. It just took
two readers before the page deserved to say so.
