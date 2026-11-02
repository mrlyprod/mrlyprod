# DESIGNS

- A design is a look: one folder whose `index.ts` exports a `Design`; the shell's behaviour stays in `../net/shell/`, and only the look lives here.
- The contract is `../net/shell/design.ts`: `Design` is `id`, `title`, `skin` (light and dark tokens), `bar` (`{ light, dark }`, required: the browser's `theme-color` per theme), `css` (the folder's sheets, in order), `measure` (the sizes the code before React reads: window widths, step, panes, and the required `gutter`, `edge` and `thumb`; `dock` and `lens` optional), `parts`, optional `knobs` (the settings the design honours: `icons`, and its `own`, which Settings lists under the design's title), an optional `pulldown` (the module that builds the bar menus; the sdk's plain one when absent) and an optional `start` (what runs once the chrome is up).
- `REQUIRED` parts: `bar`, `window`, `app`, `menu`, `control`, `group`, `dialog`, `prose`, `tile`, `launcher`; `OPTIONAL`: `desk`, `dock`, `widgets`, `lock`, `saver`, `preview` (the design drawing itself small, light or dark, for Settings > Appearance); a design without an optional part gets none of that piece, and Settings hides its knobs.
- A part takes `Props` (data, callbacks and `bind` bags, no style) and draws them; it spreads every bind the shell hands it, which is how the shell finds the element: the shell never reads a design's class. The bar menus go where the design spreads `bind.menus`, in its bar or its window, once.
- `lacks(design)` names the required parts a design misses.
- A design imports `../net/sdk/`, `shared/` and its contract; no design imports another (`../net/test/imports.test.ts`); boxy reaching MrlyFont (`../apps/shared/font/`) is the KNOWN exception, so boxy depends on MrlyProd's font, `@mrlyprod/font` once it leaves this repo.

## THE DRAW

- The build wears one design: `MRLY_DESIGN`, else `site.json` `design` (the pin), else a random folder whose design `lacks()` passes; a named design that misses a part fails the build with the parts named.
- The folders come from the shelves: the site's own (`site.json` `designs`, folders relative to the config folder), then this one; a site's design takes the name first, and imports the host by package (`@mrlyprod/sdk/...`, `@mrlyprod/design-shared/...`); a design's own package is its bare name (`@mrlyprod/modern`).
- `MRLY_DESIGN=boxy bun run build`, from the repo root, builds mrly.net in another design; the build prints the design it wears.
- `../net/hosts/web/scripts/design.ts` is the draw and the `site:design*` virtual modules; `../net/hosts/web/scripts/README.md` has them.

## THE FIVE

- `modern/`: mrly.net's pin, macOS and iPadOS: bubbles over the desk in a plain CSS backdrop blur. Its knobs: `material` (Glass, a blur under a thin veil of `--bg`; Frosted, a heavier blur under a light tint; Solid, opaque; the tokens `--frost` and `--blur`, set per `html[data-material]`), `font` (the Noto faces) and `icons`; reduce transparency (`html[data-clear]`) and more contrast keep it solid. A `[data-lens]` piece (bar, window, dock, widget, menu, panel, pill) wears the material; `start` sets `data-lens="window"` on a page or wide window only.
- The modern bar is two bubbles: the trio (`+` to the Launchpad, the site's mark, the cart ring when `site.json` sets `cart`, then the app's name and its menus in an app) and the tray (date, clock, search).
- The modern lock is home's welcome: the site's word, large, over the saver, fading after 3 s of no mouse; scrolling raises the bar's bubbles; asked for a password, the word stands over one field.
- `boxy/`: box in box, one unit; hashes each app id to a hue in `start`, and writes in MrlyFont.
- `classic/`: plain windows and pixel glyphs (`classic/glyphs.tsx`).
- `bricks/`: toy bricks with studs and cast shadows (`bricks/brick.ts`).
- `terminal/`: a phosphor screen in dark and a teletype page in light: monospace only, framed panes, F-key menus, gauges and banners drawn in ASCII (`terminal/banner.ts`).
- `shared/`: the sheets and fonts every design wears today, `base.css` and `appicons.css` among them, and `kit.ts` (`href(bind)`, the saver clock's `spot`); no `index.ts`, so the draw skips it; `shared/README.md` has them.
- Each draws its own checkbox (a pill switch, a cell, a sunken tick, a pressed brick, `[x]`), centres its own dialogs and draws its own Appearance preview (the `preview` part).
- The liquid glass that was glass's WebGL engine lives on as the `liquid` prototype, `mrlyprod/liquid`.

## TESTS

- `contract.test.tsx` loops over every design folder with shared samples and spy binds: every required part, every part renders, every control kind, every bind the shell needs spread, the bar menus hosted once, the launcher's hits, the sheets' faces, skin and scope.
- A design keeps only what is its own beside it (`modern/modern.test.tsx`, `terminal/terminal.test.tsx`); `../net/shell/design.test.ts` holds `lacks`.
- `bun test designs` from the repo root runs them.
