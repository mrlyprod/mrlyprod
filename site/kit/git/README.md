# git

- The kit's code viewer, optional: `/git/` browses the repo the site lives in, `/raw/` serves its exact bytes.
- The input is always this repo's own tree, never another; no `git` block in `site.json` means no routes at all.
- `/git/` is one deep `spa` route: one shell answers every path under it, the browser draws the page, and no page is rendered per file or folder.
- `git.ts` is the build half, with `fs` and `git ls-files`; `view.ts` draws, with neither; `client.ts` runs the page; `code.ts` paints.
- `../ssg/build.ts` calls in: `scan()` appends the routes, `fingerprint()` dispatches on the kind, `render()` sends the shell to the `spa` build and a raw route here, `globals()` writes the tree.

## SITE.JSON

- `git`: `{ root, slug, branch }`. `root` is the repo root relative to the site, `slug` is `owner/name` on GitHub, `branch` defaults to `main`.
- `modes` must make `/git/` deep, `"/git/": { "mode": "spa", "deep": true }`; a `git` block without it stops the scan.
- The three Shiki packages `deps.json` names under `git` are needed only by a site with this block; `code.ts` imports them on first paint.

## TREE

- The file list is `git ls-files` when a `.git` is there, and a plain walk of the tree when it is not, so a checkout and an untarred Lambda see the same paths.
- Every tracked file is listed, dotfiles included; the walk fallback skips `.git .cache .venv __pycache__ node_modules dist target data pkg`.
- `/git.json` is the whole tree: `{ base, name, slug, branch, c }`, a node `{ n, k, i?, s?, u?, c? }` (name, `d` or `f`, seti icon, file size, served copy's URL, children), folders first, then by name.
- It sits at the root because a deep shell owns every path under `/git/`; the sidebar reads the same file.

## ROUTES

- `/git/` is the shell: the site's head and chrome, a one-line description of the repo, an empty body, the viewer's one script, the explorer's root with every folder lazy.
- `/raw/<path>` is the bytes, under the file's own name, extension or not: the router passes `/raw/` untouched.
- Text up to 1 MB and every SVG are `text/plain; charset=utf-8`, the rest keeps its type by extension, and the type rides on the output so `push.ts` sets the S3 header.
- A file the site already serves elsewhere gets no `/raw/` object: its node carries `u`, the viewer points at that copy, and the sitemap drops the raw URL.
- `/git/` and every `/raw/` object are on the sitemap; a raw route is `hidden`, so it stays out of the navigator.
- The node `{ "name": "Code", "href": "/git/" }` joins the site tree only when the site's own nav has no `/git/` href.
- `site.ships` answers first, for a file the build publishes byte for byte, then `spec.git.served(site, path)`, so the kit never names an extension or a folder.
- A served file a bundler may have rewritten keeps its `/raw/` copy unless it is binary, huge, an image or a PDF, because only then are the bytes known to match.

## VIEWER

- `start({ tree, mount, md, paint, after })` fetches the tree once, draws the path in `location`, and takes over every plain click on a link under `/git/`.
- A listing is its path bar, its count of folders and files, then folders with their item counts and files with their sizes and seti icons.
- A file is its path bar with Raw and GitHub links: an image inline, a PDF in an `<embed>`, a file over 1 MB a download link, all with no fetch.
- Text is fetched from `/raw/` or `u`: a numbered `<pre>`, markdown through `md`, a binary a download link; text over 200 KB drops the numbers.
- The page draws at once and fills when the bytes land; a listing's README fills above its rows the same way, and `aria-busy` holds until the fill and the paint are done.
- `md` and `paint` are the lazy halves: the site's `md` imports the pipeline, and KaTeX only for a text with a `$`; `paint` imports `code.ts`.
- Each draw sets the title, the canonical link and the sidebar's current node, opening the lazy folders above it; `after(view)` runs after each draw and each fill, and a click mid-load wins over the old draw.
- An unknown path is a not-found view in the shell; a folder asked for without its slash is its listing.

## LINKS

- A README link resolves in the viewer by `link(dir, url)` in `view.ts`, against the README's own folder, keeping its `#fragment` or `?query`.
- A path is `/git/<path>`, a trailing slash `/git/<dir>/`, an image or a PDF `/raw/<path>`.
- `https:`, `http:`, `mailto:`, `tel:`, `#` and a rooted `/path` pass through; `javascript:`, `data:` and `vbscript:` become `#`.
- No index of site routes ships, so a README link to a research file opens its `/git/` view, not its site page.

## HIGHLIGHT

- `code.ts` is the built-in highlighter: Shiki core, the JavaScript regex engine (`forgiving`), no oniguruma and no wasm.
- One highlighter per page, no grammar loaded until a file wants it; a grammar that fails to load is remembered as a miss.
- The theme is `createCssVariablesTheme` with prefix `--code-`, and nothing ships that variable: every token becomes a `tk-*` class, so no output carries a `style` attribute.
- 16 grammars: c css csv html javascript json jsx markdown python rust shellscript toml tsx typescript wgsl yaml. Anything else paints nothing and the escaped text stands.
- The numbered lines show first and the paint lands on them; `kit/code/code.css` colours the classes and sizes the gutter from the `d2`-`d6` class `block()` writes.

## FINGERPRINT

- A raw route hashes its path, size and bytes, the site stamp and any served copy's URL; the shell hashes its route and the stamp, so a template edit, the kit included, repaints it.

## HOOKS

- The module never imports the chrome, so `spec.git` carries it: `{ page, entry, served }`; no `page` means the routes are collected and nothing is rendered.
- `page(site, leaf)` wraps the empty body in the site's page template; `leaf.code` asks for the code viewer's stylesheets, `leaf.tree` is the explorer, `leaf.scripts` the viewer's script.
- `entry` is the site's browser module, resolved against the site root, which calls `start()`; the `/git/` route is born from it, and the site lists it in `spa.entries` too.
- `shell(site, route, shell, spec)` draws the shell through `page`; the site's `spa.page` calls it for the `/git/` route.
- `served(site, path)` is the mirror seam: it hands back the URL the site already serves that repo file at, or null; `site.serves` maps a bundled source file to its published URL and `site.made` holds every path the build has written.
