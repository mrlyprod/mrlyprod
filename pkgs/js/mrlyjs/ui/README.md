# ui

- The design kit of every MrlyProd site, mrly.net first: plain CSS, a little vanilla JS, a little React. No build step, no Tailwind, no CSS-in-JS, so the CSS lifts into a Shopify theme as is.
- House style: palette first, tokens second, one class per idea, semantic HTML, light on `:root` and dark twice (`prefers-color-scheme` guarded by `:root:not([data-theme="light"])`, then `:root[data-theme="dark"]`), AA contrast in both, 44px targets on coarse pointers, no motion under `prefers-reduced-motion`.
- The palette is generated, never hand-edited: `palette.css` for CSS and `palette.js` for scripts, the same numbers in both.
- `palette.css` names black, white and thirteen hues (`--red --orange --yellow --green --mint --teal --cyan --blue --indigo --purple --pink --brown --gray`), each with a `-light` and a `-dark` shade.
- Each theme resolves the roles `--ground --bg --panel --deep --line --fg --dim --accent --on-accent` and the thirteen theme inks `--ink-<hue>`: the base hues on dark, the `-dark` shades on light, so inks change with the theme and stay AA on their ground.
- `palette.js` exports `palette`, `dark`, `light`, `hues` and `inks`; `dark.blue` is what `--ink-blue` resolves to on the dark theme.
- `--art` is `var(--ground)`, the ground of every canvas and figure; figures ship as `<img class="dark">` and `<img class="light">` pairs and `--show-dark` / `--show-light` pick one.
- `chrome.js` fires `window` event `theme` whenever `data-theme` changes; scripts that paint with `palette.js` listen and repaint.
- `.tile img` is 3:2 for the demo thumbnails; a page overrides it to square for the figures.
- Panes dock at and above 74rem and slide as drawers below; state lives as `data-left` / `data-right` = `open` | `shut` on `html`, theme as `data-theme` = `light` | `dark` (absent = auto), both in localStorage; `html.js` marks a wired page.

## FILES

- `palette.css`: generated, the hues, the shades, the roles and the theme inks, light on `:root` and dark twice.
- `palette.js`: generated, the same palette for scripts.
- `tokens.css`: loaded after `palette.css`; `--art --scrim --mix` (chips and code tokens mix an ink into `--fg` by `--mix`, 45% light, 100% dark), type, space, shape, frame, motion, dark.
- `base.css`: reset, text, links, focus, `.prose` (headings, lists, tables in `.table`, code, blockquote, figure, MathML), `img.dark` / `img.light`, reduced motion, print.
- `chrome.css`: skip link, `.top` header, `.panes` with `.pane.left` / `.pane.right` and `.scrim`, `.tree`, `.contents`, `.settings`, `.base` footer, controls (`.row`, label, select, range, checkbox, button, `.tabs`), `.stats`, `.chip`, tables, `.cards`, `.gallery` / `.tile`.
- `chrome.js`: vanilla ESM, runs on load; header buttons, pane state, theme cycle and its `theme` event, contents highlight, footer mark; exports `wire()` for pages that render later. Storage keys take the `data-prefix` of `<html>`.
- `font.js`: vanilla ESM, the pixel font; uses the wasm bridge `globalThis.mrly.font_*` when present, else `font.json`.
- `font.json`: the 5x5 glyphs, `{ char: rows[] }`.
- `mark.json`: the MRLYPROD write-and-hold loop, `{ rows, cols, fps, frames }`, the footer fallback without wasm.
- `chrome.jsx`: React, renders the whole page for `react-dom/client` and `react-dom/server`.
- `config.js`: the kit's config. `configure(site)` takes the consumer's `site.json`, `conf()` reads it back, `tintCss()` writes the accent override. Keys: `title root since prefix font settings tint tree socials contact`.
- `code.css`: the `tk-*` token colours the `git` highlighter emits and the `d2`-`d6` gutter widths, both drawn from the tokens above. Loaded on code pages only.
- `seti/`: the SETI file-icon font. `seti.css` carries the `@font-face` and one `.si-*` rule per kind, `seti.woff2` sits beside it so the `url()` resolves at the served path, `seti.ts` maps a path to its classes, `LICENSE-seti.txt` is the vendor licence.
- `tsconfig.json`: points `react` at `sites/net/node_modules` so bun resolves it from here.

## EXPORTS

- `font.js`: `letters(text)` gives `{ rows, cols, grid }`; `animate(text, pad)` and `cycle(text, pad, hold)` give `{ rows, cols, fps, frames }`; `mark(canvas, anim, color)` plays an anim and returns a stop function; `glyphSvg(text)` gives the SVG markup of the glyphs.
- `chrome.js`: `wire()`, idempotent, syncs aria state and attaches the contents observer and the footer mark.
- `seti/seti.ts`: `seti(path)` gives the class string for a path, `si si-rust` for `main.rs`, plain `si` for an unknown kind.
- `chrome.jsx`: `Shell({ route, title, lead, tree, current, contents, controls, wide, brand, note, children })`, `Header({ brand })`, `Footer({ note })`, `Wordmark({ className })`, `Tree({ nodes, current })`, `Contents({ items, current })`, `Controls({ children })`, `Settings`.
- `brand` and `note` are the header and footer slots: pass nothing and the kit draws mrly.net's wordmark and footer mark.
- `font: false` swaps every pixel glyph for plain text; `settings: false` drops the theme pane; `prefix` moves the localStorage keys and is read in the browser from `<html data-prefix>`.
- Tree nodes are `{ name, href?, nodes?, open? }`; contents items are `{ id, text, level }`.
