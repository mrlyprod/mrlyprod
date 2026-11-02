# SCRIPTS

- Every `bun run` verb of the site, Bun only, run from the repo root.
- `home.ts` names the site: `REPO` the repo root, `HOME` the config folder (`MRLY_ROOT`, else `../../../mrly.net`), `SITE` its `site.json`, `top(dir)` the repo that holds a folder.
- `build.ts` (`build`): the build and its Bun plugin, `mrly` for research figures, `withSite` for the shell, filled from `site.json`, `savers` for the saver list and the desk; `../../../README.md` has it.
- `design.ts`: the draw. `choose()` takes `MRLY_DESIGN`, else `site.json` `design` (the pin), else a random folder of the shelves (`site.json` `designs`, the site's own design folders relative to the config folder, then `../../../../designs/`; one with an `index.ts`, so never `shared/`) whose design `lacks()` passes; a named design that misses a part fails the build with the parts named. `source()` and `sheets()` are the `site:design` and `site:design*.css` virtual modules; `skinned()` writes the design's `skin` at build into `site:design/tokens.css` (light on `:root`, dark under `prefers-color-scheme` and `[data-theme="dark"]`), so no boot code paints tokens; `runtime()` serves `site:design` to Bun outside the bundle (the build, the tests). The build prints the design it wears: `MRLY_DESIGN=boxy bun run build`.
- `packages.ts`: `PACKAGES` maps each `@mrlyprod/<package>` (`draw`, `shell`, `sdk`, `system`, each `designs/*` with a `package.json` under the name it gives: the bare design name, `modern` and the rest, and `design-shared`) to its folder and `exports`; the units' `@mrlyprod/<unit>` resolve through `../../../../tsconfig.json` paths onto the sibling repos instead (`home.ts`: `SIBLINGS`, `UNITS`, `unit()`, `wasm()`); `where()` resolves a specifier, `packages` is the plugin the `mrly` plugin carries into every bundle, and `design.ts` registers it with Bun so a site's own files and designs, imported outside the bundle, reach the host by name; a design's `css` or `pulldown` entry may name a package file (`@mrlyprod/modern/chrome.css`).
- `init.ts` (`init`): `bun run init <dir> [--title Name] [--domain example.com] [--design modern|boxy|classic|bricks|terminal]` writes a minimal site config folder (a home, a menu, an about page in markdown, a missing page) into a new or empty folder, and links the repo's `node_modules` into a folder outside the repo; `../../../../QUICKSTART.md` has the rest.
- `dev.ts` (`dev`): the dev server on 127.0.0.1:3000.
- `content.ts` (`content`): reports the wasm pin and runs the content gate, `../test/content.test.ts`; `../../../README.md` lists its checks. `bun run check` is the full gate, `../../../../scripts/check.sh`.
- `wasm.ts` (`wasm`): the live units from the siblings' `js/pkg/` into `<config>/wasm/`, then the font book (`cargo run --example book` in `../../../../../font`) into `../../../../apps/shared/font/font.gen.json`; `--copy` copies the live units alone; a sibling with no `js/pkg/` stops it with the sibling to run `scripts/wasm.sh` in.
- `shots.ts` (`shots`), `film.ts` (`film`): Chrome, below.
- `time.ts` (`time`): every figure drawn cold and warm in a worker; `../../../README.md` has the gate.
- `draw.ts` (`draw`): `bun run draw <name...> | --all [--svg] [--png] [--theme dark|light] [--size px] [--out dir]` draws roster figures without the site, through the `mrly` plugin and the svg or raster pen, in the shell ink of the theme and the `site.json` tint; svg, dark and the figure size by default, into `data/net/hosts/web/scripts/draw/<name>.<theme>.<ext>`, one path a line.
- `vendor.ts` (`vendor`): the fonts and the icons, below.
- `library.ts` (`library`): the Life app's encyclopedia shelves, `../../../../apps/life/library.gen.json`, trimmed from the lab's extract `../../../../../research/data/automata/lab/rs/encyclopedia/library.json` (the research repo's data folder): per row the recipe name, the stop, the class, the seed family, the generation, the period and the population's last and max, the first 4 rows of every shelf cell; the app draws each row live from its name.
- Modules of the build beside them: `git.ts` (the raw list and `git.json`), `llms.ts` (`llms.txt`), `links.ts` (the links of a built `dist/`, for the content gate); `film/page.ts` is the page `film.ts` drives.

## SHOTS

- `bun run shots [route ...]` serves `<config>/dist/` through the edge rule on 127.0.0.1:3335, drives one headless Chrome on port 9335 with a throwaway profile, writes viewport pngs at the site's `screen` (`site.json`, beside `min`: width, height, dpr) into `data/net/mrly.net/scripts/shots/latest/` (the data folder follows the config folder), with one `run.json`, and kills Chrome at the end, on Ctrl-C and on SIGTERM.
- `shots.ts` is the screenshot driver: `shoot(opts)` serves any site and resolves `{ out, chrome, frames, failures }`; it never reads `process.argv` nor exits; `answer` maps a path to a file or a redirect, the edge rule by default; the CLI and `film.ts` both call it.
- `run.json`: `origin`, `chrome` (`Browser.getVersion`), `locale`, `zone`, `screen`, `unit` (`s`), `failures`, and `frames`, each `{ file, route, n, t, clock, sha, verdict, overflow, noise }`, `t` in seconds.
- Exit 1 on a FAILED, an OVERFLOW or a NOISE; a DIFF is news, unless `--same` asks every shot to match its baseline, when a DIFF or a new shot exits 1 too; the GL driver's performance hints of headless Chrome are not noise.
- Every page sees the locale `en-US`, the zone Europe/Rome, a seeded `Math.random` and a pinned clock, 2026-01-01 09:41 in Rome: on a still shot it never moves; under `--motion` or `--count` it starts there at each page load and runs in wall time.
- A still shot hides every canvas, unless the page exposes `window.world` and every live scene seeks to t = 0; then its canvases show their first frame and the shot is marked `world`.
- `--count N --every S` shoots N frames S seconds apart (`2s`, `0.5`, `500ms`): a page whose every live scene seeks through `window.world.seek(t)` is seeked to t = 0, S, 2S of world time after it is still, and its frames are marked `world`; any other page, a scene already past 0 (`--motion`), or `SITE_URL`, waits S real seconds and marks them `wall`; N > 1 adds a `-sheet` contact sheet.
- `--screen WxH[@dpr]` overrides the screen; `--run <name>` writes to `shots/<name>/`; `--baseline` writes to `baseline/`, which every later run is compared against.
- `--print` emulates print media; `--nojs` shoots with JavaScript off; `--motion` lets canvases run.
- Every shot waits, up to 15 s, until no `aria-busy` element shows in the viewport (library thumbs and saver previews still painting).
- `--first` holds every script, shoots the first paint (`-first`), lets them run and shoots again (`-after`); `JUMP` marks a scroll that moved.
- `--keys <steps>` types after the act: steps split on commas, a named key is pressed (`Tab`, `Enter`, `Escape`, the arrows and the like), anything else is typed.
- `--walk` reads the routes as hops in one document: it clicks the page's own link to each next route, `back`, `forward` and `reload` move through history, and the run ends on the count of page loads; the last shot carries the walk's noise.
- A `<route>@<expr>` shot runs the expression first; an act that throws is a FAILED.
- `SITE_URL=https://mrly.net` shoots the live site through the same flags.

## FILM

- `bun run film <scene.ts> --name <n>` plays one app scene into a contact sheet, or a video with `--video`; it writes the page and its bundle into `data/net/hosts/web/scripts/film/<n>/` (`<MRLY_DIST>-film/<n>/` under `MRLY_DIST`), shoots it through `shoot()` as the run `film-<n>`, and moves the sheet there as `<n>.png`, the video as `<n>.webm`; exit 1 when neither comes back.

## PLAY

- Gone: the reducer apps play through the kernel now; `bun run mrlynet` (`../../../doors/README.md`) boots any app, lists its verbs, the knobs and the engine's own (`mrlyprod.snake.turn`), and its `goose` plays.

## VENDOR

- `vendor.ts` writes the font folder from the `fonts` block in `site.json` (every Google slice, then the `keep` css verbatim) and `../../../sdk/icons.gen.js` from the `icons` block (the outlined 24px Material SVG of each name it lists, from `google/material-design-icons`); draw one with `<Icon name="close" />`, add one by its Material name there (a `_fill` suffix takes the filled glyph) and run `bun run vendor`, or `bun net/hosts/web/scripts/vendor.ts --icons` for the icons alone.
