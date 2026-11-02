# NAMES

- Every name MrlyProd uses, grouped by kind: the name and a few words, nothing else.

## BRANDS

- `MrlyProd`: the company; MrlyProd, Inc.
- `MRLYPROD`: the wordmark
- `MrlyNet`: the platform; spec, kernel, shell, doors, designs
- `MrlyMath`: the naming of designs; one JSON object each
- `MrlyFont`: the stroked pixel font, 108 characters
- `mrlydraw`: the pen and its kit
- `Mrly`: the short mark; mrly.net's Tiles saver title

## SITES

- `mrly.net`: MrlyNet's first site; config in `net/mrly.net/`
- `mrlyprod.com`: MrlyProd's own computer, a self-hosted MrlyNet; apps and research
- `mrlyprod.org`: MrlyProd's projects, research, toys and docs
- `carlomitchener.com`: Carlo's site, a self-hosted MrlyNet

## REPOS

```
github.com/mrlyprod/   holds                                     today
mrlyprod               the public face                           this repo, the monorepo
spec                   MrlyNet's schemas and conformance suite   net/spec
sdk                    MrlyNet's SDK; what an app imports        net/sdk
net                    MrlyNet, the reference distribution       net/
modern                 design: macOS and iPadOS, Vista blur      designs/modern
boxy                   design: boxes, one unit, a hue per app    designs/boxy
classic                design: grey bevels, Win 95, Web 1.0      designs/classic
bricks                 design: toy bricks, studs, shadows        designs/bricks
terminal               design: phosphor and teletype, ASCII      designs/terminal
liquid                 design prototype: liquid glass            mrlyprod/liquid
pixel                  design prototype: sparks on every press   new
bridge                 mrly-bridge                               mrlyprod/bridge
core                   mrly-core                                 mrlyprod/core
num                    mrly-num                                  mrlyprod/num
math                   mrly-math                                 mrlyprod/math
tile                   mrly-tile                                 mrlyprod/tile
life                   mrly-life                                 mrlyprod/life
font                   mrly-font                                 mrlyprod/font
sequences              mrly-sequences                            mrlyprod/sequences
draw                   mrlydraw                                  mrlyprod/draw
apps                   MrlyProd's apps                           apps/
research               MrlyProd's research                       mrlyprod/research
com                    mrlyprod.com                              new
org                    mrlyprod.org                              org/
```

## NPM PACKAGES

```
name                      folder today         note
mrlynet                   ./                   the root package; bin mrlynet
@mrlyprod/spec            net/spec
@mrlyprod/kernel          net/kernel
@mrlyprod/sdk             net/sdk
@mrlyprod/shell           net/shell
@mrlyprod/system          net/system
@mrlyprod/draw            mrlyprod/draw        mrlydraw
@mrlyprod/modern          designs/modern       was @mrlyprod/design-glass
@mrlyprod/boxy            designs/boxy         was @mrlyprod/design-mrly
@mrlyprod/classic         designs/classic      was @mrlyprod/design-classic
@mrlyprod/bricks          designs/bricks       was @mrlyprod/design-blocks
@mrlyprod/terminal        designs/terminal     was @mrlyprod/design-terminal
@mrlyprod/liquid          mrlyprod/liquid      prototype
@mrlyprod/pixel           -                    prototype; new
@mrlyprod/design-shared   designs/shared       dissolves into net
@mrlyprod/core            mrlyprod/core/js     wasm
@mrlyprod/num             mrlyprod/num/js     wasm
@mrlyprod/math            mrlyprod/math/js     wasm
@mrlyprod/tile            mrlyprod/tile/js     wasm
@mrlyprod/life            mrlyprod/life/js     wasm
@mrlyprod/font            mrlyprod/font/js     wasm
```

## CRATES

```
crate            folder             holds
mrly-core        mrlyprod/core      tensors, cells, colors, images, seeded chance, names
mrly-num         mrlyprod/num       primes, divisors, series, sieves, lattices, spectra
mrly-math        mrlyprod/math      the designs: codes, cells, cubes, hexagons, wheels
mrly-tile        mrlyprod/tile      the tiles, from a recipe to a file
mrly-life        mrlyprod/life      a rule over a grid, stepped and rendered
mrly-font        mrlyprod/font      MrlyFont, painted or written stroke by stroke
mrly-sequences   mrlyprod/sequences the sequence rows and their dataset
mrly-bridge      mrlyprod/bridge    one crate in, its JS, Python and CLI out; was bridge
```

## BINDINGS

```
unit   js crate        py crate        cli crate        npm              PyPI        import      bin
core   mrly-core-js    mrly-core-py    mrly-core-cli    @mrlyprod/core   mrly-core   mrly.core   mrly-core
num    mrly-num-js     mrly-num-py     mrly-num-cli     @mrlyprod/num    mrly-num    mrly.num    mrly-num
math   mrly-math-js    mrly-math-py    mrly-math-cli    @mrlyprod/math   mrly-math   mrly.math   mrly-math
tile   mrly-tile-js    mrly-tile-py    mrly-tile-cli    @mrlyprod/tile   mrly-tile   mrly.tile   mrly-tile
life   mrly-life-js    mrly-life-py    mrly-life-cli    @mrlyprod/life   mrly-life   mrly.life   mrly-life
font   mrly-font-js    mrly-font-py    mrly-font-cli    @mrlyprod/font   mrly-font   mrly.font   mrly-font
```

- `mrly.<unit>._<unit>`: each Python package's native module
- `mrly-core-js-hand`, `mrly-core-py-hand`, `mrly-core-cli-hand`: hand-written crossings every leaf shares; unpublished
- `mrly-math-js-hand`, `mrly-math-py-hand`: math's hand-written crossings; unpublished
- `mrly-research`: the root Python project; research labs
- Hand types: `Tensor`, `Cell`, `Image`, `Color`, `Rng` (core); `CellNd`, `Cell6d`, `Code` (math)

## CRATE MODULES

```
crate            modules
mrly-core        cell colors error image name named paint ramp resample rng seeds tensor
mrly-num         apollonian blend boolean chladni factor fft gauss kummer lattice morse prime readings series sieve spiral zeta
mrly-math        arcs arrays atoms automaton bang cell coprime counts design dissection eigen game graph ladder memory moire name nodal press radix roulette rules shape six spectrum spin spirograph sumset three tourbillon two
mrly-tile        build draw name recipe variation
mrly-life        animate crop elementary mask metrics models recipe render rule source step
mrly-font        animate bitmaps glyph names paths pens raster
mrly-sequences   measure
```

## MRLYMATH

- The words (`dim`, `base`, `code`, `level`, ...): the core repo's `src/name/NAMES.md`
- `Named`: the trait in `mrly_core::name`
- Kinds: `bang`, `rule`, `sequence`, `tile`, `word`
- `mrlybang`: a code as a set of parity corners
- `mrlylife`: the Life engine's named recipes

## SPEC

- `act`: one act in; `{ verb, args, who, at }`
- `line`: one journal line; an act with its `tick`
- `move`: one legal verb now
- `manifest`: an installed app
- `envelope`: the answer to every call and read
- `config`: what boots a world
- `fault`: a broken request
- `who.kind`: `human`, `org`, `bot`, `agent`
- Fault codes: `bad-act`, `bad-config`, `unknown-verb`, `unknown-world`, `no-iden`, `refused`, `spec-mismatch`
- `SPEC`: the spec version, `0.1`

## KERNEL

- `kernel`: installs apps, returns the four calls
- `boot`, `list`, `call`, `read`: the four calls
- `world`: one JSON state of every app
- `app`: an installed app; `{ id, user, title, line, boot, verbs }`
- `verb`: `<user>.<app>.<verb>`, or `<app>.<verb>` for a system app
- `nav`: the kernel's own verbs; `nav.open`
- `journal`: the lines; replay, seek, fork
- `goose`: the seeded random player
- `Fault`: the thrown error
- `tick`, `at`, `who`, `wish`, `beat`, `seed`: the act's and world's fields
- `canon`, `digest`, `mix`, `seedOf`: the determinism helpers

## VERBS

- `set`, `reroll`, `pause`: every app's verbs
- `turn`, `tick`, `starve`: snake
- `aim`, `tick`: race
- `draw`, `take`: chaos
- `clear`, `paint`, `rule`, `tick`: life
- `size`, `tick`: matrix
- `fit`, `set`, `stage`, `tick`: sleep

```
app          key acts
apollonian   cap
arcs         design up down
chaos        up down speed grid
crop         mode radius level
cuts         lower higher level
designs      grow design up down
gaussian     walk turn
graphs       level graph
morse        up down view
planets      next prev
plot         mix design terms cycle
polygons     shape palette
radial       whole copies step
radix        flat level preset every
readings     up down view
sequences    cycle
sfx          prev next
six          crop view up down
sleep        next prev
slices       smaller larger level
spin         speed needle
three        level radius
tour         next card step
two          seams level across
ulam         next line
wolfram      next prev
zoom         next prev
```

## DOORS

- `cli`: the `mrlynet` command
- `mcp`: `mrlynet mcp`; JSON-RPC over stdio
- `http`: `mrlynet serve`; `/api/1/worlds`
- `gym`: `mrlynet gym`; many worlds for trainers
- `mrlynet.py`: the gym's Python side
- `mrlynet://world/<path>`: an app as an MCP resource

## HOSTS

- `local`: a world in a folder, for the doors
- `web`: the build, the dev server, the cloud code
- `edge`: the CloudFront function
- `push`: ships `dist/` by manifest diff
- `builder`: the Lambda that builds main
- `stats`: the Lambda that charts the bucket
- `mrlynet-site`: the mrly.net builder Lambda (`aws/net.ts`)

## SYSTEM APPS

```
id             title          note
home           {title}        the desk, at /
menu           Menu           the Launchpad
settings       Settings       Appearance, Desktop & Dock, Screen Saver, Accessibility, Privacy & Data, About
stats          Stats          the CDN, the Lambdas, the bucket
kit            Kit            every control, glass, icon, tint, theme
git            Code           the source tree and raw bytes
screensavers   Screensavers   preview and tune every saver
missing        Nothing here   the 404; hidden
redirects      Redirects      the moved urls; hidden
```

## ORG PAGES

```
id         title      note
about      About      prose
blog       Blog       the post list
cart       Cart       coming soon
contact    Contact    prose
donate     Donate     prose
figures    Figures    every live figure; hidden
method     Method     prose
mrlymath   MrlyMath   the MrlyMath names
research   Research   the topics and their figures
```

- `launching-mrlyprod-org`: the first blog post

## SHELL

- Required parts: `bar`, `window`, `app`, `menu`, `control`, `group`, `dialog`, `prose`, `tile`, `launcher`
- Optional parts: `desk`, `dock`, `widgets`, `lock`, `saver`
- `Design`: the contract; `id`, `title`, `skin`, `bar`, `css`, `measure`, `parts`, `knobs`, `pulldown`, `start`
- `Launchpad`: every app by section, at `/menu/`
- `Dock`: the default groups of apps
- `Desk`: the saver behind home
- `Lock`: the lock screen
- `Screen Saver`: the idle saver
- `Search`: apps, research, posts, settings
- `Widgets`: `Clock`, `Design of the day`, `Continue`, `Research`
- `Zen`: the canvas alone
- `Library`: an app's left panel
- `Knobs`: an app's right panel
- `Actions` pill, `Status` pill: an app's top and bottom pills
- Bar menus: `File`, `Edit`, `View`, `Go`, `Help`
- Dialogs: `Export`, `Info`, `Keys`
- Windows: `page`, `wide`, `app`
- Layouts: `left`, `right`, `wide`, `bare`, `sheet`, `window`

## SDK

```
module     holds
appicon    the app icon engine
apps       lifts an app's scene into kernel verbs
bots       seeded bots that play a world
clock      the time stamp
color      paints and colour helpers
controls   the generic controls
door       window.door, an app's verbs in the page
drag       sortable, the one reorder primitive
export     save, download, wav
feed       the glass sources
frame      the app stage
git/       the code viewer
handle     the one resize handle
history    the undo stack
hooks      the hooks every widget shares
host       Figure, the figure host
icon       Icon
icons.gen  the icon set
island     an island page
keep       held and hold, localStorage
keys       the keys map, Keys, Info, Hints
knobs      the spec pane
library    Library, Shelf, Item, Art
md/        the markdown pipeline and link resolver
menus      the bar menu model
mrly.css   the app sheet
page       page(), Scene
plate      a route figure's plate
pulldown   a bar menu's plain DOM
px         a measured length
query      the url value
rand       the seeded xorshift
reader     own, Reader
record     the canvas recorder
room       does a box fit
rows       the route types
run        the scene runner
spec       the spec rows, tidy, Roll, Nudge
svg        every SVG export
system     the kernel surface a system app imports
theme/     the palette in JS, tinted
touch      useTouch
turn       turns a colour's hue
verbs      an app's declared verbs
world      useWorld, useCall, the kernel in React
```

## DESIGNS

```
name       was        folder today       look
Modern     Glass      designs/modern     bubbles in a plain blur over the desk; mrly.net's pin
Boxy       Mrly       designs/boxy       boxes, a hue per app, MrlyFont
Classic    Classic    designs/classic    plain windows, pixel glyphs
Bricks     Blocks     designs/bricks     toy bricks, studs, cast shadows
Terminal   Terminal   designs/terminal   phosphor screen, teletype page
Liquid     Glass      mrlyprod/liquid    prototype: liquid glass, glass's WebGL engine
Pixel      -          -                  prototype: sparks on every press
```

- Faces: `sans`, `serif`, `mono`, `math`, `emoji`, `mrly`

## APPS

```
id           title        family
apollonian   Apollonian
arcs         Arcs
chaos        Chaos
chladni      Chladni
crop         Crop         two
cuts         Cuts         three
designs      Designs
font         Font
fractals     Fractals
gaussian     Gaussian
graphs       Graphs
life         Life         life
lightspeed   Lightspeed
matrix       Matrix
moire        Moire        moire
morse        Morse
planets      Planets
plot         Plot         sequences
polygons     Polygons
race         Race
radial       Radial       spin
radix        Radix
readings     Readings
sequences    Sequences    sequences
sfx          SFX
six          Six
sleep        Sleep
slices       Slices       three
snake        Snake
spin         Spin         spin
spirograph   Spirograph
star         Star         moire
techno       Techno
three        Three        three
tour         Tour         sequences
tourbillon   Tourbillon   moire
two          Two          two
ulam         Ulam
voyage       Voyage
wolfram      Wolfram      life
zeta         Zeta
zoom         Zoom
```

## APP FAMILIES

- `two`: crop, two
- `three`: cuts, slices, three
- `sequences`: plot, sequences, tour
- `spin`: radial, spin
- `moire`: moire, star, tourbillon
- `life`: life, wolfram
- `designs`: the design kit; `Pick`, `Picker`, `Thumbs`
- `gl`: the WebGL2 kit
- `audio`: the synth
- `space`: the space scenes; `bang`, `march`, `hyper`, `sky`, `globe`, the planet
- `fractal`: the deep zoom
- `font`: the pixel font that writes the wordmark
- `units`: the wasm unit loaders

## SAVERS

```
saver                   title
apollonian/gasket       Gasket
apollonian/ford         Ford circles
apollonian/curvatures   Curvatures
chaos/cloud             Design cloud
chaos/hex               Hex cloud
font/pens               Pens writing
font/words              Words folding
fractals/dive           Mandelbrot dive
fractals/places         Famous places
fractals/julia          Julia dive
graphs/webs             Cell webs
graphs/tunnels          Tunnels
life/soup               Soup
life/crystal            Crystal
life/tapestry           Tapestry
lightspeed/cloud        Hyperspace clouds
lightspeed/classic      Classic starfield
matrix/rain             Glyph rain
matrix/heavy            Heavy rain
planets/orbit           Solar system orbit
planets/flyby           Dawn flyby
planets/far             Far worlds
polygons/cubes          Cubes
polygons/honeycomb      Honeycomb
polygons/facets         Facets
race/footprints         Footprints
race/embers             Embers
sleep/crowd             Crowd
sleep/giants            Giants
snake/solo              One apple
snake/feast             Many apples
spin/table              Turntable
spin/moire              Moire disc
spirograph/rosettes     Rosettes
spirograph/blooms       Blooms
spirograph/polygon      Polygon rolls
tourbillon/swirl        Swirl
tourbillon/lattices     Lattices
tourbillon/lace         Lace
voyage/solar            Solar system
voyage/strange          Strange worlds
zeta/walk               Critical line
zeta/waves              Prime waves
zoom/studio             Studio
zoom/graphic            Graphic
zoom/haze               Haze
tiles                   Tiles; the shell's own
```

## DRAW

- `Pen`: the one drawing contract
- `canvas`, `svg`, `raster`: the three renderers
- `field`, `grid`, `hex`, `iso`, `plot`: the marks
- `figure`: runs the figure hosts
- Pen calls: `rect`, `round_rect`, `disc`, `ring`, `segment`, `polyline`, `triangle`, `polygon`, `arc`, `image`
- Types: `Board`, `Point`, `Box`, `Color`, `Pixels`, `Frame`, `Ink`, `Palette`, `Swatch`, `Ramp`, `Ramps`, `Grid`, `Raster`, `Svg`
- Ink surfaces: `ground`, `panel`, `line`, `fg`, `dim`
- Ink hues: `red`, `orange`, `yellow`, `green`, `mint`, `teal`, `cyan`, `blue`, `indigo`, `purple`, `pink`, `brown`, `gray`
- `inks`: `blue`, `orange`, `yellow`, `green`, `pink`, `indigo`
- Ramps: `heat`, `fire`, `diverge`, `tone`

## RESEARCH

```
slug           title
adjacent       Mobius and primes with no two equal adjacent digits
apollonian     The Apollonian gasket
arcs           Loops in arcs
arrays         Sparse arrays
automata       The automata
bases          What base 3 hides
beneath        Beneath a design
bijection      A design is a Boolean function
cobham         Two bases
complexity     Complexity
connectivity   Structure against noise
coprime        The coprimality spine
core           The core
crop           Crop
cuts           Cuts
dilations      Dilated stacks
dimensions     Complex dimensions
dimers         Dimers on a design
erdos          Erdos problems
farey          The stack is an RH-observable
franel         The restricted Franel identity
gelfond        The digit-sum twist on a design
hexagon        The hexagon moire
information    Information
integers       Integers
kummer         Mobius and primes on the Kummer sets
magic          Magic words
masks          Design masks
method         Method
mobius         The Mobius meter across digit designs
multilayers    The fractal mirror
pi             Pi out of the stack
progressions   Progression-free digit designs
random         Random
slices         Slices
spectra        Spectra
spin           Spin
spirograph     The spirograph
stack          The algebra of the stack
tilings        Which designs tile
toolpaths      The toolpath
walks          The walk dimension
weights        Weighted designs
windows        Windows of a design
zeta           The zeta function of a digit design
```

## COMMANDS

```
command                 does
bun run dev             the dev server
bun run build           builds a site into dist/
bun run push            ships dist/
bun run init <dir>      writes a new site config folder
bun run check           the gate: scripts/check.sh
bun run content         the wasm pin and the content gate
bun run test            every JS test
bun run types           tsc
bun run lint            Biome
bun run fix             Biome's safe fixes
bun run wasm            copies the siblings' wasm units, writes the font book
bun run shots           screenshots through Chrome
bun run film            films through Chrome
bun run draw            draws figures to SVG or PNG
bun run time            times every figure
bun run vendor          fetches fonts and icons
bun run library         the Life encyclopedia shelves
bun run mrlynet         the mrlynet CLI
scripts/check.sh        leak, install, lint, types, wasm, test, build, content
<unit>/scripts/wasm.sh  builds one sibling's wasm into its js/pkg/
```

- `mrlynet`: `boot`, `list`, `call`, `read`, `goose`, `sim`, `serve`, `mcp`, `gym`, `apps`; flags `--world`, `--as`
- `mrly-<unit>`: `list`, `help`, `<name>`; flags `--raw`, `--version`

## CONFIG

- `site.json`: a site's config
- `launchpad.json`: the Launchpad sections and the dock
- `redirects.json`: the moved urls
- `wasm.lock`: the pinned wasm copy
- `app.json`, `icon.svg`, `icon.ts`, `scene.ts`, `widget.tsx`: an app's five files
- `engine.ts`, `policy.ts`: a reducer app's engine and player
- `bridge.toml`, `skip.txt`: the bridge's config
- `.mcp.json`: registers the `mrlynet` MCP server

## BUILD

- `site:routes`, `site:figures`, `site:apps`, `site:css`: the build's virtual modules
- `site:design`, `site:design.css`, `site:design/tokens.css`, `site:design/saver.css`, `site:design/pulldown`: the drawn design
- `app:<id>`: an app's entry
- `apps:scenes`: the saver scenes
- `mrly`: the Bun plugin

## ENV

- `MRLY_ROOT`: the site config folder
- `MRLY_DIST`: the build's output folder
- `MRLY_DESIGN`: the design the build wears
- `PORT`: the dev server's port
- `DRY`: keeps push's manifest on disk
- `NODE_LAYER_DIR`: the builder's modules layer
