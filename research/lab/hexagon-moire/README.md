# Hexagon Moire

- The stack of diagonal cube slices, one hexagon per odd side n, for the carpet, net, tree and void designs.
- Cut ink of every layer as an exact rational against the four closed forms, and the void centre cell.
- Exact area-weighted Pearson correlations of layer pairs on the full hexagon: the doubling sign law, its Richardson limit, adjacent and gcd-echo pairs, the other families.
- The ideal-frame stack at 3601 samples per axis: quarter-line bands, the one-sided plateau at X = 1/4, the void star.
- The twisted triangle averages and the coarse crosshair law at 200003 samples.
- The ghost star of the rendered carpet stack on a 1200 by 2399 raster, and its log-corrected decay in the ideal and the lattice frame.
- The constants from the mrlynum series and the partial character sums at N = 53 and 55.
- Every domain is the source's own; nothing was shrunk. Thirty seconds.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p hexagon-moire`

## WITNESSES

- DISCOVERIES.md:96 and :98 the twisted average falls like 1/N, A=C excess -0.003580 at N=55 and -0.000052 at N=5555; crosshair A=1/3 excess -0.083726 against -0.083333, A=1/7 -0.058065 at N=55 against -0.036036 at N=5555.
- DISCOVERIES.md:97 and :311 star minus background -0.09385, -0.03102, -0.01756 at 5, 28, 56 layers; excess times L runs -0.7779 to -1.2519 in the ideal frame and -1.0212 to -1.3645 in the lattice frame from L=28 to L=400, ln L slopes -0.18 and -0.125.
- DISCOVERIES.md:102 the doubling sign law holds on all 18 pairs from (3,5) to (601,1201).
- DISCOVERIES.md:103, :270 and :309 Richardson r_inf from -0.11715991 to -0.11711630, branches 0.1171270 and 0.1171274, against 253/2160 = 0.11712963 and 19/162 = 0.11728395.
- DISCOVERIES.md:104 and :268 full-hexagon (5,9) = -0.14179450 and (5,7) = -0.08542646; tree and void doubling (201,401) +0.00050 and +0.00107; tree adjacent -0.07017, void adjacent +0.00787; echo (67,201) carpet +0.21473, tree +0.14985, void +0.07721.
- DISCOVERIES.md:105 L(2, chi_-3) = 0.7813024129 printed from its series and matched by nothing above.
- DISCOVERIES.md:106 quarter-line plateau 0.12210, 0.12340, 0.12412, 0.12446 at N = 151, 301, 601, 1201; the X = 1/4 bands +0.066465 and -0.090881 at N = 55.
- DISCOVERIES.md:107 void arms 0.5137, 0.4865, 0.4918, 0.4915 and the Z arm 0.3994 over a mean 0.2787; void centre cell ink 28/28.
- DISCOVERIES.md:109 the ink laws match 28/28 layers in exact rationals for all four designs, odd n <= 55.
- DISCOVERIES.md:110 pi/4 + pi^2/32 = 1.0938233009 with 1.0826656664 at N=55; (pi + pi^2)/16 = 0.8131998159 against pi^2 ln2/(7 zeta3) = 0.8130217042.
- DISCOVERIES.md:271 G/8 = 0.1144956993 with 0.1144757884 at N=55, and G/8 - 1/8 with -0.0104828892 at N=53.
