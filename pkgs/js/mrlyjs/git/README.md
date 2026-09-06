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

## FINGERPRINT

- A file route hashes its bytes and the templates; a listing hashes the sorted `[name, size, kind]` of its children, its README and the templates.
- An unchanged file renders nothing on the second build; a directory changes when a child is added, renamed or resized.

## HOOKS

- The module never knows the site's chrome, so `spec.git` carries it: `{ page, md, code }`.
- `page(site, leaf)` wraps a body in the site's page template; `leaf.code` asks it for the seti stylesheet.
- `md(text, dir)` renders markdown the site's way, with the site's math; `link(dir, url)` turns a repo-relative link into a `/git/` route.
- `code(text, lang)` is the highlighter seam: it hands back one HTML string per line, or null to fall back to escaped text.
