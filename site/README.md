# SITE

- mrly.net: one static shell, one router, one table of routes; every page is drawn in the browser.
- The designs and their numbers come from the Rust crates through `@mrlyprod/mrly`; the deep zoom (`src/lib/fractal/`) and the space flights (`src/lib/space/`) are drawn in JS.
- Browser floor: iOS 16.4 / Safari 16.4 (remark-gfm's lookbehind regex in the prose chunk, OffscreenCanvas in the figure pen).
- Dependencies: React, KaTeX, Shiki, the unified markdown chain and Three.js (the 3D stages of designs, three and graphs, loaded lazily); `bun install` fetches them, with `typescript` and `@types/bun` for `bun run types`.
- Each folder has one README: `src/shell/`, `src/styles/`, `src/lib/`, `src/pages/`, `src/apps/`, `scripts/`, `infra/`, `test/`.

## SHAPE

```
site/
  package.json  bun.lock  bunfig.toml       deps, the bun run verbs, the dev plugin, the test preload
  tsconfig.json                             tsc over every .ts file; paths: @mrlyprod/mrly/* -> ../bindings/js, #view -> its index.d.ts
  site.json  redirects.json  wasm.lock      identity and tool blocks; moved urls; the wasm pin
  public/                                   copied to the site root
  src/                                      everything that ships to the browser
    shell/                                  index.html, boot, router, chrome, the page shells
    styles/                                 the sheets, the palette, the fonts
    pages/                                  the site pages, one folder a route
    apps/                                   the apps, one folder a route
    lib/                                    what two or more routes share
  scripts/                                  the bun run verbs and the build
  infra/                                    the code that runs in AWS or talks to it
  test/                                     shared fixtures and suites that span routes

request /about/
  edge   infra/edge.js: no dot in the last segment -> /index.html
  shell  src/shell/index.html: boot.js, page.css, the tint style, the static chrome, chrome.js + router.js
  router the site:routes row -> src/shell/app.jsx (a page's index.jsx or an app's app:<id>) or src/shell/prose.jsx + /raw/<source> -> #main
  figure src/lib/figure.js draws each host from an intersection queue
```

- Imports run one way (`test/imports.test.js`): an app imports `lib/`; a page `lib/` and `shell/`; the shell `lib/` and `styles/`; `lib/` only `lib/`; no route imports another.

## THE UNIT

- A route is the only unit: one folder, `/<id>/`, under `src/pages/` for a site page or `src/apps/` for an app; an id is taken once across both; `home` is `/`.
- `app.json`: `title`, `line` (the lead), `menu` (`play`, `read` or `tool`), and `saver`, `hidden`, `deep` (each `true` or absent), `source` (a repo path of a markdown page), `route` (`/404.html` for `missing`). No other field; nothing else is configured.
- The page is exactly one of `index.jsx`, `index.md` or `source`; a folder with none of them and both `scene.js` and `widget.jsx` is an app whose entry the build writes (`app:<id>`, `src/apps/README.md`).
- `index.jsx` exports `mount(at)` and `unmount(at)`, an island under the still (title, figure, `<noscript>`), wide, bare and wearing `src/lib/app/mrly.css`; or `render(row, site, host, { text, rows, url, left, right })`, `unmount(host)`, optional `mount(host)`, a page drawn before the swap; an optional `layout` (`left`, `right`, `wide`, `bare`, `sheet`) overrides the island's.
- `index.md` and `source` are prose, rendered by `src/shell/prose.jsx` from `/raw/<source>`; front matter `opener: true` (draws the route's icon above the lede), `button` and `link`.
- `icon.js` is the route's figure, named by the id; a hidden route may omit it, a shown one may not.
- `scene.js` and `widget.jsx`: every app has them, a saver must; `src/apps/README.md` has the app contract.
- Blog posts: `src/pages/blog/<slug>/index.md` is `/blog/<slug>/` (front matter `title`, `date` YYYY-MM-DD, `lead`), with its `icon.js`, the figure `blog-<slug>`.
- `site.json` `desk` names the saver home plays and the Settings pick offers first.

## ROUTES

- `scripts/build.ts` collects the rows from `src/pages/*/app.json` and `src/apps/*/app.json`, home first, then by id: `{ route, kind, title, source, date, lead, figure, hidden, meta }`.
- `kind` is `app` (an `index.jsx` or a written entry) or `prose` (a markdown source); `meta` holds `id`, `menu`, `saver`, `deep`, and `prose` (`page`, `post` or `source`).
- `source` is the repo path served at `/raw/<source>` for a prose row, null otherwise; no row carries a body.
- `/mrlymath/` renders `crates/mrly-core/src/name/NAMES.md`, so the page and the crate never disagree.
- `research.json` is built from `../research/*/README.md` (a folder with a front matter `title`; `shared/` never): `{ topics: [{ slug, title, figure, figures }] }`, `figure` the README's, `figures` every figure of `<topic>/figures/`.
- A hidden route (`redirects`, `missing`, `figures`) has no sitemap entry, no `llms.txt` line, no search row and no menu card; it sets no `menu`.
- A missing url renders "Nothing here" with `noindex`, status 200 by design.
- The build stops on a bad `app.json`, an unknown figure, a missing source and two rows for one route.
- `redirects.json` is the moved urls, `{ "/old/": { "to": "/new/", "since": "YYYY-MM-DD" } }`; the edge answers each with a 301, `/redirects/` lists them, and the build drops a row whose target is no route.

## FIGURES

- The roster: `src/{pages,apps}/*/icon.js` (name = id), `src/pages/blog/*/icon.js` (name `blog-<slug>`), `../research/*/figures/*.ts` and `../research/shared/figures/*.ts` (name = stem); a name is lowercase words and digits joined by hyphens, taken once, and the file exports `default`.
- A figure exports `default function draw(pen, ink, t)`, `t` in [0, 1) over one loop.
- `size`: `[width, height]`, default `[1024, 1024]`; a non-square size keeps its aspect in the host.
- `still`: the `t` a still figure draws, default 0.
- `loop` in seconds, with `frames`: an animated figure only; none in the live set.
- `units`: the `@mrlyprod/mrly` wasm units it imports, e.g. `{ math }`; the caller initialises them before the first draw.
- `ink` is `ink(tinted(dark | light, ...))` from `src/lib/theme`: the tint's hue sits in the blue slot.
- A route icon is a plate drawn with `src/lib/plate.js` on a clear ground: the build lists every `src/{pages,apps}/<id>/icon.js` as a plate; a research or blog figure keeps the ink ground.
- A figure is a pure function of its inputs: no file, DOM, Date or Math.random.
- A figure asserts its facts and throws when one breaks; heavy facts are memoised at module level; pixels are never cached.
- The pen and the kit live in `src/lib/view`, imported as `#view`: `canvas` in the browser, `raster` in Bun and the tests.
- A lone image line `![caption](name)` in a page, a post or a research note is a live figure; the build stops on an unknown name in a row's source, `scripts/build.test.ts` on one anywhere.
- `scripts/figures.test.ts` loads every figure: `default` a function, every `units` value defined.
- `bun run time [name ...]` draws each figure in a fresh worker with the raster pen, cold then warm, dark and light; gate 150 ms warm, 500 ms cold on the icons, `about` excluded; a research figure over it is a named warning, judged with its topic; exit 1 when an icon is over or a figure throws.

## BUILD

- `bun run build` runs `scripts/build.ts` into `dist/`, or into `MRLY_DIST`; pure Bun, no Chrome, no cargo.
- Prime: bundle `src/shell/boot.js`, `src/styles/git.css` and `src/lib/app/mrly.css`; each woff2 ships as `/fonts/<stem>-<sha8>.woff2`.
- Collect: the routes, the rows, the figure roster, the wasm units, the kept redirects, the raw list (`git ls-files`), the research topics.
- Bundle: one `Bun.build` with splitting over `src/shell/index.html`, `src/shell/app.jsx`, `src/shell/prose.jsx`, `src/pages/git/git.js`, every live figure, every `src/pages/<id>/index.jsx` and `src/shell/lock.js`; each `app:<id>` is a chunk of the `site:apps` thunks.
- Names: entries `[name]-[hash]`, chunks `lib-[hash]`; the build refuses a bundled file with no hash.
- Shell: the bundled `index.html` with the hashed boot script swapped in for `/boot.js` and its empty `<style id="tint">` filled with `tintCss(site.tint)` (`withTint`, which dev uses too); `404.html` is its copy.
- Copy: `raw/<path>` for every listed file, `public/`, the font licences.
- Globals: `routes.json`, `llms.txt`, `sitemap.xml` with one `sitemap-<section>.xml` per section and `sitemap-raw.xml`, `robots.txt`, `git.json`, `manifest.webmanifest`, `redirects.json`.
- Write: `dist/` is emptied, then written whole; `bun run push` ships it (`infra/README.md`).

## VIRTUAL MODULES

- `scripts/build.ts` is one Bun plugin; Bun cannot import a computed path, so each lazy family is a module of `() => import()` thunks.
- `site:routes`: `{ site, rows }`.
- `site:figures`: name to figure chunk, the wasm urls of the live units, and the plates.
- `site:apps`: id to route chunk, `own` (the routes whose index.jsx exports `render`), plus `lock`, the desk and its lock screen.
- `app:<id>`: an app's entry, `entry(dir)`: `page(Widget, { seed: 0, ...PAGE.defaults }, PAGE)` from its `scene.js` and `widget.jsx`.
- `site:css`: the `git` and `mrly` sheet urls.
- `apps:scenes`: the saver scenes, the desk first, then by title.
- `@mrlyprod/mrly/<door>` resolves through `tsconfig.json` paths onto `../bindings/js` and `#view` through the `imports` of `package.json`; the `mrly` plugin, also used by the figure test and `time`, maps both for research figures outside `site/`; `wasm/<unit>/mrly_js_<unit>*` resolves to `site/wasm/`.

## WASM

- `bun run wasm` runs `scripts/wasm.ts`: `../scripts/wasm.sh` builds every unit under `../bindings/js/units/` into `../bindings/js/pkg/`, then it copies the live units into `wasm/<unit>/` and writes `src/lib/font/font.gen.json`; `--no-build` skips the build.
- A unit is live when a figure or anything under `src/` names it (`live()` in `scripts/build.ts`); `src/lib/units/` holds a loader for `font`, `life`, `math` and `num`; `core` is live through the figures. Live today: core, font, life, math, num.
- `wasm.lock` pins the copy in S3 the builder Lambda pulls through `infra/wasm.ts`; the deploy uploads `wasm/` and re-pins it.
- A pin that trails a fresh local `wasm/` is normal between deploys: `bun run check` reports it; `infra/wasm.ts` fails the build when the fetched copy does not hash to the pin.
- The build stops when a live unit is missing from `wasm/`.

## DEV

- `bun run --cwd site dev`, from `site/`, where `bunfig.toml` hands the plugin to Bun's dev server.
- `scripts/dev.ts` serves 127.0.0.1:3000 (`PORT` overrides) with HMR.
- A path the edge sends to the shell answers `src/shell/index.html`, bundled by Bun on request, its tint style filled.
- `/raw/<path>` reads the tree per request; `public/` reads from disk.
- The globals, the boot script, the sheets and the faces are built once at start, so a new page, post or app needs a restart.

## CHECK AND TEST

- `bun run test` runs `bun test`: every test beside the file it covers, plus `test/`; `bunfig.toml` ignores `dist/`, whose `raw/` holds copies.
- A fresh clone runs, in order: `../scripts/wasm.sh` (cargo and wasm-bindgen build `../bindings/js/pkg/`), `bun install`, `bun test`; `package.json` pins bun.
- `bun run check` runs `scripts/check.ts`: it reports the wasm pin, then runs `test/content.test.ts`, the content gate: every markdown link in a page, a post or a source lands, a rooted one on a route; every research link lands; the house rules over pages, posts and notes (no two blank lines, no em or en dash, a blank line after a heading); over a fresh build in a temp dir, every link on a built page lands and every redirect goes from a gone path to a route.
- `bun run types` runs `tsc` over every `.ts` file of `site/`; the `.js` and `.jsx` files are not type-checked.
- `bun run shots` and `bun run film` drive Chrome; `scripts/README.md` has them.

## PUBLIC

- `public/` copies to the site root: `og.png`, `favicon.svg`, `favicon.png`, `favicon.ico`, `apple-touch-icon.png`, `icon-192.png`, `icon-512.png`.
- They are committed images, not drawn at build; the shell and `manifest.webmanifest` name them by absolute url.

## GENERATED

- A committed file a tool writes carries `.gen.` in its name: `src/styles/palette.gen.css` and `src/lib/theme/palette.gen.js` (the palette, written outside this repo), `src/lib/icons.gen.js` (`bun run vendor`), `src/lib/font/font.gen.json` (`bun run wasm`, checked glyph by glyph against the font door by `src/lib/font/font.test.js`).
- `src/styles/fonts/` is written whole by `bun run vendor`, `keep.css` aside.
- `wasm/`, `dist/` and `node_modules/` are build output and stay out of git.
