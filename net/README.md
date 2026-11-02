# NET

- The platform: one static shell, one router, one table of routes; every page is drawn in the browser.
- A site is a config folder; `mrly.net/` is the first, MrlyNet alone, and the one the build reads by default; mrlyprod.com is another, kept outside this repo, whose roots reach `../apps/` and `../org/` and whose `research` names the research repo.
- The siblings: this repo builds on repos cloned beside it, under one parent folder: `../draw` (the pen, `@mrlyprod/draw`), `../core`, `../num`, `../math`, `../tile`, `../life` and `../font` (the Rust units; each `js/` is `@mrlyprod/<unit>`, its wasm built into `js/pkg/` by the unit's `scripts/wasm.sh`), and `../research` for mrlyprod.com; `hosts/web/scripts/home.ts` names them (`SIBLINGS`, `UNITS`, `unit()`, `wasm()`), `../tsconfig.json` maps the names for tsc and Bun.
- In the apps, the designs and their numbers come from the Rust crates through `@mrlyprod/<unit>`; the deep zoom (`../apps/shared/fractal/`) and the space flights (`../apps/shared/space/`) are drawn in JS.
- Browser floor: iOS 16.4 / Safari 16.4 (remark-gfm's lookbehind regex in the reader's chunk, OffscreenCanvas in the figure pen).
- Dependencies: the platform (`spec/`, `kernel/`, `sdk/`, `shell/`, `doors/`) pulls in React and React DOM and nothing else; every app, system or not, owns its own, loaded only when it opens, and a distribution that leaves an app out never ships them.
- A system app declares what it imports beyond React in its own `package.json` (`dependencies`; a type package under `devDependencies` as `@types/<name>`), the form that becomes optional peers when `@mrlyprod/net` publishes; `test/deps.test.ts` holds each folder to its declaration and the root `package.json` to the same ranges. Today: `system/preview/` (the unified markdown chain and KaTeX), `system/git/` (Shiki); Three.js belongs to MrlyProd's apps (`../apps/shared/three/three.ts`).
- `bun install` at the repo root fetches them all for the monorepo, with `typescript` and `@types/bun` for `bun run types`.
- Each folder has one README: `spec/`, `kernel/`, `doors/`, `shell/`, `sdk/`, `system/`, `hosts/web/`, `hosts/local/`, `test/`, `mrly.net/`; outside `net/`: `../designs/`, `../apps/`, `../org/`; `../../draw/README.md` has the pen.

## SHAPE

```
repo/
  package.json  bun.lock  bunfig.toml       one JS package: deps, the bun run verbs, the dev plugin, the test preload
  tsconfig.json                             tsc over net designs apps org aws; paths: @mrlyprod/<unit> -> ../<unit>/js, @mrlyprod/draw -> ../draw, the packages -> their folders
  net/
    spec/                                   the contract: JSON schemas for act, line, move, manifest, envelope, config, fault
    kernel/                                 one pure reducer over one JSON world: boot, list, call, read; the journal
    doors/                                  the transports: the mrlynet CLI and MCP
    shell/                                  index.html, boot, router, chrome (the bar, the window, the desk), the page shells, the design contract
    sdk/                                    what pages, apps, designs and the shell share: the app framework, the door, md, git, theme
    system/                                 the platform's own routes: home, menu, settings, stats, kit, git, screensavers, missing, redirects
    hosts/web/                              the bun run verbs and the build (scripts/), the AWS code (infra/), the build-level tests (test/)
    hosts/local/                            a world in a folder, no window, for the doors
    sim/                                    the players: the goose, the scripted bots, the round and the run; a simulation is a config plus players on the OS
    test/                                   shared fixtures and the suites that span folders
    mrly.net/                               mrly.net's config: site.json, launchpad.json, redirects.json, world.json, public/
  designs/                                  the looks the shell wears
  apps/                                     the apps, one folder a route, and their families in apps/shared/; served by mrlyprod.com
  org/                                      MrlyProd's pages, one folder a route; served by mrlyprod.com
../draw/  ../core/  ../math/  ...             the siblings: the pen and the Rust units, each its own repo
../research/                                the research repo, mrlyprod.com's topics and figures

request /about/
  edge   hosts/web/infra/edge.js: no dot in the last segment -> /index.html
  shell  shell/index.html: boot.ts, page.css, the tint style, the bar and the window, chrome.ts + router.ts
  router the site:routes row -> shell/app.tsx (a page's index.tsx or an app's app:<id>) or shell/prose.tsx -> system/preview/ + /raw/<source> -> #main
  figure @mrlyprod/draw/figure.ts (../../draw/) draws each host from an intersection queue
```

- Imports run one way (`test/imports.test.ts`): `kernel/` imports `spec/` only; `sdk/` may import `kernel/`; `doors/` the kernel and `hosts/`; the shell imports `sdk/`; a system or org page `sdk/` and `shell/`; an app `sdk/` and its families; a design `sdk/`, `../designs/shared/` and its contract, `shell/design.ts`; `sdk/` nothing above it; no route imports another; a short KNOWN list of exceptions may only shrink.

## CONFIG

- `MRLY_ROOT=<dir> bun run build` builds the site of any config folder; unset, it is `net/mrly.net` (`hosts/web/scripts/home.ts`: `REPO`, `HOME`, `SITE`).
- `bun run init <dir>` writes a new config folder (`hosts/web/scripts/init.ts`); `../QUICKSTART.md` takes a clone to a running site and a world on disk.
- `site.json` `roots` lists the route folders, relative to the config folder; mrly.net's is `../system` alone.
- `site.json` `research` names a research folder, relative to the config folder; only then does the build add its topics, their figures, `research.json` and the Research lines of `llms.txt`; mrly.net names none.
- `site.json` `widgets` lists the desk's widgets in order: the shell's `clock` and `continue`, or the id of a route holding `desk.tsx`; unset, the shell's two.
- `site.json` `designs` lists the site's own design folders, relative to the config folder, drawn before `../designs/`; such a design imports the host by package, never by path.
- The opt-ins stay off unless `site.json` sets them: `cart` (the bar's cart ring), `tiles` (the shell's Tiles saver, `{ id, title }`).
- `world.json`, beside `site.json`, is the world each tab boots (`shell/world.ts`) and `mrlynet boot`, `sim` and the gym take: a kernel config, `{ name, apps, actors, rules }`, each rule pinned or a `pick` drawn from the seed at boot (`kernel/README.md`, Rules); mrly.net's holds the system apps, the test users `test-1` to `test-8` and picks for `payments`, `gifts`, `kinds` and `grant`; without it a site runs no world.
- `dist/` and `wasm/` sit in the config folder, gitignored.

## THE UNIT

- A route is the only unit: one folder, `/<id>/`, in one of the `roots`; an id is taken once across them; `home` is `/`; a folder named `shared` is no route.
- `app.json`: `title`, `line` (the lead), and `saver`, `hidden`, `deep` (each `true` or absent), `source` (a repo path of a markdown page), `route` (`/404.html` for `missing`). No other field; nothing else is configured.
- The page is exactly one of `index.tsx`, `index.md` or `source`; a folder with none of them and both `scene.ts` and `widget.tsx` is an app whose entry the build writes (`app:<id>`, `../apps/README.md`); one with none of them and `app.ts` is a system kernel app drawn by the generic view (`system/README.md`, KERNEL APPS).
- `app.ts` may also sit beside an `index.tsx`; either way the build collects it into `site:world`.
- `index.tsx` exports `mount(at)` and `unmount(at)`, an island under the still (title, figure, `<noscript>`), wide, bare, in a wide window and wearing `sdk/mrly.css`; or `render(row, site, host, { text, rows, url, left, right })`, `unmount(host)`, optional `mount(host)`, a page drawn before the swap; an optional `layout` (`left`, `right`, `wide`, `bare`, `sheet`, `window`) overrides the island's.
- `index.md` and `source` are prose, rendered from `/raw/<source>` by the preview app (`system/preview/`), which `shell/prose.tsx` opens through `site:apps`, so a site that draws markdown holds the `reader` route; front matter `opener: true` (draws the route's icon above the lede), `button` and `link`, `legal: true` (the copyright line at the end).
- `icon.ts` is the route's figure, named by the id; a hidden route may omit it, a shown one may not.
- `desk.tsx` is the route's desk widget, `export const widget: Desk` (`sdk/rows.ts`: `{ name, size, Body }`), shown only when `site.json` `widgets` names the route.
- `icon.svg` is the route's app icon: viewBox `0 0 1024 1024`, an optional `<defs>` first, the plate as the first shape with `class="back"`, then the glyph; no text, images, style or class; `data-dark`, `data-dark-stroke`, `data-mono` (`full`, `dim`, `hide`) and `data-key` steer the Dark, Clear and Tinted looks. The build checks each with `sdk/appicon.ts` and serves them all in `site:icons`.
- `scene.ts` and `widget.tsx`: every app has them; `../apps/README.md` has the app contract.
- Savers: a scene declares them as `export const SAVERS = [{ id, title, pick }]`, 1 to 3; `id` a slug once in the app, `title` at most 32 characters, `pick(rand)` a plain patch of `SPEC` or `PAGE.defaults` keys, never `seed`, `still` an optional capture time in ms (1 to 10000, default 1000) for the Screensavers grid. A saver's id is `<app>/<id>`; the shell's Tiles saver lives in `shell/tiles.ts`, shown only when `site.json` names it with `tiles` (`{ id, title }`; mrly.net: `tiles`, its desk, since no system app holds a saver).
- `saver: true` in `app.json` is the bridge: with no `SAVERS`, the app is one saver, `<app>/<app>`, named after it; the flag beside `SAVERS` stops the build.
- Blog posts: `<root>/blog/<slug>/index.md` is `/blog/<slug>/` (front matter `title`, `date` YYYY-MM-DD, `lead`), with its `icon.ts`, the figure `blog-<slug>`.
- `site.json` `desk` names the desktop saver: an app (its first saver) or a full saver id.
- `launchpad.json` is where every shown route lives: `sections`, each `{ name, apps }` in order, every route but home and the Launchpad itself in exactly one; `dock`, the groups of the default dock. The Launchpad's Export JSON writes this file byte for byte; the build stops on an unknown id, an id twice, a left-out route and a name twice.

## ROUTES

- `hosts/web/scripts/build.ts` collects the rows from `<root>/*/app.json` of every root, home first, then by id: `{ route, kind, title, source, date, lead, figure, hidden, meta }`.
- `kind` is `app` (an `index.tsx` or a written entry) or `prose` (a markdown source); `meta` holds `id`, `scene` (an app of `scene.ts` and `widget.tsx`), `deep`, and `prose` (`page`, `post` or `source`).
- `source` is the repo path served at `/raw/<source>` for a prose row, null otherwise; no row carries a body.
- `research.json` is built from `<research>/*/README.md` of `site.json` `research` (a folder with a front matter `title`; `shared/` never): `{ topics: [{ slug, title, date, lead, figure, figures }] }`, `date` (YYYY-MM-DD, required) and `lead` the README's front matter, `figure` the README's, `figures` every figure of `<topic>/figures/`.
- A hidden route (`redirects`, `missing`, `figures`) has no sitemap entry, no `llms.txt` line, no search row and no Launchpad tile.
- A missing url renders "Nothing here" with `noindex`, status 200 by design.
- The build stops on a bad `app.json`, a bad `SAVERS`, a desk that is no saver, an unknown figure, a missing source and two rows for one route.
- `redirects.json` is the moved urls, `{ "/old/": { "to": "/new/", "since": "YYYY-MM-DD" } }`; `to` may carry a query (`/fractals/?kind=julia`), joined before the asked one; the edge answers each with a 301, the router follows one on an inner link, `/redirects/` lists them, and the build drops a row whose target is no route.

## FIGURES

- The roster: `<root>/*/icon.ts` (name = id), `<root>/blog/*/icon.ts` (name `blog-<slug>`), `<research>/*/figures/*.ts` and `<research>/shared/figures/*.ts` (name = stem); a name is lowercase words and digits joined by hyphens, taken once, and the file exports `default`.
- A figure exports `default function draw(pen, ink, t)`, `t` in [0, 1) over one loop.
- `size`: `[width, height]`, default `[1024, 1024]`; a non-square size keeps its aspect in the host.
- `still`: the `t` a still figure draws, default 0.
- `loop` in seconds, with `frames`: an animated figure only; none in the live set.
- `units`: the `@mrlyprod/<unit>` wasm units it imports, e.g. `{ math }`; the caller initialises them before the first draw.
- `ink` is `ink(tinted(dark | light, ...))` from `sdk/theme/`: the tint's hue sits in the blue slot.
- A route icon is a plate drawn with `sdk/plate.ts` on a clear ground: the build lists every `<root>/<id>/icon.ts` as a plate; a research or blog figure keeps the ink ground.
- A figure is a pure function of its inputs: no file, DOM, Date or Math.random.
- A figure asserts its facts and throws when one breaks; heavy facts are memoised at module level; pixels are never cached.
- The pen and the kit live in the `draw` repo beside this one, imported as `@mrlyprod/draw`: `canvas` in the browser, `raster` in Bun and the tests.
- A lone image line `![caption](name)` in a page, a post or a research note is a live figure; the build stops on an unknown name in a row's source, `hosts/web/scripts/build.test.ts` on one in the site's research notes.
- `hosts/web/scripts/figures.test.ts` loads every figure: `default` a function, every `units` value defined.
- `bun run time [name ...]` draws each figure in a fresh worker with the raster pen, cold then warm, dark and light; gate 150 ms warm, 500 ms cold on the icons, `about` excluded; a research figure over it is a named warning, judged with its topic; exit 1 when an icon is over or a figure throws.

## BUILD

- `bun run build`, from the repo root, runs `hosts/web/scripts/build.ts` into `<config>/dist/`, or into `MRLY_DIST`; pure Bun, no Chrome, no cargo.
- Prime: bundle `shell/boot.ts`, `../designs/shared/git.css` and `sdk/mrly.css`; each woff2 of `../designs/shared/fonts/` ships as `/fonts/<stem>-<sha8>.woff2`.
- Collect: the routes, the rows, the figure roster, the wasm units, the kept redirects, the raw list (`git ls-files`), the research topics, the savers (every `scene.ts` that exports `SAVERS`, loaded in Bun with `require`).
- Bundle: one `Bun.build` with splitting over `shell/index.html`, `shell/app.tsx`, `shell/prose.tsx`, `system/git/git.ts`, every live figure, every route's `index.tsx` and `shell/lock.ts`; each `app:<id>` is a chunk of the `site:apps` thunks.
- Names: entries `[name]-[hash]`, chunks `lib-[hash]`; the build refuses a bundled file with no hash.
- Shell: the bundled `index.html` with the hashed boot script swapped in for `/boot.js`, its empty `<style id="tint">` filled with `tintCss(site.tint)` and its `{title}`, `{tagline}`, `{prefix}`, `{root}` and `{domain}` filled from `site.json`, each `data-mark` link and `{og}` from `site.json` `marks` (a mark the site lacks drops its tag) (`withSite`, which dev uses too); an `app.json` may say `{title}`, `{tagline}`, `{domain}` and `{repo}`, filled at the build (home's line is `{tagline}`); `404.html` is its copy.
- Copy: `raw/<path>` for every listed file, the config's `public/`, the font licences.
- Globals: `routes.json`, `llms.txt`, `sitemap.xml` with one `sitemap-<section>.xml` per section and `sitemap-raw.xml`, `robots.txt`, `git.json`, `redirects.json`, and `research.json` when the site holds research.
- Write: `dist/` is emptied, then written whole; `bun run push` ships it (`hosts/web/infra/README.md`).

## VIRTUAL MODULES

- `hosts/web/scripts/build.ts` is one Bun plugin; Bun cannot import a computed path, so each lazy family is a module of `() => import()` thunks.
- `site:routes`: `{ site, rows }`; `site` carries `savers` (`{ id, app, name, title }`, the desk app's first, then the apps by name, the shell's last), `desk` (a saver id), `version` (the one version of every `@mrlyprod/<unit>`, null when no unit is live), `research` (whether the site holds research; the Launchpad and the menus fetch `research.json` only then), `units` (the live wasm units) and `licences` (`{ name, href }`, each licence under `/fonts/` and the repo `LICENSE`).
- `site:figures`: name to figure chunk, the wasm urls of the live units, and the plates.
- `site:apps`: id to route chunk, `own` (the routes whose index.tsx exports `render`), plus `lock`, the desk and its lock screen.
- `app:<id>`: an app's entry, `entry(dir)`: `page(Widget, { seed: 0, ...PAGE.defaults }, PAGE)` from its `scene.ts` and `widget.tsx`; for an `app.ts` route, `viewer(id)`: `view(id)` of `sdk/view.tsx`.
- `site:world`: `apps`, the default export of every route's `app.ts`, and `config`, the parsed `world.json` (null without one, and `apps` empty); the build boots it once with the kernel and stops on a config that does not boot or an `app.ts` route it leaves out.
- `site:widgets`: `order`, the ids of `site.json` `widgets`, and `widgets`, id to the `widget` of each named route's `desk.tsx`; `shell/widgets.tsx` draws them beside its own.
- `site:css`: the `git` and `mrly` sheet urls.
- `site:design`, `site:design.css`, `site:design/tokens.css`, `site:design/saver.css`, `site:design/pulldown`: the drawn design (`hosts/web/scripts/README.md`).
- `apps:scenes`: app id to its scene chunk, for every app with a saver, and the `tiles` id for `shell/tiles.ts` when the site names one.
- `@mrlyprod/<unit>` resolves through `../tsconfig.json` paths onto `../../<unit>/js`, the sibling repo; the `mrly` plugin, also used by the figure test and `time`, maps it and the packages for the research figures; `wasm/<unit>/mrly_<unit>_js*` resolves to `<config>/wasm/`.

## WASM

- `bun run wasm` runs `hosts/web/scripts/wasm.ts`: it copies the live units from the siblings' `js/pkg/` into `<config>/wasm/<unit>/` and writes `../apps/shared/font/font.gen.json` through `cargo run --example book` in `../../font`; a sibling with no `js/pkg/` stops it with the sibling to run `scripts/wasm.sh` in; `--copy` only copies, the verb a site outside this repo runs.
- The packages, each a folder with its own `package.json`, `private`, linked by local path: `@mrlyprod/shell` (`shell/`), `@mrlyprod/sdk` (`sdk/`), `@mrlyprod/system` (`system/`), `@mrlyprod/draw` (`../../draw/`, the sibling), `@mrlyprod/<name>` for each design (`../designs/<name>/`: `modern`, `boxy`, `classic`, `bricks`, `terminal`), `@mrlyprod/design-shared` (`../designs/shared/`); `hosts/web/scripts/packages.ts` reads their `exports` and resolves `@mrlyprod/<package>/<file>` in the build, in Bun outside the bundle and in a site's design sheets, so a site folder outside the repo imports the host by name.
- Outside the bundle Bun resolves a package only with a file after its name (`@mrlyprod/modern/index.ts`), never the bare name.
- A unit is live when the site ships code that names it (`live()` in `hosts/web/scripts/build.ts`): the files of its route roots, `shell/`, the drawn design's folder (every design folder when none is pinned), `system/git/`, the site's figures and `sdk/page.tsx` when an app is drawn from a scene, and every file they import, followed through relative imports and the packages; code names a unit by `@mrlyprod/<unit>`, its glue, `wasm("<unit>", ...)` or `useUnit("<unit>")`. `../apps/shared/units/` holds a loader for `font`, `life`, `math` and `num`; its `load.ts` table is not followed, and the build swaps it for one holding only the live units, so a site ships only the loaders and wasm its code names; each loader imports its wasm as `@mrlyprod/<unit>/pkg/mrly_<unit>_js_bg.wasm`, which the build resolves to the site's `wasm/<unit>/`, while tests and `bun run mrlynet` load it as is. `core` is live through the figures. Live today: none on mrly.net; core, font, life, math, num on mrlyprod.com.
- `wasm.lock` pins the copy in S3 the builder Lambda pulls through `hosts/web/infra/wasm.ts`; the deploy uploads `wasm/` and re-pins it; a config with no `wasm.lock`, mrly.net's today, pins and fetches none.
- A pin that trails a fresh local `wasm/` is normal between deploys: `bun run content` reports it; `hosts/web/infra/wasm.ts` fails the build when the fetched copy does not hash to the pin.
- The build stops when a live unit is missing from `wasm/`.

## DEV

- `bun run dev` from the repo root, where `../bunfig.toml` hands the plugin to Bun's dev server.
- `hosts/web/scripts/dev.ts` serves 127.0.0.1:3000 (`PORT` overrides) with HMR.
- A path the edge sends to the shell answers `shell/index.html`, bundled by Bun on request, its tint style and site fields filled.
- `/raw/<path>` reads the tree per request; `public/` reads from disk.
- The globals, the boot script, the sheets and the faces are built once at start, so a new page, post or app needs a restart.

## CHECK AND TEST

- `bun run test` runs `bun test net designs apps org`: every test beside the file it covers, plus `test/` and `hosts/web/test/`; `../bunfig.toml` ignores `dist/`, whose `raw/` holds copies.
- A fresh clone runs, in order: clone `draw`, `core`, `num`, `math`, `tile`, `life` and `font` beside this repo (and `research` for mrlyprod.com); `scripts/wasm.sh` in each unit (cargo and wasm-bindgen build its `js/pkg/`, gitignored; the script names the wasm-bindgen version it wants); `bun install`; `bun run wasm`; `bun run test`; `../package.json` pins bun.
- `bun run content` runs `hosts/web/scripts/content.ts`: it reports the wasm pin, then runs `hosts/web/test/content.test.ts`, the content gate over the site's own roots and research: every markdown link in a page, a post or a source lands, a rooted one on a route; every research link lands; the house rules over pages, posts and notes (no two blank lines, no em or en dash, a blank line after a heading); over a fresh build in a temp dir, every link on a built page lands and every redirect goes from a gone path to a route.
- `bun run types` runs `tsc` over the `include` of `../tsconfig.json`; the `.js` and `.jsx` files are not type-checked.
- `bun run lint` runs Biome (`../biome.json`: format, lint, import order) over the folders it includes; `bun run fix` writes the format and the safe fixes.
- `bun run check` is the gate, `../scripts/check.sh`: the leak scan, `bun install --frozen-lockfile`, lint, types, wasm (`--copy`), test, build, content; one log per step under `data/scripts/check/`.
- `bun run shots` and `bun run film` drive Chrome; `hosts/web/scripts/README.md` has them.

## GENERATED

- A committed file a web tool writes carries `.gen.` in its name: `../designs/shared/palette.gen.css` and `sdk/theme/palette.gen.js` (the palette, written outside this repo), `sdk/icons.gen.js` (`bun run vendor`), `../apps/shared/font/font.gen.json` (`bun run wasm`, checked glyph by glyph against the font door by `../apps/shared/font/font.test.ts`), `../apps/life/library.gen.json` (`bun run library`; `hosts/web/scripts/README.md` has it).
- `../designs/shared/fonts/` is written whole by `bun run vendor`, `keep.css` aside.
- `wasm/`, `dist/` and `node_modules/` are build output and stay out of git.
