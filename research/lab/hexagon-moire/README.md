# Hexagon Moire

- The stack of diagonal cube slices, one hexagon per odd side n, for the carpet, net, tree and void designs.
- Cut ink of every layer as an exact rational against the four closed forms, and the void centre cell.
- Exact area-weighted Pearson correlations of layer pairs on the full hexagon: the doubling sign law, its Richardson limit, adjacent and gcd-echo pairs, the other families.
- The ideal-frame stack at 3601 samples per axis: quarter-line bands, the one-sided plateau at X = 1/4, the void star.
- The twisted triangle averages and the coarse crosshair law at 200003 samples.
- The ghost star of the rendered carpet stack on a 1200 by 2399 raster, and its log-corrected decay in the ideal and the lattice frame.
- The cell frame of the ghost star: the arm's rational ink law to n = 2001, the decay ladders at L = 0 mod 4, L = 2 mod 4 and odd L out to L = 6400, and the band half-widths with their held-out predictions.
- The constants from the mrlynum series and the partial character sums at N = 53 and 55.
- Every domain is the source's own; nothing was shrunk. Thirty seconds.

## RUN

- `bash scripts/cargo.sh cargo run --release --manifest-path research/lab/Cargo.toml -p hexagon-moire` from `mrlyprod/`

## WITNESSES

- hexagon.md the cut ink of one layer: the ink laws match 28/28 layers in exact rationals for all four designs at every odd n <= 55, and the void centre cell is ink in 28 of 28 against 14 for the other three.
- hexagon.md layer pairs and the doubling law: the sign law holds on all 18 pairs from (3,5) to (601,1201); the full-hexagon blind pairs read (5,9) = -0.14179450 and (5,7) = -0.08542646.
- hexagon.md layer pairs and the doubling law, the magnitude: Richardson r_inf from -0.11715991 to -0.11711630, branches 0.1171270 and 0.1171274, against 253/2160 = 0.11712963 and 19/162 = 0.11728395.
- hexagon.md the other pair families: tree and void doubling at (201,401) +0.00049938 and +0.00107260; tree adjacent -0.07016649, void adjacent +0.00786627; echo (67,201) carpet +0.21473346, tree +0.14984772, void +0.07721358.
- hexagon.md the quarter line and the crosshairs: plateau 0.12210, 0.12340, 0.12412, 0.12446 at N = 151, 301, 601, 1201; the X = 1/4 bands +0.066465 and -0.090881 at N = 55.
- hexagon.md the quarter line and the crosshairs, the coarse model: A = 1/3 excess -0.083726 against -0.083333 at N = 5555; A = 1/7 -0.058065 at N = 55 against -0.036036 at N = 5555.
- hexagon.md the void star: arms 0.513667, 0.486492, 0.491844, 0.491525 and the Z arm 0.399416 over a mean 0.278670.
- hexagon.md the twist that kills the ray family: the twisted average falls like 1/N at every x, the A = C excess -0.003580 at N = 55 and -0.000052 at N = 5555.
- hexagon.md the ghost star: star minus background -0.09385, -0.03102, -0.01756 at 5, 28, 56 layers; excess times L runs -0.7779 to -1.2519 in the ideal frame and -1.0212 to -1.3645 in the lattice frame from L = 28 to L = 400, ln L slopes -0.18 and -0.125.
- hexagon.md the ghost star, the cell frame: the arm ink law 1/2 + chi_8(n)/(2n) matches 1001/1001 layers at odd n <= 2001 and the three arms agree in 111/111 at odd n <= 221; excess times L + (ln L)/4 runs -0.2939128437 at L = 28 to -0.2937605886 at L = 6400 against the closed form -0.2937605857, ln L slope -0.250000.
- hexagon.md the ghost star, the parity split: the residual times L^2 runs -0.11975831 to -0.11979188 at L = 100 to 6400 with ratio 4.00 per doubling against -23/192 = -0.11979167; the L = 2 mod 4 ladder reads +0.13020825 at L = 3202 against +25/192 = +0.13020833; the odd ladder misses by +1/8 and what is left times L reads -0.250708, -0.250244, +0.249934, +0.249984, -0.250004 at L = 27, 99, 401, 1601, 6399.
- hexagon.md the ghost star, the width family: the twelve swept half-widths read -1/4, -1/6, -1/10, -3/28, -5/36, -3/22, -3/26, -9/68, -5/42, -13/100, -17/132 and -33/260 at L = 800, a best-rational search residual of 8.1e-7 and not an error bound.
- hexagon.md the ghost star, the width family checked: W = 0 to 12 recomputed to L = 3200 land within 5.1e-8 with every gap falling by four per doubling; the held-out W = 14, 18, 28, 36 land within 4.0e-8 of -7/60, -5/38, -7/58 and -9/74 at L = 1600; W = 1 and W = 3 read -0.249999 and -0.166666 against the law's -0.187500 and -0.156250.
- hexagon.md the constants: pi/4 + pi^2/32 = 1.0938233009 with 1.0826656664 at N = 55; (pi + pi^2)/16 = 0.8131998159 against pi^2 ln2/(7 zeta3) = 0.8130217042; G/8 = 0.1144956993 with 0.1144757884 at N = 55 and G/8 - 1/8 with -0.0104828892 at N = 53.
- hexagon.md the constants: L(2, chi_-3) = 0.7813024129 printed from its series and matched by nothing above.
