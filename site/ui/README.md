# ui

- mrly.net's own design kit, not a package: plain CSS, a little vanilla JS, a little React. No build step, no Tailwind, no CSS-in-JS.
- House style: palette first (the fifteen colors, generated), theme and tokens second (`tokens.css`, by hand), one class per idea, semantic HTML, light on `:root` and dark twice (`prefers-color-scheme` guarded by `:root:not([data-theme="light"])`, then `:root[data-theme="dark"]`), AA contrast in both, 44px targets on coarse pointers, no motion under `prefers-reduced-motion`.
- The palette and the two role sets sit in the kit beside this folder: `../kit/palette.css`, `../kit/theme/palette.js` and `../kit/theme/theme.js`, generated, never hand-edited.
- The code viewer's skin sits there too, `../kit/code/code.css` and `../kit/code/seti/`: it reads this folder's tokens.
- The palette names black, white and thirteen hues (`--red --orange --yellow --green --mint --teal --cyan --blue --indigo --purple --pink --brown --gray`), one flat value each, no shades.
- `tokens.css` resolves the roles `--ground --bg --panel --deep --line --fg --dim --accent --on-accent` per theme.
- `--art` is `var(--ground)`, the ground of every canvas and figure; a figure pair ships as one `<picture>`, the dark WebP in `<source media="screen and (prefers-color-scheme: dark)">` and the light WebP in the `<img>`, so only one half is fetched and print takes the light one.
- `chrome.js` fires `window` event `theme` whenever `data-theme` or `data-tint` changes; scripts that paint with `palette.js` listen and repaint.
- `config.js` `headScript(prefix)` is a tiny inline script for the `<head>`, before any stylesheet: it adds class `js` and replays `data-theme`, `data-font`, `data-tint` and `data-saver` from localStorage, so a page never flashes the wrong theme or the wrong footer.
- Tint is a runtime setting: `data-tint` on `html` is a palette hue name, absent means the site default, and `tintCss(hue)` writes the site default plus a `:root[data-tint=<hue>]` block for every hue but grey.
- A tint is the plain hue on both grounds; `--on-accent` is white and `--ring` follows `--accent`.

## CHROME

- One frame: `--frame` = `--page` + `--pane`, with `--pane` a clamp between 13rem and 17.5rem. It caps the header, the subheader and the block under them, so the two bars sit the same on every page.
- At and above 74rem `.panes` is one flex row, centred: main alone on a page with no bar, main and the bar as one block on a page with one, and on `/git/` the explorer before main. No column is kept empty. Below 74rem a pane is a drawer.
- The right bar is by need: `Shell` draws `aside.pane.right`, its opener in the subheader and the scrim only for `controls` or a contents list that `fills` (`bar.js`: three items or more). A page with neither has no pane, no button and no scrim.
- One markup, three looks: a sticky bar at and above 74rem; below it a drawer with JavaScript; below it without JavaScript the same aside in flow above main (`order: -1`).
- The contents list is one closed `<details>`: `chrome.js` opens it, the wide sheet shows it through `::details-content` before any script runs, and on a narrow screen without JavaScript it stays one collapsed line.
- A demo's bar is a slot its page fills in the browser; without JavaScript a pane that holds only an empty slot is not drawn.
- `/git/`: the built shell holds the explorer and no right pane, its right button written `hidden` with the rule as `data-few`. `lib/git.js` reads that number, makes the pane and shows the button when the file it draws has that many headings, and takes both away when not.
- Two bars. The header is three plain links in the pixel font and nothing else: `+` to `menu`, the middle to `/`, `O` to `cart`; it scrolls away.
- The middle names the app: the X on `/`, elsewhere the lowercase name of the deepest menu door whose href starts the route, else the route's first segment (`word.js`, one rule for the build and the browser). `chrome.js` re-reads it on every `wire`, so a router hop renames it; a mouse hover plays `fold` from `font.js`, the letters merging into the X, reversed on leave, still under reduced motion.
- Header glyphs are drawn at a whole `--cell` of 3px a cell (`--rows` and `--cols` on each svg), so every pixel of the font stays square. With JavaScript the `+` opens the menu as a dialog (DIALOG). The subheader under it is sticky on every page: crumbs on the left, an actions slot on the right. It never repeats menu, home or cart.
- A crumb is read from the page's own path by `crumbs.js` and from nothing else: one link per folder, the segment as its text, the last the page itself with `aria-current`. Home and `/404.html` have none. On `/git/` `lib/git.js` writes them again from the address bar on every view.
- The actions slot is `div.actions`, empty unless the route fills it: the right pane's opener on a page with a bar, and on `/git/` the explorer's opener before it. An opener shows only below 74rem and only with JavaScript; an open drawer fills its button into a solid square, one cell at a time, and drains it on close.
- Opening a drawer moves focus to its first link at once, and Escape hands it back to the opener: the open rule transitions the transform alone, so `visibility` flips with no delay, and under reduced motion nothing inside a pane transitions, since the reset's 0.01ms on every element would pass an inherited `visibility` down one level a frame.
- The scroll rule: `chrome.js` keeps `scrollY` per `location.href` in sessionStorage on `pagehide`, then on `load` drops the key and puts the spot back only on a back-forward or a reload with no hash, so a fresh load never scrolls itself; opening a drawer pins the subheader to the viewport top so the drawers hang from the bar.
- The `O` is the font's O around nine inner cells; `chrome.js` lights one cell per item in the cart, read from localStorage `${prefix}cart` (an array of `{ qty }`), and repaints on the `cart`, `storage` and `pageshow` events.
- Drawer state lives as `data-left` / `data-right` = `open` | `shut` on `html`, never persisted. Theme is `data-theme` = `light` | `dark` (absent = auto), font is `data-font` = `sans` | `serif` | `mono` | `mrly` (absent = system), tint is `data-tint` = a hue name (absent = the site's), saver is `data-saver` = a name of `SAVERS` in `savers/index.js` (absent = the wordmark; `chrome.js` keeps no list of its own, asks that door when it mounts one, and drops the attribute and the key when the door does not know the name), all four in localStorage under `prefix`. `html.js` marks a page with JS, set by the boot script before the first frame.
- Settings, on `/settings/` alone: `Settings()` is four rows in one control style, a label left and its value right. Theme is a button cycling auto, light, dark; Font, Tint and Saver are `.pick` labels around a select, Auto first on Tint and Wordmark first on Saver, every option a plain word. `tokens.css` maps `data-font` to `--face`; `base.css` sets body and `.prose` in `var(--face, ...)`, so the chrome keeps the system face and the reading text changes.
- The tree takes nodes `{ name, href?, nodes?, lazy?, icon? }` and shows them as given; only the code viewer hands it any. A branch is open only when it holds the current page, nothing else is open by default. `icon` is a seti class drawn before the name. A node with `lazy` renders a `<details data-lazy="path">` whose children arrive on first open from the JSON at `/git.json` (`{ base, c: [{ n, k: d|f, i?, c? }] }`, `i` the seti kind); the arrow expands, the name navigates. On a code page the tree is the repository alone, root first, the path to the page open, so the viewer reads like an editor.
- The footer is one screen: the site's wordmark written and held by the pixel-font animation across the whole width, and `Copyright © {company} {since}-{year}. All rights reserved.` under it. No links: the menu page holds them.
- `Grid({ nodes })` is the one gallery: a node with `figure: { dark, light }` is a picture tile, `text` its caption, `dates` its stamps, a node without a figure a plain tile. Every index page, the demo gallery, the home doors and the menu draw it, so they all look the same.
- `Menu({ tree })` lays two levels out: one `Grid` of the top level, a node with `nodes` a folder whose tile links `#<name>`, then one `<section id="<name>">` per folder, a plain heading over a `Grid` of its nodes. Inside `.menu` a grid's columns start at 6rem, so a tile is an icon and its label. `/menu/` is that over the `tree` of `site.json`, every node dressed with a figure; `island` and `search` are written on the `div.menu` for the search bar.
- The footer's legal line sits two pixels off the bottom of the page, over the full-screen animation and never under it.
- Print is one story in three files: `base.css` sets the page margin, black on white, 11pt, the light figure of every pair, the read column full width, headings kept with their text, figures, tables, code and block math unbroken, code wrapped, and every link's target in brackets after it; `chrome.css` turns the skip link, both bars, both panes, the scrim and the footer off, lets main fill the sheet and keeps the opener at 20rem; the kit's `code.css` drops the line numbers and the code frame and wraps long lines. Nothing is hidden but the chrome: a paper or a note prints from the browser as it reads.
- The opener is the page's own figure over the title: one `figure.opener`, at most 32rem wide, centred, hairline framed on the `--art` ground.

## ROUTER

- `router.js` makes the visit one document: a click on a page link fetches that page's HTML and swaps it in. Without JavaScript every link is a page load.
- A page is a same-origin path that ends in `/`, or any path under a deep prefix (`/git/`). The prefixes are the one key the router reads, `<html data-deep>`, written at build from `modes`.
- Left to the browser: another origin, a `target`, a `download`, a modified click, a file (`robots.txt`, a pdf, all of `/raw/`) and a `#fragment` of the page it is on.
- Left to the page's own router: a link under the deep prefix the page is already in. Inside `/git/` the viewer takes every `/git/` link and every pop between them.
- `pointerenter` and `focus` on a page link fetch its HTML ahead of the click; the cache serves a push for 30 seconds and a pop at any age.
- The swap is one task: the title, the head's `meta[name]`, `meta[property]`, canonical, JSON-LD and modulepreload lines, the stylesheets (the next page's are loaded switched off first) and every node between the header and the footer. Then both drawers shut, a `wire` event, and focus on `<main>`.
- Islands: the next page's entries are imported before the swap, the old page's are unmounted first thing in it, the new ones mounted last. A page entered twice mounts twice.
- Scroll: a push opens at the top, or on its `#fragment`; a pop returns to its spot. The spot is `y` in the entry's `history.state`, written before a push, before the old islands unmount and only while the address is still the page on screen, and on `scrollend` while nothing is `aria-busy`. The `/git/` viewer writes the same key, and `stamp` in `lib/query.js` hands the state and the hash through.
- While `html` is `aria-busy` the address can be ahead of the page on screen (a pop still loading): `stamp` writes nothing then, nor on an entry that carries `dialog`, and `chrome.js` keeps no scroll spot at `pagehide`. A full load that resumes an entry lands on the spot `chrome.js` kept, else on the entry's `y`.
- The last ask wins: a click, a pop or a `+` cancels whatever hop or dialog is still loading, and a pop back onto the page on screen cancels it too. A page restored from the back/forward cache drops `aria-busy`.
- `html` is `aria-busy` from the click until the islands are up. Motion is one View Transition around the swap, none under reduced motion.
- Any failure is a plain load of the same url: a fetch that fails, an error status, a redirect, an answer that is not HTML, a page with no header or footer, a page whose head names other site scripts than the ones running (a deploy changed the chrome or the router; a script from another origin, as an extension leaves, is not read), a stylesheet or an entry that does not load, an entry whose bytes changed, a mount that throws.
- Hooks: a link whose `data-router` names a module is fetched ahead but never swapped; the router imports that module on the click and calls its `open(a, live)`, and a rejection is a plain load of the link. `live()` is false once a later ask took over; a module shows nothing then. While a pop is still loading such a link is a page link. `grab(url)` answers a page's HTML from the cache, `read(text)` parses it, `go(url)` navigates, `layer(keys)` pushes an entry on the page's own url.
- A link that asks for a dialog of the page it is on is a link to that page: the router enters it again, in the same document.
- A page link followed from an entry that carries `dialog` replaces that entry, a `/git/` link inside `/git/` included, so Back from the new page lands on the page the dialog was opened on. A link to the page under the dialog is a Back: the dialog closes, nothing is fetched.

## DIALOG

- `dialog.js` shows a page's `<main>` in a fullscreen native `<dialog>`: the header's `+` asks for it with `data-router="<the module>"`, and its `href` is the page, `/menu/`. `O` stays a plain link.
- The page is fetched once per visit, through the router's cache at any age; the built dialog stays in the document, after the footer, and is shown again with no request.
- History: opening pushes one entry on the page's own url with `dialog: <href>` in its state, beside the scroll key `y`. Back closes the dialog with no swap, Forward shows it again, and a reload or a return to that entry opens it at boot; that opening never pushes, and shows nothing once the entry is left.
- A `#fragment` link inside the dialog opens its target as a view: `view: <id>` in the same entry's state, `data-view` on the target and on the dialog. Escape and the close button go up one view, then close; Back closes from any view.
- `chrome.css` draws the views of the menu: in the dialog a folder's section shows only as the view, and the top grid only without one. Without JavaScript, and on `/menu/` itself, a folder is a section of the page.
- The close button sits where the `+` is and wears its glyph turned 45 degrees. Focus goes to it on opening and back to the `+` on closing; a view takes focus to its first link and hands it back to the tile that opened it.
- The page under the dialog is inert and `html` does not scroll while a dialog is open. Escape belongs to the dialog first: `chrome.js` leaves an open drawer alone until the dialog is shut.
- Islands: the page's entries are imported before the dialog is built, mounted when it shows and unmounted when it hides; the loader skips a host inside a closed dialog.
- The dialog wears the sheets of the page under it, so what it shows is styled by `chrome.css`, the one file every page and every demo carries. It hides `.hero`. A page shown in a dialog links by rooted paths.
- Motion is native: `opacity` through `@starting-style` in, `display` and `overlay` with `allow-discrete` out.
- A failure before the dialog shows is a plain load of the link: the fetch, a page with no `<main>`, an entry or the module itself that does not load.

## SEARCH

- `search.js` is an island: the menu's `div.menu` names it and lists the indexes that exist in `data-search`, `/search.json` today. The built page holds no field: without JavaScript nothing of the bar is there.
- `mount` puts one `form[role=search]` first in the menu, an `input[type=search]` over an empty grid, on `/menu/` and in the dialog alike. On a device that hovers the field takes focus, so `+` then letters searches with no click; it leaves focus alone when a folder view or anything else already took it.
- The indexes are fetched on the field's first focus, once per visit; one that fails is asked again on the next focus.
- A row is `[title, route, icon]`, the icon a figure name the tile draws as `/figures/<icon>-dark.webp` and `-light.webp`.
- Every word of the query must be in a row's title or route, any case, any order. The menu's own tiles are matched first, so `/cart/`, `/stats/`, the root files and the socials are found too; then the index rows, in route order, 24 tiles at most.
- Results are the menu's own tile markup in the same grid, inside the form; while a query stands every other child of the menu is `hidden`, and an empty field brings the menu back. The menu's own tiles answer at once and the index rows join when it lands. A `role="status"` line under the grid says the count, the cap or that nothing was found.
- Enter follows the first result; a result is a plain link, so the router takes it. Escape empties a filled field before it reaches the dialog.
- The shop plugs in at the build: a route at `/shop/search.json` adds that path to `data-search`, and the field reads every path listed.

## FOOTER

- One `canvas.mark` inside the home link, with the static wordmark SVG `.still` beside it. `chrome.js` mounts the canvas; everything else is CSS off `data-saver`.
- Absent `data-saver` is the wordmark: the canvas is a `cols+2` by `rows+2` backing store stretched to `width: 100%` with `image-rendering: pixelated`, painted by `../kit/font/font.js` `mark()`.
- Any other `data-saver` mounts `savers/index.js` `saver(canvas, name)`, and CSS lays the same canvas out `position: absolute; inset: 0`, so `frame.js` sizes it to the footer's CSS box times dpr and the pixels are real.
- A canvas keeps its first context for life, so every mount swaps in a fresh `cloneNode` of the old one; the 2d wordmark and a WebGL fractal never share an element.
- The wordmark canvas is `role="img"` with the site title; a screensaver is decoration, so its clone takes `aria-hidden="true"` and drops the role and the label, and gets the role and the label back when the wordmark returns.
- The stretched saver canvas is `pointer-events: none`: it lies over the home link and the link stays clickable through it.
- `mark()` and `saver()` both return a stop function, kept as `canvas.stop`; switching savers stops the old one first, and both pause offscreen.
- No JS shows the SVG and hides the canvas; `prefers-reduced-motion` does the same for the wordmark, while the other savers keep the canvas and draw their one still frame.

## FILES

- `tokens.css`: joined after the kit's `palette.css`; the roles light on `:root` and dark twice, `--art --scrim --mix`, type (system stacks ending in Noto Symbols 2 and Noto Color Emoji as fallbacks), `--face` per `data-font`, space, shape, frame and motion.
- One stylesheet per page, joined by the `sheets` block in `site.json`, in this order: `palette.css`, `tokens.css`, `base.css`, `chrome.css`, `fonts/fonts.css`, `pages.css`. That is `page.css`; `git.css` adds the code viewer's `code.css` and `seti.css`; a demo shell links `fonts.css` beside its bundled `lib/mrly.css`.
- `pages.css`: what the chrome has no rule for, the tiles, the home, the openers and the plates.
- `base.css`: reset, text, links, focus, `.prose`, reduced motion, print.
- `chrome.css`: skip link, `.top` header, `.subheader` bar with its `.crumbs` and `.actions`, `.panes` with `.pane.left` / `.pane.right` and `.scrim`, `.tree`, `.contents`, `.settings` with its `.theme` button and `.pick` selects, `.menu`, `dialog`, `.base` footer, controls (`.row`, `.set`, label, select, range, checkbox, `button` and `.button`, `.tabs`), `.stats`, `.chip`, `.badge`, tables, `.cards`, `.gallery` / `.tile`, `.opener`, and the chrome's print rules.
- `chrome.js`: vanilla ESM, runs on load; the scroll spot, drawers, theme, the `<picture>` halves under a `data-theme` override, font, tint, saver, cart, contents highlight, footer mark, lazy tree; a page that draws a contents list later fires a `wire` event on `window` and the chrome wires it.
- `../kit/font/font.js`: vanilla ESM, the pixel font and its choreography, in the kit. The crate `pkgs/mrlyrs` is the source of truth: `scripts/wasm.sh` runs its `book` example into `site/kit/font/font.json`. `font.js` reads the book and ports only the arithmetic: the layout, the write, the phased merge that folds the letters into one centred stack, and the loop. It uses the wasm bridge `globalThis.mrly.font_*` when present; `../kit/font/font.test.js` pins the crate's numbers and checks every frame against the wasm, and skips itself where `site/pkg` is absent.
- `../kit/font/font.json`: generated by the crate, `{ char: { rows, path } }`, the bitmap rows and the cell-by-cell stroke order over the trimmed glyph.
- `islands.js`: vanilla ESM, the island loader; `router.js` imports it, so every page holds the same one (the site README has the contract).
- `router.js`: vanilla ESM, the site router, on every page after `chrome.js`; ROUTER below.
- `dialog.js`: vanilla ESM, loaded by the router on the first hover, focus or click of a link that names it; DIALOG above.
- `stats.js`: vanilla ESM, an island; `/stats/` names it and it polls `/stats/stats.json` into the two `[data-stats]` mounts under its host, reusing `.table`, `.prose` and `.fine` and adding no CSS.
- `claims.js`: vanilla ESM, an island; a claims page's `form.filter` names it and it filters the page's `section.claims` by tag chip and by date, reading the pressed chip back from the markup.
- `search.js`: vanilla ESM, an island; SEARCH above.
- `logo.js`: the MrlyLogo mask, which is the font's `X`, plus `grid(level)` and `logoSvg(level, fill, ground)`; the builder draws the favicon and the icons from it.
- `chrome.jsx`: React, renders the whole page at build for `react-dom/server`; no browser bundle imports it.
- `contents.jsx`: the contents list alone, a closed `<details>`, nothing under three items; the one part a demo also draws in the browser.
- `bar.js`: the bar's one rule, `FEW` and `fills(items)`; `chrome.jsx` and `contents.jsx` import it, and no other browser entry does, so it never becomes a chunk of its own.
- `crumbs.js`: the subheader's one rule, `crumbs(path)`; `chrome.jsx` reads it at build and `lib/git.js` in the browser.
- `config.js`: `configure(site)` takes `site.json`, `conf()` reads it back, `tintCss()` writes the accent blocks, `headScript()` writes the boot script. Keys the kit itself reads: `title since prefix tint menu cart company`.
- `savers/`: the four screensavers the footer can wear, vanilla ESM over one canvas, with their own README. Every one inks itself from `--accent`, so the Tint setting is their primary colour. `site.json` lists `savers/*.js` (`tiles.js` included) beside `logo.js` in the `ui` bundle, and the kit's `theme/palette.js` and `theme/theme.js` in the `kit` bundle, or the footer has nothing to import.
- `fonts/`: `fonts.css` and the faces, all OFL with their licences beside them: Noto Sans, Noto Serif and Noto Sans Mono (variable 400-700, Latin), MrlyFont from `pkgs/mrlyrs`, Noto Sans Symbols 2, and Noto Color Emoji in eleven unicode-range shards. `site.json` lists them under an `assets` bundle with `hash: true`, the builder rewrites each `url()` in the css to the hashed name, and `fonts.css` rides inside the page's one sheet; a face is fetched only when a page needs it.
- `bun run vendor` writes `fonts.css` whole from the `fonts` block in `site.json`: the Google families first, then `fonts/keep.css` verbatim, the two faces this site cuts itself, so `mrly.woff2` and `symbols.woff2` survive every run. Add a shard or a family there, not by hand, and list the new file in the `assets` bundle.
- `symbols.woff2` is cut from the master at `files/fonts/symbols.ttf` down to the glyphs the built pages print, with `bun run symbols`, which rewrites its `src` and `unicode-range` in both `keep.css` and `fonts.css`; `fonts.test.ts` fails if a page prints a symbol the cut face lacks.

## EXPORTS

- `../kit/font/font.js`: `letters(text)` gives `{ rows, cols, grid }`; `animate(text, pad)` writes, `merge(text, pad)` folds, `cycle(text, pad, hold)` chains write, hold, merge, hold, unfold, hold, unwrite, hold into `{ rows, cols, fps, frames }`; `mark(canvas, anim)` plays an anim in the canvas's own `color`, repaints on the `theme` event, and returns a stop function the caller must keep.
- `chrome.js`: none; it boots when loaded. Its `wire()` is idempotent: it syncs aria state, stops the mark and the contents observer of whatever left the document, applies theme, font, tint and saver, paints the cart, and attaches the contents observer and the footer mark, and a `wire` event on `window` runs it again.
- `islands.js`: `load(root = document)`, `mount(root = document)` and `unmount(root = document)`.
- `router.js`: `go(url, mode = 'push')`, `grab(url)`, `read(text)`, `layer(keys)`, and the rule alone, `sort(to, from, deep)` and `walled(path, deep)`; it boots when loaded.
- `dialog.js`: `open(a, live)`, a promise that rejects when the page cannot be shown.
- `stats.js`, `claims.js`, `search.js`: `mount(host)` and `unmount(host)`; `search.js` also the rule alone, `terms(query)` and `meets(words, text)`.
- `crumbs.js`: `crumbs(path)` gives `[{ name, href }]`, one per folder of the path; a path with no closing slash ends on the file itself.
- `config.js`: `configure(site)`, `conf()`, `HUES`, `tintCss(hue)`, `headScript(prefix)`, `inlineScripts(prefix)`, the boot list: the one inline script a page may carry.
- `chrome.jsx`: `Shell({ route, tree, current, contents, controls, late, wide, children })`, where `route` gives the subheader its crumbs, `tree` draws the left pane and its opener only when it holds nodes, `controls` or three `contents` items draw the right pane, `controls: true` leaves an empty slot in it and `late` writes the right button hidden, with `data-few`, for a page that makes its bar in the browser, and `dialog` is the module the header's `+` names, `Glyph({ text, className, label })`, `Grid({ nodes })`, `Menu({ tree, island, search })`, `Settings()`; the header, subheader, tree, contents, controls and footer are Shell's own parts, not doors.
- The header always draws the site's wordmark; `prefix` moves the localStorage keys and is read in the browser from `<html data-prefix>`.
