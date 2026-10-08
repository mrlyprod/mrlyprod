# ssg

- The builder of mrly.net. One function renders one route; the rest is bookkeeping.
- `scripts/site.ts` holds the spec: a `collect()` that lists routes and a `render()` per route, beside `site.json`.
- `md.ts` is the markdown pipeline: `render(md, { link, math, widget, lazy })`, `inline`, `sheet`, `front`, `title`, `summary`, `plain`, `slug`, `escape`.
- `link` answers a string, or a dark and light pair: a lone image line then renders as a `<picture>` figure, and an inline image or a link takes the light half.
- A text is parsed once: a tokenizer rule keeps an asterisk between two word characters literal, so no second pass rewrites the source.
- The first image of a text loads eagerly and every later one carries `loading="lazy" decoding="async"`; `lazy: true` says an image already stands above the text, so all of them are lazy.
- `text.ts` holds `escape` alone, so `md.ts` and the code viewer's browser half load in a page with no builder.
- `links.ts` is the one link resolver and `pic.ts` draws a dark and light `<picture>` pair.
- The first dispatch is `modes.ts`: `render` hands every `spa` route to it before anything else.
- The next is `../git`: a `git` block in `site.json` makes `scan` append the `/git/` shell and the repo's `/raw/` routes, `fingerprint` and `render` dispatch to that module; no block, nothing git-related runs.
- The other dispatch is `blog.ts`: a `blog` input makes `scan` append `/blog/` and every post, and `render` dispatches to that module; no input, no routes.
- `../serve.ts` maps a path to a route through `render` and `globals`, and `../dev.ts` serves a site through it; neither reads a file by path, and a route lists in `urls` what it publishes beside its page or, as the slash route above, holds whatever is asked for under it.

## SITE.JSON

- `title` and `root`, then whatever the chrome reads.
- `inputs`: `{ name: { path, ext?, deep? } }`. Declared, never assumed. `site.input(name).files` reads them back.
- The site resolves nothing by hand: an undeclared name throws, so every path a build reads is in one block.
- `kit`: `{ path, out, hash, files, ext }`, one bundle copied into `out/`; `site.asset(name)` gives the href, hashed when `hash` is true.
- `files` names the bundle's files and `ext` narrows them; with no `files` the folder's top level is the list.
- A hashed `.js` file has every relative `import` and `import()` of another listed file rewritten to that file's hashed href, and a hashed `.css` file the same for every `url()` and `@import`, dependencies first, so the kit needs no bundler.
- A cycle throws, and naming a file the bundle does not list throws unless an earlier bundle already placed it, which answers with that file's hashed href.
- `assets`: more bundles of the same shape.
- `sheets`: `[{ name, out?, files }]`. A sheet is its members' placed bytes joined in the order listed, under one hashed name; `site.asset(name)` gives its href.
- A member ships only inside its sheets: its own copy, its asset name and its `/raw/` mirror are dropped. A member no bundle places throws.
- No bundler touches a sheet, so the cascade is the listed order; an `@import` or an `@charset` in any member but the first throws.
- `manifest`: the webmanifest, written as is. `robots`: `{ disallow }`, appended to the wildcard block alone.
- `modes`: `{ prefix: mode or { mode, deep } }`, optional; see MODES.
- `llms`: `{ about, legend, links }`, optional. `about` is the paragraph llms.txt opens on, `legend` one line under it; a link is `{ href, name, note }` and is dropped unless the site publishes that route. No block, no `llms.txt`.

## BLOG

- `blog.ts` is the one blog: a post is `site/blog/<slug>/index.md` with every figure and file beside it.
- The input is declared like any other, `"blog": { "path": "blog", "deep": true }`; no input, no folder or an empty one and the site gets no routes at all.
- Front matter is `title`, `date` (YYYY-MM-DD) and `lead`; a missing or malformed one throws naming the post, and a slug that is not lowercase words joined by hyphens throws too.
- A file loose in the blog folder throws: the shape is one folder per post, never a bare `<slug>.md`.
- `/blog/` lists the posts newest first, then by slug; `/blog/<slug>/` is the post, and its `lastmod` is the front matter date.
- Every file beside `index.md` ships at `/blog/<slug>/<path>` with its content type on the output, so `push.ts` sets the S3 header without re-rendering.
- Each one is named in `site.ships` at collect time, repo file to published URL, so the code viewer links that copy and writes no `/raw/` twin, whatever its type.
- Those files ride a hidden route that never enters the sitemap, and each one is named in the link index, so a relative image in the body answers with its served path.
- A markdown sibling ships too but is never indexed, so a link to it falls through the resolver to its `/git/` path where the site has the viewer.
- `leaf.image` is the og:image: the first figure in the body when it names a shipped file, its bare name when the body names a site figure, else empty.
- `posts(site)` is the parsed list, memoised per site, so a site's own `collect` can read it for its tree, its cards and its home page.
- The module never imports the chrome: `spec.blog` carries it, `{ page, md }`, and the tests draw through it. No `spec.blog.page` means the routes are collected and nothing is rendered.
- `page(site, leaf)` draws the page, listing and post alike: `leaf.kind` says which, `leaf.posts` is every card, and `leaf.out` is the outputs the page may add beside itself.
- `md(site, text, from, out)` renders the body the site's way, with `from` set to the post's `index.md`, so relative links and images resolve beside the post.

## MODES

- `modes.ts` says when a route's page is made: `ssg` at build, `spa` in the browser, `ssr` at request.
- A route's own `mode` wins, then the longest `modes` prefix its path starts with, then `ssg`.
- `modes` in `site.json` maps a prefix to a word or a rule: `{ "/demos/": "spa", "/git/": { "mode": "spa", "deep": true } }`.
- A prefix starts and ends with `/`, a mode is `ssg`, `spa` or `ssr`, and only an `spa` prefix may be deep; anything else throws.
- A deep prefix's shell answers every path under it: in `serve()`, in `shots.ts`, and in the CloudFront router, which reads the same block.
- `spec.spa` is `{ entries, page, plugins?, place? }`; every field is set by `scripts/site.ts`.
- `entries(site)` lists every browser entry, an `.html` or a script; one `Bun.build`, split and minified, bundles them all once per build, chunks named `lib-[hash]`.
- The site roots that build; `plugins(site)` hands it Bun plugins; `place(path)` moves a bundled file to its published path and rewrites its relative imports to match.
- An `spa` route names its entry in `entry`, or a group names one per page in `urls`.
- Each page gets a `Shell`, `{ route, entry, html, script }`: `html` the bundled html entry, empty for a script entry; `script` the module it loads.
- A bundled html entry carries Bun's own `modulepreload` lines, one per chunk; the kit adds none.
- `page(site, route, shell, out)` returns the page's bytes and may push more outputs into `out`.
- An `spa` route's outputs are its shells alone; the bundle's files have one owner, `globals`, so each is pushed by its own bytes, and the manifest's `@spa` record lists them so a dead chunk is deleted.
- The bundle's digest rides in every `spa` route's fingerprint, so a shell re-renders when any entry, import or installed package changes a chunk.
- The build throws on an `ssr` route, an `spa` route with no entry or no `spec.spa`, a deep prefix with no `spa` route at it, a failed bundle, an entry outside `root` or missing from `entries`, and any output under a deep prefix but its `index.html`.
- `ssr` is reserved: dev renders it on request like any route, and the build refuses it until an origin exists.

## EXPORTS

- `scan(spec)`: reads `site.json` and the declared inputs, places every bundle, joins the sheets, calls `collect()`, returns `Site`.
- `Site`: `root out config inputs kit routes stamp index copies serves made ships asks asset input bytes`.
- `copies` are the placed bundle files and the sheets, which `globals` writes.
- `serves` maps a bundled source file to its href and `made` holds every path the build has written; the code viewer's `served` hook reads both.
- `ships` maps a repo file the build publishes byte for byte to that URL, filled at collect time, and the code viewer reads it before it asks the hook.
- `render(site, route, spec)`: pure, returns `[{ path, bytes, type? }]` for that route alone. A missing input throws here.
- `type` overrides the content type a path would earn by its extension; it rides in the manifest so a push sets the S3 header without re-rendering.
- `globals(site, spec, lean?)`: the copies, the `spa` bundle's files unless `lean`, sitemap.xml and its children, robots.txt, llms.txt, the webmanifest, the icons, `git.json`, the public copy, then the site's own extras.
- robots.txt allows everything: an `Allow: /` block per named crawler (GPTBot, ClaudeBot, Claude-Web, CCBot, Google-Extended, anthropic-ai, PerplexityBot), then `*`, then the sitemap line.
- The map is one `<url>` per entry in every route's `urls`, `lastmod` from the route's `at`, so a group route fills the map with the pages it publishes.
- sitemap.xml is an index: one child `sitemap-<segment>.xml` per first path segment holding two urls or more, every other url (`/` and each lone segment) in `sitemap-pages.xml`; each entry's `lastmod` is its child's newest.
- The children sit at the root because a sitemap may only list urls under its own folder. A `/pages/` of two urls throws, and so does any route output, public file or extra on a sitemap's path.
- llms.txt is the title, the site root, `about`, `legend`, the declared links, then one `## name` block per section of `spec.llms`; a row whose route the site does not publish is dropped, an emptied section with it, and no route writes itself in.
- `fingerprint(site, route, spec?, asks?, reads?)`: sha256 of the route's input bytes, its data, the site stamp, the `spa` bundle's digest on an `spa` route, the answer to each of its `asks` and the bytes of each of its `reads`. Never a day, never an absolute path.
- The stamp is the bytes of every module that draws, `site.json` whole, the asset names and the calendar year, which a footer prints. No route list is in it.
- `graph(roots)` follows every relative import, static or dynamic, from the roots; `drawn(spec)` is that graph from `build.ts` and the spec's `templates`.
- So a test, a README or a tool that draws no page is outside the stamp, and editing one repaints nothing but its own `/raw/` copy.
- `asks` are the questions a route's links put to the route map and to the tracked tree, kept in its manifest record: the route repaints only when one of those answers changes.
- So a new, renamed or removed page repaints itself, the pages whose data lists it and the pages that link it; a new tracked file repaints only the pages that link to it.
- `reads` are the files a render read through `bytes()` or looked for through `probe()` outside its inputs, kept in the record relative to the site root: a figure pressed again, or one that appears later, repaints the pages that ship it.
- A file is named by its declaration, `research/foo.md`, not by where the tree sits; an undeclared file is named by its basename, a directory hashes every inner path and byte relative to itself.
- So a checkout, a tarball and a lambda fingerprint the same bytes the same way, and one manifest serves them all.
- `build(spec, { manifest, force, verify })`: scan, check the modes, fingerprint, render only what changed, write only bytes that differ; it returns the globals it wrote as `shared`.
- `force` renders every route whatever its fingerprint: the backstop, `--force` on `bun run build` and `bun run push`; a forced push also reseals every output, so bytes a fingerprint missed still upload.
- An output that turns from a file into a folder, or back, replaces the old shape.
- Then it drops the outputs of dead routes, prunes every folder that empties, and sweeps each bundle's `out/` of any file this build did not write.
- `verify` is on by default and re-renders a route whose outputs went missing from `out/`; a build against a remote manifest turns it off, because there the disk is a scratch pad.
- `walk bytes probe forget digest short escape page today guard label jsonScript`: the small helpers. `probe(file)` says whether a file is there. `forget()` drops the byte cache, which the dev watcher calls before it rescans.

## SPEC

- `root out templates collect render globals llms inline icons git blog spa`.
- `llms(site)` returns `[{ name, rows }]`, a row `{ href, name, note }`; it runs in `globals`, after every route is collected.
- `templates` names the modules that draw, `["scripts/site.ts"]`; their import graph, with `build.ts`'s, is hashed into the stamp.
- `scripts/stamp.test.ts` loads the site in a child process and fails when a module it loaded is missing from that graph.
- `collect(site)` returns `{ routes }`. A page that lists other routes carries that list in its `data` or its `inputs`.
- `inline` lists every inline script a page may carry; `build` refuses a page carrying one it does not know.
- `icons`: `{ rows, svg }`, a square glyph grid of `0`/`1` strings and the favicon svg; the builder writes the svg as `favicon.svg` and draws `favicon.png`, `apple-touch-icon.png`, `icon-192.png` and `icon-512.png` from the grid, and wraps a 32 px png as `favicon.ico` for clients that ask for it unlinked. No field, no icons.
- `git` is the code viewer's hooks, `{ page, entry, served }`: the chrome, the browser module and the mirror seam.
- `spa` is the browser build's hooks; see MODES.
- `blog` is the blog's hooks, `{ page, md }`.
- `Route`: `{ route, kind, name, data, source, inputs, urls, at, hidden, sitemap, mode, entry }`; `mode` and `entry` are MODES'.
- `hidden` keeps a route out of the sitemap; `sitemap` puts it back on the map anyway.
- `shown(site)` is every url of a route no `hidden` covers, `{ route, name }` in route order: what a page may list.
- A `/raw/` route sets both, so every file the tree never shows is still crawlable; `/404.html` sets only `hidden` and stays off.
- One route may be a group: `urls` lists the pages it publishes, so a bundler route still fills the sitemap.
- `render` may be async, so a route can run a bundler and hand back its bytes before anything is written.
- The manifest is `{ route: { hash, at, outputs, types?, asks?, reads? } }` and lives wherever the caller points it.
- `at` is the route's own date, else the manifest's while the hash holds, else today, so a tree with no git keeps the dates it was given.

## LINKS

- `resolve(site, from, url)` in `links.ts` is the one resolver: every markdown render at build sends its links through it, and `from` is the file the link is written in, absolute or relative to the repo root.
- `https:`, `http:`, `mailto:`, `tel:`, a bare `#fragment` and a rooted `/path` pass through untouched; everything else is a path.
- The path resolves against the directory of `from`, and its `#fragment` or `?query` is set aside and put back on whatever the resolver answers.
- `scan()` builds the index once per build: every route's `source`, and every `source` a route names in its `urls`, mapped to that route under both its absolute path and its declared name, so `research/core.md` and `demos/spin` are keys as much as the full paths are.
- A route that wants to be found by a link names the input it publishes in `source`; a group route names one per page in `urls`, which is how a shelf hands each folder its own route.
- The index is asked first, for the path, the path plus `.md`, the path with `.md` stripped and the path's own `README.md`, so `bases.md`, `bases`, `../demos/spin/` and a folder whose README is a page all land on the route the site publishes.
- A miss falls to the repo: `/raw/<path>` for an image or a PDF, `/git/<path>` for a file, `/git/<path>/` for a directory, and only when the repo tracks that path.
- An image or a PDF the site serves elsewhere answers with that copy, from `site.ships` or `spec.git.served`, because its `/raw/` object is never written.
- Each question is logged in `site.asks` while a route renders: a path from the repo root for the route map, a `/git/` route for the tracked tree.
- `answer(index, ask)` is the route the map gives that path, empty when it has none, or whether the tree tracks that route.
- A site with git routes but no slug stops there; a site with no git routes falls to `https://github.com/<slug>/blob/<branch>/<path>` when `site.json` names one, and a site with no git block leaves the link as written.
- Anything else is left exactly as written: a target outside the repo, a paper fetched from another tree, a path nothing publishes.
- So the resolver never asks which page is doing the reading, only which file the link was written in.
- The `/git/` viewer renders in the browser and resolves its own links with `../git/view.ts`, never with this index.
