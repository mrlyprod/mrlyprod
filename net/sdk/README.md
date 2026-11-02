# SDK

- What pages, apps, designs and the shell share; flat, one file a feature, plus `md/`, `git/` and `theme/`.
- The sdk imports nothing above it (`../test/imports.test.ts`); its KNOWN exceptions reach `../../apps/shared/designs/` and `../../apps/shared/units/`, and may only shrink; `plate.ts` reaches `../../draw/` through `@mrlyprod/draw`.
- `package.json` names it `@mrlyprod/sdk`: a site or design outside the repo imports `@mrlyprod/sdk/<file>` (`../hosts/web/scripts/packages.ts`).
- `bun test net/sdk` from the repo root runs every test, each beside the file it covers.

## APP

- The app framework; `../../apps/README.md` has its contract.
- `run.ts`: `run()`, the scene runner, and `board()`; `page.tsx`: `page()`, `Scene` (its canvas a glass source reaching over the frame), `Held`; `frame.tsx`: the stage, its panes (`usePanes`: open, widths, `[` and `]`), the pills and `Bar`; `library.tsx`: `Library`, `Shelf`, `Item`, `Art` (`measure.thumb` px by default); `reader.tsx`: `own(draw)` (a page that owns the app window: `layout`, `render`, `mount`, `unmount` around one React root), `Reader` (the `Frame` plus the bar menus it publishes, `pane:` acts handled), `rule`, `library`, `COPY`, `useCopy`, for Research, Code and Stats.
- `icon.tsx`: `Icon`; `room.ts`: `useRoom` (does a box fit its content, for a part that folds when narrow), `loose`, `classes`.
- `controls.tsx`: the generic controls, drawn through the active design's `control`, `group`, `dialog` and `menu` parts (`Icon`, `Row`, `Group`, `Fold`, `Pick`, `Slider`, `Text`, `Toggle`, `Segment`, `Btn`, `Modal`, `Menu`, `Export`); `knobs.tsx`: the spec pane (`Knob`, `Knobs`, `gist`, `useAccordion`); `spec.ts`: `tidy` and the row readers.
- `hooks.ts`: `useLatest`, `useStatus`, `useReroll` (the reroll event through `Rolls`; outside an app, a seed change), `useUnits`, `useUnit(name, given)`, `useThree`, the hooks every widget shares.
- `history.ts`: the undo stack (`record`, `undo`, `redo`, `patch`); `menus.ts`: the bar menu model an app publishes (`menus`, `publish`, the `menus` event) and `label`, a key's word; `pulldown.ts`: the plain DOM of a bar menu (`button`, `sheet`, `item`, `chord`, `glyph`) that a design's `pulldown` builds from.
- `keys.tsx` (the keys map, `Keys`, `Info`, `Hints`; `Keys` and `Info` are `light` dialogs, `Modal light`, which mark `data-light` and leave the keys live), `island.tsx`, `query.ts` (the url value), `export.ts` (`save`, `text`, `download`, and `wav`, an audio buffer as a 16-bit WAV), `svg.ts`, `touch.ts`, `turn.ts`, `record.ts` (the canvas recorder), `mrly.css` (the app sheet).

## DOOR

- `verbs.ts` reads what an app declares as verbs: `list` (`<user>.<app>.set`, `<user>` the site title in lowercase letters and digits, with a JSON Schema per spec row and the seed, `reroll`, `pause`, one verb per key act), `read` (the tidied value and the readouts as text), `call` (`set` through the page's `onChange`, any other verb through its act).
- `door.ts`: `useDoor`, which `page.tsx` mounts, puts `window.door` (`id`, `list()`, `read()`, `call(name, args)`, which answers the `read` once the url settles) on an app page, so a driver plays any app.

## ROWS

- `rows.ts` holds the route types the build writes and the shell, pages and designs read: `Row`, `App`, `Topic`, `Saver`, `Host`, `Shelf`, `Pad`.

## HELPERS

- `rand.ts`: `rng`, the seeded xorshift, and `pick`; `color.ts`: `paints`, `rgb`, `bright` (a light colour), `veil`, `mix` (rgb arrays), `blend` (colour strings), `hex`, `HUES`, `TINTS`.
- `px.ts`: `px(n)`, a measured length as a css string, so no shell file writes a px size by hand.
- `appicon.ts` is the app icon engine: `appicon(id, svg)` checks a route's `icon.svg` and returns its namespaced `defs` and `art` (the plate, the `lift` shadow layer under the glyph, the glyph); `markup(art)` cuts Apple's continuous-corner shape (radius 26%) and adds the glow, rim and shade strokes; `SPRITE` holds the shared shape, plates, glow, rim and shade gradients, translucency mask, the `lift` shadow filter and the mono filters (luminance to white or `--accent`) the six looks of `../../designs/glass/appicons.css` use.
- `host.tsx` is `Figure`, the figure host's markup; `../../draw/figure.ts` runs the hosts (`../shell/README.md`); `plate.ts` draws a route figure's plate; `icons.gen.js` holds the Material paths `bun run vendor` writes from the `icons` block of `site.json`.
- `feed.ts` holds the glass sources (`source`, `fed`, which `run.ts` calls after every draw, `tap`); the engine that reads them is the `glass` design's (`../../designs/glass/engine/`: one WebGL2 context composes the sources over `--bg`, blurs them down six levels, and paints every `[data-lens]` piece through `shaders.ts`, a rounded-box bezel refracted at n = 1.5, blur, saturation, tint and a lit rim, then copies it into the piece's own `canvas.lens`).
- Glass styles: `clear`, `tinted` (the default) and `frosted` from `html[data-glass]`, `off` is solid CSS; a piece kind (`bar`, `window`, `dock`, `widget`, `launchpad`, `menu`, `panel`, `pill`) adds its own bezel, blur, tint and saturation. It renders only when a source draws (at most every 50 ms), a piece resizes, the route, theme, tint or style changes, or a `shift` event asks (its `detail` keeps it rendering that many ms, for pieces that move).
- `source(canvas, reach)`: a source may reach past its own box (the app canvas reaches over its frame); the engine draws it there mirrored at its edges, so pieces beside the picture bend its colours.
- `drag.ts`: `sortable(list, { kind, takes, out, axis, scroller, name, preview, commit })`, the one drag-and-reorder primitive (widgets, dock, Launchpad apps and sections). Items are the list's children with `data-id`; a mouse lifts after 5 px, a finger or pen after a 400 ms hold; a ghost (the item, its computed style and canvas pixels) follows the pointer; `takes(fromKind)` answers `move`, `copy` or nothing; `out` drops an item dragged off the list; Alt with an arrow moves the focused item; Escape cancels and eats the release click. `preview(draft)` during and `commit(draft)` at the end must update the DOM before they return; a draft is `{ id, index }` among the others, `index` null for gone, the draft null for unchanged. `placed(list, draft)` and `slot(rects, x, y, axis)` are the pure rules. Siblings glide by FLIP; each move fires `shift` so the glass follows.
- `clock.ts`: `stamp(now, mode, bare)`, the time in 12 or 24 hours (`mode` `12`, `24` or empty for the locale's), `bare` dropping AM and PM.
- `keep.ts`: `held(name, fallback)` and `hold(name, value)`, JSON in localStorage under the site prefix.
- `handle.ts`: `handle(grip, { get, set, done, reset, min, max, rate, step, label, busy })`, the one drag and keyboard resize handle (a `separator`); a negative `rate` is a left edge, 2 a centered window.

## KIT

- `md/md.ts` is the markdown pipeline: GFM, `$` and `$$` handed to the caller's math (KaTeX as MathML in the browser), heading ids, a lone image line naming a figure as a figure host, raw HTML as text, `3*n*(n+1)` kept literal.
- `md/links.ts` is the resolver, `resolve(source, href, index)`: every relative href comes out absolute; order: a denied scheme to `#`; a scheme, `#` or rooted href passes; a source the rows publish to its route; a folder to `/git/<dir>/`; an image or a pdf to `/raw/`; any other file to `/git/<path>`.
- `md/text.ts` holds `esc`, so a browser module imports no pipeline.
- `git/` is the code viewer and its skin (`code.css`, `seti/`); `git/README.md` has it.
- `theme/` is the palette in JS: `palette.gen.js` the fifteen colours (generated outside this repo, as `../../designs/shared/palette.gen.css` is), `theme.ts` the `dark` and `light` role maps and `tinted()`, `hues.ts` the twelve tint hues (the palette less black, white and gray), `tint.ts` `tintCss`, the shell's tint rules.
- `tinted(roles, { accent, link, theme })` swaps the tint's hue into the blue slot, so every figure follows the tint; in light the slot takes the hue's link shade.
- `md/`, `git/` and `theme/` never import `../shell/`.
