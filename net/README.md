# NET

- The platform: one static shell, one router, one table of routes; every page is drawn in the browser.
- A site is a config folder; `mrly.net/` is the first, and the one the build reads by default.
- The designs and their numbers come from the Rust crates through `@mrlyprod/<unit>`; the deep zoom (`../apps/shared/fractal/`) and the space flights (`../apps/shared/space/`) are drawn in JS.
- Browser floor: iOS 16.4 / Safari 16.4 (remark-gfm's lookbehind regex in the prose chunk, OffscreenCanvas in the figure pen).
- Dependencies: React, KaTeX, Shiki, the unified markdown chain and Three.js (the 3D stages of designs, three and graphs, loaded lazily); `bun install` at the repo root fetches them, with `typescript` and `@types/bun` for `bun run types`.
- Each folder has one README: `shell/`, `sdk/`, `system/`, `hosts/web/`, `test/`, `mrly.net/`; outside `net/`: `../designs/`, `../apps/`, `../org/`, `../draw/`.

## SHAPE

```
repo/
  package.json  bun.lock  bunfig.toml       one JS package: deps, the bun run verbs, the dev plugin, the test preload
  tsconfig.json                             tsc over net designs apps org draw, the research figures, aws; paths: @mrlyprod/<unit> -> crates/<unit>/js, the packages -> their folders
  net/
    shell/                                  index.html, boot, router, chrome (the bar, the window, the desk), the page shells, the design contract
    sdk/                                    what pages, apps, designs and the shell share: the app framework, the door, md, git, theme
    system/                                 the platform's own routes: home, menu, settings, stats, kit, git, screensavers, missing, redirects
    hosts/web/                              the bun run verbs and the build (scripts/), the AWS code (infra/), the build-level tests (test/)
    test/                                   shared fixtures and the suites that span folders
    mrly.net/                               mrly.net's config: site.json, launchpad.json, redirects.json, wasm.lock, public/
  designs/                                  the looks the shell wears
  apps/                                     the apps, one folder a route, and their families in apps/shared/
  org/                                      MrlyProd's pages, one folder a route
  draw/                                     the pen, @mrlyprod/draw

request /about/
  edge   hosts/web/infra/edge.js: no dot in the last segment -> /index.html
  shell  shell/index.html: boot.ts, page.css, the tint style, the bar and the window, chrome.ts + router.ts
  router the site:routes row -> shell/app.tsx (a page's index.tsx or an app's app:<id>) or shell/prose.tsx + /raw/<source> -> #main
  figure ../draw/figure.ts draws each host from an intersection queue
```

- Imports run one way (`test/imports.test.ts`): the shell imports `sdk/`; a system or org page `sdk/` and `shell/`; an app `sdk/` and its families; a design `sdk/`, `../designs/shared/` and its contract, `shell/design.ts`; `sdk/` nothing above it; no route imports another; a short KNOWN list of exceptions may only shrink.

## CONFIG

- `MRLY_ROOT=<dir> bun run build` builds the site of any config folder; unset, it is `net/mrly.net` (`hosts/web/scripts/home.ts`: `REPO`, `HOME`, `SITE`).
- `bun run init <dir>` writes a new config folder (`hosts/web/scripts/init.ts`); `../QUICKSTART.md` takes it from clone to deploy.
- `site.json` `roots` lists the route folders, relative to the config folder; mrly.net's are `../system`, `../../org`, `../../apps`.
- `site.json` `designs` lists the site's own design folders, relative to the config folder, drawn before `../designs/`; such a design imports the host by package, never by path.
- The opt-ins stay off unless `site.json` sets them: `cart` (the bar's cart ring), `tiles` (the shell's Tiles saver, `{ id, title }`).
- `dist/` and `wasm/` sit in the config folder, gitignored.

## THE UNIT

- A route is the only unit: one folder, `/<id>/`, in one of the `roots`; an id is taken once across them; `home` is `/`; a folder named `shared` is no route.
- `app.json`: `title`, `line` (the lead), and `saver`, `hidden`, `deep` (each `true` or absent), `source` (a repo path of a markdown page), `route` (`/404.html` for `missing`). No other field; nothing else is configured.
- The page is exactly one of `index.tsx`, `index.md` or `source`; a folder with none of them and both `scene.ts` and `widget.tsx` is an app whose entry the build writes (`app:<id>`, `../apps/README.md`).
- `index.tsx` exports `mount(at)` and `unmount(at)`, an island under the still (title, figure, `<noscript>`), wide, bare, in a wide window and wearing `sdk/mrly.css`; or `render(row, site, host, { text, rows, url, left, right })`, `unmount(host)`, optional `mount(host)`, a page drawn before the swap; an optional `layout` (`left`, `right`, `wide`, `bare`, `sheet`, `window`) overrides the island's.
- `index.md` and `source` are prose, rendered by `shell/prose.tsx` from `/raw/<source>`; front matter `opener: true` (draws the route's icon above the lede), `button` and `link`, `legal: true` (the copyright line at the end).
- `icon.ts` is the route's figure, named by the id; a hidden route may omit it, a shown one may not.
- `icon.svg` is the route's app icon: viewBox `0 0 1024 1024`, an optional `<defs>` first, the plate as the first shape with `class="back"`, then the glyph; no text, images, style or class; `data-dark`, `data-dark-stroke`, `data-mono` (`full`, `dim`, `hide`) and `data-key` steer the Dark, Clear and Tinted looks. The build checks each with `sdk/appicon.ts` and serves them all in `site:icons`.
- `scene.ts` and `widget.tsx`: every app has them; `../apps/README.md` has the app contract.
- Savers: a scene declares them as `export const SAVERS = [{ id, title, pick }]`, 1 to 3; `id` a slug once in the app, `title` at most 32 characters, `pick(rand)` a plain patch of `SPEC` or `PAGE.defaults` keys, never `seed`, `still` an optional capture time in ms (1 to 10000, default 1000) for the Screensavers grid. A saver's id is `<app>/<id>`; the shell's Tiles saver lives in `shell/tiles.ts`, shown only when `site.json` names it with `tiles` (`{ id, title }`; mrly.net: `mrly`).
- `saver: true` in `app.json` is the bridge: with no `SAVERS`, the app is one saver, `<app>/<app>`, named after it; the flag beside `SAVERS` stops the build.
- Blog posts: `<root>/blog/<slug>/index.md` is `/blog/<slug>/` (front matter `title`, `date` YYYY-MM-DD, `lead`), with its `icon.ts`, the figure `blog-<slug>`.
- `site.json` `desk` names the desktop saver: an app (its first saver) or a full saver id.
- `launchpad.json` is where every shown route lives: `sections`, each `{ name, apps }` in order, every route but home and the Launchpad itself in exactly one; `dock`, the groups of the default dock. The Launchpad's Export JSON writes this file byte for byte; the build stops on an unknown id, an id twice, a left-out route and a name twice.

## ROUTES

- `hosts/web/scripts/build.ts` collects the rows from `<root>/*/app.json` of every root, home first, then by id: `{ route, kind, title, source, date, lead, figure, hidden, meta }`.
- `kind` is `app` (an `index.tsx` or a written entry) or `prose` (a markdown source); `meta` holds `id`, `scene` (an app of `scene.ts` and `widget.tsx`), `deep`, and `prose` (`page`, `post` or `source`).
- `source` is the repo path served at `/raw/<source>` for a prose row, null otherwise; no row carries a body.
- `/mrlymath/` renders `../crates/core/src/name/NAMES.md`, so the page and the crate never disagree.
- `research.json` is built from `../research/*/README.md` (a folder with a front matter `title`; `shared/` never): `{ topics: [{ slug, title, date, lead, figure, figures }] }`, `date` (YYYY-MM-DD, required) and `lead` the README's front matter, `figure` the README's, `figures` every figure of `<topic>/figures/`.
- A hidden route (`redirects`, `missing`, `figures`) has no sitemap entry, no `llms.txt` line, no search row and no Launchpad tile.
- A missing url renders "Nothing here" with `noindex`, status 200 by design.
- The build stops on a bad `app.json`, a bad `SAVERS`, a desk that is no saver, an unknown figure, a missing source and two rows for one route.
- `redirects.json` is the moved urls, `{ "/old/": { "to": "/new/", "since": "YYYY-MM-DD" } }`; `to` may carry a query (`/fractals/?kind=julia`), joined before the asked one; the edge answers each with a 301, the router follows one on an inner link, `/redirects/` lists them, and the build drops a row whose target is no route.

## FIGURES

- The roster: `<root>/*/icon.ts` (name = id), `<root>/blog/*/icon.ts` (name `blog-<slug>`), `../research/*/figures/*.ts` and `../research/shared/figures/*.ts` (name = stem); a name is lowercase words and digits joined by hyphens, taken once, and the file exports `default`.
- A figure exports `default function draw(pen, ink, t)`, `t` in [0, 1) over one loop.
- `size`: `[width, height]`, default `[1024, 1024]`; a non-square size keeps its aspect in the host.
- `still`: the `t` a still figure draws, default 0.
- `loop` in seconds, with `frames`: an animated figure only; none in the live set.
- `units`: the `@mrlyprod/<unit>` wasm units it imports, e.g. `{ math }`; the caller initialises them before the first draw.
- `ink` is `ink(tinted(dark | light, ...))` from `sdk/theme/`: the tint's hue sits in the blue slot.
- A route icon is a plate drawn with `sdk/plate.ts` on a clear ground: the build lists every `<root>/<id>/icon.ts` as a plate; a research or blog figure keeps the ink ground.
- A figure is a pure function of its inputs: no file, DOM, Date or Math.random.
- A figure asserts its facts and throws when one breaks; heavy facts are memoised at module level; pixels are never cached.
- The pen and the kit live in `../draw/`, imported as `@mrlyprod/draw`: `canvas` in the browser, `raster` in Bun and the tests.
- A lone image line `![caption](name)` in a page, a post or a research note is a live figure; the build stops on an unknown name in a row's source, `hosts/web/scripts/build.test.ts` on one anywhere.
- `hosts/web/scripts/figures.test.ts` loads every figure: `default` a function, every `units` value defined.
- `bun run time [name ...]` draws each figure in a fresh worker with the raster pen, cold then warm, dark and light; gate 150 ms warm, 500 ms cold on the icons, `about` excluded; a research figure over it is a named warning, judged with its topic; exit 1 when an icon is over or a figure throws.

## BUILD

- `bun run build`, from the repo root, runs `hosts/web/scripts/build.ts` into `<config>/dist/`, or into `MRLY_DIST`; pure Bun, no Chrome, no cargo.
- Prime: bundle `shell/boot.ts`, `../designs/shared/git.css` and `sdk/mrly.css`; each woff2 of `../designs/shared/fonts/` ships as `/fonts/<stem>-<sha8>.woff2`.
- Collect: the routes, the rows, the figure roster, the wasm units, the kept redirects, the raw list (`git ls-files`), the research topics, the savers (every `scene.ts` that exports `SAVERS`, loaded in Bun with `require`).
- Bundle: one `Bun.build` with splitting over `shell/index.html`, `shell/app.tsx`, `shell/prose.tsx`, `system/git/git.ts`, every live figure, every route's `index.tsx` and `shell/lock.ts`; each `app:<id>` is a chunk of the `site:apps` thunks.
- Names: entries `[name]-[hash]`, chunks `lib-[hash]`; the build refuses a bundled file with no hash.
- Shell: the bundled `index.html` with the hashed boot script swapped in for `/boot.js`, its empty `<style id="tint">` filled with `tintCss(site.tint)` and its `{title}`, `{tagline}`, `{prefix}`, `{root}` and `{domain}` filled from `site.json`, each `data-mark` link and `{og}` from `site.json` `marks` (a mark the site lacks drops its tag) (`withSite`, which dev uses too); an `app.json` may say `{title}`, `{domain}` and `{repo}`, filled at the build; `404.html` is its copy.
- Copy: `raw/<path>` for every listed file, the config's `public/`, the font licences.
- Globals: `routes.json`, `llms.txt`, `sitemap.xml` with one `sitemap-<section>.xml` per section and `sitemap-raw.xml`, `robots.txt`, `git.json`, `redirects.json`.
- Write: `dist/` is emptied, then written whole; `bun run push` ships it (`hosts/web/infra/README.md`).

## VIRTUAL MODULES

- `hosts/web/scripts/build.ts` is one Bun plugin; Bun cannot import a computed path, so each lazy family is a module of `() => import()` thunks.
- `site:routes`: `{ site, rows }`; `site` carries `savers` (`{ id, app, name, title }`, the desk app's first, then the apps by name, the shell's last), `desk` (a saver id), `version` (the one version of every `@mrlyprod/<unit>`), `units` (the live wasm units) and `licences` (`{ name, href }`, each licence under `/fonts/` and the repo `LICENSE`).
- `site:figures`: name to figure chunk, the wasm urls of the live units, and the plates.
- `site:apps`: id to route chunk, `own` (the routes whose index.tsx exports `render`), plus `lock`, the desk and its lock screen.
- `app:<id>`: an app's entry, `entry(dir)`: `page(Widget, { seed: 0, ...PAGE.defaults }, PAGE)` from its `scene.ts` and `widget.tsx`.
- `site:css`: the `git` and `mrly` sheet urls.
- `site:design`, `site:design.css`, `site:design/tokens.css`, `site:design/saver.css`, `site:design/pulldown`: the drawn design (`hosts/web/scripts/README.md`).
- `apps:scenes`: app id to its scene chunk, for every app with a saver, and the `tiles` id for `shell/tiles.ts` when the site names one.
- `@mrlyprod/<unit>` resolves through `../tsconfig.json` paths onto `../crates/<unit>/js`; the `mrly` plugin, also used by the figure test and `time`, maps it and the packages for the research figures; `wasm/<unit>/mrly_<unit>_js*` resolves to `<config>/wasm/`.

## WASM

- `bun run wasm` runs `hosts/web/scripts/wasm.ts`: `../scripts/wasm.sh` builds every leaf's wasm into `../crates/<unit>/js/pkg/`, then it copies the live units into `<config>/wasm/<unit>/` and writes `../apps/shared/font/font.gen.json`; `--no-build` skips the build.
- The packages, each a folder with its own `package.json`, `private`, linked by local path: `@mrlyprod/shell` (`shell/`), `@mrlyprod/sdk` (`sdk/`), `@mrlyprod/system` (`system/`), `@mrlyprod/draw` (`../draw/`), `@mrlyprod/design-<name>` (`../designs/<name>/`, `shared` too); `hosts/web/scripts/packages.ts` reads their `exports` and resolves `@mrlyprod/<package>/<file>` in the build, in Bun outside the bundle and in a site's design sheets, so a site folder outside the repo imports the host by name.
- Outside the bundle Bun resolves a package only with a file after its name (`@mrlyprod/design-glass/index.ts`), never the bare name.
- A unit is live when the site ships code that names it (`live()` in `hosts/web/scripts/build.ts`): the files of its route roots, `shell/`, `sdk/`, the drawn design's folder (every design folder when none is pinned), `system/git/` and its figures, and every file they import, followed through relative imports and the packages; code names a unit by `@mrlyprod/<unit>`, its glue, `wasm("<unit>", ...)` or `useUnit("<unit>")`. `../apps/shared/units/` holds a loader for `font`, `life`, `math` and `num`; its `load.ts` table is not followed, and the build swaps it for one holding only the live units, so a site ships only the loaders and wasm its code names; each loader imports its wasm from `crates/<unit>/js/pkg/`, which the build resolves to the site's `wasm/<unit>/`, while tests and `bun run play` load it as is. `core` is live through the figures. Live today: core, font, life, math, num on mrly.net; math on the smaller sites.
- `wasm.lock` pins the copy in S3 the builder Lambda pulls through `hosts/web/infra/wasm.ts`; the deploy uploads `wasm/` and re-pins it.
- A pin that trails a fresh local `wasm/` is normal between deploys: `bun run check` reports it; `hosts/web/infra/wasm.ts` fails the build when the fetched copy does not hash to the pin.
- The build stops when a live unit is missing from `wasm/`.

## DEV

- `bun run dev` from the repo root, where `../bunfig.toml` hands the plugin to Bun's dev server.
- `hosts/web/scripts/dev.ts` serves 127.0.0.1:3000 (`PORT` overrides) with HMR.
- A path the edge sends to the shell answers `shell/index.html`, bundled by Bun on request, its tint style and site fields filled.
- `/raw/<path>` reads the tree per request; `public/` reads from disk.
- The globals, the boot script, the sheets and the faces are built once at start, so a new page, post or app needs a restart.

## CHECK AND TEST

- `bun run test` runs `bun test net designs apps org draw`: every test beside the file it covers, plus `test/` and `hosts/web/test/`; `../bunfig.toml` ignores `dist/`, whose `raw/` holds copies.
- A fresh clone runs, in order: `../scripts/wasm.sh` (cargo and wasm-bindgen build `../crates/<unit>/js/pkg/`), `bun install`, `bun run test`; `../package.json` pins bun.
- `bun run check` runs `hosts/web/scripts/check.ts`: it reports the wasm pin, then runs `hosts/web/test/content.test.ts`, the content gate: every markdown link in a page, a post or a source lands, a rooted one on a route; every research link lands; the house rules over pages, posts and notes (no two blank lines, no em or en dash, a blank line after a heading); over a fresh build in a temp dir, every link on a built page lands and every redirect goes from a gone path to a route.
- `bun run types` runs `tsc` over the `include` of `../tsconfig.json`; the `.js` and `.jsx` files are not type-checked.
- `bun run lint` runs Biome (`../biome.json`: format, lint, import order) over the folders it includes; `bun run fix` writes the format and the safe fixes. `../scripts/check.sh` runs it, and Ruff over every `.py` of the repo (`../pyproject.toml`).
- `bun run shots` and `bun run film` drive Chrome; `hosts/web/scripts/README.md` has them.

## GENERATED

- A committed file a tool writes carries `.gen.` in its name: `../designs/shared/palette.gen.css` and `sdk/theme/palette.gen.js` (the palette, written outside this repo), `sdk/icons.gen.js` (`bun run vendor`), `../apps/shared/font/font.gen.json` (`bun run wasm`, checked glyph by glyph against the font door by `../apps/shared/font/font.test.ts`), `../apps/life/library.gen.json` (`bun run library`; `hosts/web/scripts/README.md` has it).
- `../designs/shared/fonts/` is written whole by `bun run vendor`, `keep.css` aside.
- `wasm/`, `dist/` and `node_modules/` are build output and stay out of git.
