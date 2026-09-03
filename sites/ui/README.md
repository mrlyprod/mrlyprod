# ui

- The design kit of mrly.net: plain CSS, a little vanilla JS, a little React. No build step, no Tailwind, no CSS-in-JS, so the CSS lifts into a Shopify theme as is.
- House style: tokens first, one class per idea, semantic HTML, light on `:root` and dark twice (`prefers-color-scheme` guarded by `:root:not([data-theme="light"])`, then `:root[data-theme="dark"]`), AA contrast in both, 44px targets on coarse pointers, no motion under `prefers-reduced-motion`.
- Data inks (`--blue --orange --gold --green --pink`) never change with the theme because the demos and the figures paint with them; `--mix` dims them into text colour on light ground; `--art` is the dark ground every canvas and every figure keeps in both themes.
- `.tile img` is 3:2 for the demo thumbnails; a page overrides it to square for the figures.
- Panes dock at and above 74rem and slide as drawers below; state lives as `data-left` / `data-right` = `open` | `shut` on `html`, theme as `data-theme` = `light` | `dark` (absent = auto), both in localStorage; `html.js` marks a wired page.

## FILES

- `tokens.css`: custom properties only, surfaces, inks, type, space, shape, frame, motion, dark.
- `base.css`: reset, text, links, focus, `.prose` (headings, lists, tables in `.table`, code, blockquote, figure, MathML), reduced motion, print.
- `chrome.css`: skip link, `.top` header, `.panes` with `.pane.left` / `.pane.right` and `.scrim`, `.tree`, `.contents`, `.settings`, `.base` footer, controls (`.row`, label, select, range, checkbox, button, `.tabs`), `.stats`, `.chip`, tables, `.cards`, `.gallery` / `.tile`.
- `chrome.js`: vanilla ESM, runs on load; header buttons, pane state, theme cycle, contents highlight, footer mark; exports `wire()` for pages that render later.
- `font.js`: vanilla ESM, the pixel font; uses the wasm bridge `globalThis.mrly.font_*` when present, else `font.json`.
- `font.json`: the 5x5 glyphs, `{ char: rows[] }`.
- `mark.json`: the MRLYPROD write-and-hold loop, `{ rows, cols, fps, frames }`, the footer fallback without wasm.
- `chrome.jsx`: React, renders the whole page for `react-dom/client` and `react-dom/server`.
- `site.json`: title, root, since, the tree skeleton (`fill` names a list the site fills), socials, contact (the address the footer links).
- `tsconfig.json`: points `react` at `../net/node_modules` so bun resolves it from here.

## EXPORTS

- `font.js`: `letters(text)` gives `{ rows, cols, grid }`; `animate(text, pad)` and `cycle(text, pad, hold)` give `{ rows, cols, fps, frames }`; `mark(canvas, anim, color)` plays an anim and returns a stop function; `glyphSvg(text)` gives the SVG markup of the glyphs.
- `chrome.js`: `wire()`, idempotent, syncs aria state and attaches the contents observer and the footer mark.
- `chrome.jsx`: `Shell({ route, title, lead, tree, current, contents, controls, wide, children })`, `Header`, `Footer`, `Wordmark({ className })`, `Tree({ nodes, current })`, `Contents({ items, current })`, `Controls({ children })`.
- Tree nodes are `{ name, href?, nodes?, open? }`; contents items are `{ id, text, level }`.
