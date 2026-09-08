# ui

- The design kit of every MrlyProd site, mrly.net and carlomitchener.com: plain CSS, a little vanilla JS, a little React. No build step, no Tailwind, no CSS-in-JS.
- House style: palette first, tokens second, one class per idea, semantic HTML, light on `:root` and dark twice (`prefers-color-scheme` guarded by `:root:not([data-theme="light"])`, then `:root[data-theme="dark"]`), AA contrast in both, 44px targets on coarse pointers, no motion under `prefers-reduced-motion`.
- The palette is generated, never hand-edited: `palette.css` for CSS and `palette.js` for scripts, the same numbers in both.
- `palette.css` names black, white and thirteen hues (`--red --orange --yellow --green --mint --teal --cyan --blue --indigo --purple --pink --brown --gray`), each with a `-light` and a `-dark` shade.
- Each theme resolves the roles `--ground --bg --panel --deep --line --fg --dim --accent --on-accent` and the thirteen theme inks `--ink-<hue>`.
- `--art` is `var(--ground)`, the ground of every canvas and figure; figures ship as `<img class="dark">` and `<img class="light">` pairs and `--show-dark` / `--show-light` pick one.
- `chrome.js` fires `window` event `theme` whenever `data-theme` changes; scripts that paint with `palette.js` listen and repaint.

## CHROME

- One frame: `--frame` = `--pane` + `--page` + `--pane`. At and above 74rem the page is three fixed columns centred as one block, tree on the left, main in the middle, tools on the right; main never drifts, and zoomed out the block floats in the middle. Below 74rem the two panes are drawers.
- The header is three glyphs from the pixel font: `+` links to `menu`, the wordmark to `/`, `O` to `cart`. On narrow screens `+` opens the left drawer and `O` the right one; open, either shows `×`.
- The `O` is a ring with nine inner cells; `chrome.js` lights one cell per item in the cart, read from localStorage `${prefix}cart` (an array of `{ qty }`), and repaints on the `cart` and `storage` events. A `[data-cart-count]` badge gets the number, and the right drawer opens on a Cart row that carries it.
- Drawer state lives as `data-left` / `data-right` = `open` | `shut` on `html`, never persisted. Theme is `data-theme` = `light` | `dark` (absent = auto), font is `data-font` = `sans` | `serif` | `mono` | `mrly` (absent = system), both in localStorage under `prefix`. `html.js` marks a wired page.
- Settings, in the right pane when `settings` is true: a Theme button that cycles auto, light, dark, and a Font select. `tokens.css` maps `data-font` to `--face`; `base.css` sets body and `.prose` in `var(--face, ...)`, so the chrome keeps the system face and the reading text changes.
- The tree takes nodes `{ name, href?, nodes?, open?, lazy? }`. A node with `lazy` renders a `<details data-lazy="path">` whose children arrive on first open from the JSON at `explorer` (default `/git/tree.json`, `{ base, c: [{ n, k: d|f, c? }] }`); the arrow expands, the name navigates.
- The footer is classic: the animated wordmark and `tagline`, one column per `footer` section of links, a Social column of plain underlined `socials`, then `Copyright {company} {since}-{year}. All rights reserved.` and the `contact` mailto.
- `Menu({ tree })` lays a whole tree out as sections, one per group, leaves under Pages; a site's `/menu/` route is that over its navigator.

## FILES

- `palette.css`: generated, the hues, the shades, the roles and the theme inks, light on `:root` and dark twice.
- `palette.js`: generated, the same palette for scripts.
- `tokens.css`: loaded after `palette.css`; `--art --scrim --mix`, type (system stacks ending in Noto Symbols 2 and Noto Color Emoji as fallbacks), `--face` per `data-font`, space, shape, frame, motion, dark.
- `base.css`: reset, text, links, focus, `.prose`, `img.dark` / `img.light`, reduced motion, print.
- `chrome.css`: skip link, `.top` header, `.panes` with `.pane.left` / `.pane.right`, `.scrim` and `.cartrow`, `.tree`, `.contents`, `.settings`, `.menu`, `.base` footer, controls (`.row`, `.pager`, label, select, range, checkbox, `button` and `.button`, `.tabs`), `.stats`, `.chip`, tables, `.cards`, `.gallery` / `.tile`.
- `chrome.js`: vanilla ESM, runs on load; drawers, theme, font, cart, contents highlight, footer mark, lazy tree; exports `wire()` for pages that render later.
- `font.js`: vanilla ESM, the pixel font; uses the wasm bridge `globalThis.mrly.font_*` when present, else `font.json`.
- `font.json`: the 5x5 glyphs, `{ char: rows[] }`.
- `logo.js`: the MrlyLogo mask, `grid(level)` and `logoSvg(level, fill, ground)`; the builder draws every site's favicon and icons from it.
- `chrome.jsx`: React, renders the whole page for `react-dom/client` and `react-dom/server`.
- `config.js`: `configure(site)` takes the consumer's `site.json`, `conf()` reads it back, `tintCss()` writes the accent override. Keys: `title root since prefix font settings tint tree menu cart explorer company tagline footer socials contact`.
- `code.css`: the `tk-*` token colours the `git` highlighter emits and the `d2`-`d6` gutter widths. Loaded on code pages only.
- `fonts/`: `fonts.css` and the vendored faces, all OFL with their licences beside them: Noto Sans, Noto Serif and Noto Sans Mono (variable 400-700, Latin), MrlyFont from `crates/mrlyfont`, Noto Sans Symbols 2, and Noto Color Emoji in ten unicode-range shards. A site lists them under `kit.files` with `hash: false`, since the css names them by relative url, and links `fonts/fonts.css` in its head; a face is fetched only when a page needs it.
- `seti/`: the SETI file-icon font for the code viewer.
- `tsconfig.json`: points `react` at `sites/net/node_modules` so bun resolves it from here.

## EXPORTS

- `font.js`: `letters(text)` gives `{ rows, cols, grid }`; `animate(text, pad)` and `cycle(text, pad, hold)` give `{ rows, cols, fps, frames }`; `mark(canvas, anim, color)` plays an anim and returns a stop function; `glyphSvg(text)` gives the SVG markup of the glyphs.
- `chrome.js`: `wire()`, idempotent, syncs aria state, applies theme and font, paints the cart and attaches the contents observer and the footer mark.
- `seti/seti.ts`: `seti(path)` gives the class string for a path.
- `chrome.jsx`: `Shell({ route, title, lead, tree, current, contents, controls, wide, brand, children })`, `Header({ brand })`, `Footer()`, `Wordmark({ className })`, `Tree({ nodes, current })`, `Menu({ tree })`, `Contents({ items, current })`, `Controls({ children })`, `Settings`.
- `brand` is the one slot: pass nothing and the header draws the site's wordmark. `font: false` swaps the wordmark for plain text; `settings: false` drops the settings pane; `prefix` moves the localStorage keys and is read in the browser from `<html data-prefix>`.
