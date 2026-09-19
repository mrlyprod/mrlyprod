---
title: Cellular automata
lead: A row of cells, a rule that reads each cell with its two neighbours, and a new row written underneath; run it and the rows draw a picture.
prerequisites: parity
---

Draw a long row of cells and switch each one on or off. That row is the whole state of the world. Everything that happens next is decided by one rule applied to every cell at the same moment.

The rule looks at three cells: the cell itself and its two neighbours, left and right. Three cells that are each on or off make eight possible patterns, from all three off to all three on. The rule has to answer on or off for each of those eight, so a rule is eight answers.

Eight answers, each a 0 or a 1, is a byte, and a byte is a number from 0 to 255. That is how these rules are named. Write the eight answers in order and read them as a binary number and you get the rule's number, so there are exactly 256 rules of this kind and every one of them has a name.

Apply the rule to every cell at once and write the result as a new row under the old one. Do it again and again, and the rows stack into a picture with space running across and time running down. That picture is all there is to see, and it is very different from one rule to the next.

Rule 90 says: switch on exactly when one of your two neighbours is on, but not both. The cell's own state is ignored entirely. The figure runs it from a single live cell for 64 generations and the result is a triangle of triangles, holes inside holes, growing without ever repeating. It is Pascal's triangle read even and odd, the odd entries lit and the even ones left dark, sheared so the rows line up square.

Rule 110 is the famous one. Started from a soup it makes patches of stripes with small structures drifting between them, colliding, and coming out as other structures. Nothing settles and nothing repeats, and it has been shown that with the right starting row it can carry out any computation a computer can. Three cells and eight answers are enough.

![Any of the 256 rules from one live cell: 128 generations, time running down, the rule number on the slider.](demos/wolfram/rule)

The slider above walks all 256 of them. Most do nothing interesting: they die out, or fill the plane, or settle into stripes. A handful draw triangles like rule 90, and a smaller handful never settle at all. That the whole family is finite is the point, because it can be examined one rule at a time rather than argued about.

The eight patterns are worth a second look. Three cells each on or off is exactly three bits, and three bits are the eight corners of a cube. So a rule is a choice of which corners to switch on, which is the same kind of object as the choice of corners that makes a design in [parity](/wiki/parity/). The 256 rules and the 256 three-dimensional designs are the same 256 things read two ways.

Conway's Life is the two-dimensional cousin. The cells sit on a grid instead of a line, the neighbourhood is the eight cells around each one instead of two, and the rule reads only how many of those eight are alive: a dead cell with exactly three live neighbours is born, a live cell with two or three stays alive, everything else dies. Those two clauses are enough to produce blocks that sit still, blinkers that flash, and gliders that walk across the grid for ever.

## In the tree

[The automata](/research/automata/) proves the identity above, that an elementary rule and a three-dimensional design are one object, and follows what the identity does and does not buy. [The wolfram demo](/demos/wolfram/) runs any of the 256 rules beside the design card the same byte fills in, [the life demo](/demos/life/) runs Conway's rule from a soup or a glider, and [mrlylife](/demos/mrlylife/) swaps the neighbourhood for any design the tree draws and runs the rule on that instead.
