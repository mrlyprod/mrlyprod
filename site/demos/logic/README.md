# demos

The eyes of Mrly: the wasm bridge that hands the designs to a browser. Every function here wraps one call into `mrlyrs` or `ledger` and flattens its answer into what JavaScript reads without ceremony: byte grids, float buffers, decimal strings for the wide integers, and JSON for anything with more than one field. `mrlyrs::core` supplies the colors, sheets and errors every module shares. Nothing is computed twice and nothing is computed in JavaScript; the page only draws what the crates hand it.

Codes cross the boundary as decimal strings, because a design code is a u128 and a JavaScript number is not. A grid comes back as its width, height and row-major bytes; a cube comes back as packed faces, six floats per vertex, or as the x, y, z triples of its filled sites; a census, a universe and a parsed name come back as JSON text.

## Modules

- **automata** wraps `mrlyrs::life`'s elementary rules: rows stepped, space-time diagrams, one rule's card.
- **bang** wraps `mrlyrs::math`'s universes: codes, symmetries, counts, closed-form fills and names.
- **blend** wraps `ledger` and `mrlyrs::num`: registry sequences as terms, ratios, differences, recurrences and term operations.
- **carry** wraps `mrlyrs::math` and `mrlyrs::num`: the slice carry automaton's digit polynomial, even block, ladder, sign law and spectral ratio.
- **census** wraps `ledger`: which integers the whole registry writes inside a pinned window, how often, and which rows write one.
- **chladni** wraps `mrlyrs::life` and `mrlyrs::num`: soups stepped on design masks by FFT, the kernel drawn, the spectrum and its ring profile.
- **crop** wraps `mrlyrs::math` and `mrlyrs::num`: designs trimmed to rational shapes, tallied, swept, drawn and masked.
- **echo** wraps `mrlyrs::num`: the design Mobius meter against log x, its density echo, residual and spectral ordinates.
- **font** wraps `mrlyrs::font`: text laid out as a grid, written in stroke order, cycled and read glyph by glyph.
- **formulas** wraps `mrlyrs::num`: eight partial sums, products and prime counts read at one depth or walked to it.
- **gauss** wraps `mrlyrs::num`: the Gaussian and Eisenstein windows painted, counted, clicked and weighed by norm.
- **graph** wraps `mrlyrs::math` and `mrlyrs::num`: design networks as nodes, branches, roles and censuses, relaxed by a seeded force layout.
- **lab** wraps `mrlyrs::math`: the sequence press listed and counted, the moire presets rendered to pixels.
- **lattice** wraps `mrlyrs::num`: the Farey nodes and totients, the lit window counted, painted and walked into pi.
- **ledger** wraps `ledger` and `mrlyrs::math`: every measure of every design as a sequence, searched, identified and spelled.
- **life** wraps `mrlyrs::life`: life grids stepped, run to a fate, and driven by named sequences.
- **magic** wraps `mrlyrs::math`: the folded design, its census, its press readings and its prefix rates.
- **modes** wraps `mrlyrs::math`: a mask's eigenvalue field on the frequency torus, one eigenvalue, the large-values count and a real mode.
- **morse** wraps `mrlyrs::math` and `mrlyrs::num`: the Thue-Morse word's two constructions, plane lifts, runs and difference filter.
- **novelty** wraps `mrlyrs::num`: the totients sieved once and read through the smooth and the sharp window on a log grid of y, the first zeros with their wave coefficients, and the wave the first so many zeros sum to.
- **prime** wraps `mrlyrs::math` and `mrlyrs::num`: the sieve stepped, the stone pile, the count chart and the carpet witness.
- **race** wraps `mrlyrs::math`: seeded walkers loose on a flat design and how far they wander.
- **shell** wraps `mrlyrs::math`: the crossing shell of a circle on a design, read as a rooted tree and painted.
- **sieve** wraps `mrlyrs::math` and `mrlyrs::num`: the Wallis sieve rastered, its punctures packed, its ratios walked and its limits read.
- **six** wraps `mrlyrs::math` and `mrlyrs::num`: a cube projected to its hexagon and rendered as SVG.
- **snail** wraps `mrlyrs::num`: the whole numbers wound on the square spiral, each cell a design tile sized by its digit count.
- **spectrum** wraps `mrlyrs::math` and `mrlyrs::num`: the Laplacian spectra of the designs, their degeneracy and spectral exponent.
- **spin** wraps `mrlyrs::math` and `mrlyrs::num`: designs, moire fields and slices spun about their centre into ring profiles and wheels.
- **spirograph** wraps `mrlyrs::num`: a flat design as the wheel, a pencil in every cell, rolled on a line, a circle or a polygon, traced, read and posed.
- **spiral** wraps `mrlyrs::num`: the whole numbers wound on a square or hexagonal sheet, painted, clicked and read along a quadratic.
- **star** wraps `mrlyrs::math`: the ghost star of the hexagon moire, its stacked cut, band, arm ink law and cell-frame decay.
- **three** wraps `mrlyrs::math`: the cubes as packed faces, filled cells and censuses.
- **tile** wraps `mrlyrs::math`: a design repeated across the plane, the cube and the hexagonal mesh, drawn and counted.
- **tourbillon** wraps `mrlyrs::num`: the odd parity carpets spun about the centre, one angle per layer, inside the inscribed disc.
- **tube** wraps `mrlyrs::math`: a design's distance field, its tube area at a radius, its Minkowski profile and the interior-hole limit.
- **two** wraps `mrlyrs::math`: the flat designs as byte grids, painted pixels and censuses.
- **volume** wraps `mrlyrs::math`: cube designs stacked into a moire volume, its faces at a level, and the planes that cut it.
- **weights** wraps `mrlyrs::math`: a weighted design's mass field, pressure, multifractal spectrum and local dimensions.
- **zeta** wraps `mrlyrs::num`: zeta walked on the critical line, its zeros counted and listed, and the prime staircase.

## Running

- `cargo test -p demos` runs the host tests over every export.
- `cargo test --release -p demos --test census -- --ignored` walks the whole registry to the pinned 48-term window, minutes not seconds, and pins the census against the research page.
- `scripts/wasm.sh` builds the wasm package the demos import.
