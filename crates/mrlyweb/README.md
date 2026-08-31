# mrlyweb

The eyes of Mrly: the wasm bridge that hands the designs to a browser. Every function here wraps one call into mrlynum, mrlymath or mrlylab and flattens its answer into what JavaScript reads without ceremony: byte grids, float buffers, decimal strings for the wide integers, and JSON for anything with more than one field. Nothing is computed twice and nothing is computed in JavaScript; the page only draws what the crates hand it.

Codes cross the boundary as decimal strings, because a design code is a u128 and a JavaScript number is not. A grid comes back as its width, height and row-major bytes; a cube comes back as packed faces, six floats per vertex, or as the x, y, z triples of its filled sites; a census, a universe and a parsed name come back as JSON text.

## Modules

- **bang** enumerates a universe, counts the distinct designs and the fill classes, folds fills and voids in closed form, and prints and parses names.
- **two** and **three** build the flat designs and the cubes: grids, painted pixels, packed faces, cell lists and censuses.
- **six** projects a cube to its hexagon and renders the SVG.
- **life** steps a grid one generation, runs a seed to its fate, and lays down the counts a named sequence gives.
- **lab** lists and counts the members of the sequence press and renders the moire presets to pixels.
- **race** looses seeded walkers on a flat design and measures how far they wander.
- **lattice** walks the Farey sequence and sieves the totients.
- **ledger** builds the sequence catalog tier by tier, whole or a span of keys at a time, searches it by terms or name, reads any design sequence or its diagonal profile to a cell budget, identifies typed terms against the curated records, and spells the closed forms.
- **census** walks the whole ledger registry inside a pinned window, deepening pass by pass, and answers which integers it writes, how many rows write each, which are missed, and which rows write one.
- **prime** steps the sieve one prime at a time, reads a number as a pile of stones, charts the prime count against its smooth guesses, and puts a scale on trial in the carpet stack.
- **volume** stacks a cube design at odd scales into a moire volume, packs the faces of its level sets, and paints any plane through it.
- **gauss** paints the Gaussian or the Eisenstein window with its primes by class or norm, counts the classes, maps a click to its point with its units and conjugate, and weighs the norm shells.
- **spiral** winds the whole numbers on a square or hexagonal sheet, paints the marked cells and one quadratic, maps a click back to its number, and reads the prime hits along the quadratic.
- **zeta** walks the critical line, reads zeta and Z at any t, lists and counts the zeros, and lays the prime staircase against the explicit formula.
- **graph** hands out the node positions, branches, roles and census of a design's core, edge, tunnel or slice network, bounds its size in closed form, and relaxes it with a seeded force layout.
- **spin** spins a square field about its centre into an exact ring profile, paints it back as a wheel, stacks it radially, reads its circular harmonics, and rasterizes a hex slice or a moire field to feed it.

## Running

- `cargo test -p mrlyweb` runs the host tests over every export.
- `cargo test --release -p mrlyweb --test census -- --ignored` walks the whole registry to the pinned 48-term window, minutes not seconds, and pins the census against the research page.
- `scripts/wasm.sh` builds the wasm package the demos import.
