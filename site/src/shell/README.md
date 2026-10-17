# SHELL

- mrly.net's single-page shell: vanilla chrome, React pages; the sheets are in `../styles/`.
- Every page is drawn in the browser from one static shell, `index.html`; the build bundles it with Bun and `404.html` is its copy.

## SHELL

- `index.html`: head (title, description, canonical, og tags, icons, manifest), `boot.js` as a blocking classic script, `../styles/page.css`, the empty `<style id="tint">` the build and dev fill with `tintCss(site.tint)`, then the static chrome: header, subheader with crumbs and the two pane openers, `.welcome`, `#left`, `#main`, `#right`, scrim, footer.
- The header was rendered once by hand: the `+` to the menu, the fold of the wordmark in the middle (home's welcome needs it before any script), the cart ring.
- `boot.js` imports nothing: it adds `js`, replays theme, font and tint from localStorage, marks `/` with `data-desk` and opens the welcome hall there.
- `chrome.js` and `router.js` are the two module scripts; Bun merges them into one entry chunk.
- `lock.js` runs a saver for the desk and the lock screen: it picks a scene from `apps:scenes`, awaits its `units` and runs it with `../lib/app/run.js`.
- `crumbs.js` draws the subheader trail; `figures.js` loads `../lib/figure.js` with `site:figures` once, for the router and the research page.

## ROUTER

- `route.js` is pure: `match(rows, path)` (exact row, else the longest `meta.deep` row, else missing; `/index.html` is `/`), `sort(to, from, deep)` (page, inner, mark or native), `named`, `trail`.
- `router.js` reads `site:routes`, `site:css`; a click or a pop loads the row's shell (and, for an app, its chunk) and `/raw/<source>` in parallel, renders off-DOM, swaps the ready node inside a View Transition, then mounts the page, registers figure hosts and fires `wire` with the route.
- It sets title, description, canonical, crumbs and `noindex` on missing from the row; the word comes from `wire`.
- Kept from the old router: latest-wins, aria-busy, `y` in `history.state` on scrollend, pagehide and visibilitychange, hash scroll after render, focus `#main`, drawers shut, reduced-motion calm, `hasUAVisualTransition`, pageshow.
- `hint()` on pointerenter, focus and pointerdown fetches the shell, a render page's chunk and the raw source ahead.
- Links inside `/git/` are `inner`: the git client keeps its own clicks and popstate.
- A failed load reloads once (a sessionStorage key), then shows an error page.

## PAGES

- Two shells, one per row kind; each exports `page(row)`, a page `{ layout, render(row, site, host, { text, rows, url, left, right }), mount?(host, { rows, site, url }), unmount(host) }`.
- `app.jsx`: an island app (`mount(at)`/`unmount(at)`) swaps in as its still at once and loads its chunk from `site:apps` in `mount`; an app that exports `render` (`own`, scanned at build) is the page itself and loads before the swap.
- `prose.jsx`: pages, posts and `/mrlymath/` through `../lib/md` and its resolver; KaTeX only when the text holds `$`.
- `layout`: `left` (the git tree), `right` (`contents`, or `late` for git), `wide`, `bare` (a div, not `article.prose`), `sheet`.
- `hub.jsx`: `hub(body)`, the page of `/blog/` and `/menu/`: a lede, the body, the search on the menu.
- `parts.jsx`: `show`/`hide` (a React root per host, flushed sync), `Figure`, `Lede`, `Lost` (the missing page, also research's unknown slug), `Card`, `Grid`, `Menu`, `Settings`, `Contents`, `heads`, `filled` (Play, Read, Tools from the rows, then Root and Elsewhere).
- `search.js` searches the rows by title, route and lead and lists text rows; menu tiles are figure hosts.

## FIGURES

- A host is `<figure data-figure="name"><canvas></canvas></figure>`; `../lib/figure.js` sizes its canvas to the css box times DPR, capped at 512 in a tile and 1024 elsewhere, draws the figure at its own size on one scratch canvas and downscales into it.
- Hosts queue on intersection, one draw a frame, nearest first, dropped on a route change; a queued host is `aria-busy`.
- Ink is `ink(tinted(theme, { accent, link, theme }))` from `--accent` and `--link`; a real theme or tint change repaints.

## CHROME

- `chrome.js` owns the drawers, theme, font, tint, saver, cart dots, contents highlight, the git tree's lazy folders, the wordmark (`word.js`, the fold on home) and the welcome hall, all by delegated listeners.
- `theme` fires only when the theme or the tint really changes.
- The desk is home: the header and one full-screen saver canvas, no scroll, no subheader, no footer (`data-desk`). `chrome.js` appends it on `/` and drops it on leaving; it loads the saver through the `lock` thunk of `site:apps`.
- The pick is the Settings Saver, unset meaning the desk saver of `site.json`; the lock rolls a seed and names what it ran (`stop.as`), and the chrome reuses that while the pick stands, so the desk is the same picture all visit.
- First visit (the welcome hall open) is the lock screen: the saver plays under the big wordmark, which fades after 3 s of no mouse. Unlocking leaves the scene (`leave`), glides the wordmark into the X and coasts the saver to a standstill over 3 s (`stop.coast`): the screensaver becomes the desktop picture.
- A later visit plays the desk 3 s, then coasts. Tapping the X replays the lock: the frozen desk resumes and the wordmark unfolds over it.

## TESTS

- `route.test.js`, `shells.test.js`, `search.test.js`, `word.test.js`, `welcome.test.js`, `chrome.test.js`, `lock.test.js`, and `../lib/figure.test.js`.
