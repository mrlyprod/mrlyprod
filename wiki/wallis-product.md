---
title: The Wallis product
lead: Multiply 4/3 by 16/15 by 36/35 and keep going, and the running product climbs forever without ever reaching half of pi.
prerequisites:
---

The figure is forty bars. Bar `n` is the product of the first `n` factors, and the dashed line near the top is the number those products are climbing towards, half of pi. The first bar is short, the fortieth almost touches the line, and no bar ever crosses it.

The factors are built out of the even numbers. Factor `k` is `4k^2 / (4k^2 - 1)`, the square of the `k`th even number over that same square less one. The first four are 4/3, 16/15, 36/35 and 64/63.

Every factor is bigger than 1, so the running product only ever climbs. Every factor is also close to 1, and closer the further out you go: by the time `k` is 40 the factor is 6400/6399. The climb never stops, and it slows down fast.

Since `4k^2 - 1 = (2k - 1)(2k + 1)`, factor `k` is `(2k)(2k) / ((2k - 1)(2k + 1))`. Splitting each factor into its two halves gives the form the product is usually written in: 2/1 times 2/3 times 4/3 times 4/5 times 6/5 times 6/7 and on. Every even number is used twice on top and every odd number twice underneath.

Half of pi is 1.5707963. After forty factors the product is 1.5611, short by about 0.0097. After a thousand factors it is 1.5704, short by about 0.00039. The gap shrinks like one over four times the number of factors, so every extra decimal place costs about ten times as many factors. The product is a true statement about pi and a poor way to compute it.

Nothing in the recipe mentions a circle. The factors are ratios of whole numbers, the rule for making them is arithmetic, and pi arrives at the end anyway. That is the whole reason the product is famous.

Here is one place to see where the pi sits. For any whole number `n` above 1, `1 - 1/n^2 = (n - 1)(n + 1)/n^2`. Multiply those from `n = 2` up to `n = N` and almost everything cancels: the numerators of one factor eat the denominators of the next, and what is left is `(N + 1)/(2N)`, which settles at exactly 1/2.

Now split that product in two. The even `n`, which are `n = 2k`, give the factors `1 - 1/(4k^2)`, and those are the Wallis factors upside down, so they multiply to `2/pi`. The odd `n`, which are 3, 5, 7 and on, must therefore multiply to `(1/2)` divided by `(2/pi)`, which is `pi/4`. The same product, read over the odd numbers instead of the even ones, is the area a square keeps when you punch holes in it forever.

## In the tree

The odd half of the product is the area of [the Wallis sieve](/wiki/wallis-sieve/), and [the pi note](/research/pi/) uses that sieve to explain why no single fixed design can hold pi: one design has one ratio, and pi needs a product of changing ones. The move the sieve makes, a different tile folded in at every scale, is the one [the words demo](/demos/words/) lets you build by hand, and [the famous formulas](/wiki/famous-formulas/) race this product against seven other rules of the same kind.
