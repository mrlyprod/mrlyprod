# SITE

- mrly.net: one static shell, one router, one table of routes; every page is drawn in the browser.
- Rust is the only math; the browser only draws.
- Browser floor: iOS 16.4 / Safari 16.4 (remark-gfm's lookbehind regex in the prose chunk, OffscreenCanvas in the figure pen).
- Dependencies: React, KaTeX, Shiki, the unified markdown chain and Three.js (the 3D stage of the designs app, loaded lazily); `bun install` fetches them.
- `ui/README.md` has the shell, the router, the pages and the chrome; `kit/README.md` has the tools; `apps/README.md` has the apps.

## SHAPE

```
request /about/
  edge   kit/edge.js: no dot in the last segment -> /index.html
  shell  ui/index.html: boot.js, page.css, the static chrome, chrome.js + router.js
  router the site:routes row -> site:pages[kind]() + fetch /raw/<source> -> React into #main
  figure lib/figure.js draws each host from an intersection queue
```

## ROUTES

- `scripts/site.ts` collects the rows: `{ route, kind, title, source, date, lead, figure, hidden, meta }`.
- `source` is the repo path served at `/raw/<source>` for a markdown row, null otherwise; no row carries a body.
- Kinds: home, hub, prose, app, settings, git, stats, cart, paused, missing, moved, figures.
- `pages/<slug>.md` is `/<slug>/`: front matter `title`, `lead`, optional `figure`, `button` and `link`.
- `blog/<slug>/index.md` is `/blog/<slug>/`: front matter `title`, `date` (YYYY-MM-DD), `lead`, optional `figure` (default `blog-<slug>`).
- `/mrlymath/` renders `pkgs/mrlyrs/NAMES.md`, so the page and the crate never disagree.
- `apps/apps.json` makes `/<id>/` per row.
- `/blog/` and `/menu/` are hubs; the menu is the `tree` of `site.json`, filled with the pages, the apps and the socials.
- `/research/`, `/research/wiki/` and `/demos/` are paused doors: the door's figure, its title, one line that the section is being rebuilt; anything under them is missing.
- `/cart/`, `/stats/`, `/redirects/`, `/404.html` and `/figures/` are hidden: no sitemap, no `llms.txt`, no search.
- `/figures/` draws every live figure with its draw time.
- A missing url renders "Nothing here" with `noindex`, status 200 by design.
- Every row wears a live figure; the build stops on an unknown name, on a missing source and on two rows for one route.
- `redirects.json` is the moved urls, `{ "/old/": { "to": "/new/", "since": "YYYY-MM-DD" } }`; the edge answers each with a 301, `/redirects/` lists them, and the build drops a row whose target is no route.

## BUILD

- `bun run build` runs `scripts/site.ts` into `dist/`, or into `MRLY_DIST`; pure Bun, no Chrome, no cargo.
- Prime: bundle `ui/boot.js`, `ui/git.css` and `lib/mrly.css`; each woff2 ships as `/fonts/<stem>-<sha8>.woff2`.
- Collect: the rows, the figure roster (`../figures/*.ts`), the wasm units the figures and apps import, the kept redirects, the raw list (`git ls-files` less `research/`).
- Bundle: one `Bun.build` with splitting over `ui/index.html`, `ui/pages/*.jsx`, `lib/git.js`, every live figure, every `apps/<id>/index.jsx` and `lib/lock.js`.
- Names: entries `[name]-[hash]`, chunks `lib-[hash]`; the build refuses a bundled file with no hash.
- Shell: the bundled `index.html` with the hashed boot script swapped in for `/boot.js`; `404.html` is its copy.
- Copy: `raw/<path>` for every listed file, `public/`, the font licences.
- Globals: `routes.json`, `llms.txt`, `sitemap.xml` with one `sitemap-<section>.xml` per section and `sitemap-raw.xml`, `robots.txt`, `git.json`, `manifest.webmanifest`, `redirects.json`.
- Write: `dist/` is emptied, then written whole.

## VIRTUAL MODULES

- `scripts/site.ts` is one Bun plugin; Bun cannot import a computed path, so each lazy family is a module of `() => import()` thunks.
- `site:routes`: `{ site, rows }`.
- `site:pages`: kind to page chunk, from `ui/pages/*.jsx`.
- `site:figures`: name to figure chunk, plus the wasm urls of the live units.
- `site:apps`: id to app chunk, plus `lock`, the lock screen.
- `site:css`: the `git` and `mrly` sheet urls.
- `apps:scenes`: the saver scenes, for the lock screen.
- `mrlyjs/<door>` resolves through the exports of `pkgs/mrlyjs/package.json`; `pkg/<unit>/mrlyjs_<unit>*` resolves to `site/pkg/`.

## WASM

- `bun run wasm` runs `../scripts/wasm.sh`: it builds every unit of `pkgs/bridge/units.txt` and copies the units `../figures/` and `apps/` import into `pkg/<unit>/`.
- Live units today: font, life, math.
- `pkg.lock` pins the copy in S3 the builder Lambda pulls through `scripts/pkg.ts`; the deploy uploads `pkg/` and re-pins it.
- The build stops when a live unit is missing from `pkg/`.

## DEV

- `bun run --cwd mrlyprod/site dev`, from `site/`, where `bunfig.toml` hands the plugin to Bun's dev server.
- `scripts/dev.ts` serves 127.0.0.1:3000 (`PORT` overrides) with HMR.
- A path the edge sends to the shell answers `ui/index.html`, bundled by Bun on request.
- `/raw/<path>` reads the tree per request, never `research/`; `public/` reads from disk.
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
- Source rows: the rows collect; every figure a page or post names is live; every markdown link lands, a rooted one on a route; the house rules (no two blank lines, no em or en dash, a blank line after a heading).
- Dist rows, over a built `dist/`: every url in the shell is a route or a file; every row's source is under `raw/`; every redirect goes from a gone path to a route.
- `bun run test` runs every test, each beside the file it covers.

## SHOTS

- `bun run shots [route ...]` serves `dist/` through the edge rule on port 3335 and drives one headless Chrome on port 9335 with a throwaway profile.
- It writes full-page pngs at phone and desktop into `data/mrlyprod/site/scripts/shots/latest/`.
- The `shots` block in `site.json` names the default routes and sizes; `kit/README.md` has the flags.
- `bun run film <scene.js> --name <n>` plays one app scene into a contact sheet, or a video with `--video`, under `data/mrlyprod/site/scripts/film/<name>/`.

## FONTS

- `bun run vendor` writes `ui/fonts/` from the `fonts` block in `site.json`: the Google faces, then `ui/fonts/keep.css` (MrlyFont, Noto Sans Symbols 2) verbatim.
- `bun run symbols` cuts Noto Sans Symbols 2 to the glyphs the routes, their sources and the browser code print.
- `ui/fonts.test.ts` checks that a shipped face draws every such glyph.
- Material Symbols by Google, Apache 2.0: eight paths vendored 2026-10-08 and 2026-10-09 into `lib/icons.js`, plus `sound`, `music` and `wave` drawn there on the same grid.

## PUBLIC

- `public/` copies to the site root: `og.png`, `favicon.svg`, `favicon.png`, `favicon.ico`, `apple-touch-icon.png`, `icon-192.png`, `icon-512.png`.
- They are committed images, not drawn at build; the shell and `manifest.webmanifest` name them by absolute url.

## IGNORED

- `../research/` is the archive: no route, no figure, no `/raw/research/`, no `git.json` row, no `llms.txt` line.
- `pkg/`, `dist/` and `node_modules/` are build output and stay out of git.
