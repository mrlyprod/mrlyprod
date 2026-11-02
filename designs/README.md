# DESIGNS

- A design is a look: one folder whose `index.ts` exports a `Design`; the shell's behaviour stays in `../net/shell/`, and only the look lives here.
- The contract is `../net/shell/design.ts`: `Design` is `id`, `title`, `skin` (light and dark tokens), `bar` (`{ light, dark }`, required: the browser's `theme-color` per theme), `css` (the folder's sheets, in order), `measure` (the sizes the code before React reads: window widths, step, panes, and the required `gutter`, `edge` and `thumb`; `dock` and `lens` optional), `parts`, an optional `pulldown` (the module that builds the bar menus) and an optional `start` (what runs once the chrome is up).
- `REQUIRED` parts: `bar`, `window`, `app`, `menu`, `control`, `group`, `dialog`, `prose`, `tile`, `launcher`; `OPTIONAL`: `desk`, `dock`, `widgets`, `library`, `lock`, `saver`; a design without an optional part gets none of that piece.
- A part takes `Props` (data, callbacks and `bind` bags, no style) and draws them; `lacks(design)` names the required parts a design misses.
- A design imports `../net/sdk/`, `shared/` and its contract; no design imports another (`../net/test/imports.test.ts`); glass and mrly reaching `../apps/shared/font/` are KNOWN exceptions.

## THE DRAW

- The build wears one design: `MRLY_DESIGN`, else `site.json` `design` (the pin), else a random folder whose design `lacks()` passes; a named design that misses a part fails the build with the parts named.
- The folders come from the shelves: the site's own (`site.json` `designs`, folders relative to the config folder), then this one; a site's design takes the name first, and imports the host by package (`@mrlyprod/sdk/...`, `@mrlyprod/design-shared/...`).
- `MRLY_DESIGN=mrly bun run build`, from the repo root, builds mrly.net in another design; the build prints the design it wears.
- `../net/hosts/web/scripts/design.ts` is the draw and the `site:design*` virtual modules; `../net/hosts/web/scripts/README.md` has them.

## THE FIVE

- `glass/`: mrly.net's pin: glass bubbles over the desk, lit by its WebGL engine (`glass/engine/`), which `start` loads after the first route; a `[data-lens]` piece is solid until the engine lights it (`data-lit`), and `html[data-glass="off"]` keeps everything solid.
- `mrly/`: hashes each app id to a hue in `start`, and writes in MrlyFont.
- `classic/`: plain windows and pixel glyphs (`classic/glyphs.tsx`).
- `blocks/`: hard ink-on-paper blocks with cast shadows (`blocks/brick.ts`).
- `terminal/`: a phosphor screen in dark and a teletype page in light: monospace only, framed panes, F-key menus, gauges and banners drawn in ASCII (`terminal/banner.ts`).
- `shared/`: the sheets and fonts every design wears today, `base.css` among them; no `index.ts`, so the draw skips it; `shared/README.md` has them.

## TESTS

- Each design has its own test beside it (`glass/glass.test.tsx`, `mrly/mrly.test.tsx`, `classic/classic.test.tsx`, `blocks/blocks.test.tsx`, `terminal/terminal.test.tsx`); `../net/shell/design.test.ts` holds the contract.
- `bun test designs` from the repo root runs them.
