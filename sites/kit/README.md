# kit

- The site kit every MrlyProd site wears: `ui/` the design kit, `ssg/` the static builder, `git/` the code viewer.
- Public but unpublished: `private: true`, no npm, no version. A site takes it from this tree, or fetches it by sha.
- `bun install` here pulls only Shiki, the highlighter the code viewer runs server-side.
- React is the consuming site's: `ui/chrome.jsx` imports it and `tsconfig.json` maps it at `../net/node_modules`, so the server renderer holds one copy.
- `bun test` runs the kit's own tests: the config head script, the pixel font, the screensavers, the builder and the code viewer.
