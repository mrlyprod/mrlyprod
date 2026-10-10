# Figures

- The figures of mrly.net, drawn live in the browser: one file a figure, the folder is the roster.
- Every `*.ts` here but `time.ts` is a live figure; the archive's figures sit in `research/figures/`.

## CONTRACT

- `<name>.ts` exports `default function draw(pen, ink, t)`, `t` in [0, 1) over one loop.
- `size`: `[width, height]`, default `[1024, 1024]`; a non-square size keeps its aspect in the host.
- `still`: the `t` a still figure draws, default 0.
- `loop` in seconds, with `frames`: an animated figure only; none in the live set.
- `units`: the `mrlyjs` wasm units it imports, e.g. `{ math }`; the caller initialises them before the first draw.
- `ink` is `ink(tinted(dark | light, ...))` from `site/kit/theme`: the tint's hue sits in the blue slot.
- A figure is a pure function of its inputs: no file, DOM, Date or Math.random.
- A figure asserts its facts and throws when one breaks; heavy facts are memoised at module level; pixels are never cached.

## NAMING

- `site-<door>`: the doors, the menu folders, the og card `site-og` and the icon `site-icon`.
- `app-<id>`: the apps.
- `blog-<slug>`: the posts.

## KIT

- The pen and the kit live in `pkgs/mrlyjs/view`: `canvas` draws in the browser, `raster` fills rgba bytes in Bun and the tests, `svg` builds a string.
- The kit: `frame`, `ink`, `grid`, `hex`, `iso`, `plot`, `field`.

## TIME

- `bun run time [name ...]` draws each figure in a fresh worker with the raster pen, cold then warm, dark and light, and prints a table sorted by warm ms.
- Gate: 150 ms warm, 500 ms cold on the desk; `site-og` and `site-icon` excluded; exit 1 when a figure is over.
- The raster pen is a proxy for the browser's `canvas()` pen.

## LICENCE

- The images the figures draw are CC BY 4.0, `LICENSE.md`.

## FIGURES

- One bullet per figure, `subject; artist; parameters; palette`; the artist is the kit module the figure leans on.
- `site-home`: the sponge, code 23 at base 2 grown to level 3, in isometric projection with three-tone faces, standing clear on the ground inside an 8 percent margin; iso; number 3, level 3, side 27; top faces fg, left faces blue, right faces dim
- `site-demos`: twenty-eight tiles in a 7 by 4 board, each a different base-3 plane design at level 3, one tile per live page, ground gutters between; grid; base 3, level 3, side 27 a tile; inks cycling blue, orange, yellow
- `site-papers`: twelve thin slabs stacked in isometric and offset so each stays legible, every top face carrying a different design's level-2 cells; iso; 12 sheets, base 2, level 2, 8 percent margin; tops fg, sides blue on ground
- `site-research`: the moire stack of one design sampled at scales 1 to 20, the layers summed into a scalar field and run through a ramp, twenty layers for twenty pages; field; code 495, base 3, scales 1..20, 1024 board; blue-to-yellow on ground
- `site-og`: the mark, rows 11111 10101 11111 10101 11111, tiled edge to edge as a lattice across the 1200 by 630 card at 15 px a cell, the pattern centred so the crop is symmetric; grid; level 1, 80 by 42 cells; fg on ground
- `site-icon`: the mark alone at level 1, five cells a side filling 76 percent of a 512 by 512 board, centred on the ground; grid; level 1; fg on ground
- `site-wiki`: the door to the wiki, one concept a page in prerequisite order: seven base-2 plane designs at level 2 as a graph of rows one, three and three, each parent joined to its children; grid; codes 7, 14, 3, 5, 9, 6, 15, base 2, level 2, side 9, 283 cells; blue on ground, the root yellow, the edges in line
- `site-apps`: the icon of the menu's Apps folder: the base-2 carpet, code 7, at level 4 as one sheet inside an 8 percent margin, its 4096 cells the metal and every void punched through to the ground; grid; code 7, base 2, level 4, side 81; dim on ground
- `site-math`: the door to /mrlymath, where a design is one string: the eight corner bits of `{"kind":"bang","dim":2,"code":7}` as a row of eight cells, three of them lit, over the level-3 carpet those bits grow, 512 cells centred below; grid; code 7, base 2, level 3, side 27; fg bits, line empties, blue carpet on ground
- `site-research-folder`: the icon of the menu's Research folder, where the tree is searched: a magnifying glass, a ring lens and a diagonal handle, drawn cell by cell; grid; 13 by 13 cells; lens fg, handle blue on ground
- `site-notes`: the door to the notes, one page per idea: a spiral pad, three rings over a sheet ruled with four lines, one short; grid; 11 by 12 cells; sheet fg, rings blue, lines dim on ground
- `site-claims`: the door to the claims, every line with its witness: a check mark in a box; grid; 13 by 11 cells; box fg, check green on ground
- `site-blog`: the door to the blog, posts written by hand: a pencil on the diagonal over the line it wrote; grid; 13 by 13 cells; body fg, eraser pink, tip orange, line blue on ground
- `app-settings`: the door to Settings, the dials of the site: a gear of eight teeth around a hub; grid; 13 by 13 cells; gear fg, hub blue on ground
- `app-matrix`: the door to Matrix, the falling code: five columns of MrlyFont glyphs, each drop a head over a trail that fades upwards, heads at five different rows; grid; 5 by 7 glyphs of 5 by 5 cells, 37 by 41 cells; heads fg, trails blue fading to ground
- `app-sleep`: the door to Sleep, the bouncing tile: the mark, the carpet of 5, leaning on the right wall of a screen, its dotted path bouncing off the floor and the left wall; grid; 23 by 23 cells, tile 5; tile fg, path blue, screen dim on ground
- `app-mandelbrot`: the door to Mandelbrot: the set over the saver's home window, each cell's centre iterated 100 times, the 695 cells that never escape and the escape bands around them; grid; window -2..1 by -1.5..1.5, 63 by 63 cells, bands from 24, 10, 5 and 3 steps; set fg, bands blue fading to ground
- `app-julia`: the door to Julia: the set of c = -0.8 + 0.156i, one of the saver's six presets, each cell lit by the slowest of its 3 by 3 samples, 761 cells unescaped after 100 steps and symmetric under z to -z; grid; window -1.5..1.5 square, 63 by 63 cells, bands from 24, 10, 5 and 3 steps; set fg, bands blue fading to ground
- `app-lightspeed`: the door to Lightspeed, the jump: 24 star streaks converging on an empty vanishing point, each bright at its head and fading to its tail; grid; 33 by 33 cells, golden-angle spread; heads fg, streaks cyan and blue on ground
- `app-snake`: the door to Snake, the game: six marks, the carpet of 5, winding as a J toward a round apple, the head, the tail and the turn corners rounded as the game rounds them; grid; 4 by 3 marks of 5 by 5 cells; head fg, body blue, apple orange on ground
- `app-sfx`: the door to SFX, the sounds of the voyage: one burst as a waveform of 16 bars about a silent centre line, a three-bar attack then an exponential decay to a hair; grid; 33 by 33 cells, decay 4.5 bars, a cosine ripple; loud bars fg, body blue, tail blue fading, line dim on ground
- `app-techno`: the door to Techno, the step sequencer: a 16-step grid of kick, hat, clap and bass rows in four beats, the kick four on the floor, the fifth step lit as the playhead; grid; 4 by 16 pads of 2 by 4 cells; lit hits fg, lit rests dim, hits blue, rests faint dim on ground
- `app-zoom`: the door to Zoom, the endless flythrough: the face of the sponge, code 23, as three nested carpets, the level-3 carpet whose centre hole shows the level-2 carpet whose centre hole shows the level-1 carpet, each the next cell down the zoom; grid; 27 by 27 cells, levels 3, 2 and 1, 512, 64 and 8 lit; outer fg, middle blue, inner blue fading to ground
- `app-planets`: the door to Planets, a world lit as a camera sees it: a disc lit from the upper left with its terminator, the night side dim, a tilted ring with the Cassini division passing behind the disc and across its face; grid; 33 by 33 cells, disc radius 8.6, ring 11 to 15.6 squashed to 0.27 and tilted -0.32 rad; day fg, dusk blue, night dim, inner ring fg, outer ring cyan on ground
- `app-voyage`: the door to Voyage, the jump to a world: a planet lit from the upper left with its terminator, and three streak arcs sweeping round it from above to its left, each bright at its head and fading to its tail; grid; 33 by 33 cells, planet radius 7.6, arcs of radius 12, 15 and 18; day fg, dusk blue, night dim, heads fg, streaks cyan and blue on ground
- `app-designs`: the door to Designs, the gallery: the sixteen plane designs of base 2 at side 3, level 2, in a 4 by 4 sheet of panels, the six class leaders in blue and their mates faded; grid; 16 panels of 9 by 9 cells; leaders blue, mates blue faded to ground
- `app-font`: the door to Font, the pens of MrlyFont: `Aa` in the face, the A written whole and the a being penned, its stroke path three fifths done; grid; the raster of the two glyphs; done cells fg, the written stroke blue, the stroke to come blue faded to ground
- `app-life`: the door to Life, the game on a design: the carpet of 3 at level 2, code 7, as the seed, run four generations under B3/S23 on the carpet's own mask, the living cells over the ghost of the seed; grid; 21 by 21 cells; alive blue, the seed's dead cells dim faded on ground
- `app-sequences`: the door to Sequences, the ledger: the fills of design 7 at level 1 by odd side as eight pins, the octagonal numbers 8, 21, 40 and on, each pin a stem under a two-cell head; grid; 25 by 25 cells, a floor line, stems 3 cells apart; heads fg over blue, stems blue faded, floor dim on ground
- `app-spirograph`: the door to Spirograph, the design as the wheel: the carpet of 3 in base 3, code 495, seated on a wheel of 3 inside a ring of 7, every filled cell a pencil, the curves traced to the end of the track in three bands by seat radius, the wheel and its pencils drawn at rest; the ring line, wheel dim; curves blue in three shades, tile fg, pencil tips in their band's shade on ground
- `site-pages`: the icon of the menu's Pages folder: two sheets, the back one offset behind the front one, which carries three ruled lines; grid; 13 by 13 cells; front fg, back dim, lines blue on ground
- `site-method`: the door to the method, a direction, a writer, a generator: three squares joined by an elbowed path down the diagonal; grid; 13 by 13 cells, squares 3 a side; fg, blue, yellow, path dim on ground
- `site-stats`: the door to the stats of the cloud: four bars over a bare L axis, heights 4, 7, 5 and 10; grid; 13 by 12 cells; axis fg, bars blue on ground
- `site-root`: the icon of the menu's Root folder, the files at `/`: a folder with a slash inside; grid; 13 by 12 cells; folder fg, slash blue on ground
- `site-elsewhere`: the icon of the menu's Elsewhere folder, links off the site: a box open at its corner and an arrow leaving it; grid; 13 by 13 cells; box fg, arrow blue on ground
- `site-menu`: the door to the menu, every door at once: a 3 by 3 board of squares, the centre lit; grid; 13 by 13 cells, squares 3 a side; fg, centre blue on ground
- `blog-launching-mrlyprod-org`: three counted blocks of unit squares on one baseline, 28 demos, 12 papers, 20 research pages, each a tight rectangle with a wide gap between; grid; 28 as 7 by 4, 12 as 3 by 4, 20 as 5 by 4; blue, yellow, green on ground
- `site-page`: the fallback icon of a page: a sheet with its top right corner folded, three ruled lines inside; grid; 11 by 13 cells; sheet fg, lines blue on ground
- `site-code`: the door to the code: a pair of angle brackets with a slash between them; grid; 15 by 9 cells; brackets fg, slash blue on ground
- `site-contact`: the door to contact: an envelope, its flap a V from the top corners; grid; 14 by 10 cells; envelope fg, flap blue on ground
- `site-donate`: the door to donate: a heart with a highlight on its upper left lobe; grid; 11 by 10 cells; heart fg, highlight red on ground
- `site-cart`: the door to the cart: a shopping cart, its handle, basket and two wheels; grid; 14 by 11 cells; cart fg, wheels orange on ground
