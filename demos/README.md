# demos

- The eyes of MrlyMath: browser pages that draw what the crates compute through `mrlyweb` and wasm.
- Rust is the only math; the pages only draw. React renders the pages, Three.js draws the 3D; no other dependency.
- `bun install` fetches them; `bun run wasm` builds `pkg/` from `crates/mrlyweb` with wasm-pack.
- One page is one folder: a thin `<name>/index.html` shell plus `<name>/index.jsx`; the gallery is the root `index.html` + `index.jsx`.
- `lib/` holds the shared code: `mrly.js`, `app.jsx`, `draw.jsx`, `select.jsx`, `stage.jsx` + `stage.js`, `chart.js`, `series.jsx`, `query.js`, `mrly.css`; `series.jsx` is the sequence-view kit, `Pins`, `Staircase`, `Digits`, `Ratios`, `Differences` and the `Terms` ribbon, so no page prints a bare comma list.
- `app.jsx` is the chrome: `mount`, `Page`, `Row` and the controls; `draw.jsx` wraps every canvas: `Grid`, `Signs`, `Pixels`, `Sketch`, `Markup`.
- `Signs` is the plus-minus primitive: a warm hue for plus one, a cool hue for minus one, and the dark ground for empty.
- `select.jsx` is the one picker: design list, code, base and Randomize; `?seed=7` replays the seventh tap, and a typed code drops the seed.
- `useQuery` in `query.js` keeps page state in the URL, so every view is a link.
- `bun run dev` serves every page at `localhost:3000` as `/<name>`; `bun run build` writes the static site to `dist/`.
- `bun run build` also generates the papers and research pages into `dist/` from the shelf and the research tree, through `scripts/site.ts` and `lib/md.js`.
- `bun run check` prints the fixture numbers the crate's host test asserts; both must agree.
- `pkg/`, `dist/` and `node_modules/` are build output and stay out of git.

## PAGES

- The shelves, the cards and this list are one file: `pages.json`; a new page is one row there and nothing else.

### Designs

- A rule on the corners of a cube, folded into itself, then grown, cropped and counted.

- [universe](universe/) - Rotations and reflections fold the corner masks of a hypercube into orbits, so the distinct designs of dimensions 1 to 4 are a finite gallery you can grow one by one.
- [sponge](sponge/) - A code picks the filled corners of a cube and grows it level by level, with fills, voids and exposed faces answered by closed formulas before a cube is built.
- [tile](tile/) - One design repeated: side by side on the square lattice in the plane and in the cube, interlocked as a hexagon on the triangular one, where the fills multiply by the copy count exactly and the exposed faces do not.
- [crop](crop/) - A named shape of rational radius keeps only the cells of a design it reaches, with the in, cut and out regions counted exactly before anything is drawn.
- [tour](tour/) - A dozen cards, each drawing a design live beside the integer sequence it counts and the OEIS record that holds the terms.

### Slices and stacks

- Meet a design with a plane, or lay it over turned and scaled copies of itself.

- [cuts](cuts/) - Every plane x + y + z = s through a level-L solid meets exactly 3^L cells, and the height's binary digits make each cut a Sierpinski gasket.
- [slices](slices/) - The central diagonal cut of a cube of odd side n = 2k-1 is a regular hexagon of 6n^2 unit triangles, and a design's parity rule fills them into many pieces or one pierced piece as k alternates.
- [spectrometer](spectrometer/) - The inked share of the diagonal slice is an exact closed form in a design's Walsh spectrum, so the hexagon's two-step blink over the odd sides reads the eight-corner recipe back.
- [volume](volume/) - The moire stack of a cube design as a solid field, shelled at a level set and cut on any plane, where the central diagonal cut is the hexagon.
- [tower](tower/) - The tile held to one axis with the word rising a letter per block, where a block's fill fraction falls geometrically and its exposed count climbs, so the volume converges while the surface diverges.
- [carry](carry/) - The diagonal cut of a base-q sponge remembers only a carry, so ceil(D/2) past terms decide every count and the growth exponent misses the generic value, above it at odd dimensions and below it at even ones.
- [moire](moire/) - One design sampled at scale 1, 3, 5 and on, the layers stacked into a field where the interference is the finer grids landing on the coarse.
- [radial](radial/) - Turned copies of a design laid on each other keep only the circular harmonics whose order is a multiple of the copy count, and a design of rotation order g shows lcm(q, g) petals.
- [spin](spin/) - A design on a turntable strobed against the frame rate, beside the exact circle mean at every radius, which is the bullseye it becomes at infinite speed.

### Words and order

- Let the rule change with the scale, and read what the order of the letters costs.

- [words](words/) - One design per level folded by the Kronecker product, with the census, the component exponent, and what changes when the letters swap places.
- [morse](morse/) - The Thue-Morse word built twice from one digit rule, lifted to four plane grids of which three are Kronecker powers of a plus-minus tile and one is not, with its runs and its boundary word.

### Graphs, walks and spectra

- Join the filled cells and listen: the network, its spectrum, and the walk it carries.

- [graphs](graphs/) - Joining every filled cell to its neighbours turns a design into a network with tips, junctions, pieces, length and a box dimension, flat, in the cube, on the hexagonal slice, or relaxed by force.
- [spectra](spectra/) - The normalised Laplacian of a design's graph puts a third of the Sierpinski triangle's spectrum on the single eigenvalue 1, and the slope of the low end reads the random-walk spectral dimension.
- [race](race/) - Two base-3 designs of the same mass and the same fractal dimension carry random walkers from home at different speeds, so the shape sets the walk, not the density.

### Primes in the lattice

- Where the primes fall once the whole numbers are laid out on a grid.

- [primes](primes/) - A number is prime when its stones make one rectangle, shown by the sieve, the divisor pairs, the pi(x) staircase against x / ln x and li(x), and a carpet stack whose layers correlate to zero exactly at the primes.
- [ulam](ulam/) - The whole numbers wound on squares or hexagons with the primes lit, where every straight line reads a quadratic and the prime-rich ones stand out as diagonals.
- [gaussian](gaussian/) - The Gaussian and Eisenstein primes as four- and six-armed snowflakes, coloured by whether an ordinary prime split, stayed inert, or ramified on entering the plane.

### Fractions and zeros

- Rational scales and the critical line, both counting the primes the long way.

- [farey](farey/) - A line drawn at every k/n and stacked over the scales lights a reduced fraction a/b to a brightness of Q/b, and the scales of maximal novelty are the primes.
- [zeta](zeta/) - Zeta walked at s = 1/2 + it passes through the origin once per zero, and the zeros added one at a time fold a smooth curve into the prime staircase.

### The ledger

- Every sequence the designs write, every integer they reach, and rules read out of the registry.

- [sequences](sequences/) - The searchable ledger of every integer sequence the designs write, with closed forms and the OEIS entry each one matches.
- [plot](plot/) - Any sequence the ledger holds drawn rather than listed, with the smallest linear recurrence its terms satisfy, its characteristic polynomial and its growth read out beside it, and a second sequence mixed in to see the rule a blend inherits.
- [integers](integers/) - The union of every sequence the registry writes, read integer by integer: which of the first thousand the designs write, how many rows write each, and which are missed inside the pinned window.
- [life](life/) - Cellular automata whose birth and survival rules are read from named sequences, run until the grid dies, freezes or loops.
