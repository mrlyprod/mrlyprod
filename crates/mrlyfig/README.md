# mrlyfig

- The figure press of Mrly: it does one thing, draw.
- The library is the kit every figure shares; each figure is one script under `examples/`, and the press turns it into a square png on the dark ground of the site.
- Every figure is hand-authored from a one-line brief tied to its page's mathematics: subject, artist, parameters, palette; the briefs are the FIGURES list below.

## KIT

- `board` the rgba canvas and its anti-aliased primitives: rect, round rect, disc, ring, segment, polyline, triangle, polygon, arc, all signed-distance with a one pixel feather.
- `board::Frame` the drawing rectangle: inset, cell, unit coordinates, centre, the largest centred square, rows and columns.
- `ink` the palette of the house, the six inks, mix and fade, and `Ramp` with its heat, fire, diverging and two-tone recipes.
- `grid` the square lattice: fill a cell, paint the type bytes of a flat design, grow a 0/1 mask by Kronecker substitution, lay a carpet.
- `hex` the triangle mesh of a hex slice, fitted equilateral into a frame, and the plain hexagon of side n whose triangles always number six n squared.
- `iso` the exposed faces of a cube in isometric, back to front, three tones for the top, the left and the right.
- `plot` the plain marks: bars, dots, rings, staircases, curves and a bare hairline axis, never a tick and never a label.
- `field` a scalar field or a sampled function painted through a ramp.

## PRESS

- One figure is one example: `examples/<folder>/<file>.rs`, listed by hand in `Cargo.toml` as `[[example]] name = "<figure>"`, because cargo does not look inside nested example folders.
- Run one with `bash scripts/figures.sh <name>`, several by naming them, or every figure by passing nothing.
- Every png lands in `files/figures/`, square, 1024 by 1024, on the ground, with no text and no wordmark. The art speaks.
- The pngs are CC BY 4.0, the licence beside them in `files/figures/LICENSE.md`.
- The examples carry private helpers the kit could absorb (a stroked rectangle, a hairline lattice, a hexagon cell reader, an isometric stamp, a frame-mapped scatter); fold one in when a third figure needs it.
- `research-integers` sweeps the whole ledger and takes about five minutes; every other figure prints in seconds.

## FIGURES

- One bullet per figure, `subject; artist; parameters; palette`; the artist is the kit module the figure leans on: grid, hex, iso, field or plot.
- Names are the routes: `site-*` the doors and the card, `research-<page>`, `paper-<slug>`, `blog-<slug>`; `research-index` also opens DISCOVERIES and REFS.

- `site-home`: the sponge, code 23 at base 2 grown to level 3, in isometric projection with three-tone faces, standing clear on the ground inside an 8 percent margin; iso; number 3, level 3, side 27; top faces fg, left faces blue, right faces dim
- `site-demos`: twenty-eight tiles in a 7 by 4 board, each a different base-3 plane design at level 3, one tile per live page, ground gutters between; grid; base 3, level 3, side 27 a tile; inks cycling blue, orange, gold
- `site-papers`: twelve thin slabs stacked in isometric and offset so each stays legible, every top face carrying a different design's level-2 cells; iso; 12 sheets, base 2, level 2, 8 percent margin; tops fg, sides blue on ground
- `site-research`: the moire stack of one design sampled at scales 1 to 20, the layers summed into a scalar field and run through a ramp, twenty layers for twenty pages; field; code 495, base 3, scales 1..20, 1024 board; blue-to-gold on ground
- `site-og`: the mark, rows 11111 10101 11111 10101 11111, tiled edge to edge as a lattice across the 1200 by 630 card at 15 px a cell, the pattern centred so the crop is symmetric; grid; level 1, 80 by 42 cells; fg on ground
- `site-icon`: the mark alone at level 1, five cells a side filling 76 percent of a 512 by 512 board, centred on the app ground #0b0d10; grid; level 1; fg on #0b0d10
- `research-index`: one design grown by the tree's one move: the base-3 carpet, code 495, at levels 1, 2, 3 and 4 as a 2 by 2 board of equal squares, each level drawn to the same size; grid; code 495, base 3, levels 1 to 4; gold on ground
- `research-core`: the 16 base-2 D=2 designs each grown to level 4, laid as a 4x4 census of 16x16 boards in code order; the six canonical representatives gold, the ten orbit-mates blue; grid; base 2, D=2, level 4; gold + blue on ground
- `research-bijection`: all 256 base-2 D=3 designs as a 16x16 census, each a 2x4 stamp of its eight corner bits in binary order; the 22 canonical representatives in gold, the rest in blue; grid; D=3, 256 codes, 22 orbits; gold + blue on ground
- `research-complexity`: the normalised-Laplacian eigenvalue staircase of the level-6 Sierpinski triangle, 729 values in [0,2], the riser at eigenvalue 1 a full third of the stair and drawn gold, the rest blue; plot; L=6; gold + blue on ground
- `research-cuts`: the two central diagonal cuts of mrly_bang_d3_126 at level 7 seen down (1,1,1): 4374 points as six Sierpinski gaskets of 729 tiling a hexagon; hex; L 7, heights t=63,64; three up gaskets blue, three down orange on ground
- `research-slices`: the central diagonal hexagon of the cube at odd side 11, all 726 unit triangles, the carpet's 486 against the net's 240, so the net paints the carpet's 19 hexagram holes; hex; codes 23 and 232, level 1, base 2; blue carpet, orange net
- `research-dimensions`: the pole line of the base-3 Cantor design, an exact column at Re 0.630930 spaced 5.719202 in Im, beside the 21 scattered poles of the two-ratio control; plot; m -6..6, box Re [-1,1] Im [-40,40]; column blue, scatter orange
- `research-connectivity`: two 64x64 boards side by side: left the level-6 gasket, 729 cells in one green piece; right exactly 729 uniform random cells in pink, shattered; grid; base 2, code 7, 729 cells each; green + pink on ground
- `research-magic`: one word in both orders: the composite tiles carpet(3) (x) net(5) and net(5) (x) carpet(3), each 15x15, equal fill and different shape; grid; two panels, level 1, side 15; left tile blue, right tile orange on ground
- `research-walks`: codes 127 and 239 at level 4 side by side, same fill 7, each carrying one blind-ant walker's traced path over its own cells; plot; base 3, level 4, one seed per side, cells dim; 127's path blue, 239's orange
- `research-method`: the whole 3D design space walked: all 22 symmetry classes as 2x2x2 corner sets in isometric projection, laid in rows by popcount 0 to 8; iso; 22 cubes, three face tones; blue three-tone on ground
- `research-farey`: the Farey stack on the unit square, the grids of every scale n = 1..60 summed so a node a/b lights at brightness floor(60/b); field; N 60, unit square; ramp ground to blue to gold at the brightest nodes
- `research-mobius`: the base-4 anti-symmetric pair: the Mobius meters of digit sets {0,1} and {0,2} walked as staircases, exact mirror images across zero inside a +-sqrt(A_F) envelope; plot; q 4, L 16, 65536 steps; blue, orange, envelope dim
- `research-pi`: the corner window of the grid at N = 100 with every point of gcd 1 lit as a square dot and the 3913 hidden points left dim, 6087 lit of 10000; plot; N = 100, origin at lower left; gold lit, dim hidden, on ground
- `research-coprime`: the base-3 carpet at level 4, side 81, its 4096 cells as squares, those with coprime coordinates lit gold and the rest dim; grid; base 3, code 495, level 4, origin at the lower left corner; gold + dim on ground
- `research-bases`: the hexagonal Eisenstein lattice out to radius 30, points visible from the origin (gcd 1) as filled blue dots, points hidden behind a nearer one as small dim dots; plot; hexagonal lattice, radius 30; blue + dim on ground
- `research-spectra`: the centroid diagonal slice of the odd-coordinate solid at base 5, level 2, drawn triangle by triangle with the two tile kinds separated: hexagon cells gold, triangle cells blue; hex; code 23, number 5, level 2, base 5; gold + blue on ground
- `research-spin`: the level-5 carpet spun about its corner fixed point: the exact circle mean at every radius painted as a bullseye, rings repeating by a factor of 3; field; code 495, base 3, level 5, centre (0,0); blue-to-gold ramp on ground
- `research-crop`: the base-3 carpet at level 4, side 81, under the inscribed circle of radius 1/2: 2908 In cells gold, 204 Cut cells orange, Out cells left as ground; grid; base 3, code 7, level 4, r = 1/2; gold + orange on ground
- `research-sequences`: a census board: plane designs 1, 3, 7, 9, 11, 15 down six rows against odd sides 3, 5, 7, 9 across four columns, each tile the design's fill painted cell by cell; grid; base 2, level 1; gold fills on ground, gutters in line
- `research-integers`: the integer census as a 100x100 raster of 1..10000 in reading order, a cell dark where no registry row writes it and brightening with the number of rows that do; grid; window 1..10000, 48-term rows; dim to gold on ground
- `paper-coprime-density-above-dimension-one`: the level-5 gasket design in a 32x32 grid, corners (0,0),(1,0),(0,1): its 243 cells as squares, the 122 of coordinate gcd 1 gold, the other 121 dim, the rest ground; grid; base 2, level 5; gold + dim on ground
- `paper-lemma-b-pincer`: the exponent line 0 to 1 as one bar: closed arms [0,0.4475978] and [0.6402122,1] dim, the open window between orange, hairlines at 0.447931, 0.5, 0.605303; plot; window 0.1926 wide; orange + dim + line on ground
- `paper-menger-pairwise-coprimality`: the 27 base-3 digit vectors as three 3x3 slices c=0,1,2: the 7 sponge-deleted cells left as ground in line outlines, the 13 with at most one zero green, the other 7 survivors dim; grid; base 3, 13/20; green + dim + line
- `paper-walsh-spectrometer`: the diagonal hexagon slice of code 105 at n = 7, all 6n^2 = 294 unit triangles drawn, the inked ones (even-weight macro parity) blue and the paper ones as line-thin outlines on ground; hex; code 105, n 7, level 1, base 2; blue + line
- `paper-slice-recurrence-order`: thirteen bar pairs, one per dimension D = 2..14: the free order 2D+1 in dim behind, the proved order ceil(D/2) in blue in front, a baseline rule in line, no ticks; plot; D 2..14, 5..29 against 1..7; blue + dim
- `paper-slice-sign-even-half`: certificate depth against even dimension: base 3's quadratic climb from 0 to 89 as an orange dotted curve, bases 5 to 11 flat at depth at most 2 as dim steps below it; plot; even D 2..40, K_min 0..89; orange + dim
- `paper-gasket-ray-machine`: the level-5 base-3 gasket: a ray from the origin along every occupied direction, blue and fainter as its mass falls, the heaviest ray (1,3) gold, the 243 points as fg dots; plot; level 5, mass(1,3) = F(6)-1; blue + gold + fg
- `paper-order-sensitivity-of-kronecker-words`: the witness, two 4x4 pictures side by side: code 3 outside code 6 as four isolated blue cells, code 6 outside code 3 as two orange dominoes, four filled cells each, thin line grid; grid; L = 2, comp 4 against 2; blue + orange + line
- `paper-component-exponent-of-kronecker-words`: the 105 two-letter alphabets over designs 1..15 as an upper-triangular 15x15 grid: the 89 that meet the fill ceiling green, the 16 short orange, a gold pip on the 27 where the constant rule is exact; grid; 105 cells; green + orange + gold
- `paper-moire-correlation-laws`: the 31x31 correlation matrix of parity carpets at odd scales 3 to 63, cell = exact r(C_m,C_n): zero left as ground so prime rows and columns run black, positive ramped blue to gold; field; odd 3..63; blue-to-gold on ground
- `paper-divisor-avatars`: the first Menger step at side 3 in isometric: the 20 kept cells as solid cubes in three gold face tones, the 7 cells with two or more even coordinates as dim wire boxes in their holes; iso; side 3, 20 = d(240); gold three-tone + dim
- `paper-sequence-census`: the six planar rules that keep the all-even corner, drawn at side 5 as six 5x5 panels in a 3x2 block: codes 1,3,7 blue, codes 9,11,15 violet, each with its k=2 inner 3x3 frame in line; grid; side 5, k = 3; blue + violet + line
- `blog-launching-mrlyprod-org`: three counted blocks of unit squares on one baseline, 28 demos, 12 papers, 20 research pages, each a tight rectangle with a wide gap between; grid; 28 as 7 by 4, 12 as 3 by 4, 20 as 5 by 4; blue, gold, green on ground
- `research-automata`: the 256 elementary rules as a 16 by 16 census of single-seed diagrams in rule order, each 32 generations on a 63-cell window; the 14 nonconstant affine rules gold, the other 16 surjective rules blue, the rest dim; grid; T 32, 256 panels; gold + blue + dim on ground
- `demo-wolfram`: rule 110 from one live cell, 256 generations on a 511-cell window, the space-time triangle with time running down and the live cells the only ink; grid; rule 110, T 256; fg on ground
- `demo-mrlylife`: B3/S23 on the level-2 base-2 carpet mask (side 9, 64 cells, centre already void) run 64 generations from a seeded soup on a 192 torus, live cells blue, the mask stamped top left in gold; grid; code 7, side 9, level 2, T 64, fixed seed; blue + gold on ground
