# PAGES

- Every folder here is one site page, `/<id>/`; `../apps/` holds the apps.
- A page is the unit an app is: `app.json`, `icon.svg`, `icon.js` and one page of `index.jsx`, `index.md` or a `source`; `../../README.md` has the fields and the build checks.
- A `deep: true` page (`git`, `research`, `settings`) owns every path under its route and is no scene page; the router leaves the links inside it to the page.
- A page may import `../shell/` as well as `../lib/`: `missing` and `research` draw `Lost` from `../shell/parts.jsx`, research loads figures through `../shell/figures.js` and names its title with `named` from `../shell/route.js`.
- Page-only code sits in its folder: `git/git.js` (the viewer's client, its own build entry), `stats/stats.js` (the poll of `/stats/stats.json` every minute, the pure views, series and facts, and `draw`), `settings/` (below).
- `stats/fixture.json` is a sample `stats.json` that `bun run dev` serves with every `at` moved to now; `../../infra/test_stats.py` keeps its keys equal to what the Lambda returns.

## MAP

- `about`: prose: one written mark, the mathematics under it, and the code that draws it.
- `blog`: the post list; each `blog/<slug>/` holds a post's `index.md` and `icon.js`.
- `cart`: coming soon.
- `contact`: prose: where to write, and every account the company keeps.
- `donate`: prose: a donation keeps everything free.
- `figures`: hidden: every live figure, drawn with its draw time.
- `git`: Code, an app: the file tree in the Library, the file or folder in the window with its path in its own toolbar, the raw bytes under `/raw/`.
- `home`: `/`, a bare page with no window; the chrome plays the desk behind it and the lock screen on it.
- `kit`: Developer: every control, the knobs pane, the library, the design chip and browser, the glass pieces and styles, the icon styles, tints, themes and the type and shape scales on one page; its buttons set the real site settings.
- `menu`: the Launchpad over the desk (`../shell/launchpad.jsx`).
- `method`: prose: Carlo directs, Claude writes and computes, every number names its generator.
- `missing`: hidden: the 404 page, "Nothing here".
- `mrlymath`: prose from `crates/mrly-core/src/name/NAMES.md`: a design as one string.
- `redirects`: hidden: the moved urls of `redirects.json`.
- `research`: Research app: the topics in the Library, the grid at `/research/`, a topic's README with its live figures at `/research/<slug>/`.
- `screensavers`: deep: `/screensavers/` shows every saver of `../shell/savers.js` by app, with stills shot one at a time (`screensavers/shots.js`); `/screensavers/<app>/<id>/` plays one live, with Reroll, its value, Open in the app, Use as desktop picture, In shuffle and Run full screen.
- `settings`: Settings, one url a pane: `/settings/<pane>/` for Appearance, Glass, Desktop & Dock, Screen Saver, Accessibility, Privacy & Data and About.
- `stats`: Stats app: the views (CDN, each Lambda, Bucket, Errors) in the Library, one chart on the canvas, refreshed every minute from `/stats/stats.json`.

## SETTINGS

- `../../shell/panes.js`: `PANES` (`id`, `title`, `art` a 5x5 glyph, `hue`, `more` the search words, `href`), shared with the Launchpad search.
- `index.jsx`: the frame: a sidebar (search, More, one row a pane) and the pane; it catches its own links (`depth` grows on each), popstate, the title and the canonical link.
- Desk: two columns, `/settings/` shows Appearance. Phone: `/settings/` is the list, a pane has a back row.
- Search ranks panes by title, `more` and each pane's `FIND`; `/` or Cmd/Ctrl+F focuses it, Enter opens the first hit, Escape clears; arrows walk the list.
- More: Copy settings link (`share`), Export settings (`mrly-settings.json` of `pack()`), Import settings (`unpack`, then reload).
- `/settings/?apply=<code>` asks before it applies a shared link (`unpack(decode(code))`); a bad code says the link is broken.
- `ui.jsx`: the kit every pane draws with: `Pane`, `Section`, `Item`, `Switch`, `Select`, `Segment`, `Tiles`, `Swatches`, `Action`, `Nav`, `Note`.
- `use.js`: `usePref(key)` (a storage key, live) and `useForm()` (`phone` or `desk`, live).
- One file a pane (`<id>.jsx`): `default function Pane({ site })` and `FIND`, the labels search reads; the styles are `../../styles/settings.css`.
