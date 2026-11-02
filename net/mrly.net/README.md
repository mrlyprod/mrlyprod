# MRLY.NET

- mrly.net's site config: the one folder the platform reads to build mrly.net, and the default of `MRLY_ROOT`.
- Every brand name, domain, tint and social of mrly.net lives here, so the brand lane (`../test/brand.test.ts`) skips this folder.
- `bun run build` from the repo root builds it into `dist/`; `../README.md` has the build.

## FILES

- `site.json`: identity (`title`, `wordmark` (the word the mark folds), `root`, `since`, `prefix` (`html[data-prefix]`, the localStorage keys), `mark`, `marks` (`png`, `touch`, `og`), `tint`, `company`, `tagline`, `contact`, `socials`), the routes (`roots`, `menu`, `desk`, `design`, `min`, `tree`; `designs`, the site's own design folders, absent here), the opt-ins (`cart`, the bar's cart; `tiles`, `{ id, title }` of the shell's Tiles saver), and the tool blocks (`shots`, `push`, `git`, `llms`, `fonts`, `icons`); `push` names the env keys of its buckets, `push.wasm` the one holding the wasm copies.
- `roots` are the route folders, relative to this one: `../system`, `../../org`, `../../apps`; `git.root` is the repo, `../..`.
- `launchpad.json`: the Launchpad sections and the default dock; the Launchpad's Export JSON writes it byte for byte.
- `redirects.json`: the moved urls, each `{ "to", "since" }`; the edge answers each with a 301.
- `wasm.lock`: the hash of the wasm copy in S3 the builder pulls (`../hosts/web/infra/README.md`).
- `public/`: copied to the site root: `og.png`, `favicon.svg`, `favicon.png`, `favicon.ico`, `apple-touch-icon.png`; committed images, not drawn at build; `site.json` `mark` and `marks` name them; there is no web manifest, so browsers offer no install.
- `wasm/` and `dist/` are build output, gitignored.
