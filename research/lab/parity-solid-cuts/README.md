# parity-solid-cuts

- Diagonal planar cuts `x + y + z = s` of the base-2 parity solid of code 126, the six corners of popcount 1 or 2.
- Canonical codes by orbit walk over the 48 signed permutations; the level set by digit build and by full cube sweep.
- Support `[2^L - 1, 2^(L+1) - 2]` and the constant slice count `3^L`, again to `L = 14` by a height recursion.
- Set equality of every slice with the gasket scheduled by the binary digits of the height offset, all 126 slices to `L = 6`.
- The two central slices as six disjoint gaskets of `3^(L-1)`, their order-12 symmetry and the coordinate-order split, `L = 2..8`.
- The flat slice against the odd trinomial layer, by Kummer and by integer parity; code 23 slice counts against `3^wt(s)`.
- Slice profiles of codes 63, 105, 111, 126, 127 at `L = 1..4`, the retracted `4(L+5) 3^(L-1)` form, and the exact octahedron conjugation.
- `makefig.py` rebuilds the figure at `L = 7`: six gaskets projected along `(1,1,1)`, one disc per lattice point, injectivity asserted on integers.
- Every domain is the source's own; nothing was shrunk.

## RUN

- `uv run python research/lab/parity-solid-cuts/cuts.py 8`
- `uv run python research/lab/parity-solid-cuts/makefig.py`
- One core, about one second each; the second writes `research/figures/cuts-fig.png`.

## WITNESSES

- `cuts.md:23,362` - the six codes are canonical under the 48 signed permutations; `cuts.md:36` - centred basis determinant `-1/2`, six offsets onto the six octahedron axes.
- `cuts.md:58-61` - support `[2^L - 1, 2^(L+1) - 2]` and `3^L` to a slice at `L = 1..8` by two enumerations, and again to `L = 14` by the height recursion.
- `cuts.md:76-77` - all 126 scheduled slices equal their gasket as sets, `L = 1..6`.
- `cuts.md:98-99` - union sizes `18, 54, 162, 486, 1458, 4374, 13122`; `cuts.md:115-118` - six sectors of `3^(L-1) - 1` plus `6` central points, `L = 2..8`.
- `cuts.md:104-107` - the figure at level 7: `4374` points in six gaskets of `729`, projection injective, byte for byte the committed file.
- `cuts.md:133` - `3, 9, 27, 81, 243` by integer parity; `cuts.md:147` - `A048883` terms `1, 3, 3, 9, 3, 9, 9, 27`.
- `cuts.md:364-370` - the neighbour table at `L = 4`, code 105 non-empty `16 of 31`, code 111 max `111`, code 127 max `162`.
- `cuts.md:378-380` - `24, 84, 288, 972, 3240, 10692` against maxima `3, 12, 45, 162, 594, 2187`, minima `1`, totals `7^L`.
- `README.md:48` - the same union sizes at `L = 2..8`, group order 12, six pieces of `3^(L-1)`.
