# demos

- The eyes of MrlyMath: browser pages that draw what the crates compute through `mrlyweb` and wasm.
- Rust is the only math; the pages only draw. Three.js is the one JavaScript dependency.
- `bun install` fetches it; `bun run wasm` builds `pkg/` from `crates/mrlyweb` with wasm-pack.
- One page is one folder: `<name>/index.html` and `<name>/index.js`; the gallery is `index.html` at the root.
- `lib/` holds the shared code: `mrly.js`, `stage.js`, `chart.js`, `query.js`, `ramp.js`, `select.js`, `mrly.css`.
- `select.js` is the one picker: design list, code, base and Randomize; `?seed=7` replays the seventh tap, and a typed code drops the seed.
- `bun run dev` serves every page at `localhost:3000` as `/<name>`; `bun run build` writes the static site to `dist/`.
- `bun run check` prints the fixture numbers the crate's host test asserts; both must agree.
- `pkg/`, `dist/` and `node_modules/` are build output and stay out of git.

## PAGES

- [cuts](cuts/) - Every plane `x + y + z = s` through a level-L solid meets exactly 3^L cells, and the height's binary digits make each cut a Sierpinski gasket.
- [farey](farey/) - A line drawn at every `k/n` and stacked over the scales lights a reduced fraction `a/b` to a brightness of `Q/b`, and the scales of maximal novelty are the primes.
- [gaussian](gaussian/) - The Gaussian and Eisenstein primes as four- and six-armed snowflakes, coloured by whether an ordinary prime split, stayed inert, or ramified on entering the plane.
- [graphs](graphs/) - Joining every filled cell to its neighbours turns a design into a network with tips, junctions, pieces, length and a box dimension, flat, in the cube, on the hexagonal slice, or relaxed by force.
- [integers](integers/) - The union of every sequence the registry writes, read integer by integer: which of the first thousand the designs write, how many rows write each, and which are missed inside the pinned window.
- [life](life/) - Cellular automata whose birth and survival rules are read from named sequences, run until the grid dies, freezes or loops.
- [moire](moire/) - One design sampled at scale 1, 3, 5 and on, the layers stacked into a field where the interference is the finer grids landing on the coarse.
- [primes](primes/) - A number is prime when its stones make one rectangle, shown by the sieve, the divisor pairs, the `pi(x)` staircase against `x / ln x` and `li(x)`, and a carpet stack whose layers correlate to zero exactly at the primes.
- [race](race/) - Two base-3 designs of the same mass and the same fractal dimension carry random walkers from home at different speeds, so the shape sets the walk, not the density.
- [radial](radial/) - Turned copies of a design laid on each other keep only the circular harmonics whose order is a multiple of the copy count, and a design of rotation order `g` shows `lcm(q, g)` petals.
- [sequences](sequences/) - The searchable ledger of every integer sequence the designs write, with closed forms and the OEIS entry each one matches.
- [slices](slices/) - The central diagonal cut of a cube of odd side `n = 2k-1` is a regular hexagon of `6n^2` unit triangles, and a design's parity rule fills them into many pieces or one pierced piece as `k` alternates.
- [spectra](spectra/) - The normalised Laplacian of a design's graph puts a third of the Sierpinski triangle's spectrum on the single eigenvalue 1, and the slope of the low end reads the random-walk spectral dimension.
- [spin](spin/) - A design on a turntable strobed against the frame rate, beside the exact circle mean at every radius, which is the bullseye it becomes at infinite speed.
- [sponge](sponge/) - A code picks the filled corners of a cube and grows it level by level, with fills, voids and exposed faces answered by closed formulas before a cube is built.
- [tour](tour/) - A dozen cards, each drawing a design live beside the integer sequence it counts and the OEIS record that holds the terms.
- [ulam](ulam/) - The whole numbers wound on squares or hexagons with the primes lit, where every straight line reads a quadratic and the prime-rich ones stand out as diagonals.
- [universe](universe/) - Rotations and reflections fold the corner bitmasks of a hypercube into orbits, so the distinct designs of dimensions 1 to 4 are a finite gallery you can grow one by one.
- [volume](volume/) - The moire stack of a cube design as a solid field, shelled at a level set and cut on any plane, where the central diagonal cut is the hexagon.
- [zeta](zeta/) - Zeta walked at `s = 1/2 + it` passes through the origin once per zero, and the zeros added one at a time fold a smooth curve into the prime staircase.
