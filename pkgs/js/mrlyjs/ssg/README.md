# ssg

- The site builder every MrlyProd site calls. One function renders one route; the rest is bookkeeping.
- Generic: it knows routes, inputs, fingerprints and manifests, never markdown, papers or products.
- A consumer brings its `site.json`, a `collect()` that lists routes and a `render()` per route.
- The one exception is `../git`: a `git` block in `site.json` makes `scan` append the repo's own `/git/` and `/raw/` routes, and `render` and `fingerprint` dispatch to that module.

## SITE.JSON

- `title root` and whatever the kit reads: `prefix font settings tint tree socials contact`.
- `inputs`: `{ name: { path, ext?, deep? } }`. Declared, never assumed. `site.input(name).files` reads them back.
- A consumer resolves nothing by hand: an undeclared name throws, so every path a build reads is in one block.
- `kit`: `{ path, out, hash, files }`. Copied into `out/`; `site.asset(name)` gives the href, hashed when `hash` is true.
- `manifest`: the webmanifest, written as is. `robots`: `{ disallow }`, appended to the wildcard block alone.
- `llms`: `{ about, links }`. `about` is the paragraph llms.txt opens on; a link is `{ href, name, note }` and is dropped unless the site publishes that route.

## EXPORTS

- `scan(spec)`: reads `site.json`, `pages.json` and the declared inputs, calls `collect()`, returns `Site`.
- `render(site, route, spec)`: pure, returns `[{ path, bytes, type? }]` for that route alone. A missing input throws here.
- `type` overrides the content type a path would earn by its extension; it rides in the manifest so `push.ts` sets the S3 header without re-rendering.
- `globals(site, spec)`: sitemap.xml, robots.txt, llms.txt, the webmanifest, the kit copy, the public copy, then the site's own extras.
- robots.txt allows everything: an `Allow: /` block per named crawler (GPTBot, ClaudeBot, Claude-Web, CCBot, Google-Extended, anthropic-ai, PerplexityBot), then `*`, then the sitemap line.
- sitemap.xml is one `<url>` per entry in every route's `urls`, `lastmod` from the route's `at`, so a group route fills the map with the pages it publishes.
- llms.txt is the title, the site root, the `about` paragraph and the declared links; nothing is listed by accident and no route writes itself in.
- `fingerprint(site, route)`: sha256 of the route's input bytes, its data, the templates and the navigator. Never a date, never an absolute path.
- A file is named by its declaration, `research/foo.md`, not by where the tree sits; an undeclared file is named by its basename, a directory hashes every inner path and byte relative to itself.
- So a checkout, a tarball and a lambda fingerprint the same bytes the same way, and one manifest serves them all.
- `build(spec, { manifest, force, verify })`: scan, fingerprint, render only what changed, write only bytes that differ, drop the outputs of dead routes.
- `verify` is on by default and re-renders a route whose outputs went missing from `out/`; a build against a remote manifest turns it off, because there the disk is a scratch pad.
- `walk bytes forget digest short escape page`: the small helpers a consumer would write twice. `forget()` drops the byte cache, which a watcher calls before it rescans.

## SPEC

- `root out templates collect render globals git`. `templates` are the dirs whose bytes rebuild every route.
- `git` is the code viewer's hooks, `{ page, md, code }`: the chrome, the markdown pipeline and the highlighter the module cannot know by itself.
- The package itself is always a template: edit the kit, every route re-renders.
- `Route`: `{ route, kind, name, data, source, inputs, urls, at, hidden, sitemap }`.
- `hidden` keeps a route out of the navigator and out of every list a reader browses; `sitemap` puts it back on the map anyway.
- The code viewer sets both, so a thousand pages the tree never shows are still crawlable; `/404.html` sets only `hidden` and stays off.
- One route may be a group: `urls` lists the pages it publishes, so a bundler route and a code page still fill the sitemap.
- `render` may be async, so a route can run a bundler and hand back its bytes before anything is written.
- The manifest is `{ route: { hash, at, outputs, types? } }` and lives wherever the caller points it.
- `at` is the route's own date, else the manifest's while the hash holds, else today, so a tree with no git keeps the dates it was given.
