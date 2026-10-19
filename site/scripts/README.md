# SCRIPTS

- Every `bun run` verb of the site, Bun only, run from `site/`.
- `build.ts` (`build`): the build and its Bun plugin, `mrly` for research figures, `withTint` for the shell, `savers` for the saver list and the desk; `../README.md` has it.
- `dev.ts` (`dev`): the dev server on 127.0.0.1:3000.
- `check.ts` (`check`): reports the wasm pin and runs the content gate, `../test/content.test.ts`; `../README.md` lists its checks.
- `wasm.ts` (`wasm`): `../../scripts/wasm.sh`, then the live units into `../wasm/` and the font book into `../src/lib/font/font.gen.json`.
- `shots.ts` (`shots`), `film.ts` (`film`): Chrome, below.
- `time.ts` (`time`): every figure drawn cold and warm in a worker; `../README.md` has the gate.
- `draw.ts` (`draw`): `bun run draw <name...> | --all [--svg] [--png] [--theme dark|light] [--size px] [--out dir]` draws roster figures without the site, through the `mrly` plugin and the svg or raster pen, in the shell ink of the theme and the `site.json` tint; svg, dark and the figure size by default, into `data/site/scripts/draw/<name>.<theme>.<ext>`, one path a line.
- `vendor.ts` (`vendor`): the fonts and the icons, below.
- Modules of the build beside them: `git.ts` (the raw list and `git.json`), `llms.ts` (`llms.txt`), `links.ts` (the links of a built `dist/`, for the content gate); `film/page.js` is the page `film.ts` drives.

## SHOTS

- `bun run shots [route ...]` serves `dist/` through the edge rule on port 3335, drives one headless Chrome on port 9335 with a throwaway profile, writes full-page pngs at the sizes of the `shots` block in `site.json` into `data/site/scripts/shots/latest/` and kills Chrome at the end.
- `shots.ts` is the screenshot driver: it serves `dist/` (or `MRLY_DIST`) through `decide()` from `../infra/edge.ts`, drives one headless Chrome on port 9335 with a throwaway profile, and waits out any `aria-busy`.
- A `<route>@<expr>` shot is the viewport as scrolled after the expression; every other shot is the full page.
- `--size <name>` shoots one size; `--print` emulates print media; `--nojs` shoots with JavaScript off.
- `--first` holds every script, shoots the first paint (`-first`), lets them run and shoots again (`-after`); `JUMP` marks a scroll that moved.
- `--keys <steps>` types after the act: steps split on commas, a named key is pressed (`Tab`, `Enter`, `Escape`, the arrows and the like), anything else is typed.
- `--walk` reads the routes as hops in one document: it clicks the page's own link to each next route, `back`, `forward` and `reload` move through history, and the run ends on the count of page loads.
- `--frames <ms,ms,...>` shoots the viewport at each offset; with `--motion` the stills show motion.
- `--baseline` writes to `baseline/` instead of `latest/`.

## FILM

- `bun run film <scene.js> --name <n>` plays one app scene into a contact sheet, or a video with `--video`, under `data/site/scripts/film/<name>/`.

## VENDOR

- `vendor.ts` writes the font folder from the `fonts` block in `site.json` (every Google slice, then the `keep` css verbatim) and `../src/lib/icons.gen.js` from the `icons` block (the outlined 24px Material SVG of each name it lists, from `google/material-design-icons`); draw one with `<Icon name="close" />`, add one by its Material name there (a `_fill` suffix takes the filled glyph) and run `bun run vendor`, or `bun scripts/vendor.ts --icons` for the icons alone.
