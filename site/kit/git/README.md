# git

- The kit's code viewer: `/git/` browses the repo the site lives in, `/raw/` serves its exact bytes.
- The input is always this repo's own tree, never another; the `git` block in `site.json` is `{ root, slug, branch }`, `branch` defaulting to `main`.
- `/git/` is one row of kind `git` with `meta.deep`: the router answers every path under it with that row, and the browser draws each file and folder.
- `git.ts` is the build half, with `fs` and `git ls-files`; `view.ts` draws, with neither; `client.ts` runs the page; `code.ts` paints.
- `../../scripts/site.ts` calls `tree()` for the raw list and `forest()` for `git.json`; `../../lib/git.js` calls `start()`.

## TREE

- The file list is `git ls-files` when a `.git` is there, and a plain walk of the tree when it is not, so a checkout and an untarred Lambda see the same paths.
- Every tracked file is listed, dotfiles included, and `scripts/site.ts` drops `research/`; the walk fallback skips `.git .cache .venv __pycache__ node_modules dist target data pkg`.
- `/git.json` is the whole tree: `{ base, name, slug, branch, c }`, a node `{ n, k, i?, s?, c? }` (name, `d` or `f`, seti icon, file size, children), folders first, then by name.
- It sits at the root because the `/git/` row owns every path under `/git/`; the left pane's tree reads the same file.

## ROUTES

- `/git/<path>` is the shell; the router renders the `git` page, which hands `#main` to the viewer and the tree to `#left`.
- `/raw/<path>` is the bytes, under the file's own name, extension or not: the edge passes `/raw/` untouched.
- Text up to 1 MB and every SVG are `text/plain; charset=utf-8`, the rest keeps its type by extension, and the type rides on the output so `push.ts` sets the S3 header.
- `/git/` and every `/raw/` object are on the sitemap.

## VIEWER

- `start({ tree, mount, md, paint, after })` takes the element it draws into, fetches the tree once, draws the path in `location`, and takes over every plain click on a link under `/git/`. It returns a stop: the click and popstate listeners go, a draw under way lands nowhere, and the path retries end.
- Its popstate draws only a path under the tree's base, so a Back that leaves `/git/` is another router's.
- A listing is its path bar, its count of folders and files, then folders with their item counts and files with their sizes and seti icons.
- A file is its path bar with Raw and GitHub links: an image inline, a PDF in an `<embed>`, a file over 1 MB a download link, all with no fetch.
- Text is fetched from `/raw/`: a numbered `<pre>`, markdown through `md`, a binary a download link; text over 200 KB drops the numbers.
- The page draws at once and fills when the bytes land; a listing's README fills above its rows the same way, and `aria-busy` holds until the fill and the paint are done.
- `md` and `paint` are the lazy halves: the site's `md` imports the pipeline, and KaTeX only for a text with a `$`; `paint` imports `code.ts`.
- Each draw sets the title, the canonical link and the sidebar's current node, opening the lazy folders above it; `after(view)` runs after each draw and each fill, and a click mid-load wins over the old draw.
- An unknown path is a not-found view in the shell; a folder asked for without its slash is its listing.

## LINKS

- A README link resolves in the viewer by `link(dir, url, wood)` in `view.ts`, against the README's own folder and the tree, keeping its `#fragment` or `?query`.
- A path is `/git/<path>`, a trailing slash `/git/<dir>/`, an image or a PDF `/raw/<path>`.
- Given the tree, the viewer's `/git.json`, a path missing from it is plain text and a folder is `/git/<dir>/` with or without its slash; `md` unwraps a link, inline or reference, its resolver answers `null`.
- `https:`, `http:`, `mailto:`, `tel:`, `#` and a rooted `/path` pass through; `javascript:`, `data:` and `vbscript:` become `#`.
- The viewer has no route table, so a README link to a page's markdown opens its `/git/` view, not the page.

## HIGHLIGHT

- `code.ts` is the built-in highlighter: Shiki core, the JavaScript regex engine (`forgiving`), no oniguruma and no wasm.
- One highlighter per page, no grammar loaded until a file wants it; a grammar that fails to load is remembered as a miss.
- The theme is `createCssVariablesTheme` with prefix `--code-`, and nothing ships that variable: every token becomes a `tk-*` class, so no output carries a `style` attribute.
- 16 grammars: c css csv html javascript json jsx markdown python rust shellscript toml tsx typescript wgsl yaml. Anything else paints nothing and the escaped text stands.
- The numbered lines show first and the paint lands on them; `kit/code/code.css` colours the classes and sizes the gutter from the `d2`-`d6` class `block()` writes.
