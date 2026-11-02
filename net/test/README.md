# TEST

- What the tests share, and the suites that span folders; every other test sits beside the file it covers, and the suites that read a built site are in `../hosts/web/test/`.
- `preload.ts` (named in `../../bunfig.toml`) initialises every wasm unit from `../../crates/<unit>/js/pkg`, so a test imports `@mrlyprod/<unit>` with no init line; without the pkg it stops with one line: run `scripts/wasm.sh` first.
- Fixtures: `dom.ts` (a small DOM to mount React into), `fake.ts` (a WebGL2 context, with its own test), `pen.ts` (a canvas pen that logs its calls, for the sequences family).
- `imports.test.ts`: every relative import points down, over `net/shell`, `net/sdk`, `net/system`, `designs`, `apps`, `org` and `draw`; `../README.md` states the rules; its KNOWN list may only shrink.
- `brand.test.ts`: no brand name, domain, prefix, mark, font, tint, social or email anywhere in `net/` but `../mrly.net/` and markdown, beyond a KNOWN list of five that may only shrink; a site's brand belongs in its config folder.
- The other lanes sit beside what they guard: `../shell/clean.test.ts` (no colour, font or px size in a shell file, no exceptions), `../../designs/shared/css.test.ts` (the sheets).
- `bun test net/test` from the repo root runs them.
