# SDK

- What pages, apps, designs and the shell share; flat, one file a feature, plus `md/`, `git/` and `theme/`.
- The sdk imports nothing above it (`../test/imports.test.ts`); its KNOWN exceptions reach `../../apps/shared/designs/` and `../../apps/shared/units/`, and may only shrink; `plate.ts` reaches the `draw` repo through `@mrlyprod/draw`.
- `package.json` names it `@mrlyprod/sdk`: a site or design outside the repo imports `@mrlyprod/sdk/<file>` (`../hosts/web/scripts/packages.ts`).
- `bun test net/sdk` from the repo root runs every test, each beside the file it covers.

## APP

- The app framework; `../../apps/README.md` has its contract.
- `run.ts`: `run()`, the scene runner, and `board()`; `page.tsx`: `page()`, `Scene` (its canvas a glass source reaching over the frame), `Held`; `frame.tsx`: the stage, its panes (`usePanes`: open, widths, `[` and `]`), the pills and `Bar`; `library.tsx`: `Library`, `Shelf`, `Item`, `Art` (`measure.thumb` px by default); `reader.tsx`: `own(draw)` (a page that owns the app window: `layout`, `render`, `mount`, `unmount` around one React root), `Reader` (the `Frame` plus the bar menus it publishes, `pane:` acts handled), `rule`, `library`, `COPY`, `useCopy`, for Research, Code and Stats.
- `icon.tsx`: `Icon`; `room.ts`: `useRoom` (does a box fit its content, for a part that folds when narrow), `loose`, `classes`.
- `controls.tsx`: the generic controls, drawn through the active design's `control`, `group`, `dialog` and `menu` parts (`Icon`, `Row`, `Group`, `Fold`, `Pick`, `Slider`, `Text`, `Toggle`, `Segment`, `Btn`, `Modal`, `Menu`, `Export`); `knobs.tsx`: the spec pane (`Knob`, `Knobs`, `gist`, `useAccordion`); `spec.ts`: `tidy`, the row readers, `Roll` and `Nudge` (a scene's pure reroll and knob acts) and `rolled` (the seed and the knobs a reroll writes).
- `hooks.ts`: `useLatest`, `useStatus`, `useUnits`, `useUnit(name, given)`, the hooks every widget shares; the sdk names no Three.js, `../../apps/shared/three/three.ts` holds `useThree` for the apps that stage in 3D.
- `history.ts`: the undo stack (`record`, `undo`, `redo`, `patch`); `menus.ts`: the bar menu model an app publishes (`menus`, `publish`, the `menus` event) and `label`, a key's word; `pulldown.ts`: the plain DOM of a bar menu (`button`, `sheet`, `item`, `chord`, `glyph`) that a design's `pulldown` builds from.
- `keys.tsx` (the keys map, `Keys`, `Info`, `Hints`; `Keys` and `Info` are `light` dialogs, `Modal light`, which mark `data-light` and leave the keys live), `island.tsx`, `query.ts` (the url value), `export.ts` (`save`, `text`, `download`, and `wav`, an audio buffer as a 16-bit WAV), `svg.ts`, `touch.ts`, `turn.ts`, `record.ts` (the canvas recorder), `mrly.css` (the app sheet).

## DOOR

- `verbs.ts` reads what an app declares as verbs: `list` (`<user>.<app>.set`, `<user>` the site title in lowercase letters and digits, with a JSON Schema per spec row and the seed, `reroll`, `pause`, one verb per key act), `read` (the tidied value and the readouts as text), `call` (`set` through the page's `onChange`, any other verb through its act).
- `door.ts`: `useDoor`, which `page.tsx` mounts, puts `window.door` (`id`, `list()`, `read()`, `call(name, args)`, which answers the `read` once the url settles) on an app page, so a driver plays any app.

## WORLD

- The world a tab runs, as pages see it; the shell holds it (`../shell/world.ts`) and fills the port, so the sdk never imports the shell.
- `system.ts`: what a system app's `app.ts` imports: `app` (the kernel's, plus the hooks of `Hooked`: an optional `bot`, the policy `../sim/` plays for it, typed `Bot`, `Seen`, `Step` here, and an optional `after(called)`, the act it raises after every act of the world; it also records the app in `installed`, so `manuals()` lists every app this host imported with its verbs' lines), `below`, `stream`, `digest`, `canon` and the types from the kernel, `pick(draw, list)`, `named` (`test-3` reads `Test 3`), `SLICE` (`@`), `walk` and `sliced(slice, whole?)`, a `read` that answers `<app>/@<id>` with `slice(state, id)` and walks every other path as the kernel does; `notice(to, text)`, the wish an app raises for the inbox, `act(verb, args)`, the wish that becomes that act by the same user, `noticed(called, apps)`, the bus: the follow-up acts after one act (its notice wishes, its act wishes, then each installed app's `after` in the world's app order, every one by the same user at the same time, none after `nav.open`), and `bus(apps, step)`, the driver every host runs: one act stepped, its follow-ups stepped after it, a follow-up's fault dropped, the first act's `Called` returned; the bus names no app.
- `blocks.ts`: the block tree the making apps share (`docs`, `slides`, `sites`): `Block { kind, text, kids? }`, `KINDS`, the `TREE` and `PATH` schemas (3 deep, 100 blocks, 1000 characters a text), `count`, `text`, `at`, `edited`, `checked`.
- `media.ts`: `media(shelf)`, the one catalogue three system apps share (`music`, `podcasts`, `tv`): titles published as metadata, a library, plays and a queue per user, and the test users' policy; `media.test.ts` is its proof, each app's `app.test.ts` the rest.
- `world.ts`: the port, `{ subscribe, read, list, call, who, actor, at }`, and `plug(port)`; the hooks, each over `useSyncExternalStore` and re-rendering only when its own answer changes (compared by its canonical JSON): `useWorld(path, shape?)` (the read's `state`, null with no world), `useMoves(app)` (its legal verbs now), `useApp(app)` (its manifest), `useWho()` (`{ who, actor }`); `useCall()` gives `call(verb, args)`, which stamps `who` (the signed-in user) and `at` (the wall clock, never before the world's last) and answers `{ ok, note, envelope }`.
- `kinds.ts`: the test users' kinds: `palette(rule)` reads the `kinds` rule (one kind or a list of them), `kindFor(seed, id, kinds)` draws one user's kind from a seed and its id, `kindOf(world, id)` is the kind a host puts in `who` (the `people` card's, else a pinned rule, else human); `bots.ts` only re-exports `palette` and `kindFor` for `../system/people/app.ts` until it imports `kinds.ts`; the players themselves are `../sim/`.
- `view.tsx`: the generic view, `view(id)`, the island the build writes for a route whose only page is `app.ts`: the app's title and line, who is signed in, its state as a tree (the signed-in user's slice when the app offers one, else the whole state), each legal verb as a form of the design's controls built from its args schema (`enum` a Pick, a bounded number a Slider, any other number or string a Text, a boolean a Toggle, the rest JSON in a Text), the last refusal as the note.

## ROWS

- `rows.ts` holds the route types the build writes and the shell, pages and designs read: `Row`, `App`, `Topic`, `Saver`, `Host`, `Shelf`, `Pad`, and `Desk`, a route's desk widget (`{ name, size, Body }`).

## HELPERS

- `rand.ts`: `rng`, the seeded xorshift (a missing seed is seed 0, never `Math.random`), and `pick`; `color.ts`: `paints`, `rgb`, `bright` (a light colour), `veil`, `mix` (rgb arrays), `blend` (colour strings), `hex`, `HUES`, `TINTS`.
- `px.ts`: `px(n)`, a measured length as a css string, so no shell file writes a px size by hand.
- `appicon.ts` is the app icon engine: `appicon(id, svg)` checks a route's `icon.svg` and returns its namespaced `defs` and `art` (the plate, the `lift` shadow layer under the glyph, the glyph); `markup(art)` cuts Apple's continuous-corner shape (radius 26%) and adds the glow, rim and shade strokes; `SPRITE` holds the shared shape, plates, glow, rim and shade gradients, translucency mask, the `lift` shadow filter and the mono filters (luminance to white or `--accent`) the six looks of `../../designs/modern/appicons.css` use.
- `host.tsx` is `Figure`, the figure host's markup; `@mrlyprod/draw/figure.ts` runs the hosts (`../shell/README.md`); `plate.ts` draws a route figure's plate; `icons.gen.js` holds the Material paths `bun run vendor` writes from the `icons` block of `site.json`.
- `feed.ts` holds the canvas sources (`source`, `fed`, which `run.ts` calls after every draw, `tap`) a design may read; today only the `liquid` prototype does, in its own repo: its WebGL engine refracts them through every `[data-lens]` piece.
- `source(canvas, reach)`: a source may reach past its own box (the app canvas reaches over its frame); a reader draws it there mirrored at its edges, so pieces beside the picture bend its colours.
- The `shift` event (its `detail` in ms) tells such a reader that pieces move.
- `drag.ts`: `sortable(list, { kind, takes, out, axis, scroller, name, preview, commit })`, the one drag-and-reorder primitive (widgets, dock, Launchpad apps and sections). Items are the list's children with `data-id`; a mouse lifts after 5 px, a finger or pen after a 400 ms hold; a ghost (the item, its computed style and canvas pixels) follows the pointer; `takes(fromKind)` answers `move`, `copy` or nothing; `out` drops an item dragged off the list; Alt with an arrow moves the focused item; Escape cancels and eats the release click. `preview(draft)` during and `commit(draft)` at the end must update the DOM before they return; a draft is `{ id, index }` among the others, `index` null for gone, the draft null for unchanged. `placed(list, draft)` and `slot(rects, x, y, axis)` are the pure rules. Siblings glide by FLIP; each move fires `shift`.
- `clock.ts`: `stamp(now, mode, bare)`, the time in 12 or 24 hours (`mode` `12`, `24` or empty for the locale's), `bare` dropping AM and PM.
- `keep.ts`: `held(name, fallback)` and `hold(name, value)`, JSON in localStorage under the site prefix.
- `handle.ts`: `handle(grip, { get, set, done, reset, min, max, rate, step, label, busy })`, the one drag and keyboard resize handle (a `separator`); a negative `rate` is a left edge, 2 a centered window.

## KIT

- `md/md.ts` is the dependency-free half of markdown: `front` (the front matter), `plain`, `summary`, `slug`, `NAME` and `figures` (the lone image lines naming a figure, found without a parser, so the build never loads one), the `Link`, `Lazy` and `Reader` types, and `reader()`, the door to the pipeline: it opens the `reader` route's chunk through `site:apps` and answers its `markdown(text, { link?, lazy? })`; the pipeline itself is `../system/preview/md.ts`, the preview app's own dependency.
- `md/links.ts` is the resolver, `resolve(source, href, index)`: every relative href comes out absolute; order: a denied scheme to `#`; a scheme, `#` or rooted href passes; a source the rows publish to its route; a folder to `/git/<dir>/`; an image or a pdf to `/raw/`; any other file to `/git/<path>`.
- `md/text.ts` holds `esc`, so a browser module imports no pipeline.
- `git/` is the code viewer and its skin (`code.css`, `seti/`), with no highlighter: `../system/git/code.ts` paints, Shiki being the git app's own dependency; `git/README.md` has it.
- `theme/` is the palette in JS: `palette.gen.js` the fifteen colours (generated outside this repo, as `../../designs/shared/palette.gen.css` is), `theme.ts` the `dark` and `light` role maps and `tinted()`, `hues.ts` the twelve tint hues (the palette less black, white and gray), `tint.ts` `tintCss`, the shell's tint rules, `watch.ts` `useTheme()`, a tick that bumps on every theme change, for a canvas that repaints.
- `tinted(roles, { accent, link, theme })` swaps the tint's hue into the blue slot, so every figure follows the tint; in light the slot takes the hue's link shade.
- `md/`, `git/` and `theme/` never import `../shell/`.
