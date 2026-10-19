# LIB

- Browser code two or more routes share, one folder a feature; edited in place, not published.
- lib imports only lib (`../../test/imports.test.js`); a route hands lib what it draws with.
- `bun test ./src/lib` from `site/` runs every test, each beside the file it covers.

## APP

- `app/` is the app framework; `../apps/README.md` has its contract.
- `run.js`: `run()`, the scene runner, and `board()`; `page.jsx`: `page()`, `Scene`, `Held`; `frame.jsx`: the stage, its panes and `Bar`.
- `controls.jsx`: the generic controls (`Icon`, `Row`, `Group`, `Fold`, `Pick`, `Slider`, `Text`, `Toggle`, `Segment`, `Btn`, `Modal`, `Menu`, `Export`); `knobs.jsx`: the spec pane (`Knob`, `Knobs`, `gist`, `useAccordion`); `spec.js`: `tidy` and the row readers.
- `hooks.js`: `useLatest`, `useStatus`, `useReroll`, `useUnits`, `useUnit(name, given)`, `useThree`, the hooks every widget shares.
- `keys.jsx` (the keys map, `Keys`, `Info`, `Hints`), `island.jsx`, `query.js` (the url value), `export.js` (`save`, `text`, `download`, and `wav`, an audio buffer as a 16-bit WAV), `svg.js`, `touch.js`, `turn.js`, `record.js` (the canvas recorder), `mrly.css` (the app sheet).

## UNITS

- `units/<unit>.js` loads one wasm unit: `{ <unit>, ready }`, for `font`, `life`, `math`, `num`.
- `units/index.js` holds `wasm(...names)`, the thunk a scene exports as `units`: it loads the named units and answers `{ ...units, ready }`.

## DRAWING

- `rand.js`: `rng`, the seeded xorshift, and `pick`; `color.js`: `paints`, `rgb`, `veil`, `mix` (rgb arrays), `blend` (colour strings), `hex`, `HUES`, `TINTS`.
- `gl/` is the WebGL2 kit (`gl2`, `post`, `vec` with `clamp01` and `smooth`, `camera`, `clock`).
- `audio/` is the synth: `voices.js` (the noise, the shapes, the voices and `EFFECTS`), `audio.js` (`pattern`, the player, `engine`, `render`), `tempo.js`.
- `space/` holds the space scenes: `bang`, `march`, `hyper`, `sky`, and the planet in four files: `planet.js` (the driver: motion, rings, bake and draw), `planet.glsl.js` (its shaders), `worlds.js` (the named worlds and `world()`), `exo.js` (the rolled worlds and the colour helpers).
- `fractal/` is the deep zoom of julia and mandelbrot: `deep.js` (the driver), `orbit.js` (BigInt fixed point, the orbit, period, nucleus and size), `probe.js` (scoring the probe, the next stop), `shader.js` (the programs and the palettes), `consts.js` (the constants two or more of them share).
- `space/bang.js` takes the cube's cells and classes from the math door (`math` is its first argument) and flies them in JS; `fractal/` does its own arithmetic.
- `figure.js` runs the figure hosts (`../shell/README.md`); `plate.js` draws a route icon's plate; `icons.gen.js` holds the Material paths `bun run vendor` writes from the `icons` block of `../../site.json`.
- `view/` is the pen and its kit, imported as `#view` by the figures here and in `../../../research/`.

## DESIGNS AND FAMILIES

- `designs/` is the design kit every design app shares: `engine.js` (the rules, and `DESIGN`, the spec rows of a pick), `pick.jsx` (`Pick`, `Picker`, `Thumbs`), `thumb.js`, `mesh.js`, `solid.js`, and `tiles.js`, the five classic design tiles sleep and snake draw, checked cell for cell against the math door.
- `two/`, `three/`, `sequences/`, `spin/`, `moire/`, `life/` hold what a family of apps shares: the engines, and the scene and widget parts (`scene.js`, `parts.jsx`) the head app and its followers both use.
- Two hooks keep their family's name on purpose: `two/parts.jsx` `useStatus` reads the scene's facts after a pause, and `useReroll` rolls a design into the value; `spin/source.jsx` `useReroll` hands the roll a seeded `rng`, not the seed.

## KIT

- `md/md.ts` is the markdown pipeline: GFM, `$` and `$$` handed to the caller's math (KaTeX as MathML in the browser), heading ids, a lone image line naming a figure as a figure host, raw HTML as text, `3*n*(n+1)` kept literal.
- `md/links.ts` is the resolver, `resolve(source, href, index)`: every relative href comes out absolute; order: a denied scheme to `#`; a scheme, `#` or rooted href passes; a source the rows publish to its route; a folder to `/git/<dir>/`; an image or a pdf to `/raw/`; any other file to `/git/<path>`.
- `md/text.ts` holds `escape`, so a browser module imports no pipeline.
- `git/` is the code viewer and its skin (`code.css`, `seti/`); `git/README.md` has it.
- `theme/` is the palette in JS: `palette.gen.js` the fifteen colours (generated outside this repo, as `../styles/palette.gen.css` is), `theme.js` the `dark` and `light` role maps and `tinted()`, `hues.js` the twelve tint hues (the palette less black, white and gray), `tint.js` `tintCss`, the shell's tint rules.
- `tinted(roles, { accent, link, theme })` swaps the tint's hue into the blue slot, so every figure follows the tint; in light the slot takes the hue's link shade.
- `font/` is the pixel font that writes the wordmark: `font.js` lays out, writes and folds a text, and `glyph()` is the trimmed glyph the shell's welcome shares; `font.gen.json` is the glyph book `mrly::font` generates.
- `md/`, `git/`, `theme/` and `font/` import only each other and never `../shell/`.
