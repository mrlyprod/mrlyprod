# kit

- mrly.net's own chrome, not a package: `ui/` the design kit, `ssg/` the static builder, `git/` the code viewer.
- No `package.json` here: the site's `bun install` pulls Shiki, the highlighter the code viewer runs server-side, and React resolves from the site's `node_modules`.
- `bun test kit` from the site runs the kit's own tests: the config head script, the pixel font, the screensavers, the builder and the code viewer.
