# SYSTEM

- The platform's own routes, one folder a route, `/<id>/`; any site that names `../system` in its `roots` gets them.
- MrlyProd's pages are in `../../org/`, the apps in `../../apps/`.
- A route is the unit an app is: `app.json`, `icon.svg`, `icon.ts` and one page of `index.tsx`, `index.md` or a `source`; `../README.md` has the fields and the build checks.
- A `deep: true` route (`git`, `screensavers`, `settings`) owns every path under its route and is no scene page; the router leaves the links inside it to the page.
- A route may import `../shell/` as well as `../sdk/`: `missing` draws `Lost` from `../shell/parts.tsx`.
- Route-only code sits in its folder: `git/git.ts` (the viewer's client, its own build entry), `stats/stats.ts` (the poll of `/stats/stats.json` every minute, the pure views, series and facts, and `draw`), `settings/` (below).
- `stats/fixture.json` is a sample `stats.json` that `bun run dev` serves with every `at` moved to now; `../hosts/web/infra/test_stats.py` keeps its keys equal to what the Lambda returns.
- `bun test net/system` from the repo root runs their tests.

## MAP

- `git`: Code, an app: the file tree in the Library, the file or folder in the window with its path in its own toolbar, the raw bytes under `/raw/`.
- `home`: `/`, a bare page with no window; the chrome plays the desk behind it and the lock screen on it.
- `kit`: Developer: every control, the knobs pane, the library, the design chip and browser, the design's pieces and styles, the icon styles, tints, themes and the type and shape scales on one page; its buttons set the real site settings.
- `menu`: the Launchpad over the desk (`../shell/launchpad.tsx`).
- `missing`: hidden: the 404 page, "Nothing here".
- `redirects`: hidden: the moved urls of the config's `redirects.json`.
- `screensavers`: deep: `/screensavers/` shows every saver of `../shell/savers.ts` by app, with stills shot one at a time (`screensavers/shots.ts`); `/screensavers/<app>/<id>/` plays one live, with Reroll, its value, Open in the app, Use as desktop picture, In shuffle and Run full screen.
- `settings`: Settings, one url a pane: `/settings/<pane>/` for Appearance, Glass, Desktop & Dock, Screen Saver, Accessibility, Privacy & Data and About.
- `stats`: Stats app: the views (CDN, each Lambda, Bucket, Errors) in the Library, one chart on the canvas, refreshed every minute from `/stats/stats.json`.

## SETTINGS

- `../shell/panes.ts`: `PANES` (`id`, `title`, `art` a 5x5 glyph, `more` the search words, `href`), shared with the Launchpad search; each pane's hue lives in `settings/index.tsx`.
- `index.tsx`: the frame: a sidebar (search, More, one row a pane) and the pane; it catches its own links (`depth` grows on each), popstate, the title and the canonical link.
- Desk: two columns, `/settings/` shows Appearance. Phone: `/settings/` is the list, a pane has a back row.
- Search ranks panes by title, `more` and each pane's `FIND`; `/` or Cmd/Ctrl+F focuses it, Enter opens the first hit, Escape clears; arrows walk the list.
- More: Copy settings link (`share`), Export settings (`<prefix>settings.json` of `pack()`, `mrly-settings.json` on mrly.net), Import settings (`unpack`, then reload).
- `/settings/?apply=<code>` asks before it applies a shared link (`unpack(decode(code))`); a bad code says the link is broken.
- `ui.tsx`: the kit every pane draws with: `Pane`, `Section`, `Item`, `Switch`, `Select`, `Segment`, `Tiles`, `Swatches`, `Action`, `Nav`, `Note`.
- `use.ts`: `usePref(key)` (a storage key, live) and `useForm()` (`phone` or `desk`, live).
- One file a pane (`<id>.tsx`): `default function Pane({ site })` and `FIND`, the labels search reads; the styles are `../../designs/shared/settings.css`.
