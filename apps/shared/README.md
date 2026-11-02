# SHARED

- The app families: code two or more apps share, one folder a family, the head app included; no route of its own, so the build skips this folder.
- A family imports `../../net/sdk/`, another family and `@mrlyprod/draw`; no family imports an app (`../../net/test/imports.test.ts`).
- `bun test apps/shared` from the repo root runs every test, each beside the file it covers.

## FAMILIES

- `two/`: crop, two; `three/`: cuts, slices, three; and `three/three.ts` (`Three`, `useThree`, Three.js loaded only when a stage wants it) for designs, graphs and three; `sequences/`: plot, sequences, tour; `spin/`: radial, spin; `moire/`: moire, star, tourbillon; `life/`: life, wolfram. Each holds the engines and the scene and widget parts (`scene.ts`, `parts.tsx`) the head app and its followers both use.
- `two/parts.tsx` `useStatus` keeps its family's name on purpose: it reads the scene's facts after a pause; `two/engine.ts` `reroll` is the family's `PAGE.reroll`.
- `designs/` is the design kit every design app shares (not the looks of `../../designs/`): `engine.ts` (the rules, and `DESIGN`, the spec rows of a pick), `pick.tsx` (`Pick`, `Picker`, `Thumbs`; a `Thumb` draws once in sight; `useTheme` re-exported from `../../net/sdk/theme/watch.ts`), `chip.tsx` (the chip, its popover grid and the Browse all modal), `browse.tsx` (`Browser`), `store.ts` (the slot readers, and the current, recent and saved designs in localStorage), `thumb.ts`, `mesh.ts`, `solid.ts`, and `tiles.ts`, the five classic design tiles sleep and snake draw, checked cell for cell against the math door.
- `gl/` is the WebGL2 kit (`gl2`, `post`, `vec` with `clamp01` and `smooth`, `camera`, `clock`): fractals, lightspeed, planets, voyage, zoom.
- `audio/` is the synth: `voices.ts` (the noise, the shapes, the voices and `EFFECTS`), `audio.ts` (`pattern`, the player, `engine`, `render`), `tempo.ts`: sfx, techno, voyage.
- `space/` holds the space scenes: `bang`, `march`, `hyper`, `sky`, `globe`, and the planet in four files: `planet.ts` (the driver: motion, rings, bake and draw), `planet.glsl.ts` (its shaders), `worlds.ts` (the named worlds and `world()`), `exo.ts` (the rolled worlds and the colour helpers); `bang.ts` takes the cube's cells and classes from the math door (`math` is its first argument) and flies them in JS.
- `fractal/` is the deep zoom of fractals, with its own arithmetic: `deep.ts` (the driver, a looping dive or a still view that flies once), `view.ts` (the url view: exact decimals, pan and zoom), `orbit.ts` (BigInt fixed point, the orbit, period, nucleus and size), `probe.ts` (scoring the probe, the next stop), `shader.ts` (the programs and the palettes), `consts.ts` (the constants two or more of them share).
- `life/` also holds the recipe runs of the Life app: `recipe.ts` (a recipe, its mode and stop), `plane.ts` (the plane scene of a recipe) and `run.ts`, the pure reducer (`boot`, `moves`, `step`, `read`) of the Island and Wallpaper modes, replayed frame for frame against the life unit's run.
- `font/` is the pixel font that writes the wordmark: `font.ts` lays out, writes and folds a text, `glyph()` is the trimmed glyph, and `fold.ts` (`fold(word)`, `markup`) folds any word in four beats into the cells its letters stack on, the X for the wordmark; `font.gen.json` is the glyph book `mrly_font` generates (`bun run wasm`).

## UNITS

- `units/<unit>.ts` loads one wasm unit: `{ <unit>, ready }`, for `font`, `life`, `math`, `num`; it imports `@mrlyprod/<unit>/pkg/mrly_<unit>_js_bg.wasm`, the sibling repo's build, which the build points at the site config's `wasm/`.
- `units/load.ts` is the table of loaders, `LOAD`; the build swaps it for one holding only the units the site names, so each site ships its own table.
- `units/index.ts` holds `wasm(...names)`, the thunk a scene exports as `units`: it loads the named units through `LOAD` and answers `{ ...units, ready }`.

## REACHED FROM OUTSIDE

- `designs/`, `units/` and `font/` are also reached by the sdk and the boxy design; each such import is a KNOWN row of the imports test, to shrink as they move down.
