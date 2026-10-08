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
- `palette.css` is generated: the fifteen colours and a `--<hue>-link` shade per hue per theme, as CSS variables; it leads every joined stylesheet.
- `theme/` is the same colours in JS: `palette.js` the fifteen as an object, `theme.js` the two role maps, `dark` and `light`.
- `font/` is the pixel font that writes the wordmark: `font.js` lays out, writes and folds a text, `font.json` the glyph book `mrlyrs::font` generates.
- `types.ts` is the one extension table: `kind(path)` answers a path's content type, the extension lowercased; `serve.ts`, `push.ts`, `dev.ts` and `shots.ts` share it.
- `s3.ts` is the S3 client `push.ts`, `lambda.ts` and `scripts/pkg.ts` share: names and credentials from the environment, no SDK.

## TOOLS

- `push.ts` ships the built site to its bucket by manifest diff; the `push` block in `site.json` names the prefix, the guarded paths, the bucket env keys, the manifest store and the immutable rule.
- It uploads hashed files first, then the other files, then every HTML page, each group done before the next starts, then deletes; `--dry` lists every hashed path and says that order, `--force` repaints every route, and `DRY=1` holds the manifest on disk instead of S3.
- `lambda.ts` is the builder Lambda: the event, the head, the GitHub ancestor check and ETag poll, the `/opt/node` modules layer copied into the checkout, the build and the push, over the config object `aws/net.ts` passes in.
- A `manual` event with `force: true` skips the seen and unchanged answers and runs the push with `--force`; any other source ignores the field.
- `serve.ts` is the request seam: `serve(spec, site, path)` finds the route that publishes a path, renders it and answers a `Response`, the 404 page for anything else, and never a file read by path.
- A path under a deep `spa` prefix answers that prefix's shell, and a file of the `spa` build answers from the bundle, built only when a path an entry lands on is asked for.
- `dev.ts` is the dev server over `serve.ts`: it binds 127.0.0.1, renders on request, keeps the Bun HTML routes for the React entries, answers each through the spec's `spa.page` so it wears the site's page, and builds the script entries on demand.
- It watches every declared input, the modules of the stamp and this folder, pushes `css` or `reload` over one socket, and injects the overlay into every HTML answer, never into `dist/`.
- Under `bun --hot` a script edit re-runs the entry with fresh modules while the socket stays up.
- `shots.ts` is the screenshot driver: it serves the built `dist/` or the folder its caller names, a deep shell for every path under its prefix, drives one headless Chrome on one port with one throwaway profile, and waits out any `aria-busy`.
- `--nojs` shoots with JavaScript off, inline scripts included.
- `--first` holds every script request and shoots the viewport as first painted (`-first`), then lets them run, runs the route's `@<expr>` if it has one, and shoots it again (`-after`); each line prints its scroll offset, `JUMP` when it moved.
- A `<route>@<expr>` shot is the viewport as scrolled; every other shot is the full page.
- `--size <name>` shoots one of the sizes instead of all.
- `--keys <steps>` types after the act and shoots the viewport: steps split on commas, a named key is pressed (`Tab`, `Shift+Tab`, `Enter`, `Escape`, `Space`, `Backspace`, `Delete`, the arrows, `Home`, `End`, `PageUp`, `PageDown`), anything else is typed as text. A comma cannot be typed.
- `--walk` reads the routes as hops inside one document: it loads the first, then clicks the page's own link to each next one with the mouse (`synthetic` when the link is covered or hidden), `back` and `forward` move through history, `reload` loads the page again, a hop's `@<expr>` runs on arrival, and `--js` prints per hop. A hop has landed on the link's address, or on its path once the page has rewritten its own query; a hop to the path it is already on has landed once `<main>` is another node, the document is new, or the dialog it was pressed in has shut. It has also landed when the page opened a `dialog` whose `data-page` is that address; while a dialog is open the next link is looked for inside it and each line names it. `back` and `forward` have moved once the address changed or the document heard a `popstate`. Each hop prints its url, how it got there, the page loads so far and its scroll; the run ends on the count of page loads and one shot of the last hop.
- `--frames <ms,ms,...>` shoots the viewport at each offset after the act and the keys (`-f0`, `-f1`), each line with its real `t=`; with `--motion` the stills show motion.
- The `shots` block in `site.json` names the default routes and the sizes; shots land in `data/<repo>/site/scripts/shots/{latest,baseline}`.
- `vendor.ts` writes the font folder from the `fonts` block in `site.json`: the Google families and their axes, then the `keep` css of the faces the site cuts itself, verbatim.
- `stats/handler.py` is the 15-minute stats Lambda: CloudFront, Lambda and bucket numbers into one JSON key.
- It reads only `STATS_BUCKET`, `STATS_DISTRIBUTION` (empty means no cdn block), `STATS_FUNCTIONS`, `STATS_KEY` and three `label=prefix` count lists: `STATS_SIZES`, `STATS_COUNTS` and `STATS_FOLDERS`.

## RULES

- Every import between kit files is relative and stays inside the kit.
- The kit never imports the chrome: `spec.git.page`, `spec.blog.page` and `spec.blog.md` carry it, and the kit's tests draw through those hooks with no site.
- `bun test ./kit` from `site/` runs every test, each one beside the file it covers.
