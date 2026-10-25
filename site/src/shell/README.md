# SHELL

- mrly.net's single-page shell: vanilla chrome, React pages; the sheets are in `../styles/`.
- Every page is drawn in the browser from one static shell, `index.html`; the build bundles it with Bun and `404.html` is its copy.

## SHELL

- `index.html`: head (title, description, canonical, og tags, icons; no manifest, so no install), `boot.ts` as a blocking classic script, `../styles/page.css`, the empty `<style id="tint">` the build and dev fill with `tintCss(site.tint)`, then the static chrome: the bar, `.welcome`, the window (title bar, `#main`, `#right`, scrim).
- The bar is two glass bubbles drawn once by hand: the trio (`+` to the Launchpad, the mark, the cart ring, then the app's name and its menus in an app) and the tray (date, clock, search). The mark holds the fold of the wordmark (home's welcome needs it before any script) and shows only its X.
- `boot.ts` imports nothing: it adds `js`, replays theme, font, tint, glass (`off` under Reduce transparency, the site's or the OS's), text size, motion, contrast, targets, the icon look and the window widths from localStorage, sets `data-form` (`phone` on a coarse pointer up to 44rem, so a portrait iPad is a desk, or any screen up to 40rem, else `desk`) and keeps it current, marks `/` with `data-desk` and `data-window="none"` and opens the welcome hall there.
- `chrome.ts` and `router.ts` are the two module scripts; Bun merges them, with what they import, into one entry chunk.
- `savers.ts` is the saver registry: `all` (`site.savers`), `find` (a saver id, or an app id for its first saver), `on` (the ids not switched off), `load` (the entry, its scene from `apps:scenes`, its `SAVERS` item and its validated `still`, or the implied one of an app with `"saver": true`), `roll` (the seed plus `pick` fed by `rng(seed)`), `shuffle`.
- `lock.ts` runs a saver for the desk, the lock screen and the screen saver: `lock(canvas, pick, { pool })`, where the pick is '' (`site.desk`), `random` (one of the pool, `on()` by default, else every saver), a saver id (a rolled value) or a saver id with a query (that value); an unknown id is `random`. It awaits the scene's `units`, runs it with `../lib/app/run.ts` (still under reduced motion) and names what it ran (`pick`, `id`, `as`).
- `idle.ts` is the screen saver: `watch()` starts one idle timer (any input resets it; `minutes()`, re-read when the setting changes), `start({ only })` shows it now, `stop()` ends it. It never starts with no saver on, on a hidden page, under the welcome hall, an open dialog or zen. While up: `html[data-saver]`, a full-screen `.saver` (`../styles/saver.css`) over the shuffled savers that are on, the next one every 90 s with a cross-fade, the time in MrlyFont and the date unless the clock is off, no cursor; a key, click, wheel, touch or a pointer move past 8 px ends it after its first second. It fires `saver` (`detail` true, then false).
- `menus.ts` draws the app's bar menus from the model its `menus` event hands over (`../lib/app/menus.ts`), adds Go (Home, Launchpad, the last five apps, the section's other apps) and the topic's research note under Help, and runs each pick through the model; a menu is a popover of glass (`pulldown`), arrows walk it and across menus, Escape closes it.
- `figures.ts` loads `../lib/figure.ts` with `site:figures` once, for the router, the research page and the home pieces.

## HOME

- Three pieces sit on the desk, each a React root of its own: the dock (`dock.tsx`), the widgets (`widgets.tsx`) and the Launchpad (`launchpad.tsx`, the page of `/menu/`). `chrome.ts` lays the dock on the first route and the widgets on the first visit home; both stay for the visit.
- `launch.ts` is pure and tested: `arrange` (a saved layout over `site.launchpad`: its order and names kept, lost ids dropped, new ids back in their own section), `docked`, `spare` and `renamed` (no name twice), `exported` (the JSON of `../../launchpad.json`, byte for byte), `visited` and `recents`, `gist`, `ago`, `rank` (search: a title that starts with the query, then a word that does, then one that holds it, the lead only for words of 3 letters or more), `today` (the UTC date) and `daily` (one index per date).
- `catalog.ts` binds them to the site and localStorage: `app(id)`, `shelves` and `shelve`, `dock` and `redock`, `unshelve` (both back to `launchpad.json`), `recent` and `track` (each `wire` and each app `stamp` keeps the app's exact url), `notes` (`/research.json`, fetched once), `type` and `take` (keys typed on home before the Launchpad is up), `away` (back while `depth` is above 0, else home).
- `tile.tsx`: `Tile`, the one launcher: a real link holding the route's app icon and, unless `name` is false, its title.
- `appicon.tsx`: `AppIcon`, a route's app icon from `site:icons` (`../lib/appicon.ts`), inline SVG in a `.face`; the first one prepends the shared sprite (shape, plates, rims, filters, every icon's gradients). Tile, the dock's Launchpad (`menu`) and Continue wear it.
- The dock: the Launchpad, then each group of `launchpad.json` `dock` behind a divider; it shows on home, on page windows and over the Launchpad, never in an app (`[data-window="app"]`), during widget editing or under a phone window. Drag to reorder, drop from the Launchpad or Recent to add, drag out to remove; Magnify (Settings) grows the icons under the pointer.
- Settings > Desktop & Dock lists the dock apps, each with Remove, and an Add to Dock picker of the apps not in it.
- The Launchpad (`/menu/`): a glass panel over the blurred desk, between the bar and the dock. The chips filter by section; All shows Recent (the last 9 apps opened in this browser) and every section. Drag apps within and across sections, drag chips to reorder sections, double-click a chip or heading or press F2 on a chip to rename, `+` for a new section; an empty section offers its removal. The context-menu key or Shift+F10 on a tile opens Add to Dock (or Remove from Dock) and Move to each other section (none from Recent). More: New Section, Rename Section (with a section chip on, not while searching), Export JSON (downloads and copies `launchpad.json`), Reset Layout (with Undo).
- Search: typing on home, Cmd/Ctrl+K and the tray's search open it; results group Apps, Research, Posts and Settings; Tab, Shift+Tab and the arrows move the highlight, Enter opens it, Escape clears the field and then closes. The `+`, the dock's Launchpad and Escape close it.
- Widgets (home only): Clock (the zone's city, the time in MrlyFont), Design of the day (one design a date for everyone, from the classes of the 2D and 3D spaces whose dimension is past half their own; it links to Designs), Continue (the last app at its exact url), Research (the newest topic by its `date`). Drag to rearrange; Edit Widgets (the button under the column, or right-click on the desk) shows a remove badge on each widget and the gallery, where a widget is added by its button or by dragging it in.
- The order of each piece lives in localStorage (`launchpad`, `dock`, `widgets`, `recent`); `magnify` keeps the dock setting.

## ROUTER

- `route.ts` is pure: `match(rows, path)` (exact row, else the longest `meta.deep` row, else missing; `/index.html` is `/`), `sort(to, from, deep)` (page, inner, mark or native), `named`.
- `router.ts` reads `site:routes`, `site:css`; a click or a pop loads the row's shell (and, for an app, its chunk) and `/raw/<source>` in parallel, renders off-DOM, swaps the ready node inside a View Transition, then mounts the page, registers figure hosts and fires `wire` with the route.
- A `data-form` change redraws a gated app through `go(location, replace)`, so a rotated or resized screen swaps the app and its gate.
- It sets title, description, canonical and `noindex` on missing from the row, and dresses the window: `html[data-window]` from the layout, the title bar's name, `data-lens` on a page or wide window, the app's name in the trio.
- `history.state.depth` counts the pages this tab opened on the site; the window's close goes back while it is above 0, else home.
- Kept from the old router: latest-wins, aria-busy, `y` (the scroll of `#main`, the window's scroller) in `history.state` on scrollend, pagehide and visibilitychange, hash scroll after render, focus `#main`, drawers shut, reduced-motion calm, `hasUAVisualTransition`, pageshow.
- `hint()` on pointerenter, focus and pointerdown fetches the shell, a render page's chunk and the raw source ahead.
- Links inside `/git/` are `inner`: the git client keeps its own clicks and popstate.
- A failed load reloads once (a sessionStorage key), then shows an error page.

## PAGES

- Two shells, one per row kind; each exports `page(row)`, a page `{ layout, render(row, site, host, { text, rows, url, right }), mount?(host, { rows, site, url }), unmount(host) }`.
- `app.tsx`: an island app (`mount(at)`/`unmount(at)`) swaps in as its still at once and loads its chunk from `site:apps` in `mount`; an app that exports `render` (`own`, scanned at build) is the page itself and loads before the swap.
- `prose.tsx`: pages, posts and `/mrlymath/` through `../lib/md` and its resolver; KaTeX only when the text holds `$`.
- `layout`: `right` (`contents`, the prose page's table of contents), `wide`, `bare` (a div, not `article.prose`), `sheet`, `window`.
- `window`: `page` (the default: 720px, at most 85% of the height, scrolling inside, both edges resize it together), `wide` (the default of a `wide` layout: up to 80rem, panes docked while the window is at least 60rem wide), `app` (a scene app: one window under the bar, no title bar), `none` (home).
- `app.tsx` puts a scene app (`meta.scene`) in the `app` window and any other island in a `wide` one.
- On `html[data-form="phone"]`, `app.tsx` draws a gated app (`gated(row)`: `meta.scene`, Kit, Screensavers, Figures) as a `page` window: the icon, title, lead, still figure, "Needs a bigger screen", Open on a computer and Copy link (both copy the url); no chunk loads.
- `hub.tsx`: `hub(body)`, the page of `/blog/`: a lede and the body.
- `parts.tsx`: `show`/`hide` (a React root per host, flushed sync), `Figure`, `Lede`, `Lost` (the missing page, also research's unknown slug), `Card`, `Grid`, `ART` and `Glyph` (the 5x5 pixel glyphs), `PANES` (the Settings panes the Launchpad search finds, re-exported from `panes.ts`), `Contents`, `heads`.

## FIGURES

- A host is `<figure data-figure="name"><canvas></canvas></figure>`; `../lib/figure.ts` sizes its canvas to the css box times DPR, capped at 512 in a tile and 1024 elsewhere, draws the figure at its own size on one scratch canvas and downscales into it.
- Hosts queue on intersection, one draw a frame, nearest first, dropped on a route change unless scanned with `keep` (the dock and the widgets); a queued host is `aria-busy`.
- Ink is `ink(tinted(theme, { accent, link, theme }))` from `--accent` and `--link`; a real theme or tint change repaints.

## CHROME

- The chrome is four files, wired by delegated listeners.
- `chrome.ts`: the drawers, the cart dots, the clock, the contents highlight, the scroll spot, the window's edges (`../lib/handle.ts`), close and zoom, Cmd+K or Ctrl+K to search, typing on home, the dock and the widgets, the glass (`../lib/glass/`, loaded after the first route), and `wire`, which runs the rest on every route.
- `prefs.ts`: localStorage (`KEY`, `read`, `write`, `subscribe`), theme, tint, font, glass, icons, magnify, the saver pick, the screen saver (`offs`, `savers`, `idle`, `minutes`, `clock`), the clock's `hours` (12, 24, or unset for the locale's) and `stamp` (the time in that cycle), `replay`, and accessibility (`text`, `motion`, `contrast`, `targets`, `clear`, `calm`).
- `write` tells subscribers only when the stored value changes; another tab's change arrives through the `storage` event, heard from load; `replay(key)` re-applies the html attribute or style that key drives, then subscribers hear it.
- Glass is `off` while Reduce transparency is on, the site's (`clear`) or the OS's; `calm()` is the site's Reduce motion or the OS's.
- `SETTINGS` names what a settings link or file carries: `pack` and `unpack` (validated; a name it lacks is cleared), `encode` and `decode` (unpadded base64url of the JSON), `share(origin)` for the link; `stored()` lists every key for Privacy & Data.
- Icons copy Apple's Icon & widget style: Default, Dark, Clear, Tinted, then Auto (follow the theme) or Always (the dark look); Settings > Appearance sets them through `icons(style, always)`. `look(style, always, dark)` resolves `html[data-icons]`: absent, `dark`, `clear-light`, `clear-dark`, `tinted-light` or `tinted-dark`; `shade()` reruns it on a theme change.
- `desk.ts`: the mark (the fold on home, the X elsewhere), the welcome hall, the desk and its lock screen; `visit(detail)` hands it the route; the first `desk()` starts the idle watch of `idle.ts`.
- `tree.ts`: the git tree's lazy folders (`branch`, `turns`, `expand`).
- `theme` fires only when the theme or the tint really changes.
- The desk is one full-screen saver canvas under every route; the page never scrolls, the window does. `desk.ts` appends it on the first route and keeps it; it loads the saver through the `lock` thunk of `site:apps` and hands its canvas to the glass as a source; the screen saver's canvases are never a source. Home (`data-desk`) is the desk with no window.
- The pick is the Settings Saver, unset meaning `site.desk`; the lock rolls a seed and the saver's value and names what it ran (`stop.as`), and the chrome reuses that while the pick stands, so the desk is the same picture all visit.
- First visit (the welcome hall open) is the lock screen: the saver plays under the big wordmark, which fades after 3 s of no mouse. Unlocking leaves the scene (`leave`), glides the wordmark into the trio's X, flies in `+` and the cart, lights the bubbles and coasts the saver to a standstill over 3 s (`stop.coast`): the screensaver becomes the desktop picture.
- A later visit plays the desk 3 s, then coasts. The X replays the lock on home and goes home anywhere else.

## TESTS

- `route.test.ts`, `router.test.ts`, `shells.test.ts`, `launch.test.ts`, `launchpad.test.tsx`, `widgets.test.tsx`, `app.test.tsx`, `prose.test.tsx`, `lens.test.ts`, `tiles.test.ts`, `welcome.test.ts`, `tree.test.ts`, `lock.test.ts`, `savers.test.ts`, `idle.test.ts`, `prefs.test.ts`, `store.test.ts`, and `../lib/figure.test.ts`.
