# ssg

- The site builder every MrlyProd site calls. One function renders one route; the rest is bookkeeping.
- Generic: it knows routes, inputs, fingerprints and manifests, never markdown, papers or products.
- A consumer brings its `site.json`, a `collect()` that lists routes and a `render()` per route.

## SITE.JSON

- `title root` and whatever the kit reads: `prefix font settings tint tree socials contact`.
- `inputs`: `{ name: { path, ext?, deep? } }`. Declared, never assumed. `site.input(name).files` reads them back.
- A consumer resolves nothing by hand: an undeclared name throws, so every path a build reads is in one block.
- `kit`: `{ path, out, hash, files }`. Copied into `out/`; `site.asset(name)` gives the href, hashed when `hash` is true.
- `manifest`: the webmanifest, written as is. `robots`: `{ disallow }`.

## EXPORTS

- `scan(spec)`: reads `site.json`, `pages.json` and the declared inputs, calls `collect()`, returns `Site`.
- `render(site, route, spec)`: pure, returns `[{ path, bytes }]` for that route alone. A missing input throws here.
- `globals(site, spec)`: sitemap.xml, robots.txt, llms.txt, the webmanifest, the kit copy, the public copy, then the site's own extras.
- `fingerprint(site, route)`: sha256 of the route's inputs, its data, the templates and the navigator. A directory input hashes every path and byte under it.
- `build(spec, { manifest, force })`: scan, fingerprint, render only what changed, write only bytes that differ, drop the outputs of dead routes.
- `walk bytes forget digest short escape page`: the small helpers a consumer would write twice. `forget()` drops the byte cache, which a watcher calls before it rescans.

## SPEC

- `root out templates collect render globals`. `templates` are the dirs whose bytes rebuild every route.
- The package itself is always a template: edit the kit, every route re-renders.
- `Route`: `{ route, kind, name, data, source, inputs, urls, at, hidden }`. `hidden` keeps it out of the sitemap.
- One route may be a group: `urls` lists the pages it publishes, so a bundler route still fills the sitemap and llms.txt.
- `render` may be async, so a route can run a bundler and hand back its bytes before anything is written.
- The manifest is `{ route: { hash, at, outputs } }` and lives wherever the caller points it.
