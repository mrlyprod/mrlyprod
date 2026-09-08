# ui

- The design kit of every MrlyProd site, mrly.net and carlomitchener.com: plain CSS, a little vanilla JS, a little React. No build step, no Tailwind, no CSS-in-JS.
- House style: palette first, tokens second, one class per idea, semantic HTML, light on `:root` and dark twice (`prefers-color-scheme` guarded by `:root:not([data-theme="light"])`, then `:root[data-theme="dark"]`), AA contrast in both, 44px targets on coarse pointers, no motion under `prefers-reduced-motion`.
- The palette is generated, never hand-edited: `palette.css` for CSS and `palette.js` for scripts, the same numbers in both.
- `palette.css` names black, white and thirteen hues (`--red --orange --yellow --green --mint --teal --cyan --blue --indigo --purple --pink --brown --gray`), each with a `-light` and a `-dark` shade.
- Each theme resolves the roles `--ground --bg --panel --deep --line --fg --dim --accent --on-accent` and the thirteen theme inks `--ink-<hue>`.
- `--art` is `var(--ground)`, the ground of every canvas and figure; figures ship as `<img class="dark">` and `<img class="light">` pairs and `--show-dark` / `--show-light` pick one.
- `chrome.js` fires `window` event `theme` whenever `data-theme` changes; scripts that paint with `palette.js` listen and repaint.

## CHROME

- One frame: `--frame` = `--pane` + `--page` + `--pane`, with `--pane` a clamp between 13rem and 17.5rem so a small screen keeps a wide main. At and above 74rem the page is three fixed columns centred as one block, tree on the left, main in the middle, tools on the right; main never drifts, and zoomed out the block floats. Below 74rem the two panes are drawers.
- Two bars. The header is three routes in the pixel font: `+` to `menu`, the wordmark to `/`, `O` to `cart`; it scrolls away. The dock under it is sticky: a left-drawer button, the last route segment in capitals (`/demos/spectra/` reads SPECTRA), a right-drawer button; the buttons show only below 74rem, and an open drawer turns its button into `×`.
- On arrival `chrome.js` scrolls the dock to the top (never when a hash is set or the browser restored a scroll), and opening a drawer pins it the same way, so the drawers always hang from the bar.
- The `O` is a ring with nine inner cells; `chrome.js` lights one cell per item in the cart, read from localStorage `${prefix}cart` (an array of `{ qty }`), and repaints on the `cart` and `storage` events.
- Drawer state lives as `data-left` / `data-right` = `open` | `shut` on `html`, never persisted. Theme is `data-theme` = `light` | `dark` (absent = auto), font is `data-font` = `sans` | `serif` | `mono` | `mrly` (absent = system), both in localStorage under `prefix`. `html.js` marks a wired page.
- Settings, in the right pane when `settings` is true: a Theme button that cycles auto, light, dark, and a Font select. `tokens.css` maps `data-font` to `--face`; `base.css` sets body and `.prose` in `var(--face, ...)`, so the chrome keeps the system face and the reading text changes.
- The tree takes nodes `{ name, href?, nodes?, open?, lazy?, icon? }`. `icon` is a seti class drawn before the name. A node with `lazy` renders a `<details data-lazy="path">` whose children arrive on first open from the JSON at `explorer` (default `/git/tree.json`, `{ base, c: [{ n, k: d|f, i?, c? }] }`, `i` the seti kind); the arrow expands, the name navigates. On a code page the tree is the repository alone, root first, the path to the page open, so the viewer reads like an editor.
- The footer is one screen: the site's wordmark written and held by the pixel-font animation across the whole width, and `Copyright © {company} {since}-{year}. All rights reserved.` under it. No links: the menu page holds them.
- `Menu({ tree })` lays a whole tree out as sections, one per group with a shelf per subgroup, leaves under Pages; a leaf with `figure: { dark, light }` is a picture tile, `text` its caption, a leaf without one a plain tile. A site's `/menu/` route is that over its navigator dressed with figures.

## FILES

- `palette.css`: generated, the hues, the shades, the roles and the theme inks, light on `:root` and dark twice.
- `palette.js`: generated, the same palette for scripts.
- `tokens.css`: loaded after `palette.css`; `--art --scrim --mix`, type (system stacks ending in Noto Symbols 2 and Noto Color Emoji as fallbacks), `--face` per `data-font`, space, shape, frame, motion, dark.
- `base.css`: reset, text, links, focus, `.prose`, `img.dark` / `img.light`, reduced motion, print.
- `chrome.css`: skip link, `.top` header and `.dock` bar, `.panes` with `.pane.left` / `.pane.right` and `.scrim`, `.tree`, `.contents`, `.settings`, `.menu`, `.base` footer, controls (`.row`, `.pager`, label, select, range, checkbox, `button` and `.button`, `.tabs`), `.stats`, `.chip`, tables, `.cards`, `.gallery` / `.tile`.
- `chrome.js`: vanilla ESM, runs on load; the dock pin, drawers, theme, font, cart, contents highlight, footer mark, lazy tree; exports `wire()` for pages that render later.
- `font.js`: vanilla ESM, the pixel font; uses the wasm bridge `globalThis.mrly.font_*` when present, else `font.json`.
- `font.json`: the 5x5 glyphs, `{ char: rows[] }`.
- `logo.js`: the MrlyLogo mask, `grid(level)` and `logoSvg(level, fill, ground)`; the builder draws every site's favicon and icons from it.
- `chrome.jsx`: React, renders the whole page for `react-dom/client` and `react-dom/server`.
- `config.js`: `configure(site)` takes the consumer's `site.json`, `conf()` reads it back, `tintCss()` writes the accent override. Keys: `title root since prefix font settings tint tree menu cart explorer company tagline socials contact`.
- `code.css`: the code viewer: the path bar, the file list, the numbered `<pre>` with its `d2`-`d6` gutter, images and PDFs, and the `tk-*` token colours the `git` highlighter emits. Loaded on code pages only, beside `seti/seti.css`.
- `fonts/`: `fonts.css` and the vendored faces, all OFL with their licences beside them: Noto Sans, Noto Serif and Noto Sans Mono (variable 400-700, Latin), MrlyFont from `crates/mrlyfont`, Noto Sans Symbols 2, and Noto Color Emoji in ten unicode-range shards. A site lists them under `kit.files` with `hash: false`, since the css names them by relative url, and links `fonts/fonts.css` in its head; a face is fetched only when a page needs it.
- `seti/`: the SETI file-icon font for the code viewer.
- `tsconfig.json`: points `react` at `sites/net/node_modules` so bun resolves it from here.

## EXPORTS

- `font.js`: `letters(text)` gives `{ rows, cols, grid }`; `animate(text, pad)` and `cycle(text, pad, hold)` give `{ rows, cols, fps, frames }`; `mark(canvas, anim, color)` plays an anim and returns a stop function; `glyphSvg(text)` gives the SVG markup of the glyphs.
- `chrome.js`: `wire()`, idempotent, syncs aria state, applies theme and font, paints the cart and attaches the contents observer and the footer mark.
- `seti/seti.ts`: `seti(path)` gives the class string for a path.
- `chrome.jsx`: `Shell({ route, title, lead, tree, current, contents, controls, wide, brand, children })`, `Header({ brand })`, `Dock({ route })`, `Footer()`, `Wordmark({ className })`, `Tree({ nodes, current })`, `Menu({ tree })`, `Contents({ items, current })`, `Controls({ children })`, `Settings`.
- `brand` is the one slot: pass nothing and the header draws the site's wordmark. `font: false` swaps the wordmark for plain text; `settings: false` drops the settings pane; `prefix` moves the localStorage keys and is read in the browser from `<html data-prefix>`.
