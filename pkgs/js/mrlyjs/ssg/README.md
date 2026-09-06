# ssg

- The site builder every MrlyProd site calls. One function renders one route; the rest is bookkeeping.
- Generic: it knows routes, inputs, fingerprints and manifests, never markdown, papers or products.
- A consumer brings its `site.json`, a `collect()` that lists routes and a `render()` per route.

## SITE.JSON

- `title root` and whatever the kit reads: `prefix font settings tint tree socials contact`.
- `inputs`: `{ name: { path, ext?, deep? } }`. Declared, never assumed. `site.input(name).files` reads them back.
- `kit`: `{ path, out, hash, files }`. Copied into `out/`; `site.asset(name)` gives the href, hashed when `hash` is true.
- `manifest`: the webmanifest, written as is. `robots`: `{ disallow }`.

## EXPORTS

- `scan(spec)`: reads `site.json`, `pages.json` and the declared inputs, calls `collect()`, returns `Site`.
- `render(site, route, spec)`: pure, returns `[{ path, bytes }]` for that route alone. A missing input throws here.
- `globals(site, spec)`: sitemap.xml, robots.txt, llms.txt, the webmanifest, the kit copy, the public copy, then the site's own extras.
- `fingerprint(site, route)`: sha256 of the route's inputs, its data, the templates and the navigator.
- `build(spec, { manifest, force })`: scan, fingerprint, render only what changed, write only bytes that differ, drop the outputs of dead routes.
- `walk bytes digest short escape page`: the small helpers a consumer would write twice.

## SPEC

- `root out templates collect render globals`. `templates` are the dirs whose bytes rebuild every route.
- The package itself is always a template: edit the kit, every route re-renders.
- `Route`: `{ route, kind, name, data, source, inputs, at, hidden }`. `hidden` keeps it out of the sitemap.
- The manifest is `{ route: { hash, at, outputs } }` and lives wherever the caller points it.
