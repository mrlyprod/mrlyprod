# org

- The mrly.net site: the demos that draw MrlyMath, the papers, the research pages, a blog and an about page, every route static HTML.
- Rust is the only math; the pages only draw. React renders the demo pages, Three.js draws the 3D; no other dependency.
- `bun install` fetches them; `bun run wasm` builds `pkg/` from `crates/mrlyweb` with wasm-pack.
- One demo is one folder: a thin `demos/<name>/index.html` shell plus `demos/<name>/index.jsx`; the gallery is `demos/index.html` + `demos/index.jsx`.
- `lib/` holds the shared code: `mrly.js`, `app.jsx`, `tree.js`, `draw.jsx`, `select.jsx`, `stage.jsx` + `stage.js`, `chart.js`, `series.jsx`, `query.js`, `md.js`, `logo.js`, `thumbs.jsx`, `mrly.css`; `series.jsx` is the sequence-view kit, `Pins`, `Staircase`, `Digits`, `Ratios`, `Differences` and the `Terms` ribbon, so no page prints a bare comma list.
- `thumbs.jsx` is the one place a demo's thumbnail is drawn: the gallery tile calls `thumb(m, name)`.
- The chrome is the kit in `../ui`: `app.jsx` wraps its `Shell` as `Page` beside `mount`, `Row` and the controls, `tree.js` fills the site tree from `pages.json` plus the papers, research and blog lists, and `mrly.css` imports the kit and keeps only demo rules; `draw.jsx` wraps every canvas: `Grid`, `Signs`, `Pixels`, `Sketch`, `Markup`.
- `Signs` is the plus-minus primitive: a warm hue for plus one, a cool hue for minus one, and the dark ground for empty.
- `select.jsx` is the one picker: design list, code, base and Randomize; `?seed=7` replays the seventh tap, and a typed code drops the seed.
- `useQuery` in `query.js` keeps page state in the URL, so every view is a link.
- Words live as markdown: `blog/<slug>.md` and `pages/about.md` open with a `---` front matter block (title, date, lead, optional figure naming a file in `files/figures/`); `public/` copies straight to the site root.
- `bun run dev` is the one command: it generates the static routes into `dist/` and serves them with every React page at `localhost:3000`.
- `bun run build` writes the whole site to `dist/`: `scripts/clean.ts` empties it, `bun build` bundles the React pages, `scripts/site.ts` writes every static route; pure bun, no Chrome, no cargo, no Python.
- Two inputs are made on the desk and read at build time: `pkg/` from `bun run wasm` and `../../files/figures/` from `bun run figures` (the `mrlyfig` crate); the build fails with the list when a route's figure is missing.
- Figures are named by route: `research-<page>`, `paper-<slug>`, `blog-<slug>`, `site-home`, `site-demos`, `site-papers`, `site-research`, `site-og` (1200x630) and `site-icon`; a research or blog page opens on its square figure, a paper page opens on its avatar, the cards and the doors use the same files.
- `scripts/shelf.ts` fetches the paper shelf from GitHub into `data/shelf/` at every build and falls back to the cached copy offline; `MRLY_SHELF=/path/to/research` reads a local checkout instead.
- Routes: `/`, `/demos/`, `/demos/<name>/`, `/papers/`, `/papers/<slug>/`, `/research/`, `/research/<name>/`, `/blog/`, `/blog/<slug>/`, `/about/` and `/404.html`, beside `sitemap.xml`, `robots.txt`, `favicon.svg`, `apple-touch-icon.png`, `icon-512.png` and `manifest.webmanifest`.
- Every route carries a canonical link, a description, Open Graph and Twitter cards pointing at the one `/og.png`, and JSON-LD where it has an author.
- `bun run check` prints the fixture numbers the crate's host test asserts; both must agree.
- `pkg/`, `dist/`, `data/` and `node_modules/` are build output and stay out of git.
## PAGES

- The shelves, the cards and this list are one file: `pages.json`; a new page is one row there and nothing else.

### Designs

- A rule on the corners of a cube, folded into itself, then grown, cropped and counted.

- [universe](demos/universe/) - Rotations and reflections fold the corner masks of a hypercube into orbits, so the distinct designs of dimensions 1 to 4 are a finite gallery you can grow one by one.
- [sponge](demos/sponge/) - A code picks the filled corners of a cube and grows it level by level, with fills, voids and exposed faces answered by closed formulas before a cube is built.
- [tile](demos/tile/) - One design repeated: side by side on the square lattice in the plane and in the cube, interlocked as a hexagon on the triangular one, where the fills multiply by the copy count exactly and the exposed faces do not.
- [crop](demos/crop/) - A named shape of rational radius keeps only the cells of a design it reaches, with the in, cut and out regions counted exactly before anything is drawn.
- [tour](demos/tour/) - A dozen cards, each drawing a design live beside the integer sequence it counts and the OEIS record that holds the terms.
- [mrlylife](demos/mrlylife/) - Life with the neighbourhood set free: the mask is a design at any side and level, the birth and survival counts come by hand or from a named sequence, and the board runs in one dimension or two.
- [wolfram](demos/wolfram/) - Wolfram's 256 elementary rules are the 256 three-dimensional parity designs bit for bit, so every rule arrives with a design's card, and the additive rules draw the plane designs in time.

### Slices and stacks

- Meet a design with a plane, or lay it over turned and scaled copies of itself.

- [cuts](demos/cuts/) - Every plane x + y + z = s through a level-L solid meets exactly 3^L cells, and the height's binary digits make each cut a Sierpinski gasket.
- [slices](demos/slices/) - The central diagonal cut of a cube of odd side n = 2k-1 is a regular hexagon of 6n^2 unit triangles, and a design's parity rule fills them into many pieces or one pierced piece as k alternates.
- [spectrometer](demos/spectrometer/) - The inked share of the diagonal slice is an exact closed form in a design's Walsh spectrum, so the hexagon's two-step blink over the odd sides reads the eight-corner recipe back.
- [volume](demos/volume/) - The moire stack of a cube design as a solid field, shelled at a level set and cut on any plane, where the central diagonal cut is the hexagon.
- [tower](demos/tower/) - The tile held to one axis with the word rising a letter per block, where a block's fill fraction falls geometrically and its exposed count climbs, so the volume converges while the surface diverges.
- [carry](demos/carry/) - The diagonal cut of a base-q sponge remembers only a carry, so ceil(D/2) past terms decide every count and the growth exponent misses the generic value, above it at odd dimensions and below it at even ones.
- [moire](demos/moire/) - One design sampled at scale 1, 3, 5 and on, the layers stacked into a field where the interference is the finer grids landing on the coarse.
- [radial](demos/radial/) - Turned copies of a design laid on each other keep only the circular harmonics whose order is a multiple of the copy count, and a design of rotation order g shows lcm(q, g) petals.
- [spin](demos/spin/) - A design on a turntable strobed against the frame rate, beside the exact circle mean at every radius, which is the bullseye it becomes at infinite speed.

### Words and order

- Let the rule change with the scale, and read what the order of the letters costs.

- [words](demos/words/) - One design per level folded by the Kronecker product, with the census, the component exponent, and what changes when the letters swap places.
- [morse](demos/morse/) - The Thue-Morse word built twice from one digit rule, lifted to four plane grids of which three are Kronecker powers of a plus-minus tile and one is not, with its runs and its boundary word.

### Graphs, walks and spectra

- Join the filled cells and listen: the network, its spectrum, and the walk it carries.

- [graphs](demos/graphs/) - Joining every filled cell to its neighbours turns a design into a network with tips, junctions, pieces, length and a box dimension, flat, in the cube, on the hexagonal slice, or relaxed by force.
- [spectra](demos/spectra/) - The normalised Laplacian of a design's graph puts a third of the Sierpinski triangle's spectrum on the single eigenvalue 1, and the slope of the low end reads the random-walk spectral dimension.
- [race](demos/race/) - Two base-3 designs of the same mass and the same fractal dimension carry random walkers from home at different speeds, so the shape sets the walk, not the density.

### Primes in the lattice

- Where the primes fall once the whole numbers are laid out on a grid.

- [primes](demos/primes/) - A number is prime when its stones make one rectangle, shown by the sieve, the divisor pairs, the pi(x) staircase against x / ln x and li(x), and a carpet stack whose layers correlate to zero exactly at the primes.
- [ulam](demos/ulam/) - The whole numbers wound on squares or hexagons with the primes lit, where every straight line reads a quadratic and the prime-rich ones stand out as diagonals.
- [gaussian](demos/gaussian/) - The Gaussian and Eisenstein primes as four- and six-armed snowflakes, coloured by whether an ordinary prime split, stayed inert, or ramified on entering the plane.

### Fractions and zeros

- Rational scales and the critical line, both counting the primes the long way.

- [farey](demos/farey/) - A line drawn at every k/n and stacked over the scales lights a reduced fraction a/b to a brightness of Q/b, and the scales of maximal novelty are the primes.
- [zeta](demos/zeta/) - Zeta walked at s = 1/2 + it passes through the origin once per zero, and the zeros added one at a time fold a smooth curve into the prime staircase.

### The ledger

- Every sequence the designs write, every integer they reach, and rules read out of the registry.

- [sequences](demos/sequences/) - The searchable ledger of every integer sequence the designs write, with closed forms and the OEIS entry each one matches.
- [plot](demos/plot/) - Any sequence the ledger holds drawn rather than listed, with the smallest linear recurrence its terms satisfy, its characteristic polynomial and its growth read out beside it, and a second sequence mixed in to see the rule a blend inherits.
- [integers](demos/integers/) - The union of every sequence the registry writes, read integer by integer: which of the first thousand the designs write, how many rows write each, and which are missed inside the pinned window.
- [life](demos/life/) - Conway's rule on the eight cells around, which are the side-3 carpet tile with its centre popped, seeded by soup, glider or R-pentomino and run to its fate.
