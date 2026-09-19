---
title: The spectral radius
lead: Multiply an arrow by the same matrix again and again and it settles on one direction; the factor it stretches by there is the spectral radius.
prerequisites: transfer-matrix
---

A matrix takes an arrow and gives back another arrow. Write the arrow as a pair of numbers, the matrix as a square block of numbers, and the rule is the usual one: each row of the matrix pairs off against the arrow and gives one number of the answer. The arrow generally comes back pointing somewhere else and with a different length.

The figure does it eight times with one matrix. The matrix is `[[2, 1], [1, 3]]`, four positive numbers. The first arrow points down and to the right. Multiply, and you get the second arrow. Multiply again for the third, and so on. Each arrow has been cut back to the same length before it was drawn, so the picture shows only the turning and not the growth.

The turning stops. The arrows swing round and crowd together near one direction, the dim ray drawn across the figure, and after that they barely move at all. Every arrow you start with ends up there, apart from a few unlucky ones, and starting further away only means the swing takes a little longer.

That settled direction has a name. An arrow that the matrix does not turn at all, but only stretches, is an eigenvector of the matrix, and the number it is stretched by is the eigenvalue. The dim ray in the figure is an eigenvector: multiply an arrow lying along it and it stays along it, longer by a factor of about 3.618 each time.

A two by two matrix has two such directions, with a stretch for each. Any starting arrow is a mixture of the two. Multiplying multiplies each part by its own factor, so the part belonging to the bigger factor pulls ahead, doubling its lead over the other part at every step. After enough steps the mixture is almost entirely that one part, which is why the arrows in the figure settle where they do. The bigger stretch wins simply by growing faster.

The largest of these stretching factors, measured without regard to sign, is the spectral radius of the matrix. It is the growth rate of the whole process. Lengths eventually multiply by it every step, so after `n` steps an arrow is roughly its own starting size times the spectral radius to the power `n`.

Here is the useful case. Suppose the matrix has no negative entries anywhere, which is what any table of counts looks like: a count is never below zero. Perron and Frobenius proved that such a table behaves in the tidiest possible way. There is one largest stretch, it is a real number and not a pair of them, it is positive, and the direction it belongs to can be drawn with all its coordinates positive too. No other direction stretches as much.

The last part matters as much as the first. A positive arrow settling on a positive direction is what lets you say where the counting ends up as well as how fast it grows. The matrix `[[2, 1], [1, 3]]` in the figure has all four entries positive, its top stretch is 3.618, and the direction it settles on points up and to the right into the positive quarter, exactly as the theorem says.

Put that together with the previous page. A [transfer matrix](/wiki/transfer-matrix/) is a table of non-negative counts, so it has one clean top eigenvalue, and the counts of walks it produces grow like that eigenvalue to the power of the length. The whole growth rate of a counting problem is one number sitting inside one table, and it can be computed without ever listing a single walk.

Nothing here derives the number. Finding an eigenvalue exactly means solving a polynomial whose degree is the size of the matrix, which is hopeless by hand past a few states and easy for a machine. The name and the guarantee are what a reader needs; a computer supplies the digits.

## In the tree

[The memory dial demo](/demos/memory/) reads the growth rate of a digit rule straight off its table and prints it beside the plain doubling it replaces. [Beneath a design](/research/beneath/) treats that number as the dial, showing which rules cost nothing and which pay for their memory, and [the cuts note](/research/cuts/) gets the dimension of a diagonal slice as the logarithm of the top root of an integer matrix. [The circle on a design](/research/crop/) brackets one of these roots by hand with certificates rather than trusting a float.
