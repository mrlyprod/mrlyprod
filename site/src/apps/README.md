# APPS

- Every folder here is one app, one route, `/<id>/`; `../pages/` holds the site pages; `../../README.md` has the `app.json` fields and the build checks.
- The code holds every constant: a timing, a colour or a size lives in its file, never here.

## UNIT

- A new app is 5 files: `app.json`, `icon.svg` (the app icon, `../lib/appicon.ts`), `icon.ts` (the figure, the app's still, a plate by `../lib/plate.ts`), `scene.ts` (`SPEC`, `PAGE`, `make`; no React, no DOM beyond its canvas) and `widget.tsx` (the React around it).
- No app holds an `index.tsx`: the build writes each entry as the virtual module `app:<id>`, `page(Widget, { seed: 0, ...PAGE.defaults }, PAGE)` (`entry` in `../../scripts/build.ts`).
- `PAGE.defaults` holds any url key outside the spec (`base`, `code`, a design); the seed is always 0.
- `scene.ts` names its wasm units with `export const units = wasm('math', ...)` from `../lib/units/`; a widget that needs one before the page hands it over takes it with `useUnit(name, given)` from `../lib/app/hooks.ts`; no scene, engine or widget imports a unit, a figure names its own, and the test preload initialises them.
- A family shares its code through `../lib/<family>/`, the head app included: two (crop), three (cuts, slices), sequences (plot, tour), spin (radial), moire (star, tourbillon), life (wolfram); no app imports another (`../../test/imports.test.ts`).
- `../lib/designs/` is the shared design kit: the rules and `DESIGN` in `engine.ts`, the picker (`Pick`, `Picker`, `Thumbs`) in `pick.tsx`.
- An id is typed nowhere but its folder: the chrome and the lock name no saver (`apps.test.ts`).

## SCENE

- `make(canvas, view, opts)` returns `{ draw, ... }`; `opts` is the value plus what the widget or the lock adds (units, callbacks).
- `view` holds `rand` (the seed's xorshift), `look()` (the paints `{ paper, accent, ink }`), `still` (reduced motion or a still scene), `lock`, `w`, `h` and `dpr` (the backing store), `t` (the milliseconds of play), `fixed` and `wake`.
- A scene's time is `view.t` alone: no wall clock, no `Math.random`, so one url paints one picture.
- `draw()` paints the frame at `t`; `size()` follows a resize, `theme()` a theme change, `stop()` lets go.
- `set(value)`: the new value on the same canvas, `t` and caches kept; `false` asks for a remount; a scene without `set` remounts on every value.
- `toggle()` and `playing()`: the scene owns the pause; Space, the word `paused` and a function label follow it.
- `still: true` (or `PAGE.still`): no frame loop, no Space, no `paused`; drawn on make, set, size and theme; `view.wake()` starts the loop for a replay.
- `every`: the milliseconds between draws of a ticking scene; `font`: a face loaded before the first draw.
- The lock reads `hold()` and `windDown(onDone)`, and a saver's `units`, whose `ready` it awaits before spreading the units into `opts`.
- `run(canvas, make, opts, { still, lock })` in `../lib/app/run.ts` drives a scene and returns `stop`, with `pause`, `play`, `coast`, `fix`, `set` and `scene`.
- `stop.fix({ frame, step, dpr })` pins the store and the step for a take or a sized export: `view.fixed` true for a take; a frame with its own `dpr` is a still; `stop.fix()` returns to the box.

## WIDGET

- `Widget({ value, onChange, onReady, onStatus, onExport, units })`; `units` is `PAGE.units` loaded, and the page mounts once they are in.
- `value` comes tidied; `onChange(patch)` merges, tidies and writes the url.
- `onReady(handle)`: the `Scene` handle up, or `null` when the canvas goes; the lib reads `canvas`, `toggle`, `playing`, `still`, `fix`, `draw` and `sound` on it, and `PAGE.actions` receive it.
- `onStatus(rows)` takes `[label, value]` readouts; `onExport(rows)` takes `[label, fn]` rows for the Export dialog (SVG, JSON, CSV, OBJ, WAV).
- `<Scene make value onReady ...canvas props />` (`../lib/app/page.tsx`) is one canvas and one `run` per `make`, so a widget binds its units into `make` with `useCallback`.
- `<Bar side>` portals into a slot: `left` (the Library panel), `right` (folds beside the spec's), `status`, `hints`, `actions`.
- `Library`, `Shelf`, `Item` and `Art` (`../lib/app/library.tsx`) fill the left panel: a title, an optional search field and filter chips, then named shelves of rows or thumb tiles; `Art` draws a cheap thumb lazily, one a frame.
- `useReroll(seed, fn)` (`../lib/app/hooks.ts`) runs `fn(next)` when the user rerolls (the key, the button, the menu), never when the url or Undo changes the seed; a widget that writes on a reroll uses it.
- `Fold` from `../lib/app/controls.tsx` with no `open` joins the accordion; `Knob` from `../lib/app/knobs.tsx` draws one spec row inside a widget fold.

## PAGE

- `PAGE` in `scene.ts` holds the `page()` options: `defaults`, `spec` (`SPEC`), `units` (the `wasm()` thunk), `keys`, `actions` (an `act` to `(handle, event)`), `gestures`, `record`, `about`, `still`, `name`, `design`.
- `design` is the slot of an app that takes a design: `{ dim, base, code, dims, bases, spaces }`, a string naming the url key, a number fixing it, `dims` may include 1 (a line design grown as a one-row strip), `spaces` the `[dim, base]` pairs when not every pair works, `holds(design, value)` keeps only the designs that work with the other knobs, `when(value)` hides the chip, `blank` names an empty code that means something else than the home design (`Random`); the chip in the actions pill picks through it, and a url with no code opens on the current design (`../lib/designs/store.ts`).
- `gestures: true` hands touch to the canvas; `record: true` adds the Video exports, `v` and Stop.
- `about`: plain paragraphs at the top of Info; `[text](/route/)` is a link.
- `name(value, units)` names the PNG, WebP and video exports; the default is `<id>-<seed>`.
- A keys row is `{ key, label, act, button, when, does, menu }`: one key or a list, a label or `(value, handle)`, the actions pill's primary, out while `when(value)` is false, a second line in the keys card, the bar menu it also sits in.
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
- A `view` row (key `view`, or any row with `view: true`), the first row of the first group, picks the picture; the phone bar and the View menu show it.

## CHROME

- `Frame` (`../lib/app/frame.tsx`): the canvas fills the app window; the Library panel on the left and the Knobs panel on the right (280px by default, an inner edge resizes each, kept in localStorage) float as glass and push the canvas by the wider open one on both sides, so the picture stays centered; Cmd or Ctrl with `[` and `]` toggle them; when the canvas would fall under 360px they float over it.
- Actions pill, top center: the design chip, the `view` segment (phone), Stop (recording), the primary, Reroll, Export, Share (copy link).
- Status pill, bottom center: the Info button, `seed`, `paused`, `rec`, `link copied`, the first readouts; then the first three key hints and the keyboard button for the keys card.
- Right panel: the spec's groups and the widget's folds in one accordion, the open set kept per app in sessionStorage `knobs:<id>`, the first fold met open, Seed last; a summary shows its live values.
- Bar menus (`../lib/app/menus.ts`, drawn by `../shell/menus.ts`): File (Export, its rows, Copy Link), Edit (Undo, Redo, Reroll, Reset Knobs), View (the view segment, Show Library, Show Knobs, Zen), Go (Home, Launchpad, recent apps, the section's other apps), Help (Keys, Info, the topic's research note); a keys row with `menu` joins its menu.
- Undo and Redo (Cmd or Ctrl with Z, Shift for Redo): an in-app stack of url values, changes closer than 500 ms one step; a text field keeps its own.
- Reset Knobs puts every spec row back to its default but the design's keys.
- Export: a dialog of PNG or WebP, a size pick, the app's rows and the Video rows; `../lib/app/export.ts` saves, `../lib/app/svg.ts` writes every SVG.
- Info: `about`, then every readout; offered when the app has either.
- Keys (`../lib/app/keys.tsx`): Space pause, the app's rows, `r` reroll, `e` export, `i` info, `z` zen, `?` the keys card; an app row on a base key replaces it.
- A key in the knobs pane, a field or a select stays there; a pointer-changed control lets go of focus, so keys reach the page.
- Zen (`z`, the View menu, Escape or the corner button out): `data-zen` on `html`, the canvas alone; never kept.
- The glass reads the app canvas mirrored past its edge, so the panels and pills bend the picture beside them.

## RULES

- Canvas first (Carlo, 2026-10-12): an app is artistic first, a brush for a future studio app; the canvas is the app, one picture filling the stage at any aspect.
- One view at a time: several pictures of one thing are a `view` segment, never side by side.
- Charts only in chart-first or explainer apps (`sequences`, `plot`, `tour`), or as a view of its own when the chart is the app's whole point; no chart, profile, spectrum, trace or strip beside a picture.
- Readouts are numbers in `onStatus` and Info.
- A design and its numbers come through an `@mrlyprod/mrly` door; a missing door is a door gap, left out, never typed in JS.
- Two exceptions are drawn in JS: the deep zoom of fractals (`lib/fractal/`) and the flight paths of zoom and voyage (`lib/space/`), which take their cells and classes from the math door.
- A saver is what a good app turns out to be: never built as one; an app exports `SAVERS` (`{ id, title, pick(rand), still }`, 1 to 3; `still` is optional, the ms `/screensavers/` plays it before its capture, a whole number from 1 to 10000, default 1000) only when it is pretty with no hands at a random seed; one that needs a hand is an instrument.
- On the page the picture waits for a hand; loops, reseeds and holds run only under `view.lock`.
- Live: a slider moves the picture as it moves, a tap does something.
- An app opens on its canonical picture (the carpet, the glider), never a random one; Reroll rolls.
- A knob never has a key.
- `line` is one plain sentence of at most 90 characters; `about` is two to four plain paragraphs.
