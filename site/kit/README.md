# KIT

- The builder of mrly.net: one site, one kit, a folder with its own tests, not published.
- Edited in place. The copy in `carlomitchener/site/kit/` is frozen, a UI reference that is never written.

| mode | the page body is made | by | S3 holds |
|---|---|---|---|
| `ssg` | at build | the builder Lambda | one html per page |
| `spa` | in the browser | the page's own JS | one shell and its data |
| `ssr` | at request | a Lambda | nothing |
| dev | at request, on the laptop, all three | `serve()` | nothing |

- `ssg` is the default; `modes` in `site.json` or `mode` on a route picks another.
- `ssr` is reserved: dev serves it, the build refuses it.

## FILES

- `ssg/build.ts` scans, fingerprints, renders and writes the site from `site.json` and the spec in `scripts/site.ts`.
- `ssg/modes.ts` says when each route's page is made and runs the one `spa` `Bun.build`, so every browser entry shares one React.
- `ssg/md.ts` is the markdown pipeline, one parse per text; `ssg/text.ts` holds `escape`, so the browser half imports no builder.
- `ssg/links.ts` resolves every markdown link; `ssg/pic.ts` draws a dark and light picture pair; `ssg/blog.ts` is the blog.
- `git/` is the code viewer: `git.ts` its build half, `view.ts` draws a page from the tree with no `fs`, `client.ts` runs it in the browser, `code.ts` highlights.
- `code/` is the viewer's skin: `code.css` and the SETI icon font, reading the site's own tokens.
- `palette.css` is the fifteen generated colours as CSS variables; it leads every joined stylesheet.
- `theme/` is the same colours in JS: `palette.js` the fifteen as an object, `theme.js` the two role maps, `dark` and `light`.
- `font/` is the pixel font that writes the wordmark: `font.js` lays out, writes, folds and plays a text, `font.json` the glyph book `mrlyrs::font` generates.
- `s3.ts` is the S3 client `push.ts`, `lambda.ts` and `scripts/pkg.ts` share: names and credentials from the environment, no SDK.

## TOOLS

- `push.ts` ships the built site to its bucket by manifest diff; the `push` block in `site.json` names the prefix, the guarded paths, the bucket env keys, the manifest store and the immutable rule.
- `--dry` lists every hashed path, `--force` repaints every route, and `DRY=1` holds the manifest on disk instead of S3.
- `lambda.ts` is the builder Lambda: the event, the head, the GitHub ancestor check and ETag poll, the `/opt/node` modules layer, the build and the push, over the config object `aws/net.ts` passes in.
- A `manual` event with `force: true` skips the seen and unchanged answers and runs the push with `--force`; any other source ignores the field.
- `serve.ts` is the request seam: `serve(spec, site, path)` finds the route that publishes a path, renders it and answers a `Response`, the 404 page for anything else, and never a file read by path.
- A path under a deep `spa` prefix answers that prefix's shell, and a file of the `spa` build answers from the bundle, built only when a path an entry lands on is asked for.
- `dev.ts` is the dev server over `serve.ts`: it binds 127.0.0.1, renders on request, keeps the Bun HTML routes for the React entries and builds the script entries on demand.
- It watches every declared input, the modules of the stamp and this folder, pushes `css` or `reload` over one socket, and injects the overlay into every HTML answer, never into `dist/`.
- Under `bun --hot` a script edit re-runs the entry with fresh modules while the socket stays up.
- `shots.ts` is the screenshot driver: it serves the built `dist/`, a deep shell for every path under its prefix, drives one headless Chrome on one port with one throwaway profile, and waits out any `aria-busy`.
- The `shots` block in `site.json` names the default routes and the sizes; shots land in `data/<repo>/site/scripts/shots/{latest,baseline}`.
- `vendor.ts` writes the font folder from the `fonts` block in `site.json`: the Google families and their axes, then the `keep` css of the faces the site cuts itself, verbatim.
- `stats/handler.py` is the 15-minute stats Lambda: CloudFront, Lambda and bucket numbers into one JSON key.
- It reads only `STATS_BUCKET`, `STATS_DISTRIBUTION` (empty means no cdn block), `STATS_FUNCTIONS`, `STATS_KEY` and three `label=prefix` count lists: `STATS_SIZES`, `STATS_COUNTS` and `STATS_FOLDERS`.

## RULES

- Every import between kit files is relative and stays inside the kit.
- The kit never imports the chrome: `spec.git.page`, `spec.blog.page` and `spec.blog.md` carry it, and the kit's tests draw through those hooks with no site.
- `bun test ./kit` from `site/` runs every test, each one beside the file it covers.
