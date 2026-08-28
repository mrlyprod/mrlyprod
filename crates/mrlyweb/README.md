# mrlyweb

The eyes of Mrly: the wasm bridge that hands the designs to a browser. Every function here wraps one call into mrlynum, mrlymath or mrlylab and flattens its answer into what JavaScript reads without ceremony: byte grids, float buffers, decimal strings for the wide integers, and JSON for anything with more than one field. Nothing is computed twice and nothing is computed in JavaScript; the page only draws what the crates hand it.

Codes cross the boundary as decimal strings, because a design code is a u128 and a JavaScript number is not. A grid comes back as its width, height and row-major bytes; a cube comes back as packed faces, six floats per vertex, or as the x, y, z triples of its filled sites; a census, a universe and a parsed name come back as JSON text.

## Modules

- **bang** enumerates a universe, counts the distinct designs, folds fills and voids in closed form, and prints and parses names.
- **two** and **three** build the flat designs and the cubes: grids, painted pixels, packed faces, cell lists and censuses.
- **six** projects a cube to its hexagon and renders the SVG.
- **life** steps a grid one generation, runs a seed to its fate, and lays down the counts a named sequence gives.
- **lab** lists and counts the members of the sequence press and renders the moire presets to pixels.
- **race** looses seeded walkers on a flat design and measures how far they wander.
- **lattice** walks the Farey sequence and sieves the totients.

## Running

- `cargo test -p mrlyweb` runs the host tests over every export.
- `scripts/wasm.sh` builds the wasm package the demos import.
