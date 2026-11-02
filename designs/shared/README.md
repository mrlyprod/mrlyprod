# SHARED

- The sheets every design wears today: plain CSS, no CSS-in-JS; no `index.ts`, so the draw never takes it for a design.
- `palette.gen.css`: generated outside this repo: the fifteen colours and a `--<hue>-link` shade per hue per theme.
- `base.css`: the reset and the base every page wears under a design (box sizing, body, `[hidden]`), on the design's tokens, and the layer stack (`--z-*`) every design shares.
- `appicons.css`: the base of every app icon (the hidden sprite, the lift, glow, rim and shade); a design restyles or hides them in its own sheet.
- `kit.ts`: what parts of every design call: `href(bind)`, the `href` a bind hands over, and `spot(now)`, the minute's place of a saver clock.
- `pages.css`: the pages and the prose; `settings.css`: the Settings frame, its kit and one section a pane; `screensavers.css`: the Screensavers app.
- `git.css`: the code viewer's sheet: `../../net/sdk/git/code.css` and the SETI font.
- `fonts/`: the faces and `fonts.css`.
- `package.json` names the folder `@mrlyprod/design-shared`, so a design outside the repo reaches these files by name.
- Split a sheet into the designs when one design wants its own look.

## THE SHEET

- `../../net/shell/page.css` is the one sheet the shell links; the router wears `git.css` on `/git/` and `../../net/sdk/mrly.css` on an app.
- `page.css` imports, each under `(scripting: enabled)` so a page without JavaScript is raw HTML, `palette.gen.css`, the design's tokens, `base.css`, `appicons.css`, `gate.css` (the logo below the site's `min`), the design's sheets, `fonts/fonts.css`, `pages.css`, `settings.css`, the design's saver sheet, `screensavers.css`.
- The design comes in through three virtual imports: `site:design/tokens.css` and `site:design/saver.css` take the sheet of that name when the design lists one, `tokens.css` then the design's `skin` as `:root` rules, `site:design.css` every other sheet in the design's order; for `modern` that is `tokens.css`, then `chrome.css`, `desk.css`, `appicons.css`, then `saver.css`, in `../modern/`.

## RULES

- Palette first (`palette.gen.css`, and `../../net/sdk/theme/palette.gen.js` for JS), roles second (the design's tokens), one class per idea, semantic HTML, AA contrast in both themes, no motion under `prefers-reduced-motion`.
- Corners come from one concentric scale, set by each design: `--r-window`, `--r-panel`, `--r-control`, `--r-inset` (kbd, inline code, thumbs), `--round` for pills and chips.
- Desktop only: below the site's `min` the shell shows only the logo, so no sheet holds a phone or coarse-pointer rule.
- Light on `:root`, dark twice: `prefers-color-scheme` guarded by `:root:not([data-theme="light"])`, then `:root[data-theme="dark"]`.
- `--art` is the ground of every canvas and figure.
- Tint is a runtime setting: `data-tint` on `html` names a hue; the shell's `<style id="tint">` holds `tintCss(site.tint)` from `../../net/sdk/theme/tint.ts`, filled by the build and dev, the default as `:root:not([data-tint])` because Bun puts the stylesheet link after it.

## FONTS

- Noto Sans, Serif, Mono, Math and Color Emoji, named in the `fonts` block of `site.json`.
- `bun run vendor` downloads every slice Google serves for each face into `fonts/`, each with its `unicode-range`; a browser fetches only the slices a page prints.
- `fonts/keep.css` (MrlyFont) is appended to `fonts/fonts.css` verbatim.
- `<math>` (KaTeX writes MathML only) draws in Noto Sans Math through `--math`.

## TESTS

- `css.test.ts`, over these sheets, the shell's and modern's: no top-level selector repeats with another rule between; the git and app sheets import nothing `page.css` already links; the token rules.
- `fonts.test.ts`: a shipped face draws every character the routes, their sources and the browser code print.
- `bun test designs/shared` from the repo root runs them.
