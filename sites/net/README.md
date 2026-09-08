# org

- The mrly.net site: the demos that draw MrlyMath, the papers, the research pages, a blog and an about page, every route static HTML.
- Rust is the only math; the pages only draw. React renders the demo pages, Three.js draws the 3D; no other dependency.
- `bun install` fetches them; `bun run wasm` builds `pkg/` from `crates/mrlyweb` with wasm-pack.
- One demo is one folder: a thin `demos/<name>/index.html` shell plus `demos/<name>/index.jsx`; the gallery is `demos/index.html` + `demos/index.jsx`.
- `lib/` holds the shared code: `mrly.js`, `app.jsx`, `tree.js`, `draw.jsx`, `select.jsx`, `stage.jsx` + `stage.js`, `chart.js`, `series.jsx`, `query.js`, `md.js`, `mrly.css`; `series.jsx` is the sequence-view kit, `Pins`, `Staircase`, `Digits`, `Ratios`, `Differences` and the `Terms` ribbon, so no page prints a bare comma list.
- The demo gallery draws no thumbnails: every tile is the demo's figure pair, `demo-<name>-dark.png` and `demo-<name>-light.png`, and the demos route ships them.
- The chrome is the kit in `../../pkgs/js/mrlyjs/ui` and its README is the reference for it: the header glyphs, the three columns, the drawers, the menu and the settings read from `site.json`, and the footer is one screen, the wordmark held by the pixel-font animation over a copyright line, with no links; `app.jsx` wraps its `Shell` as `Page` beside `mount`, `Row` and the controls, `tree.js` fills the site tree from `pages.json` plus the papers, research and blog lists, and `mrly.css` imports the kit and keeps only demo rules; `draw.jsx` wraps every canvas: `Grid`, `Signs`, `Pixels`, `Sketch`, `Markup`.
- `Signs` is the plus-minus primitive: a warm hue for plus one, a cool hue for minus one, and the dark ground for empty.
- `select.jsx` is the one picker: design list, code, base and Randomize; `?seed=7` replays the seventh tap, and a typed code drops the seed.
- `useQuery` in `query.js` keeps page state in the URL, so every view is a link.
- Words live as markdown: `blog/<slug>.md` and every `pages/<slug>.md` open with a `---` front matter block (title, date, lead, optional figure naming a file in `files/figures/`); one page file is one route, `/<slug>/`; `public/` copies straight to the site root.
- `bun run dev` renders on request: it scans once, watches every declared input and template, and renders the route you ask for; nothing is prebuilt and `dist/` is never read.
- In dev the kit and the figures are served straight from disk and the demos keep Bun's HTML routes, so a CSS or markdown edit shows on the next refresh with no build.
- `bun run build` writes the whole site to `dist/` with `scripts/site.ts` alone: pure bun, no Chrome, no cargo, no Python; `bun run clean` empties `dist/` by hand when you want a cold start.
- The demos are one route: one in-process `Bun.build()` over `demos/index.html` and `demos/*/index.html`, its SEO head injected before the shells are written, fingerprinted over `demos/`, `lib/`, `pkg/` and the kit.
- A second `bun run build` renders nothing: every route is fingerprinted into `.cache/manifest.json` with the files it wrote, and dead outputs are deleted by that record.
- Two inputs are made on the desk and read at build time: `pkg/` from `bun run wasm` and `../../files/figures/` from `bun run figures` (the `mrlyfig` crate); every figure is a pair, `<name>-dark.png` and `<name>-light.png`, the page carries both as `img.dark` and `img.light` and the theme shows one; a missing half throws while its own route renders and names the route.
- Figures are named by route: `research-<page>`, `paper-<slug>`, `blog-<slug>`, `site-home`, `site-demos`, `site-papers`, `site-research`, `site-contact`, `site-donate`, `demo-<name>` and `site-og` (1200x630); a research or blog page opens on its square figure, a markdown page opens on the one its front matter names, a paper page opens on its avatar, the cards and the doors use the same files.
- `scripts/shelf.ts` fetches the paper shelf from GitHub into `data/shelf/` at every build and falls back to the cached copy offline; `MRLY_SHELF=/path/to/research` reads a local checkout instead, which `bun run dev` sets to the cached copy so a rescan never waits on the network.
- Routes: `/`, `/demos/`, `/demos/<name>/`, `/papers/`, `/papers/<slug>/`, `/research/`, `/research/<name>/`, `/blog/`, `/blog/<slug>/`, `/menu/`, `/about/`, `/contact/`, `/donate/`, `/cart/`, `/git/...`, `/raw/...` and `/404.html`, beside `sitemap.xml`, `robots.txt`, `llms.txt`, `manifest.webmanifest` and the icons the builder draws from the logo.
- `/menu/` lays the whole navigator out with the kit's `Menu` as a figure grid, one picture tile per route with its blurb, closing on an `Elsewhere` list of the socials and the contact address; `/cart/` is a placeholder and, like `/404.html`, stays out of the tree and out of the sitemap.
- `/git/` is the code viewer from `../../pkgs/js/mrlyjs/git`: it browses this repo's own tracked tree, `/raw/` serves the bytes, and the `git` block in `site.json` names the root, the GitHub slug and the branch.
- The tree carries one collapsed `Code` node; the pages under it stay out of the navigator, and every one of them plus every `/raw/` object enters the sitemap.
- `site.json` carries the `llms` block: the paragraph `llms.txt` opens on and the links it points at, `/raw/README.md`, `/research/`, `/git/` and `/papers/`.
- `robots.txt` allows everything and names GPTBot, ClaudeBot, Claude-Web, CCBot, Google-Extended, anthropic-ai and PerplexityBot one block each.
- The highlighter is the kit's, server-side Shiki over 16 grammars; `ui/code.css` rides with `seti.css` on every code page.
- `public/pages.css` carries only what the kit has no rule for: the tiles, the home, the openers, the plates and the `Elsewhere` list; the code viewer's own CSS lives in the kit's `code.css`.
- Every route carries a canonical link, a description, Open Graph and Twitter cards pointing at the one `/og.png`, and JSON-LD where it has an author.
- `bun run check` prints the fixture numbers the crate's host test asserts; both must agree.
- `site.json` declares every input the build reads: `readme pages blog research figures demos lib pkg ui public`; nothing is resolved by hand, so a path moves in one place.
- `pkg/`, `dist/`, `data/`, `.cache/` and `node_modules/` are build output and stay out of git.

## PAGES

- The shelves, the cards and this list are one file: `pages.json`; a new page is one row there and nothing else.

### Designs

- A rule on the corners of a cube, folded into itself, then grown, cropped and counted.

- [universe](demos/universe/) - Rotations and reflections fold the corner masks of a hypercube into orbits, so the distinct designs of dimensions 1 to 4 are a finite gallery you can grow one by one.
- [sponge](demos/sponge/) - A code picks the filled corners of a cube and grows it level by level, with fills, voids and exposed faces answered by closed formulas before a cube is built.
- [tile](demos/tile/) - One design repeated: side by side on the square lattice in the plane and in the cube, interlocked as a hexagon on the triangular one, where the fills multiply by the copy count exactly and the exposed faces do not.
- [crop](demos/crop/) - A named shape of rational radius keeps only the cells of a design it reaches, counted exactly in in, cut and out regions, and the disc count read radius by radius carries a self-similar main term with a log-periodic multiplier.
- [shell](demos/shell/) - The cells a circle crosses on a design collapse by threes into a rooted tree whose every level holds exactly 2 floor(r / 3^j) + 1 boxes, and the cells the design fills are the leaves whose path never takes a centre seat.
- [tube](demos/tube/) - Fattening a design by a radius and measuring what it swallows gives an inner tube whose Minkowski reading never settles but circles one log-periodic profile, exact in closed form wherever the holes are isolated squares.
- [weights](demos/weights/) - Give every filled corner of a design a weight and the support never moves while the mass does, so the pressure and its Legendre transform are closed forms in the weights alone and the multifractal spectrum becomes a curve you steer with sliders.
- [tour](demos/tour/) - A dozen cards, each drawing a design live beside the integer sequence it counts and the OEIS record that holds the terms.
- [mrlylife](demos/mrlylife/) - Life with the neighbourhood set free: the mask is a design at any side and level, the birth and survival counts come by hand or from a named sequence, and the board runs in one dimension or two.
- [chladni](demos/chladni/) - A Larger-than-Life rule on a big design mask runs a soup to a still, and the ring where the still's spectrum peaks, read beside the mask's own spectrum, is the wavelength the rule prefers.
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

- [star](demos/star/) - Stacking the hexagonal cuts of a carpet, one per odd side, stands a six-armed star at the centre that is not there in the limit: its arm's ink is exactly 1/2 + chi_8(n)/(2n) and its decay is exactly -1/4 read on the cube's own cells, but a different number in every frame that resamples it or widens it.

### Words and order

- Let the rule change with the scale, and read what the order of the letters costs.

- [words](demos/words/) - One design per level folded by the Kronecker product, with the census, the component exponent, and what changes when the letters swap places.
- [morse](demos/morse/) - The Thue-Morse word built twice from one digit rule, lifted to four plane grids of which three are Kronecker powers of a plus-minus tile and one is not, with its runs and its boundary word.
- [wallis](demos/wallis/) - Level k drops the centre of every surviving square cut into (2k + 1)^2, so a schedule of strictly increasing odd letters keeps a positive area and hands back pi over four, while one that reuses a letter, alternating or not, loses the area and buys a dimension instead.

### Graphs, walks and spectra

- Join the filled cells and listen: the network, its spectrum, and the walk it carries.

- [graphs](demos/graphs/) - Joining every filled cell to its neighbours turns a design into a network with tips, junctions, pieces, length and a box dimension, flat, in the cube, on the hexagonal slice, or relaxed by force.
- [spectra](demos/spectra/) - The normalised Laplacian of a design's graph puts a third of the Sierpinski triangle's spectrum on the single eigenvalue 1, and the slope of the low end reads the random-walk spectral dimension.
- [race](demos/race/) - Two base-3 designs of the same mass and the same fractal dimension carry random walkers from home at different speeds, so the shape sets the walk, not the density.
- [modes](demos/modes/) - Laying a design's level-L mask over the torus picks out its modes, so every eigenvalue is a product of L rescaled copies of the tile's own transform and the field of eigenvalues is self-similar, a picture of the tile.

### Primes in the lattice

- Where the primes fall once the whole numbers are laid out on a grid.

- [primes](demos/primes/) - A number is prime when its stones make one rectangle, shown by the sieve, the divisor pairs, the pi(x) staircase against x / ln x and li(x), and a carpet stack whose layers correlate to zero exactly at the primes.
- [ulam](demos/ulam/) - The whole numbers wound on squares or hexagons with the primes lit, where every straight line reads a quadratic and the prime-rich ones stand out as diagonals.
- [snail](demos/snail/) - Every cell of the square winding that grows carries a design tile whose side is a power of the base, so the spiral widens by that factor at each new digit and curls outward like a shell, built of the primes alone or of every number by the same law.
- [gaussian](demos/gaussian/) - The Gaussian and Eisenstein primes as four- and six-armed snowflakes, coloured by whether an ordinary prime split, stayed inert, or ramified on entering the plane.

### Fractions and zeros

- Rational scales and the critical line, both counting the primes the long way.

- [farey](demos/farey/) - A line drawn at every k/n and stacked over the scales lights a reduced fraction a/b to a brightness of Q/b, and the scales of maximal novelty are the primes.
- [pi](demos/pi/) - The points of the grid a corner can see take a share of six over pi squared, so counting the lit ones in a window hands pi back, and the dimension counted in picks which zeta value falls out.
- [zeta](demos/zeta/) - Zeta walked at s = 1/2 + it passes through the origin once per zero, and the zeros added one at a time fold a smooth curve into the prime staircase.
- [formulas](demos/formulas/) - Eight elementary systems on one dial: the Wallis product, the Leibniz series and the Basel sum closing on pi, the harmonic sum on gamma and (1 + 1/n)^n on e, the prime count against li, Goldbach's partition count of 2n and the Mertens sum against the square root of n.
- [echo](demos/echo/) - The Mobius meter of a digit design oscillates at the ordinates of the Riemann zeta zeros and never at the design's own pole lattice, because it is the classical Mertens function heard through the design's density: split that echo off and the zeros leave with it.

### The ledger

- Every sequence the designs write, every integer they reach, and rules read out of the registry.

- [sequences](demos/sequences/) - The searchable ledger of every integer sequence the designs write, with closed forms and the OEIS entry each one matches.
- [plot](demos/plot/) - Any sequence the ledger holds drawn rather than listed, with the smallest linear recurrence its terms satisfy, its characteristic polynomial and its growth read out beside it, and a second sequence mixed in to see the rule a blend inherits.
- [integers](demos/integers/) - The union of every sequence the registry writes, read integer by integer: which of the first thousand the designs write, how many rows write each, and which are missed inside the pinned window.
- [life](demos/life/) - Conway's rule on the eight cells around, which are the side-3 carpet tile with its centre popped, seeded by soup, glider or R-pentomino and run to its fate.

### The alphabet

- The pixel font that writes the wordmark: every glyph and the order its pen draws it.

- [font](demos/font/) - Every glyph of MrlyFont writing itself in its hand-penned stroke order, with its stroke count against the parity floor and a chip on any glyph that could be drawn in fewer.
