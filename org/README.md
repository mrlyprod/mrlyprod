# ORG

- MrlyProd's pages, one folder a route, `/<id>/`; mrly.net serves them through the `roots` of its `site.json`.
- A page is the unit an app is: `app.json`, `icon.svg`, `icon.ts` and one page of `index.tsx`, `index.md` or a `source`; `../net/README.md` has the fields and the build checks.
- A page may import `../net/sdk/` and `../net/shell/`: research loads figures through `../net/shell/figures.ts`, draws `Lost` from `../net/shell/parts.tsx` and names its title with `named` from `../net/shell/route.ts`.
- `research` is `deep: true`: it owns every path under `/research/`; the router leaves the links inside it to the page.
- `bun test org` from the repo root runs their tests.

## MAP

- `about`: prose: one written mark, the mathematics under it, and the code that draws it.
- `blog`: the post list; each `blog/<slug>/` holds a post's `index.md` and `icon.ts`.
- `cart`: coming soon.
- `contact`: prose: where to write, and every account the company keeps.
- `donate`: prose: a donation keeps everything free.
- `figures`: hidden: every live figure, drawn with its draw time.
- `method`: prose: Carlo directs, Claude writes and computes, every number names its generator.
- `mrlymath`: prose from `../crates/core/src/name/NAMES.md`: a design as one string.
- `research`: Research app: the topics in the Library, the grid at `/research/`, a topic's README with its live figures at `/research/<slug>/`; the topics are `../research/`.
