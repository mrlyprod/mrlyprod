# SCRIPTS

- Every `bun run` verb of the site, Bun only, run from the repo root.
- `home.ts` names the site: `REPO` the repo root, `HOME` the config folder (`MRLY_ROOT`, else `../../../mrly.net`), `SITE` its `site.json`.
- `build.ts` (`build`): the build and its Bun plugin, `mrly` for research figures, `withSite` for the shell, filled from `site.json`, `savers` for the saver list and the desk; `../../../README.md` has it.
- `design.ts`: the draw. `choose()` takes `MRLY_DESIGN`, else `site.json` `design` (the pin), else a random folder of the shelves (`site.json` `designs`, the site's own design folders relative to the config folder, then `../../../../designs/`; one with an `index.ts`, so never `shared/`) whose design `lacks()` passes; a named design that misses a part fails the build with the parts named. `source()` and `sheets()` are the `site:design` and `site:design*.css` virtual modules; `skinned()` writes the design's `skin` at build into `site:design/tokens.css` (light on `:root`, dark under `prefers-color-scheme` and `[data-theme="dark"]`), so no boot code paints tokens; `runtime()` serves `site:design` to Bun outside the bundle (the build, the tests). The build prints the design it wears: `MRLY_DESIGN=mrly bun run build`.
- `packages.ts`: `PACKAGES` maps each `@mrlyprod/<package>` (`draw`, `shell`, `sdk`, `system`, `design-<name>` for every `designs/*` with a `package.json`, `design-shared` included) to its folder and `exports`; the crates' `@mrlyprod/<unit>` resolve through `../../../../tsconfig.json` paths instead; `where()` resolves a specifier, `packages` is the plugin the `mrly` plugin carries into every bundle, and `design.ts` registers it with Bun so a site's own files and designs, imported outside the bundle, reach the host by name; a design's `css` or `pulldown` entry may name a package file (`@mrlyprod/design-glass/chrome.css`).
- `init.ts` (`init`): `bun run init <dir> [--title Name] [--domain example.com] [--design glass|mrly|classic|blocks]` writes a minimal site config folder (a home, a menu, an about page in markdown, a missing page) into a new or empty folder, and links the repo's `node_modules` into a folder outside the repo; `../../../../QUICKSTART.md` has the rest.
- `dev.ts` (`dev`): the dev server on 127.0.0.1:3000.
- `check.ts` (`check`): reports the wasm pin and runs the content gate, `../test/content.test.ts`; `../../../README.md` lists its checks.
- `wasm.ts` (`wasm`): `../../../../scripts/wasm.sh`, then the live units into `<config>/wasm/` and the font book into `../../../../apps/shared/font/font.gen.json`.
- `shots.ts` (`shots`), `film.ts` (`film`): Chrome, below.
- `time.ts` (`time`): every figure drawn cold and warm in a worker; `../../../README.md` has the gate.
- `draw.ts` (`draw`): `bun run draw <name...> | --all [--svg] [--png] [--theme dark|light] [--size px] [--out dir]` draws roster figures without the site, through the `mrly` plugin and the svg or raster pen, in the shell ink of the theme and the `site.json` tint; svg, dark and the figure size by default, into `data/net/hosts/web/scripts/draw/<name>.<theme>.<ext>`, one path a line.
- `play.ts` (`play`): an agent plays the reducer apps from the terminal, no browser; below.
- `vendor.ts` (`vendor`): the fonts and the icons, below.
- `library.ts` (`library`): the Life app's encyclopedia shelves, `../../../../apps/life/library.gen.json`, trimmed from the lab's extract `data/research/automata/lab/rs/encyclopedia/library.json`: per row the recipe name, the stop, the class, the seed family, the generation, the period and the population's last and max, the first 4 rows of every shelf cell; the app draws each row live from its name.
- Modules of the build beside them: `git.ts` (the raw list and `git.json`), `llms.ts` (`llms.txt`), `links.ts` (the links of a built `dist/`, for the content gate); `film/page.ts` is the page `film.ts` drives.

## SHOTS

- `bun run shots [route ...]` serves `<config>/dist/` through the edge rule on port 3335, drives one headless Chrome on port 9335 with a throwaway profile, writes viewport pngs at the `screen` of the `shots` block in `site.json` (width, height, dpr) into `data/net/mrly.net/scripts/shots/latest/` (the data folder follows the config folder), with one `run.json`, and kills Chrome at the end.
- `shots.ts` is the screenshot driver: `main(root, config, dist, answer)` serves any site; `answer` maps a path to a file or a redirect, the edge rule by default; another site calls it by path with its own config and answer.
- Every page sees a pinned clock (2026-01-01 09:41, Europe/Rome) and a seeded `Math.random`, so a route shoots the same picture each run.
- `--count N --every S` shoots N frames S apart (`2s`, `0.5`, `500ms`): a page with a running scene is seeked through `window.world.seek(t)`, t = 0, S, 2S, after it is still, and its frames are marked `world`; any other page, or `SITE_URL`, waits S real seconds and marks them `wall`; N > 1 adds a `-sheet` contact sheet.
- `--screen WxH[@dpr]` overrides the screen; `--run <name>` writes to `shots/<name>/`; `--baseline` writes to `baseline/`, which every later run is compared against.
- `--print` emulates print media; `--nojs` shoots with JavaScript off; `--motion` lets canvases show, hidden on still shots otherwise.
- Every shot waits, up to 15 s, until no `aria-busy` element shows in the viewport (library thumbs and saver previews still painting).
- `--first` holds every script, shoots the first paint (`-first`), lets them run and shoots again (`-after`); `JUMP` marks a scroll that moved.
- `--keys <steps>` types after the act: steps split on commas, a named key is pressed (`Tab`, `Enter`, `Escape`, the arrows and the like), anything else is typed.
- `--walk` reads the routes as hops in one document: it clicks the page's own link to each next route, `back`, `forward` and `reload` move through history, and the run ends on the count of page loads.
- A `<route>@<expr>` shot runs the expression first.
- `SITE_URL=https://mrly.net` shoots the live site through the same flags.

## FILM

- `bun run film <scene.ts> --name <n>` plays one app scene into a contact sheet, or a video with `--video`, under `data/net/hosts/web/scripts/film/<name>/`, the sheet as `<name>.png`.

## PLAY

- A reducer app is any `apps/<id>/engine.ts` that exports `boot`, `moves`, `step` and `read`; `play.ts` finds them by that rule alone.
- `bun run play` prints one JSON line per reducer app: its id, its bots and its verbs, as the page door lists them from `SPEC` and `PAGE.keys`.
- `bun run play <app> [--seed N] [key=value ...]` tidies the knobs through the app's `SPEC` as the url would, boots the state the page would deal for that seed (0 by default), prints `{app, seed, value, read}` and reads JSON lines on stdin.
- One line in, one line out: `{"moves":true}` answers `{moves}`; `{"step":<move>}` plays one of those moves and answers `{read}`; `{"bot":"<name>","n":K}` lets a bot play K steps (1 by default) and answers `{steps, read}`; `{"read":true}` answers `{read}`; `{"seed":N}` and `{"set":{key:value}}` deal again and answer `{app, seed, value, read}`.
- A bad line answers `{"error":"..."}` and the session goes on; a move must equal one that `moves` lists.
- `--bot <name> --until-done [--max N]` plays until no move is left, the bot passes, or N steps (100000 by default), and prints the last read with `bot` and `steps`: `bun run play snake --seed 3 --bot smart --until-done` plays snake to its end.
- Bots: the app's `policy.ts` players named in its `SPEC` (`smart`, `silly`, `greedy`...), plus `first` and `random` everywhere; `smart,random` gives each player of a two-player game its own, by the state's `turn`.
- The wasm units load headless from `crates/*/js/pkg/`, as `../../../test/preload.ts` does; run `bun run wasm` first if one is missing.
- The deal is the scene's own: an engine that exports `deal(value, table)`, the opening the scene deals too (through it or the pieces it joins), boots through it with the table `{ w, h, dpr, lock, seed, rand, units }` on a 1280 by 800 stage; an engine with no `deal` boots from the tidied knobs and the rng's first draw.
- Life also exports `reducer(value)`: a fixed board plays its own reducer, Island and Wallpaper play `../../../../apps/shared/life/run.ts`.
- `WAYS` in `play.ts` holds what is left: Snake's bot (a turn then a tick).

## VENDOR

- `vendor.ts` writes the font folder from the `fonts` block in `site.json` (every Google slice, then the `keep` css verbatim) and `../../../sdk/icons.gen.js` from the `icons` block (the outlined 24px Material SVG of each name it lists, from `google/material-design-icons`); draw one with `<Icon name="close" />`, add one by its Material name there (a `_fill` suffix takes the filled glyph) and run `bun run vendor`, or `bun net/hosts/web/scripts/vendor.ts --icons` for the icons alone.
