# Life Atlas

- Runs the mini preset of `mrlylab::atlas` and reads every settled frame as a design: the fates, the per-frame readings, and the Kronecker block test of [code-factorisation](../code-factorisation/README.md) on every settled frame and on every run's heatmap level set.
- The mini preset: the five filled side-3 classes of the plane at level 1, tessellations 1 and 3, the Moore mask, the side-3 class masks, the seed board copied and copied-inverted with the centre popped, the fifteen sequences with the zero and one rolls off, `B3/S23`, the wrap boundary, canvas 27, 64 generations; masks equal cell for cell at one seed and tessellation run once.
- The five seed classes, printed as rows: 1 is the four corners `#.#/.../#.#`, 3 is the two-bar tile `###/.../###` of rows 0 and 2, 6 is the diamond `.#./#.#/.#.`, 7 is the carpet tile `###/#.#/###`, 15 is the full square; a class mask pops the centre, so `d6s3l1` is the cross mask, `d1s3l1` the four diagonals, `d3s3l1` the two bars, and `d15s3l1` repeats the Moore mask and is dropped.
- A reading is fill, 4-adjacent components, complement components, Euler characteristic, the dyadic box slope, the dihedral subgroup, the strongest non-zero frequency over the full Fourier square with its ring share, the ring the corner-dropping ring cut reads instead, entropy, churn, every proper divisor of the side the block test cuts at, and the smallest cut's two factors as row-major codes.
- The spectrum of a 27-torus is an exact separable transform, since the crate FFT wants a power of two; the mask lobe is the first ring where the ring mean of the mask's signed transform on the canvas turns negative.
- The block test cuts the 27-canvas at `d = 3` and `d = 9` only, its two proper divisors, and reads every one of the 729 torus shifts of every frame at both cuts.
- A cut is classed by its outer factor: confined to one block of the cut, a full tiling, the board's own footprint block up to a torus shift, or proper; a frame is proper when some shift and some cut is proper, and the census prints the reading in place beside the reading after shifts.
- Every rule reads its counts at the mask's own budget, the largest neighbour count the mask can reach, so the budget of every mask cell is printed beside the fate table.
- The lemma the study checks frame by frame: when every mask offset lies in `3Z^2` and zero is outside birth at that run's own budget, a frame `A (x) B` steps to `A' (x) B`, with `A'` one step of the same rule on the 9-torus under the mask divided by 3, because the count at a cell at position `p` of its tile is `B(p)` times the quotient count and an empty block sees count zero.
- The self-check pins the blinker's period 2 under `B3/S23`, the level-2 carpet's factorisation `(3, 495, 495)` and the agreement of the shift split with `block_split` at shift `(0, 0)`; the study exits non-zero when any of them fails or the lemma fails on any run.
- The odd rule `B1357/S1357` is not in the mini domain, since the one roll is off; its replication is Sloane's odd-rule theorem ([REFS](../../REFS.md)), cited and not run.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p life-atlas`
- About twenty seconds; prints only, writes nothing.

## WITNESSES

- The study is the generator behind the "The atlas" section of [automata.md](../../automata.md); every number below is one the run prints, and no row of that section is quoted here.
- The cut: 1120 nominal runs, 272 dropped as duplicate masks, 848 run, none sampled.
- The fate table: number rules `106 dead, 24 still, 56 loop, 132 timeout`, design rules `448, 2, 0, 27`, `B3/S23` `31, 17, 4, 1`, all `585, 43, 60, 160`; 160 timeouts, no mover among them.
- The budgets: 53 mask cells, 45 of budget 8 or less and 8 above it at budgets 26 to 80, every one of the 8 a tessellation-3 copy mask; a reading of the rules at budget 8 covers the 45 and never the 8.
- The closure in place: 103 settled frames, 34 distinct up to the dihedral group and torus translation; 23 no cut, 72 confined, 4 full tilings, 1 the board footprint, 3 proper, all three on the index-9 copied-inverted mask of the carpet tile.
- The closure after every torus shift: 22 no cut, 67 confined, 4 full tilings, 1 the board footprint, 9 proper; per mask index the frames with a proper cut after some shift are 6 of 48 at index 1, 0 of 51 at index 2, 0 of 1 at index 3 and 3 of 3 at index 9.
- The six index-1 witnesses, all at `d = 9` and all on tessellation 3: the 16-cell still of the corner seed, four 2 by 2 blocks at spacing 3, outer fill 4 and inner tile 432 after shift `(1, 1)`, reached on the Moore mask under `odds`, `tree_voids` and `B3/S23` and on the two-bar and cross masks under `B3/S23`; and the 36-cell still of the two-bar seed under the cross mask and `B3/S23`, outer fill 6 and inner tile 504 after shift `(1, 0)`.
- The heat table in place: 848 heat frames, 249 no cut, 402 confined, 0 tilings, 188 the board footprint, 9 proper, 8 of them lemma runs and 1 on an index-1 mask.
- The heat table after every torus shift: 249 no cut, 401 confined, 0 tilings, 188 the board footprint, 10 proper; the one frame the shifts add is the index-1 heat frame of the two-bar seed under the cross mask and `B3/S23`, outer fill 6 and inner tile 504 at `d = 9` after shift `(1, 0)`.
- The lemma: verified frame by frame on 16 runs, all 16 rules, mask `7s3 copyinv` at budget 8, the only mask cell whose offsets all lie in `3Z^2`.
- The stills: 21 stills from tessellated seeds, 5 with the peak ring on the seed comb, 3 of them the four-cell frame of share 0.06 on the four-diagonal mask and 2 of them copied-inverted masks; every Moore still peaks at ring 1, 4 or 6 against comb 9 and lobe 9; one still of share at least one half, the 486-cell tiling of the two-bar seed under its copied-inverted mask at share 1.00.
- `mrlylab::atlas::readings::block_split`, `mrlylab::atlas::readings::first_negative_lobe`, `mrlylab::atlas::run::heat_frame`, `mrlylab::atlas::run::census`, the crate paths behind every number.
