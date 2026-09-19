---
title: The greatest common divisor
lead: The largest number dividing two others at once; Euclid finds it by taking remainders, and when it comes out 1 the two numbers share nothing.
prerequisites:
---

A divisor of a number is a number that goes into it exactly. The divisors of 24 are 1, 2, 3, 4, 6, 8, 12 and 24. The divisors of 36 are 1, 2, 3, 4, 6, 9, 12, 18 and 36. Six numbers appear on both lists, and the largest of them is 12, so the greatest common divisor of 24 and 36 is 12.

There is always one. The number 1 divides everything, so the shared list is never empty, and no divisor of 24 is bigger than 24, so the list is finite and has a largest member.

Listing all the divisors is a bad way to find it. Euclid's way is to take remainders and never factor anything. Divide the larger by the smaller, keep the remainder, and repeat with the smaller and the remainder until the remainder is 0. The last number before the 0 is the answer.

Take 252 and 198. Divide 252 by 198 and the remainder is 54. Divide 198 by 54 and the remainder is 36. Divide 54 by 36 and the remainder is 18. Divide 36 by 18 and the remainder is 0. The answer is 18, found in four steps, and no factorisation of either number was needed.

It works because a number dividing both 252 and 198 divides the difference too, and the remainder is just a pile of differences. So the pair (252, 198) and the pair (198, 54) have exactly the same common divisors, and every step shrinks the numbers while leaving the answer untouched. The numbers shrink fast, so this stays quick on numbers with hundreds of digits.

Two numbers are coprime when their greatest common divisor is 1. They need not be prime themselves. 8 and 15 are coprime, because 8 is built from 2s and 15 from a 3 and a 5, and the two builds have no part in common.

Coprimality is what lowest terms means. The fraction 252 over 198 reduces by 18 to 14 over 11, and 14 and 11 are coprime, so it will not reduce again. Every fraction has exactly one lowest-terms form, and the greatest common divisor is what gets you there in one step.

The figure is a 24 by 24 board. The cell in column `a` and row `b` stands for the pair of numbers `(a, b)`, with `a` running 1 to 24 left to right and `b` running 1 to 24 bottom to top. A cell is lit when `a` and `b` are coprime, and otherwise shaded by how much they share, faint for a small divisor and bright blue for a large one.

Three things read straight off it. The bottom row and the left column are fully lit, because everything is coprime to 1. The diagonal is the brightest line on the board, because a number shares all of itself with itself. And the shaded cells fall into slanted families, the darkest and commonest being the even column and even row crossings, where the pair shares a 2.

Count the lit cells and there are 359 of 576, a little under two thirds. That ratio is not an accident of the window. Pick two whole numbers at random and the chance that they are coprime settles at 6 divided by pi squared, which is 0.6079 and a bit. Why a circle constant should decide a question about divisors is a longer story and it is told elsewhere in the tree.

## In the tree

That density is the whole point of [pi out of the stack](/research/pi/), which counts the lit points of this same grid and hands back pi from the count, and it is the question [the coprimality spine](/research/coprime/) asks again with the grid replaced by a design. [What base 3 hides](/research/bases/) asks it on the hexagonal lattice instead of the square one, where the shared-divisor rule is the same and the constant that falls out is not. The lit cells are also the fractions in lowest terms, which is exactly the list drawn by [the Farey sequence](/wiki/farey-sequence/). Seen from the corner of a grid, those same cells are [the visible lattice points](/wiki/visible-lattice-points/).
