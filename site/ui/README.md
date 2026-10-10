# ui

- mrly.net's own design kit and its single-page shell: plain CSS, vanilla chrome, React pages, no CSS-in-JS.
- Every page is drawn in the browser from one static shell, `index.html`; the build bundles it with Bun and `404.html` is its copy.

## STYLE

- Palette first (`../kit/palette.css`, `../kit/theme/palette.js`), roles second (`tokens.css`), one class per idea, semantic HTML, AA contrast in both themes, 44px targets on coarse pointers, no motion under `prefers-reduced-motion`.
- Light on `:root`, dark twice: `prefers-color-scheme` guarded by `:root:not([data-theme="light"])`, then `:root[data-theme="dark"]`.
- `--art` is the ground of every canvas and figure.
- Tint is a runtime setting: `data-tint` on `html` names a hue; the shell's `<style>` is `tintCss(site.tint)` from `config.js`, the default as `:root:not([data-tint])` because Bun puts the stylesheet link after it.
- `page.css` is the one sheet the shell links; the router wears `git.css` on `/git/` and `../lib/mrly.css` on an app.

## SHELL

- `index.html`: head (title, description, canonical, og tags, icons, manifest), `boot.js` as a blocking classic script, `page.css`, the tint style, then the static chrome: header, subheader with crumbs and the two pane openers, `.welcome`, `#left`, `#main`, `#right`, scrim, footer.
- The header was rendered once by hand: the `+` to the menu, the fold of the wordmark in the middle (home's welcome needs it before any script), the cart ring.
- `boot.js` imports nothing: it adds `js`, replays theme, font and tint from localStorage and opens the welcome hall on `/`.
- `chrome.js` and `router.js` are the two module scripts; Bun merges them into one entry chunk.

## ROUTER

- `route.js` is pure: `match(rows, path)` (exact row, else the longest `meta.deep` row, else missing; `/index.html` is `/`), `sort(to, from, deep)` (page, inner, mark or native), `named`, `trail`.
- `router.js` reads `site:routes`, `site:pages`, `site:css`; a click or a pop loads the kind chunk and `/raw/<source>` in parallel, renders off-DOM, swaps the ready node inside a View Transition, then mounts the page, registers figure hosts and fires `wire` with the route.
- It sets title, description, canonical, crumbs and `noindex` on missing from the row; the word comes from `wire`.
- Kept from the old router: latest-wins, aria-busy, `y` in `history.state` on scrollend, pagehide and visibilitychange, hash scroll after render, focus `#main`, drawers shut, reduced-motion calm, `hasUAVisualTransition`, pageshow.
- `hint()` on pointerenter, focus and pointerdown fetches the kind chunk and the raw source ahead.
- Links inside `/git/` are `inner`: the git client keeps its own clicks and popstate.
- A failed load reloads once (a sessionStorage key), then shows an error page.

## PAGES

- `pages/<kind>.jsx` exports `render(row, site, host, { text, rows, url, left, right })`, an optional `mount(host)` after the swap and `unmount(host)`.
- `pages/index.js` is the layout table per kind: `left` (the git tree), `right` (`contents`, or `late` for git), `wide`, `bare` (a div, not `article.prose`), `sheet`.
- Kinds: home (heroes from `heroes.js`), hub (`/blog/` grid, `/menu/` doors and search), prose (pages, posts, `/mrlymath/` through `kit/md` and its resolver; KaTeX only when the text holds `$`), app (the chunk from `site:apps`, `mount(host)`/`unmount(host)`), git (`lib/git.js`), stats, settings, cart, paused, missing, moved, figures (hidden: every live figure with its draw ms).
- `parts.jsx`: `show`/`hide` (a React root per host, flushed sync), `Figure`, `Lede`, `Card`, `Grid`, `Menu`, `Settings`, `Contents`, `heads`, `filled` (the site tree with apps, pages and socials filled in).
- `search.js` searches the rows by title, route and lead and lists text rows; menu tiles are figure hosts.

## FIGURES

- A host is `<figure data-figure="name"><canvas></canvas></figure>`; `../lib/figure.js` sizes its canvas to the css box times DPR, capped at 512 in a tile and 1024 elsewhere, draws the figure at its own size on one scratch canvas and downscales into it.
- Hosts queue on intersection, one draw a frame, nearest first, dropped on a route change; a queued host is `aria-busy`.
- Ink is `ink(tinted(theme, { accent, link, theme }))` from `--accent` and `--link`; a real theme or tint change repaints.

## CHROME

- `chrome.js` owns the drawers, theme, font, tint, saver, cart dots, contents highlight, the git tree's lazy folders, the wordmark (`word.js`, the fold on home) and the welcome hall, all by delegated listeners.
- `theme` fires only when the theme or the tint really changes.
- The lock screen loads the saver chunk through the `lock` thunk of `site:apps`.

## TESTS

- `route.test.js`, `pages/index.test.js`, `heroes.test.js`, `search.test.js`, `word.test.js`, `welcome.test.js`, `config.test.js`, `css.test.ts`, `fonts.test.ts`, and `../lib/figure.test.js`.
