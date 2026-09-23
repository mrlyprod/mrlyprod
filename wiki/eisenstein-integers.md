---
title: The Eisenstein integers
lead: Numbers built from a cube root of one, drawn as a hexagonal lattice, with six units and a size rule that turns the honeycomb into a number system.
prerequisites: gaussian-integers
---

There are three numbers whose cube is 1. One of them is 1 itself. Call either of the other two `omega`. It satisfies `omega^2 + omega + 1 = 0`, which is the only fact about it we will use, and drawn in the plane it sits a third of the way round the unit circle from 1.

An Eisenstein integer is anything of the form `a + b omega` with `a` and `b` ordinary whole numbers. Add two of them and the coordinates add. Multiply two of them and the rule `omega^2 = -1 - omega` folds the answer back into the same form. So these numbers are closed under both operations, exactly as the whole numbers are, and you can do arithmetic in them.

Plot them and you do not get a square grid. Because `omega` sits at a third of a turn, the point `a + b omega` lands on a triangular mesh: every point has six nearest neighbours at equal distance, and the cells between them are equilateral triangles. This is the honeycomb lattice, and the figure draws it out to eight steps from the middle, which is 217 points, with the short bonds between neighbours drawn faintly so the six-fold pattern shows.

Every point has a size, its norm, which is the square of its distance from the origin: the norm of `a + b omega` is `a^2 - a b + b^2`. The minus sign is there because the two directions are at 120 degrees rather than at a right angle. The norm is always a whole number, never negative, and the one property that matters is that it multiplies: the norm of a product is the product of the norms.

Six points have norm 1, and the figure marks them in yellow around the orange origin. They are `1`, `omega`, `omega^2` and the negatives of those three, and they are the six corners of a small hexagon. These are the units, the local version of plus and minus one, and multiplying any point by one of them turns the whole picture by a sixth of a turn. That is where the six-fold symmetry of everything in this world comes from. The square lattice has four units and quarter turns; this one has six units and sixth turns.

A point is prime here when it cannot be written as a product of two points of norm bigger than 1. The norm makes that testable: to split a point you would need two norms multiplying to its norm, so any point whose norm is an ordinary prime cannot split at all.

Ordinary [primes](/wiki/prime-numbers/) behave in three ways when they move into this larger world, and which way is decided by the remainder on division by 3. A prime that leaves remainder 1 breaks in two: `7 = (3 + omega)(3 + omega^2)`, and each factor has norm 7. A prime that leaves remainder 2 stays whole: 2 and 5 cannot be split here at all. The prime 3 itself is the odd one out, because `1 - omega` has norm 3, so 3 is a unit times that point squared, a prime that becomes a square.

Division with remainder works in this lattice, for the plain geometric reason that every point of the plane is within less than one unit of some lattice point. From division with remainder you get a [greatest common divisor](/wiki/greatest-common-divisor/) by the usual repeated-remainder method, and from that you get unique factorisation into primes. So the honeycomb is not just a pretty arrangement of dots. It is a number system with the same backbone as the ordinary whole numbers, and it is the natural home for questions where three-fold symmetry is built in rather than four-fold.

## In the tree

[What base 3 hides](/research/bases/) is the note that needs this ring: base 3 brings three-fold symmetry, a flat lattice with three-fold symmetry has to be the hexagonal one, and the arithmetic of that lattice is this. [The primes in the plane demo](/demos/gaussian/) draws both rings side by side with each point coloured by how it splits. [The Gaussian integers](/wiki/gaussian-integers/) is the square case this one mirrors, and the counting of neighbours by distance is the same sort of question as in [visible lattice points](/wiki/visible-lattice-points/). The cube roots of one are where `1 + x + y` vanishes on the torus, and the sign rule of this ring's primes sets its [Mahler measure](/wiki/mahler-measure/). The Gosper curve among the [space-filling curves](/wiki/space-filling-curve/) walks this lattice in sixths and thirds of a turn.
