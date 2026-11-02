# SCRIPTS

- Every `bun run` verb of the site, Bun only, run from `site/`.
- `build.ts` (`build`): the build and its Bun plugin, `mrly` for research figures, `withSite` for the shell, filled from `site.json`, `savers` for the saver list and the desk; `../README.md` has it.
- `design.ts`: the draw. `choose()` takes `MRLY_DESIGN`, else `site.json` `design` (the pin), else a random folder of `../src/designs/` whose design `lacks()` passes; a named design that misses a part fails the build with the parts named. `source()` and `sheets()` are the `site:design` and `site:design*.css` virtual modules; `skinned()` writes the design's `skin` at build into `site:design/tokens.css` (light on `:root`, dark under `prefers-color-scheme` and `[data-theme="dark"]`), so no boot code paints tokens; `runtime()` serves `site:design` to Bun outside the bundle (the build, the tests). The build prints the design it wears: `MRLY_DESIGN=mrly bun run build`.
- `dev.ts` (`dev`): the dev server on 127.0.0.1:3000.
- `check.ts` (`check`): reports the wasm pin and runs the content gate, `../test/content.test.ts`; `../README.md` lists its checks.
- `wasm.ts` (`wasm`): `../../scripts/wasm.sh`, then the live units into `../wasm/` and the font book into `../src/lib/font/font.gen.json`.
- `shots.ts` (`shots`), `film.ts` (`film`): Chrome, below.
- `time.ts` (`time`): every figure drawn cold and warm in a worker; `../README.md` has the gate.
- `draw.ts` (`draw`): `bun run draw <name...> | --all [--svg] [--png] [--theme dark|light] [--size px] [--out dir]` draws roster figures without the site, through the `mrly` plugin and the svg or raster pen, in the shell ink of the theme and the `site.json` tint; svg, dark and the figure size by default, into `data/site/scripts/draw/<name>.<theme>.<ext>`, one path a line.
- `vendor.ts` (`vendor`): the fonts and the icons, below.
- `library.ts` (`library`): the Life app's encyclopedia shelves, `../src/apps/life/library.gen.json`, trimmed from the lab's extract `../../data/research/automata/lab/rs/encyclopedia/library.json`: per row the recipe name, the stop, the class, the seed family, the generation, the period and the population's last and max, the first 4 rows of every shelf cell; the app draws each row live from its name.
- Modules of the build beside them: `git.ts` (the raw list and `git.json`), `llms.ts` (`llms.txt`), `links.ts` (the links of a built `dist/`, for the content gate); `film/page.ts` is the page `film.ts` drives.

## SHOTS

- `bun run shots [route ...]` serves `dist/` through the edge rule on port 3335, drives one headless Chrome on port 9335 with a throwaway profile, writes viewport pngs at the `screen` of the `shots` block in `site.json` (width, height, dpr) into `data/site/scripts/shots/latest/`, with one `run.json`, and kills Chrome at the end.
- `shots.ts` is the screenshot driver: `main(root, config, dist, answer)` serves any site; `answer` maps a path to a file or a redirect, the edge rule by default; another site calls it by path with its own config and answer.
- Every page sees a pinned clock (2026-01-01 09:41, Europe/Rome) and a seeded `Math.random`, so a route shoots the same picture each run.
- `--count N --every S` shoots N frames S apart (`2s`, `0.5`, `500ms`): a page with a running scene is seeked through `window.world.seek(t)`, t = 0, S, 2S, after it is still, and its frames are marked `world`; any other page, or `SITE_URL`, waits S real seconds and marks them `wall`; N > 1 adds a `-sheet` contact sheet.
- `--screen WxH[@dpr]` overrides the screen; `--run <name>` writes to `shots/<name>/`; `--baseline` writes to `baseline/`, which every later run is compared against.
- `--print` emulates print media; `--nojs` shoots with JavaScript off; `--motion` lets canvases show, hidden on still shots otherwise.
- `--first` holds every script, shoots the first paint (`-first`), lets them run and shoots again (`-after`); `JUMP` marks a scroll that moved.
- `--keys <steps>` types after the act: steps split on commas, a named key is pressed (`Tab`, `Enter`, `Escape`, the arrows and the like), anything else is typed.
- `--walk` reads the routes as hops in one document: it clicks the page's own link to each next route, `back`, `forward` and `reload` move through history, and the run ends on the count of page loads.
- A `<route>@<expr>` shot runs the expression first.
- `SITE_URL=https://mrly.net` shoots the live site through the same flags.

## FILM

- `bun run film <scene.ts> --name <n>` plays one app scene into a contact sheet, or a video with `--video`, under `data/site/scripts/film/<name>/`, the sheet as `<name>.png`.

## VENDOR

- `vendor.ts` writes the font folder from the `fonts` block in `site.json` (every Google slice, then the `keep` css verbatim) and `../src/lib/icons.gen.js` from the `icons` block (the outlined 24px Material SVG of each name it lists, from `google/material-design-icons`); draw one with `<Icon name="close" />`, add one by its Material name there (a `_fill` suffix takes the filled glyph) and run `bun run vendor`, or `bun scripts/vendor.ts --icons` for the icons alone.
