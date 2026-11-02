# SHELL

- The single-page shell: vanilla chrome, React pages; its own sheets are `page.css` and `gate.css`, the rest are a design's or `../../designs/shared/` (`base.css` among them).
- Every page is drawn in the browser from one static shell, `index.html`; the build bundles it with Bun and `404.html` is its copy.
- `package.json` names it `@mrlyprod/shell`: a site or design outside the repo imports `@mrlyprod/shell/<file>`.

## SHELL

- `index.html`: head (title, description, canonical, og tags, icons, the light and dark `theme-color` from the design's `bar`; no manifest, so no install), `boot.ts` as a blocking classic script, `page.css`, the empty `<style id="tint">` the build and dev fill with `tintCss(site.tint)`, then a body whose only text is the domain; `{title}`, `{tagline}`, `{prefix}` (`html[data-prefix]`), `{root}` and `{domain}` are filled from `site.json` the same way.
- `page.css` imports every sheet under `(scripting: enabled)`, so a visitor without JavaScript sees the bare domain in raw HTML; the build hashes the sheet, so `boot.ts` cannot name it.
- `mount.ts` is the first module to run: it checks the screen against `site.min` (`gate.ts`, 1280x720 when absent), sets `html[data-floor]` below it and keeps it current on resize, swaps the domain for the chrome of `site:frame` and appends the logo (`site.mark`, `LOGO` px square); under `data-floor` only the logo shows (`gate.css`).
- `frame.tsx` composes the chrome through the active design's `bar` and `window` parts: the skip link, the bar inside `.barred` (with no clock), `.welcome`, the window (title bar, `#main`, `#right`, scrim); the build renders it once into `site:frame`, so the entry holds a string and no React. Its binds carry the shell's own hooks (`data-menus`, `data-close`, `data-zoom`, `data-tools`, `data-title`, `data-edge`, `data-scrim`, `data-cart`); the shell never finds a design's class.
- `active.ts` names the active design (`site:design`, the folder of `../../designs/` the build drew: `MRLY_DESIGN`, else `site.json` `design`, else a random draw among the designs `lacks()` passes) and its `parts`; `measure.ts` its sizes, in px at the default text size (window widths and their floor, the step, the pane widths, gap and stage room, the `gutter` a window keeps from each side of the viewport, the `edge` a popup menu keeps from it, the `thumb` side of a library thumbnail, the optional `dock`, the width at which a wide window docks its pane (never when left out), and the optional `lens`, the dock magnify's `big` scale and `reach` in icons (no magnify when left out)) for the code that runs before React.
- `bar.tsx` renders the design's `bar` live into `.barred` from the first route: the site, the app's name in an app window, the cart count, the clock (each minute and on the hours setting); its binds toggle the Launchpad (`launch`), focus its search (`find`), replay the lock (`mark`, `knock`) and host the bar menus (`menus`).
- `design.ts` is the design contract: `Design` (id, title, light and dark skin tokens, `bar` (the light and dark `theme-color`), css, `measure`, one component per part, optional `knobs`, an optional `pulldown`, an optional `start`), `REQUIRED` and `OPTIONAL` parts with their `Props` (data, callbacks and `bind` bags, no style), `Knobs` (`icons`: the design draws `html[data-icons]`; `own`: its own settings, each an `id`, a `name`, `options` and a `hint`, kept under `<prefix>knob-<id>` and worn as `html[data-<id>]`), `Pulldown` (the bar menus' `drop` gap and `button`, `sheet` and `item` builders, plain DOM; `Design.pulldown` names the module of the design's folder that exports it, else the shell wears `../sdk/pulldown.ts`), and `lacks(design)`, the required parts a design misses.
- `boot.ts` imports nothing: it adds `js`, replays theme, tint, text size, motion, contrast, targets, every design knob, `data-clear` (Reduce transparency, the site's or the OS's) and the icon look from localStorage under the `html[data-prefix]` keys, marks `/` with `data-desk` and `data-window="none"` and opens the welcome hall there.
- `chrome.ts` and `router.ts` are the two module scripts, each importing `mount.ts` first; Bun merges them, with what they import, into one entry chunk.
- `savers.ts` is the saver registry: `all` (`site.savers`), `find` (a saver id, or an app id for its first saver), `on` (the ids not switched off), `load` (the entry, its scene from `apps:scenes`, its `SAVERS` item and its validated `still`, or the implied one of an app with `"saver": true`), `roll` (the seed plus `pick` fed by `rng(seed)`), `shuffle`.
- `lock.ts` runs a saver for the desk, the lock screen and the screen saver: `lock(canvas, pick, { pool })`, where the pick is '' (`site.desk`), `random` (one of the pool, `on()` by default, else every saver), a saver id (a rolled value) or a saver id with a query (that value); an unknown id is `random`. It awaits the scene's `units`, runs it with `../sdk/run.ts` (still under reduced motion) and names what it ran (`pick`, `id`, `as`).
- `idle.ts` is the screen saver: `watch()` starts one idle timer (any input resets it; `minutes()`, re-read when the setting changes), `start({ only })` shows it now, `stop()` ends it. It never starts with no saver on, on a hidden page, under the welcome hall, an open dialog or zen. While up: `html[data-saver]`, the design's `saver` part (none without one; `state` `on`, then `out` until its animations end) over the shuffled savers that are on, the next one every 90 s with a cross-fade, the time in the site font and the date unless the clock is off, no cursor; a key, click, wheel, touch or a pointer move past 8 px ends it after its first second. It fires `saver` (`detail` true, then false).
- `menus.ts` draws the app's bar menus from the model its `menus` event hands over (`../sdk/menus.ts`), adds Go (Home, Launchpad, the last five apps, the section's other apps) and the topic's research note under Help, and runs each pick through the model, into the element the design spreads `bind.menus` on, in its bar or its window; the design's `pulldown` draws each button, popover and item and sets the gap under the button (`site:design/pulldown`, React-free, so the first-loaded code holds none; without one, the sdk's plain one); arrows walk a menu and across menus, Escape closes it.
- `figures.ts` loads `@mrlyprod/draw/figure.ts` with `site:figures` once, for the router, the research page and the home pieces.

## HOME

- Three pieces sit on the desk, each a React root of its own: the dock (`dock.tsx`), the widgets (`widgets.tsx`) and the Launchpad (`launchpad.tsx`, the page of `/menu/`). `chrome.ts` lays the dock on the first route and the widgets on the first visit home; both stay for the visit.
- The dock and the widgets keep their behaviour here (items, order, drag, edit mode, gallery, the desk's context menu, the magnify lens) and draw through the design's optional `dock` and `widgets` parts; a design without one gets none. The lens runs only over the elements a design marks with `bind.lens`.
- `launch.ts` is pure and tested: `arrange` (a saved layout over `site.launchpad`: its order and names kept, lost ids dropped, new ids back in their own section), `docked`, `spare` and `renamed` (no name twice), `exported` (the JSON of `../mrly.net/launchpad.json`, byte for byte), `visited` and `recents`, `gist`, `ago`, `rank` (search: a title that starts with the query, then a word that does, then one that holds it, the lead only for words of 3 letters or more), `today` (the UTC date) and `daily` (one index per date).
- `catalog.ts` binds them to the site and localStorage: `app(id)`, `shelves` and `shelve`, `dock` and `redock`, `unshelve` (both back to `launchpad.json`), `recent` and `track` (each `wire` and each app `stamp` keeps the app's exact url), `notes` (`/research.json`, fetched once), `type` and `take` (keys typed on home before the Launchpad is up), `away` (back while `depth` is above 0, else home).
- `tile.tsx`: `Tile`, the one launcher, drawn by the design's `tile` part: a real link holding the route's app icon and, unless `name` is false, its title.
- `appicon.tsx`: `AppIcon`, a route's app icon from `site:icons` (`../sdk/appicon.ts`), inline SVG in a `.face`; the first one prepends the shared sprite (shape, plates, rims, filters, every icon's gradients). Tile, the dock's Launchpad (`menu`) and Continue wear it.
- The dock: the Launchpad, then each group of `launchpad.json` `dock` behind a divider; it shows on home, on page windows and over the Launchpad, never in an app (`[data-window="app"]`) or during widget editing. Drag to reorder, drop from the Launchpad or Recent to add, drag out to remove; Magnify (Settings, when the design sets `measure.lens`) grows the icons under the pointer.
- Settings > Desktop & Dock lists the dock apps, each with Remove, and an Add to Dock picker of the apps not in it.
- The Launchpad (`/menu/`): the design's `launcher` over the desk. The chips filter by section; All shows Recent (the last 9 apps opened in this browser) and every section. Drag apps within and across sections, drag chips to reorder sections, double-click a chip or heading or press F2 on a chip to rename, `+` for a new section; an empty section offers its removal. The context-menu key or Shift+F10 on a tile opens Add to Dock (or Remove from Dock) and Move to each other section (none from Recent). More: New Section, Rename Section (with a section chip on, not while searching), Export JSON (downloads and copies `launchpad.json`), Reset Layout (with Undo).
- Search: typing on home, Cmd/Ctrl+K and the tray's search open it; results group Apps, Research, Posts and Settings; Tab, Shift+Tab and the arrows move the highlight, Enter opens it, Escape clears the field and then closes. The `+`, the dock's Launchpad and Escape close it.
- Widgets (home only), in the order of `site.json` `widgets` (`site:widgets`): the shell's own Clock (the zone's city, the time in the site font) and Continue (the last app at its exact url), and any route's `desk.tsx` the site names (mrlyprod.com: Design of the day, `../../apps/designs/`, and Research, `../../org/research/`). Drag to rearrange; Edit Widgets (the button under the column, or right-click on the desk, whose menu the design's `menu` part draws at the pointer) shows a remove badge on each widget and the gallery, where a widget is added by its button or by dragging it in.
- The order of each piece lives in localStorage (`launchpad`, `dock`, `widgets`, `recent`); `magnify` keeps the dock setting.

## ROUTER

- `route.ts` is pure: `match(rows, path)` (exact row, else the longest `meta.deep` row, else missing; `/index.html` is `/`), `sort(to, from, deep)` (page, inner, mark or native), `named`.
- `router.ts` reads `site:routes`, `site:css`; a click or a pop loads the row's shell (and, for an app, its chunk) and `/raw/<source>` in parallel, renders off-DOM, swaps the ready node inside a View Transition, then mounts the page, registers figure hosts and fires `wire` with the route.
- It sets title, description, canonical and `noindex` on missing from the row, and dresses the window: `html[data-window]` from the layout, the title through `bind.title`, `.window[data-app]` (the app's id, empty on pages; designs colour from it in CSS, or in `start` when they hash ids to hues, as mrly does); the live bar names the app.
- `history.state.depth` counts the pages this tab opened on the site; the window's close goes back while it is above 0, else home.
- Kept from the old router: latest-wins, aria-busy, `y` (the scroll of `#main`, the window's scroller) in `history.state` on scrollend, pagehide and visibilitychange, hash scroll after render, focus `#main`, drawers shut, reduced-motion calm, `hasUAVisualTransition`, pageshow.
- `hint()` on pointerenter, focus and pointerdown fetches the shell, a render page's chunk and the raw source ahead.
- Links inside `/git/` are `inner`: the git client keeps its own clicks and popstate.
- A failed load reloads once (a sessionStorage key), then shows an error page.

## PAGES

- Two shells, one per row kind; each exports `page(row)`, a page `{ layout, render(row, site, host, { text, rows, url, right }), mount?(host, { rows, site, url }), unmount(host) }`.
- `app.tsx`: an island app (`mount(at)`/`unmount(at)`) swaps in as its still at once and loads its chunk from `site:apps` in `mount`; an app that exports `render` (`own`, scanned at build) is the page itself and loads before the swap.
- `prose.tsx`: pages, posts and `/mrlymath/`: it opens the preview app (`../system/preview/index.tsx`) through `../sdk/md/md.ts` `reader()` and hands the row over; the reader draws through `../sdk/md/` and its resolver, KaTeX only when the text holds `$`, so the shell holds no markdown pipeline.
- `layout`: `right` (`contents`, the prose page's table of contents), `wide`, `bare` (a div, not `article.prose`), `sheet`, `window`.
- `window`: `page` (the default: 720px, at most 85% of the height, scrolling inside, both edges resize it together), `wide` (the default of a `wide` layout: up to 80rem, panes docked under `html[data-docked]`, which the shell sets while the window is at least `measure.dock` wide), `app` (a scene app: one window under the bar, no title bar), `none` (home).
- `app.tsx` puts a scene app (`meta.scene`) in the `app` window and any other island in a `wide` one.
- `hub.tsx`: `hub(body)`, the page of `/blog/`: a lede and the body.
- The reader, `hub.tsx`, `Lede` and `Lost` draw through the design's `prose` part; `Figure`, the figure host, is `../sdk/host.tsx`.
- `parts.tsx`: `show`/`hide` (a React root per host, flushed sync), `Figure`, `Lede`, `Lost` (the missing page, also research's unknown slug), `Card`, `Grid`, `PANES` (the Settings panes the Launchpad search finds, re-exported from `panes.ts`), `Contents`, `heads`.

## FIGURES

- A host is `<figure data-figure="name"><canvas></canvas></figure>`; `@mrlyprod/draw/figure.ts` sizes its canvas to the css box times DPR, capped at 512 in a tile and 1024 elsewhere, draws the figure at its own size on one scratch canvas and downscales into it.
- Hosts queue on intersection, one draw a frame, nearest first, dropped on a route change unless scanned with `keep` (the dock and the widgets); a queued host is `aria-busy`.
- Ink is `ink(tinted(theme, { accent, link, theme }))` from `--accent` and `--link`; a real theme or tint change repaints.

## CHROME

- The chrome is four files, wired by delegated listeners.
- `chrome.ts`: the drawers (`html[data-docked]`, from the window kind and `measure.dock`; a docked pane never shuts), the contents highlight, the scroll spot, the window's edges (each `[data-edge]` through `../sdk/handle.ts`), close and zoom, Cmd+K or Ctrl+K to search (`seek`), typing on home, the live bar, the dock and the widgets, the active design's `start`, and `wire`, which runs the rest on every route.
- `prefs.ts`: localStorage (`KEY`, `read`, `write`, `subscribe`), the window widths (`widths`, replayed before the first paint), theme, tint, the design knobs (`knob`, `dial`), icons, magnify, the saver pick, the screen saver (`offs`, `savers`, `idle`, `minutes`, `clock`), the clock's `hours` and `cycle` (12, 24, or null for the locale's; `../sdk/clock.ts` `stamp` writes the time in that cycle), `replay`, and accessibility (`text`, `motion`, `contrast`, `targets`, `clear`, `calm`).
- `write` tells subscribers only when the stored value changes; another tab's change arrives through the `storage` event, heard from load; `replay(key)` re-applies the html attribute or style that key drives, then subscribers hear it.
- `html[data-clear]` is on while Reduce transparency is, the site's (`clear`) or the OS's (`shine`); `calm()` is the site's Reduce motion or the OS's.
- `SETTINGS` names what a settings link or file carries, with every `knob-<id>`: `pack` and `unpack` (validated; a name it lacks is cleared), `encode` and `decode` (unpadded base64url of the JSON), `share(origin)` for the link; `stored()` lists every key for Privacy & Data.
- Icons copy Apple's Icon & widget style: Default, Dark, Clear, Tinted, then Auto (follow the theme) or Always (the dark look); Settings > Appearance sets them through `icons(style, always)`. `look(style, always, dark)` resolves `html[data-icons]`: absent, `dark`, `clear-light`, `clear-dark`, `tinted-light` or `tinted-dark`; `shade()` reruns it on a theme change.
- `desk.ts`: the welcome hall, the desk and its lock screen, drawn by the design's `lock` part in the hall (no welcome without one; `leaving` ends when its animations end), with or without a desk canvas; `visit(detail)` hands it the route; the first `desk()` starts the idle watch of `idle.ts`.
- `tree.ts`: the git tree's lazy folders (`branch`, `turns`, `expand`).
- `theme` fires only when the theme or the tint really changes.
- The desk is one full-screen saver canvas under every route; the page never scrolls, the window does. `desk.ts` appends it on the first route and keeps it; it loads the saver through the `lock` thunk of `site:apps` and feeds its canvas to `../sdk/feed.ts` as a source; the screen saver's canvases are never a source. A design whose `desk` draws no canvas gets no saver there. Home (`data-desk`) is the desk with no window.
- The pick is the Settings Saver, unset meaning `site.desk`; the lock rolls a seed and the saver's value and names what it ran (`stop.as`), and the chrome reuses that while the pick stands, so the desk is the same picture all visit.
- First visit (the welcome hall open) is the lock screen: the design's `lock` over the desk, `html[data-lock]` `wake`, then `rest` after 3 s of no mouse. Unlocking leaves the scene (`leave`) and coasts the saver to a standstill over 3 s (`stop.coast`): the screensaver becomes the desktop picture.
- A later visit plays the desk 3 s, then coasts. The mark (`bind.mark`, through `knock`) replays the lock on home and goes home anywhere else; a design without a mark has no replay.

## WORLD

- `world.ts` is the world store: one kernel world a tab, booted by `start()`, which `chrome.ts` calls once at load, from `site:world` (the site's `world.json` and the `app.ts` of its routes); a site without `world.json` runs none.
```
 load: seed = handover ?? ?seed= ?? Math.random   who = ?as= ?? guest
   |
   v
 create({ apps, config, as, lines, players }) -> store ---plug---> ../sdk/world.ts port ---> useWorld, useCall, the generic view
   |                                               ^
   '-- every BEAT (6 s), unless hidden: round() -- the host's players, one act per test user nobody signed in as
```
- Nothing persists across loads: each load boots a fresh world from a fresh seed; `?seed=<n>` boots that one again.
- The tab's store runs the bus after each act (`../sdk/system.ts` `bus`: notice wishes, act wishes, each app's `after`), as the local host and the gym do, so a tab's journal is the one `mrlynet` would write.
- The shell plays nobody by itself: `players` in `create(options)` is the host's, `(table, who, at) => acts`, called by `round(at?)` with every test user nobody signed in as; `site:world` carries it from the site's host (mrly.net wires `../sim/bots.ts` `round`); without it `round()` returns 0 and the tab is a watcher's. The shell imports nothing from `../sim/`.
- `create(options)` is pure and tested; the store is the sdk's port plus `world()`, `lines()` (the journal), `users()`, `sign(id)` and `round(at?)`; `current()` is the tab's live store or null, for a page that shows the world itself (Settings > About: the seed and the settled rules).
- The guest (`GUEST`, `{ id: "guest", kind: "human" }`) is signed in by default: a watcher the kernel refuses (no actor), whose act comes back as the note "Sign in as a test user to do this" (`WATCH`); `?as=test-3` signs in at boot.
- THE LOCK SCREEN'S SEAM: `users()` lists the guest and each actor as `{ id, label, name, hue, glyph, kind }` (`label` "Test 3" from the id, the rest from `people` when the world holds it); `sign(id)` signs in as one of them, `sign("guest")` signs out; both ring every reader.
- A test user's `kind` in `who` (signed in, or a bot in `round()`) is its `people` card's, the world's `kinds` rule (`../system/README.md`, `people`); without `people`, a pinned kind, else human (`../sdk/kinds.ts` `kindOf`).
- `handover()` writes the seed, who and the journal to sessionStorage once; the next load replays it and clears it, so a design Restart keeps its world; the restart calls it before it reloads.
- `paced(store, hidden)` is the beat: no act while the tab is hidden.

## TESTS

- `design.test.ts` (and `../../designs/contract.test.tsx` over every design), `clean.test.ts` (no shell file holds a colour, font or px size, with no exceptions; a measured length goes through `../sdk/px.ts`), `gate.test.ts`, `route.test.ts`, `router.test.ts`, `shells.test.ts`, `launch.test.ts`, `launchpad.test.tsx`, `widgets.test.tsx`, `app.test.tsx`, `lens.test.ts`, `tiles.test.ts`, `tree.test.ts`, `lock.test.ts`, `savers.test.ts`, `idle.test.ts`, `prefs.test.ts`, `store.test.ts`, `world.test.ts` (the tab's 100 rounds reach `mrlynet sim`'s digest on `world.json`, the guest's note, a hidden tab, a tab with no players); the brand lanes, over all of `net/` and over the shell and the system apps in any case, are `../test/brand.test.ts`.
