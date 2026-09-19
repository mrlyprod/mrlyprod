---
title: The transfer matrix
lead: A table that says which state may follow which; multiply it by itself and it counts the walks of any length, and those counts grow like a fixed power.
prerequisites: graphs
---

Some machines have a small number of states and a rule about which state may follow which. A traffic light may go green, amber, red, green. A word may be built letter by letter with a rule that forbids some pairs. In both cases all you need to write down is a list of allowed steps.

The figure has three states and four allowed steps. From the top state you may stay where you are or move to the right state. From the right state you may only move to the left state. From the left state you may only move to the top. The arrows in the left panel say exactly that, and the loop at the top state is the step that stays put.

The panel beside it says the same thing as a table. There is one row and one column for each state, rows for where you are and columns for where you go. A lit cell means the step is allowed and a dark cell means it is not. Three states give nine cells, four of them lit, one lit cell for each arrow. That table is the transfer matrix.

Now count. How many walks of length `n` are there, counting every starting state and every finishing state? Length 1 is easy, it is the four arrows. Length 2 means an arrow followed by an arrow, and there are six such pairs. The bars along the bottom of the figure are these counts for lengths 1 to 8: 4, 6, 9, 13, 19, 28, 41, 60.

Multiplying the table by itself is what produces them. Take the entry in row `i` and column `j` of the table times itself: it is the sum over every middle state `m` of (steps from `i` to `m`) times (steps from `m` to `j`), which is exactly the count of two-step walks from `i` to `j`. Multiply `n` times and every entry of the result is the count of walks of length `n` between that pair of states. Add up all nine entries and you get the bar.

So one table, multiplied, answers a question about walks of every length at once. That is the whole trick, and it is why the table is worth a name. Nothing in it is special to three states: the same multiplication counts the walks on any [graph](/wiki/graphs/) from its adjacency table.

The counts have a pattern. Each one is the one before it plus the one three before it: `13 = 9 + 4`, `19 = 13 + 6`, `60 = 41 + 19`. That is the three-step loop showing up in the arithmetic, and it means the counts can be continued for ever without ever touching a matrix again.

Look at how fast they grow. Divide each count by the one before it: 1.500, 1.500, 1.444, 1.462, 1.474, 1.464, 1.463. The ratios are settling down, and what they settle on is about 1.4656, the number `r` with `r^3 = r^2 + 1`. After a while the counts are just multiplied by that fixed number again and again, so the count of walks of length `n` is roughly some constant times `r^n`.

Every transfer matrix behaves this way. The counts of walks grow like a power, and the base of that power is a number belonging to the matrix: its largest eigenvalue, also called [the spectral radius](/wiki/spectral-radius/). That page says what the number is and why a table of non-negative counts always has one. Here it is enough to know that a table of allowed steps carries a growth rate inside it, and that reading the rate off the table is easier than counting the walks.

The name comes from physics, where the same table transfers a description of one slice of a system to the next slice. Whenever something is built one step at a time under a local rule, a transfer matrix is usually hiding in it, and the number of things you can build grows like a power whose base is that matrix's own.

## In the tree

A rule on the last few digits of a number is a machine of this kind, and [the memory dial demo](/demos/memory/) prints the accepted words a level and the growth rate that replaces plain doubling. [Beneath a design](/research/beneath/) prices that dial with the transfer matrix of the rule, and [the cuts note](/research/cuts/) counts the cells of a diagonal slice by carrying a small integer from digit to digit, which is the same table under another name. [What a second base costs](/research/cobham/) is the one place where this instrument stops working.
