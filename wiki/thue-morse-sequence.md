---
title: The Thue-Morse sequence
lead: A string of noughts and ones built by writing a block and then its opposite for ever, which is also the parity of the ones in each place number written in binary.
prerequisites: parity
---

Start with a single 0. Write it down, then write the opposite of everything you have so far, which is 1, giving 01. Do it again: the opposite of 01 is 10, so you now have 0110. Again: the opposite of 0110 is 1001, so you have 01101001. Each round doubles the length and never changes a letter already written, so the string settles down to one infinite sequence: 0 1 1 0 1 0 0 1 1 0 0 1 0 1 1 0 and on.

There is a second way to get the same letters, one place at a time, with no history at all. Take the place number, write it in binary, count the 1s, and the letter is 0 when that count is even and 1 when it is odd. Place 0 is 0 in binary, no ones, even, so the letter is 0. Place 3 is 11, two ones, even, so the letter is 0. Place 4 is 100, one 1, odd, so the letter is 1. Check those against the string above and they agree.

The two recipes agree because doubling a place number in binary just adds a 0 on the end, which does not change the count of 1s, while doubling and adding one puts a 1 on the end, which flips the parity. So the letter at place `2n` is the letter at place `n`, and the letter at place `2n + 1` is its opposite. That is exactly the copy-and-flip rule, read forwards.

The strip along the top of the figure is the first 64 letters, one cell each, orange for 0 and blue for 1. Count them and there are 32 of each, which holds for the first `2^k` letters at every `k`, because the rounds pair every letter with its opposite.

The sequence never says the same thing three times in a row. Whatever block you choose, that block repeated three times immediately does not occur anywhere in the string, and neither does any block followed by itself and then its own first letter. This is the reason the sequence was first written down: it settles the question of whether an endless string over two letters can avoid such repetition, and it does so with an explicit answer rather than a proof that one must exist.

It is also the fair way to take turns. Two players alternating by 0 1 0 1 gives the first player every early advantage. Sharing by 0 1 1 0 1 0 0 1 instead gives the second player the second pick, then the first player two in a row, and so on, so the running totals stay level far longer. That is why the pattern turns up in draw rules and in schedules.

The bottom panel of the figure is the sequence lifted to the plane. The cell in row `i` and column `j` is orange when the letters at places `i` and `j` agree and blue when they differ. That is one sequence read twice, once down and once across, and combined by the same exclusive-or that parity uses. The panel is 32 cells on a side, so it holds 1024 cells, half of each colour.

The plane picture is not a plain check pattern, and it is not random either. Blocks of 2 by 2, 4 by 4 and 8 by 8 repeat at every scale, some plain and some flipped, because the row and column rules both halve in the same way. Stand back and the same texture appears at each size, which is what self-similar means in practice.

## In the tree

[The Thue-Morse demo](/demos/morse/) builds the word twice, once by the doubling rule and once by the digit rule, and lifts it to the plane as the figure does, with its run lengths beside it. A design that changes with the scale is the subject of [magic words](/research/magic/), where this word names the order the letters are taken in, and [the words demo](/demos/words/) builds such a word letter by letter. The single bit being read is the one [parity](/wiki/parity/) sets out, and [the Mobius function](/wiki/mobius-function/) is another string of signs read off a count, there of prime factors rather than binary digits.
