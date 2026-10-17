# TEST

- What the tests share, and the suites that span routes; every other test sits beside the file it covers.
- `preload.ts` (named in `../bunfig.toml`) initialises every wasm unit from `../../bindings/js/pkg`, so a test imports `@mrlyprod/mrly/<unit>` with no init line.
- Fixtures: `dom.js` (a small DOM to mount React into), `fake.js` (a WebGL2 context, with its own test), `pen.js` (a canvas pen that logs its calls, for the sequences family).
- `routes.test.js`: every folder of `src/pages/` and `src/apps/` is a route, a root word with a title and a line of at most 90 characters.
- `imports.test.js`: the import rules of `src/`, which `../README.md` states.
