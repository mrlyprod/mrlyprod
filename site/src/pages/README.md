# PAGES

- Every folder here is one site page, `/<id>/`; `../apps/` holds the apps.
- A page is the unit an app is: `app.json`, `icon.js` and one page of `index.jsx`, `index.md` or a `source`; `../../README.md` has the fields and the build checks.
- A `deep: true` page (`git`, `research`) owns every path under its route and is no scene page.
- A page may import `../shell/` as well as `../lib/`: `missing` and `research` draw `Lost` from `../shell/parts.jsx`, research loads figures through `../shell/figures.js`.
- Page-only code sits in its folder: `git/git.js` (the viewer's client, its own build entry), `stats/stats.js` (reads `/stats/stats.json` every minute).

## MAP

- `about`: prose: one written mark, the mathematics under it, and the code that draws it.
- `blog`: the post list; each `blog/<slug>/` holds a post's `index.md` and `icon.js`.
- `cart`: coming soon.
- `contact`: prose: where to write, and every account the company keeps.
- `donate`: prose: a donation keeps everything free.
- `figures`: hidden: every live figure, drawn with its draw time.
- `git`: Code: every tracked file of mrlyprod, browsable, the raw bytes under `/raw/`.
- `home`: `/`, a bare page; the chrome plays the desk behind it.
- `menu`: Play, Read and Tools from each app's `menu`, then the site tree.
- `method`: prose: Carlo directs, Claude writes and computes, every number names its generator.
- `missing`: hidden: the 404 page, "Nothing here".
- `mrlymath`: prose from `crates/mrly-core/src/name/NAMES.md`: a design as one string.
- `redirects`: hidden: the moved urls of `redirects.json`.
- `research`: the topic grid; `/research/<slug>/` opens a topic's README over it.
- `settings`: theme, font, tint and saver, kept in this browser.
- `stats`: the CDN, the Lambdas and the bucket, read every minute.
