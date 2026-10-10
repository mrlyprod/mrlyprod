# KIT

- The tools of mrly.net: one folder, its own tests, edited in place, not published.
- The copy in `carlomitchener/site/kit/` is frozen, a UI reference that is never written.

## FILES

- `md/md.ts` is the markdown pipeline: GFM, `$` and `$$` handed to the caller's math (KaTeX as MathML in the browser), heading ids, a lone image line naming a figure as a figure host, raw HTML as text, `3*n*(n+1)` kept literal.
- `md/links.ts` is the resolver, `resolve(source, href, index)`: every relative href comes out absolute.
- Resolver order: a denied scheme to `#`; a scheme, `#` or rooted href passes; a source the rows publish to its route; `research/` to its GitHub blob; a folder to `/git/<dir>/`; an image or a pdf to `/raw/`; any other file to `/git/<path>`.
- `md/text.ts` holds `escape`, so a browser module imports no pipeline.
- `edge.js` is the CloudFront function source: one plain file, `decide(uri)` and `handler(event)`, and the `/*MOVED*/{}` mark the redirect table replaces.
- `edge.ts` evaluates `edge.js` with `../redirects.json` pasted in, for dev and shots.
- `git/` is the code viewer; `git/README.md` has it.
- `code/` is the viewer's skin: `code.css` and the SETI icon font, reading the site's tokens.
- `palette.css` is generated: the fifteen colours and a `--<hue>-link` shade per hue per theme, as CSS variables.
- `theme/` is the same colours in JS: `palette.js` the fifteen, `theme.js` the `dark` and `light` role maps and `tinted()`.
- `tinted(roles, { accent, link, theme })` swaps the tint's hue into the blue slot, so every figure follows the tint; in light the slot takes the hue's link shade.
- `font/` is the pixel font that writes the wordmark: `font.js` lays out, writes and folds a text; `font.json` is the glyph book `mrlyrs::font` generates.
- `types.ts` is the one extension table: `kind(path)` answers a content type; `push.ts`, `shots.ts` and dev share it.
- `s3.ts` is the S3 client `push.ts`, `lambda.ts` and `scripts/pkg.ts` share: names and credentials from the environment, no SDK.

## TOOLS

- `push.ts` ships a built site by manifest diff; the `push` block in `site.json` names the prefix, the guarded paths, the bucket env keys and the manifest store.
- A file the build hashed is immutable for a year; every other file is max-age 0 and s-maxage 60; `push` refuses an immutable file with no hash in its name.
- Upload order: hashed files, then the rest, then the shell (`index.html`, `404.html`), each group done before the next; a removed file is kept a week, then deleted.
- `--dry` prints the tiers and writes nothing; `--force` uploads every file; `DRY=1` holds the manifest on local disk instead of S3.
- `lambda.ts` is the builder Lambda: the event, the head, the GitHub ancestor check and ETag poll, the `/opt/node` modules layer, then `bun run push`, over the config `../aws/net.ts` passes in.
- A `manual` event with `force: true` skips the seen and unchanged answers and pushes with `--force`.
- `shots.ts` is the screenshot driver: it serves `dist/` (or `MRLY_DIST`) through `decide()`, drives one headless Chrome on port 9335 with a throwaway profile, and waits out any `aria-busy`.
- A `<route>@<expr>` shot is the viewport as scrolled after the expression; every other shot is the full page.
- `--size <name>` shoots one size; `--print` emulates print media; `--nojs` shoots with JavaScript off.
- `--first` holds every script, shoots the first paint (`-first`), lets them run and shoots again (`-after`); `JUMP` marks a scroll that moved.
- `--keys <steps>` types after the act: steps split on commas, a named key is pressed (`Tab`, `Enter`, `Escape`, the arrows and the like), anything else is typed.
- `--walk` reads the routes as hops in one document: it clicks the page's own link to each next route, `back`, `forward` and `reload` move through history, and the run ends on the count of page loads.
- `--frames <ms,ms,...>` shoots the viewport at each offset; with `--motion` the stills show motion.
- `--baseline` writes to `baseline/` instead of `latest/`.
- `vendor.ts` writes the font folder from the `fonts` block in `site.json`: the Google families and their axes, then the `keep` css verbatim.
- `stats/handler.py` is the stats Lambda: CloudFront, Lambda and bucket numbers into one JSON key, read by `/stats/`.
- It reads `STATS_BUCKET`, `STATS_DISTRIBUTION` (empty means no cdn block), `STATS_FUNCTIONS`, `STATS_KEY` and the `label=prefix` lists `STATS_SIZES`, `STATS_COUNTS` and `STATS_FOLDERS`, each empty when unset.

## RULES

- Every import between kit files is relative and stays inside the kit.
- The kit never imports `ui/`; the site hands it what it draws with.
- `bun test ./kit` from `site/` runs every test, each beside the file it covers.
