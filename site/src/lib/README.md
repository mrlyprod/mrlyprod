# LIB

- Browser code two or more routes share, one folder a feature; edited in place, not published.
- lib imports only lib (`../../test/imports.test.ts`); a route hands lib what it draws with.
- `bun test ./src/lib` from `site/` runs every test, each beside the file it covers.

## APP

- `app/` is the app framework; `../apps/README.md` has its contract.
- `run.ts`: `run()`, the scene runner, and `board()`; `page.tsx`: `page()`, `Scene` (its canvas a glass source reaching over the frame), `Held`; `frame.tsx`: the stage, its panes (`usePanes`: open, widths, `[` and `]`), the pills and `Bar`; `library.tsx`: `Library`, `Shelf`, `Item`, `Art`; `reader.tsx`: `own(draw)` (a page that owns the app window: `layout`, `render`, `mount`, `unmount` around one React root), `Reader` (the `Frame` plus the bar menus it publishes, `pane:` acts handled), `rule`, `library`, `COPY`, `useCopy`, for Research, Code and Stats.
- `controls.tsx`: the generic controls (`Icon`, `Row`, `Group`, `Fold`, `Pick`, `Slider`, `Text`, `Toggle`, `Segment`, `Btn`, `Modal`, `Menu`, `Export`); `knobs.tsx`: the spec pane (`Knob`, `Knobs`, `gist`, `useAccordion`); `spec.ts`: `tidy` and the row readers.
- `hooks.ts`: `useLatest`, `useStatus`, `useReroll` (the reroll event through `Rolls`; outside an app, a seed change), `useUnits`, `useUnit(name, given)`, `useThree`, the hooks every widget shares.
- `history.ts`: the undo stack (`record`, `undo`, `redo`, `patch`); `menus.ts`: the bar menu model an app publishes (`menus`, `publish`, the `menus` event) and `label`, a key's word.
- `keys.tsx` (the keys map, `Keys`, `Info`, `Hints`), `island.tsx`, `query.ts` (the url value), `export.ts` (`save`, `text`, `download`, and `wav`, an audio buffer as a 16-bit WAV), `svg.ts`, `touch.ts`, `turn.ts`, `record.ts` (the canvas recorder), `mrly.css` (the app sheet).

## UNITS

- `units/<unit>.js` loads one wasm unit: `{ <unit>, ready }`, for `font`, `life`, `math`, `num`.
- `units/index.ts` holds `wasm(...names)`, the thunk a scene exports as `units`: it loads the named units and answers `{ ...units, ready }`.

## DRAWING

- `rand.ts`: `rng`, the seeded xorshift, and `pick`; `color.ts`: `paints`, `rgb`, `bright` (a light colour), `veil`, `mix` (rgb arrays), `blend` (colour strings), `hex`, `HUES`, `TINTS`.
- `gl/` is the WebGL2 kit (`gl2`, `post`, `vec` with `clamp01` and `smooth`, `camera`, `clock`).
- `audio/` is the synth: `voices.ts` (the noise, the shapes, the voices and `EFFECTS`), `audio.ts` (`pattern`, the player, `engine`, `render`), `tempo.ts`.
- `space/` holds the space scenes: `bang`, `march`, `hyper`, `sky`, and the planet in four files: `planet.ts` (the driver: motion, rings, bake and draw), `planet.glsl.ts` (its shaders), `worlds.ts` (the named worlds and `world()`), `exo.ts` (the rolled worlds and the colour helpers).
- `fractal/` is the deep zoom of fractals: `deep.ts` (the driver, a looping dive or a still view that flies once), `view.ts` (the url view: exact decimals, pan and zoom), `orbit.ts` (BigInt fixed point, the orbit, period, nucleus and size), `probe.ts` (scoring the probe, the next stop), `shader.ts` (the programs and the palettes), `consts.ts` (the constants two or more of them share).
- `space/bang.ts` takes the cube's cells and classes from the math door (`math` is its first argument) and flies them in JS; `fractal/` does its own arithmetic.
- `appicon.ts` is the app icon engine: `appicon(id, svg)` checks a route's `icon.svg` and returns its namespaced `defs` and `art` (the plate, the `lift` shadow layer under the glyph, the glyph); `markup(art)` cuts Apple's continuous-corner shape (radius 26%) and adds the glow, rim and shade strokes; `SPRITE` holds the shared shape, plates, glow, rim and shade gradients, translucency mask, the `lift` shadow filter and the mono filters (luminance to white or `--accent`) the six looks of `../styles/appicons.css` use.
- `figure.ts` runs the figure hosts (`../shell/README.md`); `plate.ts` draws a route figure's plate; `icons.gen.js` holds the Material paths `bun run vendor` writes from the `icons` block of `../../site.json`.
- `glass/` is the liquid glass: `feed.ts` (the sources and `fed`, which `app/run.ts` calls after every draw; nothing else), `index.ts` (`start()`: one WebGL2 context composes the sources over `--bg`, blurs them down six levels, and paints every `[data-lens]` piece through `shaders.ts`, a rounded-box bezel refracted at n = 1.5, blur, saturation, tint and a lit rim, then copies it into the piece's own `canvas.lens`), `shaders.ts`.
- Glass styles: `clear`, `tinted` (the default) and `frosted` from `html[data-glass]`, `off` is solid CSS; a piece kind (`bar`, `window`, `dock`, `widget`, `launchpad`, `menu`, `panel`, `pill`) adds its own bezel, blur, tint and saturation. It renders only when a source draws (at most every 50 ms), a piece resizes, the route, theme, tint or style changes, or a `shift` event asks (its `detail` keeps it rendering that many ms, for pieces that move).
- `source(canvas, reach)`: a source may reach past its own box (the app canvas reaches over its frame); the engine draws it there mirrored at its edges, so pieces beside the picture bend its colours.
- `drag.ts`: `sortable(list, { kind, takes, out, axis, scroller, name, preview, commit })`, the one drag-and-reorder primitive (widgets, dock, Launchpad apps and sections). Items are the list's children with `data-id`; a mouse lifts after 5 px, a finger or pen after a 400 ms hold; a ghost (the item, its computed style and canvas pixels) follows the pointer; `takes(fromKind)` answers `move`, `copy` or nothing; `out` drops an item dragged off the list; Alt with an arrow moves the focused item; Escape cancels and eats the release click. `preview(draft)` during and `commit(draft)` at the end must update the DOM before they return; a draft is `{ id, index }` among the others, `index` null for gone, the draft null for unchanged. `placed(list, draft)` and `slot(rects, x, y, axis)` are the pure rules. Siblings glide by FLIP; each move fires `shift` so the glass follows.
- `keep.ts`: `held(name, fallback)` and `hold(name, value)`, JSON in localStorage under the site prefix.
- `handle.ts`: `handle(grip, { get, set, done, reset, min, max, rate, step, label, busy })`, the one drag and keyboard resize handle (a `separator`); a negative `rate` is a left edge, 2 a centered window.
- `view/` is the pen and its kit, imported as `#view` by the figures here and in `../../../research/`; `view/README.md` has it.

## DESIGNS AND FAMILIES

- `designs/` is the design kit every design app shares: `engine.ts` (the rules, and `DESIGN`, the spec rows of a pick), `pick.tsx` (`Pick`, `Picker`, `Thumbs`; a `Thumb` draws once in sight), `chip.tsx` (the chip, its popover grid and the Browse all modal), `browse.tsx` (`Browser`), `store.ts` (the slot readers, and the current, recent and saved designs in localStorage), `thumb.ts`, `mesh.ts`, `solid.ts`, and `tiles.ts`, the five classic design tiles sleep and snake draw, checked cell for cell against the math door.
- `two/`, `three/`, `sequences/`, `spin/`, `moire/`, `life/` hold what a family of apps shares: the engines, and the scene and widget parts (`scene.ts`, `parts.tsx`) the head app and its followers both use.
- Two hooks keep their family's name on purpose: `two/parts.tsx` `useStatus` reads the scene's facts after a pause, and `useReroll` rolls a design into the value; `spin/source.tsx` `useReroll` hands the roll a seeded `rng`, not the seed.

## KIT

- `md/md.ts` is the markdown pipeline: GFM, `$` and `$$` handed to the caller's math (KaTeX as MathML in the browser), heading ids, a lone image line naming a figure as a figure host, raw HTML as text, `3*n*(n+1)` kept literal.
- `md/links.ts` is the resolver, `resolve(source, href, index)`: every relative href comes out absolute; order: a denied scheme to `#`; a scheme, `#` or rooted href passes; a source the rows publish to its route; a folder to `/git/<dir>/`; an image or a pdf to `/raw/`; any other file to `/git/<path>`.
- `md/text.ts` holds `esc`, so a browser module imports no pipeline.
- `git/` is the code viewer and its skin (`code.css`, `seti/`); `git/README.md` has it.
- `theme/` is the palette in JS: `palette.gen.js` the fifteen colours (generated outside this repo, as `../styles/palette.gen.css` is), `theme.ts` the `dark` and `light` role maps and `tinted()`, `hues.ts` the twelve tint hues (the palette less black, white and gray), `tint.ts` `tintCss`, the shell's tint rules.
- `tinted(roles, { accent, link, theme })` swaps the tint's hue into the blue slot, so every figure follows the tint; in light the slot takes the hue's link shade.
- `font/` is the pixel font that writes the wordmark: `font.ts` lays out, writes and folds a text, and `glyph()` is the trimmed glyph the shell's welcome shares; `font.gen.json` is the glyph book `mrly::font` generates.
- `md/`, `git/`, `theme/` and `font/` import only each other and never `../shell/`.
