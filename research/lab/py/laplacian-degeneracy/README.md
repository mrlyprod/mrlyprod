# laplacian-degeneracy

- Builds the level-`level` Sierpinski triangle as the design-code-7 base-2 tile Kronecker-substituted into itself, `level = 1..8`, and checks the array against the `i AND j = 0` characterisation instead of assuming it.
- Takes the nearest-neighbour graph on the filled cells, checks it is one component, and diagonalises the normalised Laplacian `I - D^-1/2 A D^-1/2` densely, with `A` the adjacency matrix and `D` the degree matrix.
- Clusters the sorted spectrum by consecutive gaps above `1e-9` and reports, per level, the distinct and degenerate class counts, the repeated fraction, and the multiplicities of eigenvalue 1 and of `1 -/+ sqrt(30)/6`.
- Tracks the third family at `1 -/+ 0.988332421566`, the two Fibonacci counting fits, the isolation of the class at eigenvalue 1, the widest degenerate class, and the tolerance sweep at `level = 8`.
- Domain: `level = 1..8`, the same range the page states; `level = 9` is not attempted and the program prints why.

## RUN

`uv run python research/lab/py/laplacian-degeneracy/degeneracy.py`

Needs numpy and scipy. One pass, about 35 seconds, exits 0.

## WITNESSES

- complexity.md:371 `| 1 | 3 | 3 | 0 | 0.0000 | 1 | 0 |` through complexity.md:378 `| 8 | 6561 | 1975 | 465 | 0.7699 | 2187 | 244 |`, all eight table rows
- complexity.md:388 `4 * 10^-15`, the worst deviation from `1 -/+ sqrt(30)/6` over `level = 2..8`, measured as `3.61e-15`
- complexity.md:393 `1, 3, 9, 27, 81, 243, 729, 2187`, the multiplicity of eigenvalue 1 against `3^(level-1)`
- complexity.md:397 `2, 4, 10, 28, 82, 244`, the second family against `3^(level-3) + 1`
- complexity.md:400 `1 -/+ 0.988332421566` and complexity.md:401 `2, 4, 10, 28, 82`, the third family against `3^(level-4) + 1`
- complexity.md:402 `Six distinct classes at level 8 carry multiplicity 82`
- complexity.md:406 `2*Fibonacci(2 level) + 1` and complexity.md:407 `2*Fibonacci(2 level - 3) - 1`, fitting eight and seven levels
- complexity.md:417 `5.4 * 10^-14`, the widest degenerate class at `level = 8`, measured as `5.42e-14`
- complexity.md:418 `isolated by 0.442 on both sides`, the neighbours being `0.5578103942` and `1.4421896058`
- complexity.md:419 `flat across seven decades of clustering tolerance`, the sweep from `1e-12` to `1e-5`
- complexity.md:420 `19683 nodes` and complexity.md:421 `3.1 GB dense matrix`
- complexity.md:619 `Laplacian degeneracy at level 8`, the eight-point conjecture
- complexity.md:636 the tolerance sweep and complexity.md:637 the reason level 9 stops
