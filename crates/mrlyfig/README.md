# mrlyfig

- The figure press of Mrly: it does one thing, draw.
- The library is the kit every figure shares; each figure is one binary of the `figures` crate, and the press turns it into two square pngs, one on the dark ground of the site and one on the light.
- Every figure is hand-authored from a one-line brief tied to its page's mathematics: subject, artist, parameters, palette; the briefs are the FIGURES list in `figures/README.md`.

## KIT

- `board` the rgba canvas and its anti-aliased primitives: rect, round rect, disc, ring, segment, polyline, triangle, polygon, arc, all signed-distance with a one pixel feather.
- `board::Frame` the drawing rectangle: inset, cell, unit coordinates, centre, the largest centred square, rows and columns.
- `ink` the palette of the house, resolved for the theme in press: `MRLYFIG_THEME` picks the dark ground or the light one, the six inks are blue, orange, yellow, green, pink and indigo, then mix and fade, and `Ramp` with its heat, fire, diverging and two-tone recipes.
- `grid` the square lattice: fill a cell, paint the type bytes of a flat design, grow a 0/1 mask by Kronecker substitution, lay a carpet.
- `hex` the triangle mesh of a hex slice, fitted equilateral into a frame, and the plain hexagon whose triangles always number six times its side squared.
- `iso` the exposed faces of a cube in isometric, back to front, three tones for the top, the left and the right.
- `plot` the plain marks: bars, dots, rings, staircases, curves and a bare hairline axis, never a tick and never a label.
- `field` a scalar field or a sampled function painted through a ramp.

## PRESS

- One figure is one binary: `figures/src/bin/<name>.rs`, autodiscovered by cargo, no list anywhere; the crate is a workspace member and a bare `cargo check` never touches it.
- Run one with `bash scripts/figures.sh <name>`, several by naming them, or every figure by passing nothing; the script reads the bin list from `cargo metadata` and every figure prints twice, once a theme.
- Every figure lands in `files/figures/` as `<name>-dark.png` and `<name>-light.png`, square, 1024 by 1024, on the ground of its theme, with no text and no wordmark. The art speaks.
- The pngs are CC BY 4.0, the licence beside them in `files/figures/LICENSE.md`; a single png a lab study draws lives there too under its own name.
- The figures carry private helpers the kit could absorb (a stroked rectangle, a hairline lattice, a hexagon cell reader, an isometric stamp, a frame-mapped scatter); fold one in when a third figure needs it.
- A figure never computes at render: it reads `files/figures/data/<name>.json`, written once by `cargo run --release -p figures --bin <name> -- compute`, and the data file is committed beside the code.
- `research-integers` sweeps the whole ledger in that compute pass and takes about five minutes; every figure then prints in well under a second.
