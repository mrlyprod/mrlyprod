# mrlydemo

The eyes of Mrly: the wasm bridge that hands the designs to a browser. Every function here wraps one call into mrlymath, mrlynum, mrlylab or mrlyfont and flattens its answer into what JavaScript reads without ceremony: byte grids, float buffers, decimal strings for the wide integers, and JSON for anything with more than one field. mrlycore supplies the colors, sheets and errors every module shares. Nothing is computed twice and nothing is computed in JavaScript; the page only draws what the crates hand it.

Codes cross the boundary as decimal strings, because a design code is a u128 and a JavaScript number is not. A grid comes back as its width, height and row-major bytes; a cube comes back as packed faces, six floats per vertex, or as the x, y, z triples of its filled sites; a census, a universe and a parsed name come back as JSON text.

## Modules

- **automata** wraps mrlymath's elementary rules: rows stepped, space-time diagrams, one rule's card.
- **bang** wraps mrlymath's universes: codes, symmetries, counts, closed-form fills and names.
- **blend** wraps mrlylab and mrlynum: registry sequences as terms, ratios, differences, recurrences and term operations.
- **carry** wraps mrlymath and mrlynum: the slice carry automaton's digit polynomial, even block, ladder, sign law and spectral ratio.
- **census** wraps mrlylab: which integers the whole registry writes inside a pinned window, how often, and which rows write one.
- **chladni** wraps mrlymath and mrlynum: soups stepped on design masks by FFT, the kernel drawn, the spectrum and its ring profile.
- **crop** wraps mrlymath and mrlynum: designs trimmed to rational shapes, tallied, swept, drawn and masked.
- **echo** wraps mrlynum: the design Mobius meter against log x, its density echo, residual and spectral ordinates.
- **font** wraps mrlyfont: text laid out as a grid, written in stroke order, cycled and read glyph by glyph.
- **formulas** wraps mrlynum: eight partial sums, products and prime counts read at one depth or walked to it.
- **gauss** wraps mrlynum: the Gaussian and Eisenstein windows painted, counted, clicked and weighed by norm.
- **graph** wraps mrlymath and mrlynum: design networks as nodes, branches, roles and censuses, relaxed by a seeded force layout.
- **lab** wraps mrlylab: the sequence press listed and counted, the moire presets rendered to pixels.
- **lattice** wraps mrlynum: the Farey nodes and totients, the lit window counted, painted and walked into pi.
- **ledger** wraps mrlylab and mrlymath: every measure of every design as a sequence, searched, identified and spelled.
- **life** wraps mrlymath: life grids stepped, run to a fate, and driven by named sequences.
- **magic** wraps mrlylab and mrlymath: the folded design, its census, its press readings and its prefix rates.
- **modes** wraps mrlymath: a mask's eigenvalue field on the frequency torus, one eigenvalue, the large-values count and a real mode.
- **morse** wraps mrlymath and mrlynum: the Thue-Morse word's two constructions, plane lifts, runs and difference filter.
- **prime** wraps mrlylab and mrlynum: the sieve stepped, the stone pile, the count chart and the carpet witness.
- **race** wraps mrlymath: seeded walkers loose on a flat design and how far they wander.
- **shell** wraps mrlymath: the crossing shell of a circle on a design, read as a rooted tree and painted.
- **sieve** wraps mrlymath and mrlynum: the Wallis sieve rastered, its punctures packed, its ratios walked and its limits read.
- **six** wraps mrlymath and mrlynum: a cube projected to its hexagon and rendered as SVG.
- **snail** wraps mrlynum: the whole numbers wound on the square spiral, each cell a design tile sized by its digit count.
- **spectrum** wraps mrlymath and mrlynum: the Laplacian spectra of the designs, their degeneracy and spectral exponent.
- **spin** wraps mrlylab, mrlymath and mrlynum: designs, moire fields and slices spun about their centre into ring profiles and wheels.
- **spiral** wraps mrlynum: the whole numbers wound on a square or hexagonal sheet, painted, clicked and read along a quadratic.
- **star** wraps mrlymath: the ghost star of the hexagon moire, its stacked cut, band, arm ink law and cell-frame decay.
- **three** wraps mrlymath: the cubes as packed faces, filled cells and censuses.
- **tile** wraps mrlymath: a design repeated across the plane, the cube and the hexagonal mesh, drawn and counted.
- **tourbillon** wraps mrlynum: the odd parity carpets spun about the centre, one angle per layer, inside the inscribed disc.
- **tube** wraps mrlymath: a design's distance field, its tube area at a radius, its Minkowski profile and the interior-hole limit.
- **two** wraps mrlymath: the flat designs as byte grids, painted pixels and censuses.
- **volume** wraps mrlylab and mrlymath: cube designs stacked into a moire volume, its faces at a level, and the planes that cut it.
- **weights** wraps mrlymath: a weighted design's mass field, pressure, multifractal spectrum and local dimensions.
- **zeta** wraps mrlynum: zeta walked on the critical line, its zeros counted and listed, and the prime staircase.

## Running

- `cargo test -p mrlydemo` runs the host tests over every export.
- `cargo test --release -p mrlydemo --test census -- --ignored` walks the whole registry to the pinned 48-term window, minutes not seconds, and pins the census against the research page.
- `scripts/wasm.sh` builds the wasm package the demos import.
