# hexagonal-slice-census

- Builds the central diagonal section of the odd cube `n = 2k - 1` as a mesh of `6n^2` unit triangles in doubled integer coordinates and counts triangles, edges, boundary, interior edges, vertices and the Euler characteristic.
- Fits the closed forms blind through `k = 1..3`, checks them to `k = 10` and on fresh builds at `k = 12, 16, 20`, proves the adjacency lemma on twenty sub-meshes, and lists the prime vertex counts with their Eisenstein norm witnesses.
- Reads the fill polynomials of the parity designs off their corner sets, walks the 48 signed permutations for the classes, and counts the four family fills on the slice.
- Checks the carpet-net partition triangle by triangle at odd `n = 1..31` and by exact section area at `n = 1..16`, then counts the carpet's components and holes two ways each at `k = 1..14`.
- Counts the base-3 slice census to level 4 with its percolation, the base-5 census to level 2, and the visible and hidden unit faces of the substituted designs by numpy Kronecker powers, fitting the `2x2` face matrix over the rationals.
- Sweeps all 256 designs for total exposure and for the all-even corner rule, counts the grid corners each family touches in 3D and 2D, and measures the normalised-Laplacian spectral exponent on the giant piece of the carpet slice and of the solid sheet.

## RUN

- `uv run python research/lab/hexagonal-slice-census/mesh.py`
- `uv run python research/lab/hexagonal-slice-census/fills.py`
- `uv run python research/lab/hexagonal-slice-census/surface.py`
- One core, under three seconds together.

## WITNESSES

- `slices.md:41-47` - `54, 18, 90, 72, 37` at `n = 3` and the five closed forms; `slices.md:71-75` - the blind fit and the fresh sides `23, 31, 39` with `9126, 13806, 4681, 234`; `slices.md:77-90` - the lemma, `28188`, `6642`, `21546`, and `2880` against `6480`.
- `slices.md:134-168` - `CH(2k)`, `1 mod 3`, primes `7, 37, 271, 397, 547, 919, 1657, 1951, 2269, 4219` at `k = 1, 2, 5, 6, 7, 9, 12, 13, 14, 19`, the ten composites, the six to `k = 40`, `4219 = 37^2 + 37*38 + 38^2`.
- `slices.md:170-358` - fills `42, 12, 18, 12`; classes `023, 023, 003, 024`, checkerboard `105`; `42 + 12` to `3696 + 2070 = 5766`; layers `15/3, 7/12, 15/3`; components `1, 1, 7, 1, 19, ..., 127, 1` and holes `0, 1, 0, 7, ..., 0, 127`; net holes `1, 7, 19, 37`; the face table `20/24/8`, `7/6/1`, `12/8/4`, `9/0/none`; visible `72, 1056, 18048, 336384`, hidden `48, 1344, 29952, 623616`; `M = [[12, 4], [8, 16]]`; second eigenvalues `8, 1, 4`, `21, 4, 9`, `40, 9, 16`; 35 exposed designs in six classes; grid corners `8k^3` for carpet, tree and antipodal against the net's `8(k-1)^2(k+2)`, short by `24k - 16`, the 2D `4k^2 - 4`, and the 128 all-even rules.
- `slices.md:14` and `slices.md:419` are pointer lines; every object above is rebuilt here in one code base, but not the profile identity at `slices.md:92-132` nor the slice ink at `slices.md:360-404`.
- `method.md:255-262` - the four orbit polynomials and `0, 7, 44, 135, 304 = (k-1)^2 (4k-1)`.
- `complexity.md:542-607` - `6, 42, 306, 2250, 16578`, `9a' - 12a''`, `1.8184`; net `12` at every level, tree and antipodal giants `8` and `6`, base 5 level 2 `20` components with giant `192` of `1164`; exponent `0.91, 1.25, 1.44` at the 10 percent window against the solid `1.79`; `complexity.md:538-539` - `16578` nodes and `21546` edges.
- `README.md:52-53` - the two slice bullets, every number above.
