# APPS

- Every folder here is one route, `/<id>/`; `../README.md` has the `app.json` fields, the pages and the build checks.
- The code holds every constant: a timing, a colour or a size lives in its file, never here.

## UNIT

- `app.json`, `icon.js` (the menu figure, a plate by `../lib/icon.js`) and one page: `index.jsx`, `page.md` or a `source`.
- A scene app adds `scene.js` (`SPEC`, `PAGE`, `make`; no React, no DOM beyond its canvas), `widget.jsx` (the React around it) and `index.jsx`.
- `index.jsx` is one line, `export const { mount, unmount } = page(Widget, defaults, PAGE)`; `defaults` holds `seed` and any url key outside the spec.
- `unit.js` loads the wasm, one block per `@mrlyprod/mrly/<unit>`, and exports the units and `ready`; no scene, engine or widget imports a unit, a figure names its own, and a test calls `initSync` itself.
- A family shares its first app's code by relative path: life (wolfram), two (crop), three (slices, cuts), moire (tourbillon, star), spin (radial), sequences (plot, tour).
- `designs/pick.jsx` is the shared design picker (`Pick`, `Picker`, `Thumbs`).
- A `deep: true` app (`git`, `research`) owns every path under its route and is no scene page.
- An id is typed nowhere but its folder: the chrome and the lock name no saver (`apps.test.js`).

## SCENE

- `make(canvas, view, opts)` returns `{ draw, ... }`; `opts` is the value plus what the widget or the lock adds (units, callbacks).
- `view` holds `rand` (the seed's xorshift), `look()` (the paints `{ paper, accent, ink }`), `still` (reduced motion or a still scene), `lock`, `w`, `h` and `dpr` (the backing store), `t` (the milliseconds of play), `fixed` and `wake`.
- A scene's time is `view.t` alone: no wall clock, no `Math.random`, so one url paints one picture.
- `draw()` paints the frame at `t`; `size()` follows a resize, `theme()` a theme change, `stop()` lets go.
- `set(value)`: the new value on the same canvas, `t` and caches kept; `false` asks for a remount; a scene without `set` remounts on every value.
- `toggle()` and `playing()`: the scene owns the pause; Space, the word `paused` and a function label follow it.
- `still: true` (or `PAGE.still`): no frame loop, no Space, no `paused`; drawn on make, set, size and theme; `view.wake()` starts the loop for a replay.
- `every`: the milliseconds between draws of a ticking scene; `font`: a face loaded before the first draw.
- The lock reads `hold()` and `windDown(onDone)`, and a saver's `units = () => import('./unit.js')`, whose `ready` it awaits before spreading the units into `opts`.
- `run(canvas, make, opts, { still, lock })` in `../lib/scene.js` drives a scene and returns `stop`, with `pause`, `play`, `coast`, `fix`, `set` and `scene`.
- `stop.fix({ frame, step, dpr })` pins the store and the step for a take or a sized export: `view.fixed` true for a take; a frame with its own `dpr` is a still; `stop.fix()` returns to the box.

## WIDGET

- `Widget({ value, onChange, onReady, onStatus, onExport, units })`; `units` is `PAGE.units` loaded, and the page mounts once they are in.
- `value` comes tidied; `onChange(patch)` merges, tidies and writes the url.
- `onReady(handle)`: the `Scene` handle up, or `null` when the canvas goes; the lib reads `canvas`, `toggle`, `playing`, `still`, `fix`, `draw` and `sound` on it, and `PAGE.actions` receive it.
- `onStatus(rows)` takes `[label, value]` readouts; `onExport(rows)` takes `[label, fn]` rows for the Export dialog (SVG, JSON, CSV, OBJ, WAV).
- `<Scene make value onReady ...canvas props />` (`../lib/scene.jsx`) is one canvas and one `run` per `make`, so a widget binds its units into `make` with `useCallback`.
- `<Bar side>` portals into a slot: `left` (a browse list), `right` (folds beside the spec's), `status`, `hints`, `actions`.
- `Fold` from `../lib/knobs.jsx` with no `open` joins the accordion; `Knob` draws one spec row inside a widget fold.

## PAGE

- `PAGE` in `scene.js` holds the `page()` options: `spec` (`SPEC`), `units` (the `unit.js` thunk), `keys`, `actions` (an `act` to `(handle, event)`), `gestures`, `record`, `about`, `still`, `name`.
- `gestures: true` hands touch to the canvas; `record: true` adds the Video exports, `v` and Stop.
- `about`: plain paragraphs at the top of Info; `[text](/route/)` is a link.
- `name(value, units)` names the PNG, WebP and video exports; the default is `<id>-<seed>`.
- A keys row is `{ key, label, act, button, when, does }`: one key or a list, a label or `(value, handle)`, the bar's primary, out while `when(value)` is false, a second line in the keys card.
- The value is the seed and the knobs, numbers and strings, a switch 0 or 1; the url holds exactly its keys and is the share link.

## SPEC

- A row is `{ key, label, kind, def, group }` plus its kind's fields.
- Kinds: `slider` (`min`, `max`, `step`, `unit`, `zero`, `show`), `number` (`min`, `max`, `step`, `zero`), `toggle` (0 or 1), `pick` and `segment` (`options`), `list` (`options`, `sep`), `text` (`wide`).
- `options` are plain values or `[value, text]` pairs; `zero` is a word shown at 0 (`Fit`) while the url keeps 0.
- Function fields: `label`, `min`, `max`, `options` and `unit` take `(value, units)`; `show(v, value)` formats a slider's number.
- `when(value)` hides a row, and a group it empties, while `tidy` keeps its key.
- `keep: true` keeps the asked value in the url while the scene gets the clamped one, so a moving cap never destroys it.
- `tidy(spec, value, units)` runs plain rows first, then function rows in spec order, each seeing the rows before it tidied and the rows after it at default: a row goes after the rows its function reads.
- A row with no `group` sits in an always-open section.
- A `view` row, the first row of the first group, picks the picture; the phone bar shows it.

## CHROME

- `Frame` (`../lib/frame.jsx`): the canvas in the middle, the left pane (a browse list), the right pane (the knobs), the bottom bar.
- Right pane: the spec's groups and the widget's folds in one accordion, the open set kept per app in sessionStorage `knobs:<id>`, the first fold met open, Seed last; a summary shows its live values.
- Narrow screens make the panes drawers with openers, then the right pane a bottom sheet and the buttons icons (`../ui/chrome.css`).
- Status, bottom left: the Info button, `seed`, `paused`, `rec`, `link copied`, then the first readouts; the rest wait in Info.
- Hints, in the middle: the first rows of the keys map, on a desk with a mouse.
- Actions, bottom right: the `view` segment (phone), Stop (recording), the primary, Reroll, Export, More (Set as screensaver for a saver, Copy link, Keys, Zen).
- Export: a dialog of PNG or WebP, a size pick, the app's rows and the Video rows; `../lib/export.js` saves, `../lib/svg.js` writes every SVG.
- Info: `about`, then every readout; offered when the app has either.
- Keys (`../lib/keys.jsx`): Space pause, the app's rows, `r` reroll, `e` export, `i` info, `z` zen, `?` the keys card; an app row on a base key replaces it.
- A key in the knobs pane, a field or a select stays there; a pointer-changed control lets go of focus, so keys reach the page.
- Phone view mode, a coarse pointer on a narrow screen: no panes, hints, readouts or More; the bar keeps the status, the `view` segment, the primary, Reroll and Export (PNG only); Info and the app's own touch play stay.
- Zen (`z`, More, Escape or the corner button out): `data-zen` on `html`, the canvas alone with the status floating; never kept.

## RULES

- Canvas first (Carlo, 2026-10-12): an app is artistic first, a brush for a future studio app; the canvas is the app, one picture filling the stage at any aspect.
- One view at a time: several pictures of one thing are a `view` segment, never side by side.
- Charts only in chart-first or explainer apps (`sequences`, `plot`, `tour`), or as a view of its own when the chart is the app's whole point; no chart, profile, spectrum, trace or strip beside a picture.
- Readouts are numbers in `onStatus` and Info.
- Rust is the only maths: every number comes through an `@mrlyprod/mrly` door; a missing door is a door gap, left out, never typed in JS.
- A saver is what a good app turns out to be: never built as one, `saver: true` only when it is pretty with no hands at a random seed; one that needs a hand is an instrument.
- On the page the picture waits for a hand; loops, reseeds and holds run only under `view.lock`.
- Live: a slider moves the picture as it moves, a tap does something.
- An app opens on its canonical picture (the carpet, the glider), never a random one; Reroll rolls.
- A knob never has a key.
- `line` is one plain sentence of at most 90 characters; `about` is two to four plain paragraphs.

## MAP

- `about`: prose: one written mark, the mathematics under it, and the code that draws it.
- `apollonian`: tangent circles in exact integers; on the strip the line circles are Ford circles.
- `arcs`: quarter arcs in every cell join into strands and loops, counted against the law.
- `blog`: the post list; each `blog/<slug>/` holds a post's `index.md` and `icon.js`.
- `cart`: coming soon.
- `chladni`: sand on a ringing plate or a design runs to the nodal lines, as sand, field or lines.
- `contact`: prose: where to write, and every account the company keeps.
- `crop`: a shape keeps only the cells of a design it reaches: in, cut and out.
- `cuts`: the diagonal cuts of the gasket cube, all one size, as points or a section.
- `designs`: the gallery of every design, one a class, in the plane or the cube.
- `donate`: prose: a donation keeps everything free.
- `figures`: hidden: every live figure, drawn with its draw time.
- `font`: the 108 pens of MrlyFont writing themselves, as a wall of pens or your own text.
- `gaussian`: the Gaussian and Eisenstein primes, split, inert and ramified; tap a point.
- `git`: Code: every tracked file of mrlyprod, browsable, the raw bytes under `/raw/`.
- `graphs`: a design as a network in 2D, 3D or 6D: count tips and pieces, relax the lattice.
- `home`: `/`, a bare page; the chrome plays the desk behind it.
- `julia`: an endless dive into Julia sets, c from six presets.
- `life`: Life on any design mask, in a plane or on a line: paint, play, step, run to fate.
- `lightspeed`: a WebGL2 starfield and the jump to hyperspace.
- `mandelbrot`: an endless dive into the Mandelbrot set, steered toward minibrots.
- `matrix`: MrlyFont rain.
- `menu`: Play, Read and Tools from each app's `menu`, then the site tree.
- `method`: prose: Carlo directs, Claude writes and computes, every number names its generator.
- `missing`: hidden: the 404 page, "Nothing here".
- `moire`: one design stacked over its odd scales, and its r-matrix as a second view.
- `morse`: the Thue-Morse word in four views: the word, its lifts, a design's sign power and filter.
- `mrlymath`: prose from `crates/mrly-math/NAMES.md`: a design as one string.
- `planets`: one world lit per pixel: oceans, clouds, city lights, air, rings and moons.
- `plot`: one design sequence drawn large with its rule, and a second one to mix in.
- `polygons`: the desk: a mosaic of tilings coloured by a drifting field and swept by waves.
- `race`: walkers race on two designs of one mass, and the shape alone decides who wins.
- `radial`: a design stacked in turned copies about its centre.
- `radix`: a base of the plane and its digits, twisted into a curve, a dragon or a blob.
- `redirects`: hidden: the moved urls of `redirects.json`.
- `research`: the topic grid; `/research/<slug>/` opens a topic's README over it.
- `sequences`: the ledger of every count a design writes, searched by terms, name, record or code.
- `settings`: theme, font, tint and saver, kept in this browser.
- `sfx`: an instrument: the voyage's sounds, synthesised from a seed, to play and save as WAV.
- `six`: a cube design tiled into a sheet of hexagons in three views, its buried edges counted.
- `sleep`: one mrlytile bouncing off the walls, its design rolled anew at each.
- `slices`: the middle plane of a cube, a hexagon of triangles; slide the side, count the pieces.
- `snake`: Snake with mrlytile pieces, played by hand or by a smart or silly player.
- `spin`: a design on a turntable strobed by the screen, and its infinite-speed wheel.
- `spirograph`: a design as the wheel: every cell holds a pencil that draws its own rolling curve.
- `star`: the hexagonal cuts of the carpet cube stacked, a ghost star fading at the centre.
- `stats`: the CDN, the Lambdas and the bucket, read every minute.
- `techno`: an instrument: an acid loop grown from a seed on a step grid, to play and save as WAV.
- `three`: a cube design as a solid to orbit, crop, count and cut, from the sponge to a field.
- `tour`: the explainer: thirteen cards of mrlymath, each a live design beside its sequence.
- `tourbillon`: the carpet at every odd scale, each layer turned by its own angle.
- `two`: one design repeated into a sheet: the fills multiply, the exposed edges do not.
- `ulam`: the whole numbers wound in a spiral from one, the primes and a quadratic's line lit.
- `voyage`: a film from a seed: leave a world, jump, fly the ship's corridors, arrive at another.
- `wolfram`: the 256 elementary rules as the 256 cube designs: a light cone and its card.
- `zeta`: zeta walked up the critical line through its zeros, then folded into the primes.
- `zoom`: an endless flight into a 3D mrlybang code: corridor, trench, pillars or skim.
