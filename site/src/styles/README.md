# STYLES

- The sheets of mrly.net: plain CSS, no CSS-in-JS.
- `page.css` imports `palette.gen.css`, `tokens.css` (the roles), `base.css`, `chrome.css`, `fonts/fonts.css`, `pages.css`.
- `git.css` is the code viewer's sheet: `../lib/git/code.css` and the SETI font.
- `palette.gen.css` is generated outside this repo: the fifteen colours and a `--<hue>-link` shade per hue per theme.

## RULES

- Palette first (`palette.gen.css`, and `../lib/theme/palette.gen.js` for JS), roles second (`tokens.css`), one class per idea, semantic HTML, AA contrast in both themes, 44px targets on coarse pointers, no motion under `prefers-reduced-motion`.
- Light on `:root`, dark twice: `prefers-color-scheme` guarded by `:root:not([data-theme="light"])`, then `:root[data-theme="dark"]`.
- `--art` is the ground of every canvas and figure.
- Tint is a runtime setting: `data-tint` on `html` names a hue; the shell's `<style id="tint">` holds `tintCss(site.tint)` from `../lib/theme/tint.js`, filled by the build and dev, the default as `:root:not([data-tint])` because Bun puts the stylesheet link after it.
- `page.css` is the one sheet the shell links; the router wears `git.css` on `/git/` and `../lib/app/mrly.css` on an app.

## FONTS

- Noto Sans, Serif, Mono, Math and Color Emoji, named in the `fonts` block of `../../site.json`.
- `bun run vendor` downloads every slice Google serves for each face into `fonts/`, each with its `unicode-range`; a browser fetches only the slices a page prints.
- `fonts/keep.css` (MrlyFont) is appended to `fonts/fonts.css` verbatim.
- `<math>` (KaTeX writes MathML only) draws in Noto Sans Math through `--math`.

## TESTS

- `css.test.ts`: no top-level selector repeats with another rule between; the git and app sheets import nothing `page.css` already links; the token rules.
- `fonts.test.ts`: a shipped face draws every character the routes, their sources and the browser code print.
