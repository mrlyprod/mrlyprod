---
title: Goldbach's conjecture
lead: Every even number past two is a sum of two primes. Machines have checked it further than anyone can list and nobody has proved it.
prerequisites: prime-numbers, prime-counting-function
---

Take an even number and try to write it as one prime plus another. Four is 2 plus 2. Six is 3 plus 3. Eight is 3 plus 5. Ten is 3 plus 7, and also 5 plus 5. Twelve is 5 plus 7. The conjecture is that this never fails: every even number from four upward is a sum of two primes.

Count the ways and you get a function. Write `g(2n)` for the number of ways to write the even number `2n` as a sum of two primes, counting a pair once and not twice, so 3 plus 7 and 7 plus 3 are the same way. Then `g(4)` is 1, `g(10)` is 2, `g(100)` is 6 and `g(400)` is 14. The conjecture, in this shape, is the single sentence that `g(2n)` is never zero.

The figure is that count drawn as 199 bars, one per even number from 4 to 400. The bars start at height one, climb, and spread out into a widening spray. This picture is called the Goldbach comet, because at larger ranges the spray takes on a head and a tail. What matters here is that no bar has height zero.

The comet has stripes, and the figure paints them. A bar is yellow when its even number is a multiple of six and blue otherwise, and the yellow bars ride roughly twice as high. The tallest bar in the figure stands on 390, a multiple of six, with 27 different pairs.

The reason is the number three. Suppose the even number is a multiple of six. Every prime except three leaves a remainder of one or two when divided by three, and so does the partner it needs, so three never spoils a candidate pair. Now suppose the even number is not a multiple of three. Then about half the candidate primes leave a partner that three divides, and a number three divides is not prime. Half the candidates die, so the bar is about half as tall.

The counts grow because there are more primes to work with, but they grow with a lot of noise, and the noise is the same noise that makes the prime count jumpy. The smallest count in the figure is one, and it is reached at the small even numbers, four and six and eight and twelve. Past that the floor lifts away from zero and never comes back to it, in this range.

Here is the thing that a table cannot do. A table shows that the count is positive at every even number in it. The conjecture says the count is positive at every even number there is. No finite table can reach that, because a pattern can hold for a long stretch and then break: Euler's rule `4k^2 - 2k + 41` hands you a prime for its first twenty one values and then stops, and the [Ulam spiral](/wiki/ulam-spiral/) page draws exactly that line breaking. A checked range is evidence about the checked range and nothing more.

The machine check has been pushed past a billion billion even numbers, which is far enough to make almost anyone believe it and not far enough to be a proof. Statements like this one do get proved, but only when someone finds a reason rather than a table, and no reason has been found.

What has been proved is nearby and weaker. Every odd number past five is a sum of three primes. Every large even number is a prime plus a number with at most two prime factors. Both results come from sieve methods that can push a count above zero on average without ever pinning one particular even number down, which is exactly the gap that remains.

## In the tree

[The famous formulas hub](/wiki/famous-formulas/) sets this count beside seven other systems; the figure above stops at 400 and claims nothing past it. [The primes demo](/demos/primes/) sieves the primes the pairs are drawn from, and [the Ulam spiral demo](/demos/ulam/) shows Euler's quadratic running out of primes at its twenty second value. The raw material is [prime numbers](/wiki/prime-numbers/) and the staircase that counts them is [the prime counting function](/wiki/prime-counting-function/).
