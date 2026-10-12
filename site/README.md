# SITE

- mrly.net: one static shell, one router, one table of routes; every page is drawn in the browser.
- Rust is the only math; the browser only draws.
- Browser floor: iOS 16.4 / Safari 16.4 (remark-gfm's lookbehind regex in the prose chunk, OffscreenCanvas in the figure pen).
- Dependencies: React, KaTeX, Shiki, the unified markdown chain and Three.js (the 3D stages of designs, three and graphs, loaded lazily); `bun install` fetches them.
- `ui/README.md` has the shell, the router, the pages and the chrome; `kit/README.md` has the tools; `apps/README.md` has the apps.

## SHAPE

```
request /about/
  edge   kit/edge.js: no dot in the last segment -> /index.html
  shell  ui/index.html: boot.js, page.css, the static chrome, chrome.js + router.js
  router the site:routes row -> ui/app.jsx (apps/<id>/index.jsx) or ui/prose.jsx + fetch /raw/<source> -> #main
  figure lib/figure.js draws each host from an intersection queue
```

## THE UNIT

- `apps/<id>/` is the only unit: every route on mrly.net is one folder, `/<id>/`; `home` is `/`.
- `apps/README.md` has the app contract: the scene, the widget, the spec, the chrome, the rules and a map of every app.
- `app.json`: `title`, `line` (the lead), `menu` (`play`, `read` or `tool`), and `saver`, `hidden`, `deep` (each `true` or absent), `source` (a repo path of a markdown page), `route` (`/404.html` for `missing`). No other field; nothing else is configured.
- The page is exactly one of `index.jsx`, `page.md` or `source`.
- `index.jsx` exports `mount(at)` and `unmount(at)`, an island under the still (title, figure, `<noscript>`), wide, bare and wearing `lib/mrly.css`; or `render(row, site, host, { text, rows, url, left, right })`, `unmount(host)`, optional `mount(host)`, a page drawn before the swap; an optional `layout` (`left`, `right`, `wide`, `bare`, `sheet`) overrides the island's.
- `page.md` and `source` are prose, rendered by `ui/prose.jsx` from `/raw/<source>`; front matter `opener: true` (draws the app's icon above the lede), `button` and `link`.
- `icon.js` is the app's figure, named by the id; a hidden app may omit it, a shown one may not.
- `scene.js` and `widget.jsx`: required of a saver.
- Blog posts: `apps/blog/<slug>/index.md` is `/blog/<slug>/` (front matter `title`, `date` YYYY-MM-DD, `lead`), with its `icon.js`, the figure `blog-<slug>`.
- `site.json` `desk` names the saver home plays and the Settings pick offers first.

## ROUTES

- `scripts/site.ts` collects the rows from `apps/*/app.json`, home first, then by id: `{ route, kind, title, source, date, lead, figure, hidden, meta }`.
- `kind` is `app` (an `index.jsx`) or `prose` (a markdown source); `meta` holds `id`, `menu`, `saver`, `deep`, and `prose` (`page`, `post` or `source`).
- `source` is the repo path served at `/raw/<source>` for a prose row, null otherwise; no row carries a body.
- `/mrlymath/` renders `pkgs/mrlyrs/NAMES.md`, so the page and the crate never disagree.
- `/menu/` is Play, Read and Tools from each app's `menu`, alphabetical, hidden apps off, then the Root and Elsewhere folders of the `tree` in `site.json`; Home is the wordmark.
- `/research/` is the `research` app, a deep route like `/git/`: a grid of topic cards from `research.json`, and `/research/<slug>/` opens that topic's README in a dialog over the grid, its figures as a gallery; an unknown slug shows the missing state.
- `research.json` is built from `../research/*/README.md` (a folder with a front matter `title`; `shared/` never): `{ topics: [{ slug, title, figure, figures }] }`, `figure` the README's, `figures` every figure of `<topic>/figures/`.
- A hidden app (`redirects`, `missing`, `figures`) has no sitemap entry, no `llms.txt` line, no search row and no menu card; it sets no `menu`.
- `/figures/` draws every live figure with its draw time.
- A missing url renders "Nothing here" with `noindex`, status 200 by design.
- The build stops on a bad `app.json`, an unknown figure, a missing source and two rows for one route.
- `redirects.json` is the moved urls, `{ "/old/": { "to": "/new/", "since": "YYYY-MM-DD" } }`; the edge answers each with a 301, `/redirects/` lists them, and the build drops a row whose target is no route.

## FIGURES

- The roster: `apps/*/icon.js` (name = id), `apps/blog/*/icon.js` (name `blog-<slug>`), `../research/*/figures/*.ts` and `../research/shared/figures/*.ts` (name = stem); a name is lowercase words and digits joined by hyphens, taken once, and the file exports `default`.
- A figure exports `default function draw(pen, ink, t)`, `t` in [0, 1) over one loop.
- `size`: `[width, height]`, default `[1024, 1024]`; a non-square size keeps its aspect in the host.
- `still`: the `t` a still figure draws, default 0.
- `loop` in seconds, with `frames`: an animated figure only; none in the live set.
- `units`: the `mrlyjs` wasm units it imports, e.g. `{ math }`; the caller initialises them before the first draw.
- `ink` is `ink(tinted(dark | light, ...))` from `kit/theme`: the tint's hue sits in the blue slot.
- An app icon is a plate drawn with `lib/icon.js` on a clear ground: the build lists every `apps/<id>/icon.js` as a plate; a research or blog figure keeps the ink ground.
- A figure is a pure function of its inputs: no file, DOM, Date or Math.random.
- A figure asserts its facts and throws when one breaks; heavy facts are memoised at module level; pixels are never cached.
- The pen and the kit live in `../pkgs/mrlyjs/view`: `canvas` in the browser, `raster` in Bun and the tests.
- A lone image line `![caption](name)` in a page, a post or a research note is a live figure; the build and the check stop on an unknown name.
- `scripts/figures.test.ts` loads every figure: `default` a function, every `units` value defined.
- `bun run time [name ...]` draws each figure in a fresh worker with the raster pen, cold then warm, dark and light; gate 150 ms warm, 500 ms cold on the icons, `about` excluded; a research figure over it is a named warning, judged with its topic; exit 1 when an icon is over or a figure throws.

## BUILD

- `bun run build` runs `scripts/site.ts` into `dist/`, or into `MRLY_DIST`; pure Bun, no Chrome, no cargo.
- Prime: bundle `ui/boot.js`, `ui/git.css` and `lib/mrly.css`; each woff2 ships as `/fonts/<stem>-<sha8>.woff2`.
- Collect: the apps, the rows, the figure roster, the wasm units the figures and apps import, the kept redirects, the raw list (`git ls-files`), the research topics.
- Bundle: one `Bun.build` with splitting over `ui/index.html`, `ui/app.jsx`, `ui/prose.jsx`, `lib/git.js`, every live figure, every `apps/<id>/index.jsx` and `lib/lock.js`.
- Names: entries `[name]-[hash]`, chunks `lib-[hash]`; the build refuses a bundled file with no hash.
- Shell: the bundled `index.html` with the hashed boot script swapped in for `/boot.js`; `404.html` is its copy.
- Copy: `raw/<path>` for every listed file, `public/`, the font licences.
- Globals: `routes.json`, `llms.txt`, `sitemap.xml` with one `sitemap-<section>.xml` per section and `sitemap-raw.xml`, `robots.txt`, `git.json`, `manifest.webmanifest`, `redirects.json`.
- Write: `dist/` is emptied, then written whole.

## VIRTUAL MODULES

- `scripts/site.ts` is one Bun plugin; Bun cannot import a computed path, so each lazy family is a module of `() => import()` thunks.
- `site:routes`: `{ site, rows }`.
- `site:figures`: name to figure chunk, the wasm urls of the live units, and the plates.
- `site:apps`: id to app chunk, `own` (the apps whose index.jsx exports `render`), plus `lock`, the desk and its lock screen.
- `site:css`: the `git` and `mrly` sheet urls.
- `apps:scenes`: the saver scenes, the desk first, then by title.
- `mrlyjs/<door>` resolves through the exports of `pkgs/mrlyjs/package.json` (the `mrlyjs` plugin, also used by the figure test and `time`); `pkg/<unit>/mrlyjs_<unit>*` resolves to `site/pkg/`.

## WASM

- `bun run wasm` runs `../scripts/wasm.sh`: it builds every unit of `pkgs/bridge/units.txt` and copies the units the roster and `apps/` import into `pkg/<unit>/`.
- Live units today: core, font, life, math, num.
- `pkg.lock` pins the copy in S3 the builder Lambda pulls through `scripts/pkg.ts`; the deploy uploads `pkg/` and re-pins it.
- The build stops when a live unit is missing from `pkg/`.

## DEV

- `bun run --cwd mrlyprod/site dev`, from `site/`, where `bunfig.toml` hands the plugin to Bun's dev server.
- `scripts/dev.ts` serves 127.0.0.1:3000 (`PORT` overrides) with HMR.
- A path the edge sends to the shell answers `ui/index.html`, bundled by Bun on request.
- `/raw/<path>` reads the tree per request; `public/` reads from disk.
- The globals, the boot script, the sheets and the faces are built once at start, so a new page, post or app needs a restart.

## EDGE

- `kit/edge.js` is the CloudFront function source, one plain file.
- Rule: www to apex 301; a moved url 301; `/raw/` passes; `/git/...` and a slash-ended path rewrite to `/index.html`; a dotted last segment passes; a bare last segment 301s to its slash form; every 301 keeps the query.
- The deploy pastes `redirects.json` at its `/*MOVED*/` mark; `kit/edge.ts` does the same for dev and shots.

## PUSH

- `bun run push` builds and ships `dist/` to the bucket by manifest diff.
- Order: the hashed files (immutable, a year), then the rest (max-age 0, s-maxage 60), the shell last.
- A file is immutable only when the build hashed it.
- A removed file stays a week, then goes.
- `--dry` writes nothing and prints the tiers; `--force` uploads every file; `DRY=1` keeps the manifest on local disk.
- The `push` block in `site.json` names the bucket and store env keys and the guarded paths, `cdn/` and `stats/stats.json`.
- The builder Lambda (`kit/lambda.ts`, wired by `../aws/net.ts`) runs `bun run push` on a GitHub push, on its hourly schedule or by hand.

## CHECK

- `bun run check` runs `check.ts`.
- Source rows: the rows collect; every figure a page, a post or a research note names is live; every markdown link lands, a rooted one on a route; the house rules over pages and posts (no two blank lines, no em or en dash, a blank line after a heading).
- Dist rows, over a built `dist/`: every url in the shell is a route or a file; every row's source is under `raw/`; every redirect goes from a gone path to a route.
- `bun run test` runs every test, each beside the file it covers.

## SHOTS

- `bun run shots [route ...]` serves `dist/` through the edge rule on port 3335 and drives one headless Chrome on port 9335 with a throwaway profile.
- It writes full-page pngs at phone and desktop into `data/mrlyprod/site/scripts/shots/latest/`.
- The `shots` block in `site.json` names the default routes and sizes; `kit/README.md` has the flags.
- `bun run film <scene.js> --name <n>` plays one app scene into a contact sheet, or a video with `--video`, under `data/mrlyprod/site/scripts/film/<name>/`.

## FONTS AND ICONS

- Fonts: Noto Sans, Serif, Mono, Math and Color Emoji, named in the `fonts` block of `site.json`.
- `bun run vendor` downloads every slice Google serves for each face into `ui/fonts/`, each with its `unicode-range`; a browser fetches only the slices a page prints.
- `ui/fonts/keep.css` (MrlyFont) is appended to `fonts.css` verbatim.
- `<math>` (KaTeX writes MathML only) draws in Noto Sans Math through `--math`.
- `ui/fonts.test.ts` checks that a shipped face draws every character the routes, their sources and the browser code print.
- Icons: Material Symbols by Google, Apache 2.0; `icons.json` lists the names we use, the one source of truth.
- `bun run vendor` downloads each one's outlined 24px SVG from `google/material-design-icons` and writes `lib/icons.js`; draw one with `<Icon name="close" />`.
- To add an icon: add its Material name to `icons.json`, run `bun run vendor`.

## PUBLIC

- `public/` copies to the site root: `og.png`, `favicon.svg`, `favicon.png`, `favicon.ico`, `apple-touch-icon.png`, `icon-192.png`, `icon-512.png`.
- They are committed images, not drawn at build; the shell and `manifest.webmanifest` name them by absolute url.

## IGNORED

- `pkg/`, `dist/` and `node_modules/` are build output and stay out of git.
