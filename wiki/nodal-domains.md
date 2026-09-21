---
title: Nodal domains
lead: A drum skin at one pure tone splits into patches that swing up while their neighbours swing down; the still lines between them are the nodal set, the patches are the nodal domains, and Courant's theorem says the k-th tone has at most k of them.
prerequisites: graph-laplacian
---

Sprinkle sand on a metal plate and draw a violin bow along its edge. The plate rings at one note, the sand hops off every part that shakes and gathers on the lines that stay still. Chladni published the trick in 1787 in a book on the theory of sound, and the patterns are Chladni figures.

The mathematics is a membrane vibrating at one pure tone. Its shape is an eigenfunction `u` of the Laplacian, `-Delta u = lambda u`, with `u = 0` on the clamped rim, and the tone is `lambda`. The nodal set is where `u = 0`, the sand lines. The nodal domains are the connected pieces of the rest, each swinging wholly up or wholly down, every neighbour in the opposite phase.

The figure is four eigenfunctions of the unit square as sign maps: blue where `u > 0`, orange where `u < 0`, the nodal lines left in the ground. The eigenfunctions of the square are `sin(pi m x) sin(pi n y)` with tone `(m^2 + n^2) pi^2`. Top left is `(1, 1)`, the fundamental, one domain. Top right is `(2, 1)`, tone `5 pi^2`, two domains split by the line `x = 1/2`. Bottom left is `(2, 2)`, tone `8 pi^2`, four domains. Bottom right is tone `10 pi^2`, which belongs to both `(1, 3)` and `(3, 1)`, so their sum is an eigenfunction too; its nodal line is the closed curve `sin^2(pi x) + sin^2(pi y) = 3/2`, and it has two domains.

Courant's theorem: list the tones in increasing order, repeating each as many times as it has independent eigenfunctions; an eigenfunction of the `k`-th tone has at most `k` nodal domains. On a string the `k`-th mode has exactly `k` pieces; on a membrane `k` is only a ceiling. The four panels are tones 1, 2, 4 and 5 of the square and carry 1, 2, 4 and 2 domains: three touch the ceiling and the fourth falls under it.

Pleijel proved in 1956 that in the plane the ceiling is touched only finitely often. With `nu_k` the number of nodal domains of the `k`-th eigenfunction of a clamped plane membrane,

$$\limsup_{k \to \infty} \frac{\nu_k}{k} \le \left(\frac{2}{j_0}\right)^2 = 0.691...,$$

where `j_0 = 2.4048...` is the first zero of the Bessel function `J_0`. The proof applies the Faber-Krahn inequality, that a disc has the lowest fundamental tone of any domain of its area, to every nodal domain at once. For the square the ceiling is touched exactly at tones 1, 2 and 4, as Pleijel claimed and Berard and Helffer completed; the same paper proves Stern's claim of 1924 that the square has eigenfunctions with only two nodal domains at tones as high as you like.

A graph has nodal domains too. An eigenvector of the [graph Laplacian](/wiki/graph-laplacian/) gives a number to every dot; a strong nodal domain is a maximal connected set of dots on which the numbers are all positive or all negative, and a weak one lets zeros in. Davies, Gladwell, Leydold and Stadler proved the discrete theorem in Linear Algebra and its Applications 336: an eigenvector of the `k`-th eigenvalue, of multiplicity `r`, has at most `k + r - 1` strong nodal domains and at most `k` weak ones.

Courant's statement is quoted from that paper, read in the arXiv preprint math/0009120; Pleijel's bound from Polterovich, arXiv:0805.1553; the square's cases from Berard and Helffer, arXiv:1402.6054; Chladni's plate from the Wikipedia article on him.

## In the tree

[The automata note](/research/automata/) counts the strong nodal domains of the carpet's Laplacian eigenvectors against Courant's ceiling.
