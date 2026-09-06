# git

- The code viewer every MrlyProd site carries: `/git/` browses the site's own repo, `/raw/` serves its exact bytes.
- The input is always the consumer's own tree, never another repo; a site with no `git` block in `site.json` gets no routes at all.
- `ssg/build.ts` owns the bookkeeping and calls in: `scan()` appends the routes, `render()` and `fingerprint()` dispatch on the kind.

## SITE.JSON

- `git`: `{ root, slug, branch }`. `root` is the repo root relative to the site, `slug` is `owner/name` on GitHub, `branch` defaults to `main`.
- mrly.net declares `{ "root": "../..", "slug": "mrlyprod/mrlyprod", "branch": "main" }`, so the site publishes the repo it lives in.

## TREE

- The file list is `git ls-files` when a `.git` is there, and a plain walk of the tree when it is not, so a checkout and an untarred Lambda see the same paths.
- Every tracked file is routed, dotfiles included; the walk fallback skips `.git .cache .venv __pycache__ node_modules dist target data pkg`.

## ROUTES

- `/git/` and `/git/<dir>/` are listings: the README rendered first when the directory has one, then directories, then files, with sizes and seti icons.
- `/git/<path>` is one file: text as a numbered `<pre>`, markdown through the site's own pipeline, images inline, PDF in an `<embed>`, other binary as a link.
- A file over 1 MB is a raw link only; text over 200 KB drops the line numbers and stays a plain `<pre>`.
- A name with no dot gets `.txt` on both its page and its raw path, because the CloudFront router 301s any extensionless path to a slash.
- `/raw/<path>` is the bytes: text is `text/plain; charset=utf-8`, binary keeps its type by extension, and the type rides on the output so `push.ts` sets the S3 header.
- Every page links to the same path on github.com and to its own raw object.

## NAVIGATOR

- `/git` is one collapsed node in the site tree, never the whole repo; a listing carries its own children and a path bar carries its ancestors.
- The node is added only when the site's own nav has no `/git/` href, so a site may place `{ "name": "Code", "href": "/git/" }` in `site.json` itself.
- Every route below the root is `hidden` and `sitemap`: out of the navigator, on the map, so a crawler reads what a reader has to click to.
- A file route carries two `urls`, its page and its `/raw/` object, and both take the file's own `lastmod`.

## HEAD

- The chrome writes the head, so a page gets its canonical link, its title and its description from `page()` and the module only fills the leaf.
- A file's description is its first non-empty lines, whitespace collapsed, the leading comment marks stripped, clipped to 160 characters.
- A listing's is its child count and then its README read the same way, so a directory says what it holds and what it is.

## HIGHLIGHT

- `code.ts` is the built-in highlighter: Shiki core, the JavaScript regex engine (`forgiving`), no oniguruma and no wasm.
- One highlighter per process, no grammar loaded until a file wants it; a grammar that fails to load is remembered as a miss.
- The theme is `createCssVariablesTheme` with prefix `--code-`, and nothing ships that variable: every token becomes a `tk-*` class, so no output carries a `style` attribute.
- 16 grammars: c css csv html javascript json jsx markdown python rust shellscript toml tsx typescript wgsl yaml. Anything else paints nothing and the escaped text stands.
- `ui/code.css` colours the classes from the kit's tokens and sizes the gutter from the `d2`-`d6` class `block()` writes.

## FINGERPRINT

- A file route hashes its bytes and the templates; a listing hashes the sorted `[name, size, kind]` of its children, its README and the templates.
- A file route also hashes the installed Shiki version, so a bump repaints every file and no listing.
- An unchanged file renders nothing on the second build; a directory changes when a child is added, renamed or resized.

## HOOKS

- The module never knows the site's chrome, so `spec.git` carries it: `{ page, md, code }`.
- `page(site, leaf)` wraps a body in the site's page template; `leaf.code` asks it for the seti stylesheet.
- `md(text, dir)` renders markdown the site's way, with the site's math; `link(dir, url)` turns a repo-relative link into a `/git/` route.
- `code(text, lang)` is the highlighter seam: it hands back one HTML string per line, or null to fall back to escaped text.
- A site that passes no `code` gets `code.ts`, so the highlighter is the default and not a chore.
