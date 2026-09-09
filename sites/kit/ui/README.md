# ui

- The design kit of every mrly site: plain CSS, a little vanilla JS, a little React. No build step, no Tailwind, no CSS-in-JS.
- House style: palette first, tokens second, one class per idea, semantic HTML, light on `:root` and dark twice (`prefers-color-scheme` guarded by `:root:not([data-theme="light"])`, then `:root[data-theme="dark"]`), AA contrast in both, 44px targets on coarse pointers, no motion under `prefers-reduced-motion`.
- The palette is generated, never hand-edited: `palette.css` for CSS and `palette.js` for scripts, the same numbers in both.
- `palette.css` names black, white and thirteen hues (`--red --orange --yellow --green --mint --teal --cyan --blue --indigo --purple --pink --brown --gray`), each with a `-light` and a `-dark` shade.
- Each theme resolves the roles `--ground --bg --panel --deep --line --fg --dim --accent --on-accent` and the thirteen theme inks `--ink-<hue>`.
- `--art` is `var(--ground)`, the ground of every canvas and figure; figures ship as `<img class="dark">` and `<img class="light">` pairs and `--show-dark` / `--show-light` pick one.
- `chrome.js` fires `window` event `theme` whenever `data-theme` or `data-tint` changes; scripts that paint with `palette.js` listen and repaint.
- `config.js` `headScript(prefix)` is a tiny inline script for the `<head>`, before any stylesheet: it adds class `js` and replays `data-theme`, `data-font`, `data-tint` and `data-saver` from localStorage, so a page never flashes the wrong theme or the wrong footer.
- Tint is a runtime setting: `data-tint` on `html` is a palette hue name, absent means the site default, and `tintCss(hue)` writes the site default plus a `:root[data-tint=<hue>]` block for every hue but grey.
- A tint resolves to `--<hue>-dark` on light and `--<hue>-light` on dark, both AA; `--on-accent` follows (white on light, black on dark) and `--ring` follows `--accent`.

## CHROME

- One frame: `--frame` = `--pane` + `--page` + `--pane`, with `--pane` a clamp between 13rem and 17.5rem so a small screen keeps a wide main. At and above 74rem the page is three fixed columns centred as one block, tree on the left, main in the middle, tools on the right; main never drifts, and zoomed out the block floats. Below 74rem the two panes are drawers.
- Two bars. The header is three routes in the pixel font: `+` to `menu`, the wordmark to `/`, `O` to `cart`; it scrolls away. The dock under it is sticky: a left-drawer button, the last route segment in capitals (`/demos/spectra/` reads SPECTRA), a right-drawer button; the buttons show only below 74rem, and an open drawer fills its button into a solid square, one cell at a time, and drains it on close.
- The scroll rule: `chrome.js` keeps `scrollY` per `location.href` in sessionStorage on `pagehide`, then on `load` drops the key and puts the spot back only on a back-forward or a reload with no hash, so a fresh load never scrolls itself; opening a drawer pins the dock to the viewport top so the drawers hang from the bar.
- The `O` is the font's O around nine inner cells; `chrome.js` lights one cell per item in the cart, read from localStorage `${prefix}cart` (an array of `{ qty }`), and repaints on the `cart` and `storage` events.
- Drawer state lives as `data-left` / `data-right` = `open` | `shut` on `html`, never persisted. Theme is `data-theme` = `light` | `dark` (absent = auto), font is `data-font` = `sans` | `serif` | `mono` | `mrly` (absent = system), tint is `data-tint` = a hue name (absent = the site's), saver is `data-saver` = `matrix` | `sleep` | `mandelbrot` | `julia` (absent = the wordmark; `chrome.js` takes no other name and drops the attribute and the key when it reads one), all four in localStorage under `prefix`. `html.js` marks a wired page.
- Settings, in the right pane when `settings` is true: four rows in one control style, a label left and its value right. Theme is a button cycling auto, light, dark; Font, Tint and Saver are `.pick` labels around a select, Auto first on Tint and Wordmark first on Saver, every option a plain word. `tokens.css` maps `data-font` to `--face`; `base.css` sets body and `.prose` in `var(--face, ...)`, so the chrome keeps the system face and the reading text changes.
- The tree takes nodes `{ name, href?, nodes?, open?, lazy?, icon? }`. `icon` is a seti class drawn before the name. A node with `lazy` renders a `<details data-lazy="path">` whose children arrive on first open from the JSON at `explorer` (default `/git/tree.json`, `{ base, c: [{ n, k: d|f, i?, c? }] }`, `i` the seti kind); the arrow expands, the name navigates. On a code page the tree is the repository alone, root first, the path to the page open, so the viewer reads like an editor.
- The footer is one screen: the site's wordmark written and held by the pixel-font animation across the whole width, and `Copyright © {company} {since}-{year}. All rights reserved.` under it. No links: the menu page holds them.
- `Grid({ nodes })` is the one gallery: a leaf with `figure: { dark, light }` is a picture tile, `text` its caption, `dates` its stamps, a leaf without a figure a plain tile. Every index page, the demo gallery, the home doors and the menu draw it, so they all look the same.
- `Menu({ tree })` lays a whole tree out as sections, one per group with a shelf per subgroup, leaves under Pages, each a `Grid`. A site's `/menu/` route is that over its navigator dressed with figures.
- The footer's legal line sits two pixels off the bottom of the page, over the full-screen animation and never under it.

## FOOTER

- One `canvas.mark` inside the home link, with the static wordmark SVG `.still` beside it. `chrome.js` mounts the canvas; everything else is CSS off `data-saver`.
- Absent `data-saver` is the wordmark: the canvas is a `cols+2` by `rows+2` backing store stretched to `width: 100%` with `image-rendering: pixelated`, painted by `font.js` `mark()`.
- Any other `data-saver` mounts `savers/index.js` `saver(canvas, name)`, and CSS lays the same canvas out `position: absolute; inset: 0`, so `frame.js` sizes it to the footer's CSS box times dpr and the pixels are real.
- A canvas keeps its first context for life, so every mount swaps in a fresh `cloneNode` of the old one; the 2d wordmark and a WebGL fractal never share an element.
- The wordmark canvas is `role="img"` with the site title; a screensaver is decoration, so its clone takes `aria-hidden="true"` and drops the role and the label, and gets the role and the label back when the wordmark returns.
- The stretched saver canvas is `pointer-events: none`: it lies over the home link and the link stays clickable through it.
- `mark()` and `saver()` both return a stop function, kept as `canvas.stop`; switching savers stops the old one first, and both pause offscreen.
- No JS shows the SVG and hides the canvas; `prefers-reduced-motion` does the same for the wordmark, while the other savers keep the canvas and draw their one still frame.

## FILES

- `palette.css`: generated, the hues, the shades, the roles and the theme inks, light on `:root` and dark twice.
- `palette.js`: generated, the same palette for scripts.
- `tokens.css`: loaded after `palette.css`; `--art --scrim --mix`, type (system stacks ending in Noto Symbols 2 and Noto Color Emoji as fallbacks), `--face` per `data-font`, space, shape, frame, motion, dark.
- `base.css`: reset, text, links, focus, `.prose`, `img.dark` / `img.light`, reduced motion, print.
- `chrome.css`: skip link, `.top` header and `.dock` bar, `.panes` with `.pane.left` / `.pane.right` and `.scrim`, `.tree`, `.contents`, `.settings` with its `.theme` button and `.pick` selects, `.menu`, `.base` footer, controls (`.row`, `.pager`, label, select, range, checkbox, `button` and `.button`, `.tabs`), `.stats`, `.chip`, tables, `.cards`, `.gallery` / `.tile`, `.opener`, `.elsewhere`, and the chrome's print rules.
- `chrome.js`: vanilla ESM, runs on load; the scroll spot, drawers, theme, font, tint, saver, cart, contents highlight, footer mark, lazy tree; exports `wire()` for pages that render later.
- `font.js`: vanilla ESM, the pixel font and its choreography. The crate `crates/mrlyfont` is the source of truth: `scripts/wasm.sh` runs its `book` example into `font.json`, which carries every glyph's rows and its stroke path (every glyph hand-penned in the crate's `pens.rs`). `font.js` reads those and ports only the arithmetic: the layout, the write, the phased merge that folds the letters into one centred stack, and the loop. It uses the wasm bridge `globalThis.mrly.font_*` when present; `font.test.js` pins the crate's numbers and `sites/net/font.test.ts` checks every frame against the wasm.
- `font.json`: generated by the crate, `{ char: { rows, path } }`, the bitmap rows and the cell-by-cell stroke order over the trimmed glyph.
- `logo.js`: the MrlyLogo mask, which is the font's `X`, plus `grid(level)` and `logoSvg(level, fill, ground)`; the builder draws every site's favicon and icons from it.
- `chrome.jsx`: React, renders the whole page for `react-dom/client` and `react-dom/server`.
- `config.js`: `configure(site)` takes the consumer's `site.json`, `conf()` reads it back, `tintCss()` writes the accent blocks, `headScript()` writes the boot script. Keys the kit itself reads: `title since prefix font settings tint menu cart explorer company`.
- `savers/`: the four screensavers the footer can wear, vanilla ESM over one canvas, with their own README. Every one inks itself from `--accent`, so the Tint setting is their primary colour. A site lists `savers/*.js` (`tiles.js` included) beside `palette.js` and `logo.js` under `kit.files` or the footer has nothing to import.
- `code.css`: the code viewer: the path bar, the file list, the numbered `<pre>` with its `d2`-`d6` gutter, images and PDFs, and the `tk-*` token colours the `git` highlighter emits. Loaded on code pages only, beside `seti/seti.css`.
- `fonts/`: `fonts.css` and the vendored faces, all OFL with their licences beside them: Noto Sans, Noto Serif and Noto Sans Mono (variable 400-700, Latin), MrlyFont from `crates/mrlyfont`, Noto Sans Symbols 2, and Noto Color Emoji in ten unicode-range shards. A site lists them under `kit.files` with `hash: false`, since the css names them by relative url, and links `fonts/fonts.css` in its head; a face is fetched only when a page needs it.
- `seti/`: the SETI file-icon font for the code viewer.
- `tsconfig.json`: points `react` at `sites/net/node_modules` so bun resolves it from here.

## EXPORTS

- `font.js`: `letters(text)` gives `{ rows, cols, grid }`; `animate(text, pad)` writes, `merge(text, pad)` folds, `cycle(text, pad, hold)` chains write, hold, merge, hold, unfold, hold, unwrite, hold into `{ rows, cols, fps, frames }`; `mark(canvas, anim, color)` plays an anim, repaints on the `theme` event, and returns a stop function the caller must keep.
- `chrome.js`: `wire()`, idempotent, syncs aria state, applies theme, font, tint and saver, paints the cart and attaches the contents observer and the footer mark.
- `config.js`: `configure(site)`, `conf()`, `HUES`, `tintCss(hue)`, `headScript(prefix)`.
- `seti/seti.ts`: `seti(path)` gives the class string for a path.
- `chrome.jsx`: `Shell({ route, title, lead, tree, current, contents, controls, wide, brand, children })`, `Header({ brand })`, `Dock({ route })`, `Footer()`, `Wordmark({ className })`, `Glyph({ text, className, label })`, `Tree({ nodes, current })`, `Grid({ nodes })`, `Menu({ tree })`, `Contents({ items, current })`, `Controls({ children })`, `Settings`.
- `brand` is the one slot: pass nothing and the header draws the site's wordmark. `font: false` swaps the wordmark for plain text; `settings: false` drops the settings pane; `prefix` moves the localStorage keys and is read in the browser from `<html data-prefix>`.
